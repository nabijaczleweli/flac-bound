use flac_sys::{FLAC__StreamEncoder, FLAC__StreamEncoderInitStatus, FLAC__bool, FLAC__stream_encoder_new, FLAC__stream_encoder_delete,
               FLAC__stream_encoder_set_ogg_serial_number, FLAC__stream_encoder_set_verify, FLAC__stream_encoder_set_streamable_subset,
               FLAC__stream_encoder_set_channels, FLAC__stream_encoder_set_bits_per_sample, FLAC__stream_encoder_set_sample_rate,
               FLAC__stream_encoder_set_compression_level, FLAC__stream_encoder_set_blocksize, FLAC__stream_encoder_set_do_mid_side_stereo,
               FLAC__stream_encoder_set_loose_mid_side_stereo, FLAC__stream_encoder_set_apodization, FLAC__stream_encoder_set_max_lpc_order,
               FLAC__stream_encoder_set_qlp_coeff_precision, FLAC__stream_encoder_set_do_qlp_coeff_prec_search, FLAC__stream_encoder_set_do_escape_coding,
               FLAC__stream_encoder_set_do_exhaustive_model_search, FLAC__stream_encoder_set_min_residual_partition_order,
               FLAC__stream_encoder_set_max_residual_partition_order, FLAC__stream_encoder_set_rice_parameter_search_dist,
               FLAC__stream_encoder_set_total_samples_estimate /* , FLAC__stream_encoder_set_metadata */, FLAC__stream_encoder_init_file,
               FLAC__stream_encoder_init_ogg_file, FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_OK};
use std::ffi::{CString, CStr};
use std::convert::TryFrom;
use std::os::raw::c_long;
use std::path::Path;
use std::{mem, ptr};


/// extract this
pub enum FlacEncoderInitError {}

impl TryFrom<FLAC__StreamEncoderInitStatus> for FlacEncoderInitError {
    type Error = ();

    fn try_from(_: FLAC__StreamEncoderInitStatus) -> Result<FlacEncoderInitError, ()> {
        Err(())
    }
}


/// The [stream encoder](https://xiph.org/flac/api/group__flac__stream__encoder.html) can encode to native FLAC,
/// and optionally Ogg FLAC (check FLAC_API_SUPPORTS_OGG_FLAC) streams and files.
///
/// The basic usage of this encoder is as follows:
///   * The program creates an instance of an encoder using
///     `FLAC__stream_encoder_new()`.
///   * The program overrides the default settings using
///     `FLAC__stream_encoder_set_*()` functions. At a minimum, the following
///     functions should be called:
///       * `FLAC__stream_encoder_set_channels()`
///       * `FLAC__stream_encoder_set_bits_per_sample()`
///       * `FLAC__stream_encoder_set_sample_rate()`
///       * `FLAC__stream_encoder_set_ogg_serial_number()` (if encoding to Ogg FLAC)
///       * `FLAC__stream_encoder_set_total_samples_estimate()` (if known)
///   * If the application wants to control the compression level or set its own
///     metadata, then the following should also be called:
///     * `FLAC__stream_encoder_set_compression_level()`
///     * `FLAC__stream_encoder_set_verify()`
///     * `FLAC__stream_encoder_set_metadata()`
///   * The rest of the set functions should only be called if the client needs
///     exact control over how the audio is compressed; thorough understanding
///     of the FLAC format is necessary to achieve good results.
///   * The program initializes the instance to validate the settings and
///     prepare for encoding using
///       * `FLAC__stream_encoder_init_stream()` or `FLAC__stream_encoder_init_FILE()`
///         or `FLAC__stream_encoder_init_file()` for native FLAC
///       * `FLAC__stream_encoder_init_ogg_stream()` or `FLAC__stream_encoder_init_ogg_FILE()`
///         or `FLAC__stream_encoder_init_ogg_file()` for Ogg FLAC
///   * The program calls `FLAC__stream_encoder_process()` or
///     `FLAC__stream_encoder_process_interleaved()` to encode data, which
///     subsequently calls the callbacks when there is encoder data ready
///     to be written.
///   * The program finishes the encoding with `FLAC__stream_encoder_finish()`,
///     which causes the encoder to encode any data still in its input pipe,
///     update the metadata with the final encoding statistics if output
///     seeking is possible, and finally reset the encoder to the
///     uninitialized state.
///   * The instance may be used again or deleted with
///     `FLAC__stream_encoder_delete()`.
///
/// In more detail, the stream encoder functions similarly to the
/// stream decoder, but has fewer
/// callbacks and more options. Typically the client will create a new
/// instance by calling `FLAC__stream_encoder_new()`, then set the necessary
/// parameters with `FLAC__stream_encoder_set_*()`, and initialize it by
/// calling one of the `FLAC__stream_encoder_init_*()` functions.
///
/// Unlike the decoders, the stream encoder has many options that can
/// affect the speed and compression ratio. When setting these parameters
/// you should have some basic knowledge of the format (see the
/// user-level documentation or the formal description). The
/// `FLAC__stream_encoder_set_*()` functions themselves do not validate the
/// values as many are interdependent. `The FLAC__stream_encoder_init_*()`
/// functions will do this, so make sure to pay attention to the state
/// returned by `FLAC__stream_encoder_init_*()` to make sure that it is
/// `FLAC__STREAM_ENCODER_INIT_STATUS_OK`. Any parameters that are not set
/// before `FLAC__stream_encoder_init_*()` will take on the defaults from
/// the constructor.
///
/// There are three initialization functions for native FLAC, one for
/// setting up the encoder to encode FLAC data to the client via
/// callbacks, and two for encoding directly to a file.
///
/// For encoding via callbacks, use `FLAC__stream_encoder_init_stream()`.
/// You must also supply a write callback which will be called anytime
/// there is raw encoded data to write. If the client can seek the output
/// it is best to also supply seek and tell callbacks, as this allows the
/// encoder to go back after encoding is finished to write back
/// information that was collected while encoding, like seek point offsets,
/// frame sizes, etc.
///
/// For encoding directly to a file, use `FLAC__stream_encoder_init_FILE()`
/// or `FLAC__stream_encoder_init_file()`. Then you must only supply a
/// filename or open `FILE*`; the encoder will handle all the callbacks
/// internally. You may also supply a progress callback for periodic
/// notification of the encoding progress.
///
/// There are three similarly-named init functions for encoding to Ogg
/// FLAC streams. Check `FLAC_API_SUPPORTS_OGG_FLAC` to find out if the
/// library has been built with Ogg support.
///
/// The call to `FLAC__stream_encoder_init_*()` currently will also immediately
/// call the write callback several times, once with the `fLaC signature`,
/// and once for each encoded metadata block. Note that for Ogg FLAC
/// encoding you will usually get at least twice the number of callbacks than
/// with native FLAC, one for the Ogg page header and one for the page body.
///
/// After initializing the instance, the client may feed audio data to the
/// encoder in one of two ways:
///
///   * Channel separate, through `FLAC__stream_encoder_process()` - The client
///     will pass an array of pointers to buffers, one for each channel, to
///     the encoder, each of the same length. The samples need not be
///     block-aligned, but each channel should have the same number of samples.
///   * Channel interleaved, through
///     `FLAC__stream_encoder_process_interleaved()` - The client will pass a single
///     pointer to data that is channel-interleaved (i.e. channel0_sample0,
///     channel1_sample0, ... , channelN_sample0, channel0_sample1, ...).
///     Again, the samples need not be block-aligned but they must be
///     sample-aligned, i.e. the first value should be channel0_sample0 and
///     the last value channelN_sampleM.
///
/// Note that for either process call, each sample in the buffers should be a
/// signed integer, right-justified to the resolution set by
/// `FLAC__stream_encoder_set_bits_per_sample()`. For example, if the resolution
/// is 16 bits per sample, the samples should all be in the range [-32768,32767].
///
/// When the client is finished encoding data, it calls
/// `FLAC__stream_encoder_finish()`, which causes the encoder to encode any
/// data still in its input pipe, and call the metadata callback with the
/// final encoding statistics. Then the instance may be deleted with
/// `FLAC__stream_encoder_delete()` or initialized again to encode another
/// stream.
///
/// For programs that write their own metadata, but that do not know the
/// actual metadata until after encoding, it is advantageous to instruct
/// the encoder to write a PADDING block of the correct size, so that
/// instead of rewriting the whole stream after encoding, the program can
/// just overwrite the PADDING block. If only the maximum size of the
/// metadata is known, the program can write a slightly larger padding
/// block, then split it after encoding.
///
/// Make sure you understand how lengths are calculated. All FLAC metadata
/// blocks have a 4 byte header which contains the type and length. This
/// length does not include the 4 bytes of the header. See the format page
/// for the specification of metadata blocks and their lengths.
///
/// **Note**:<br />
/// If you are writing the FLAC data to a file via callbacks, make sure it
/// is open for update (e.g. mode "w+" for stdio streams). This is because
/// after the first encoding pass, the encoder will try to seek back to the
/// beginning of the stream, to the STREAMINFO block, to write some data
/// there. (If using `FLAC__stream_encoder_init*_file()` or
/// `FLAC__stream_encoder_init*_FILE()`, the file is managed internally.)
///
/// **Note**:<br />
/// The "set" functions may only be called when the encoder is in the
/// state `FLAC__STREAM_ENCODER_UNINITIALIZED`, i.e. after
/// `FLAC__stream_encoder_new()` or `FLAC__stream_encoder_finish()`, but
/// before `FLAC__stream_encoder_init_*()`. If this is the case they will
/// return `true`, otherwise `false`.
///
/// **Note**:<br />
/// `FLAC__stream_encoder_finish()` resets all settings to the constructor defaults.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct FlacEncoder(*mut FLAC__StreamEncoder);

impl FlacEncoder {
    /// Create a new stream encoder, in a configuration wrapper, or `None` if one couldn't be allocated.
    pub fn new() -> Option<FlacEncoderConfig> {
        let enc = unsafe { FLAC__stream_encoder_new() };
        if !enc.is_null() {
            Some(FlacEncoderConfig(enc))
        } else {
            None
        }
    }
}

impl Drop for FlacEncoder {
    fn drop(&mut self) {
        drop_stream_encoder(&mut self.0)
    }
}


/// Wrapper around a FLAC encoder for configuring the output settings.
///
/// `FILE*`/stream constructors unsupported as of yet
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct FlacEncoderConfig(*mut FLAC__StreamEncoder);

impl FlacEncoderConfig {
    /// Initialize the encoder instance to encode native FLAC files.
    ///
    /// This flavor of initialization sets up the encoder to encode to a plain
    /// FLAC file. If POSIX fopen() semantics are not sufficient (for example,
    /// with Unicode filenames on Windows), you must use
    /// `FLAC__stream_encoder_init_FILE()`, or `FLAC__stream_encoder_init_stream()`
    /// and provide callbacks for the I/O.
    ///
    /// The file will be opened with `fopen()`.
    pub fn init_file<P: AsRef<Path>>(self, filename: &P /* FLAC__StreamEncoderProgressCallback progress_callback, void *client_data */)
                                     -> Result<FlacEncoder, FlacEncoderInitError> {
        self.init_file_impl(filename.as_ref())
    }

    pub fn init_file_impl(self, filename: &Path /* FLAC__StreamEncoderProgressCallback progress_callback, void *client_data */)
                          -> Result<FlacEncoder, FlacEncoderInitError> {
        self.do_init(unsafe { FLAC__stream_encoder_init_file(self.0, FlacEncoderConfig::convert_path(filename).as_ptr(), None, ptr::null_mut()) })
    }

    /// Initialize the encoder instance to encode Ogg FLAC files.
    ///
    /// This flavor of initialization sets up the encoder to encode to a plain
    /// Ogg FLAC file. If POSIX fopen() semantics are not sufficient (for example,
    /// with Unicode filenames on Windows), you must use
    /// `FLAC__stream_encoder_init_ogg_FILE()`, or `FLAC__stream_encoder_init_ogg_stream()`
    /// and provide callbacks for the I/O.
    ///
    /// The file will be opened with `fopen()`.
    pub fn init_file_ogg<P: AsRef<Path>>(self, filename: &P /* FLAC__StreamEncoderProgressCallback progress_callback, void *client_data */)
                                         -> Result<FlacEncoder, FlacEncoderInitError> {
        self.init_file_impl(filename.as_ref())
    }

    pub fn init_file_ogg_impl(self, filename: &Path /* FLAC__StreamEncoderProgressCallback progress_callback, void *client_data */)
                              -> Result<FlacEncoder, FlacEncoderInitError> {
        self.do_init(unsafe { FLAC__stream_encoder_init_ogg_file(self.0, FlacEncoderConfig::convert_path(filename).as_ptr(), None, ptr::null_mut()) })
    }

    /// Initialize the encoder instance to encode native FLAC files.
    ///
    /// This flavor of initialization sets up the encoder to encode to a plain
    /// FLAC file. If POSIX fopen() semantics are not sufficient (for example,
    /// with Unicode filenames on Windows), you must use
    /// `FLAC__stream_encoder_init_FILE()`, or `FLAC__stream_encoder_init_stream()`
    /// and provide callbacks for the I/O.
    ///
    /// **Note**:  a proper SEEKTABLE cannot be created when encoding to `stdout` since it is not seekable.
    pub fn init_stdout<P: AsRef<Path>>(self) -> Result<FlacEncoder, FlacEncoderInitError> {
        self.do_init(unsafe { FLAC__stream_encoder_init_file(self.0, ptr::null(), None, ptr::null_mut()) })
    }

    /// Initialize the encoder instance to encode Ogg FLAC files.
    ///
    /// This flavor of initialization sets up the encoder to encode to a plain
    /// Ogg FLAC file. If POSIX fopen() semantics are not sufficient (for example,
    /// with Unicode filenames on Windows), you must use
    /// `FLAC__stream_encoder_init_ogg_FILE()`, or `FLAC__stream_encoder_init_ogg_stream()`
    /// and provide callbacks for the I/O.
    ///
    /// **Note**:  a proper SEEKTABLE cannot be created when encoding to `stdout` since it is not seekable.
    pub fn init_stdout_ogg<P: AsRef<Path>>(self) -> Result<FlacEncoder, FlacEncoderInitError> {
        self.do_init(unsafe { FLAC__stream_encoder_init_ogg_file(self.0, ptr::null(), None, ptr::null_mut()) })
    }

    fn convert_path(path: &Path) -> CString {
        CString::new(path.to_str().expect("non-UTF-8 filename")).expect("filename has internal NULs")
    }

    // Note: this function is actually self instead of &self, but this simplifies consumer code,
    //       and the public interfaces are actually self
    fn do_init(&self, init_result: FLAC__StreamEncoderInitStatus) -> Result<FlacEncoder, FlacEncoderInitError> {
        if init_result == FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_OK {
            Ok(FlacEncoder(self.0))
        } else {
            Err(FlacEncoderInitError::try_from(init_result).unwrap())
        }
    }
}

impl Drop for FlacEncoderConfig {
    fn drop(&mut self) {
        drop_stream_encoder(&mut self.0)
    }
}


fn drop_stream_encoder(enc: &mut *mut FLAC__StreamEncoder) {
    let ptr = mem::replace(enc, ptr::null_mut());
    if !ptr.is_null() {
        unsafe { FLAC__stream_encoder_delete(ptr) };
    }
}


impl FlacEncoderConfig {
    /// Set the serial number for the FLAC stream to use in the Ogg container.
    ///
    /// **Note**:<br />
    /// This does not need to be set for native FLAC encoding.
    ///
    /// It is recommended to set a serial number explicitly as the default of '0'
    /// may collide with other streams.
    ///
    /// **Default**: `0`
    pub fn ogg_serial_number(&mut self, serial_number: c_long) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_ogg_serial_number(self.0, serial_number) };
        self
    }

    /// Set the "verify" flag.
    ///
    /// If `true`, the encoder will verify its own
    /// encoded output by feeding it through an internal decoder and comparing
    /// the original signal against the decoded signal. If a mismatch occurs,
    /// the process call will return `false`. Note that this will slow the
    /// encoding process by the extra time required for decoding and comparison.
    ///
    /// **Default**: `false`
    pub fn verify(&mut self, value: bool) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_verify(self.0, value as FLAC__bool) };
        self
    }

    /// Set the Subset flag.
    ///
    /// If `true`, the encoder will comply with the Subset and will check the
    /// settings during `FLAC__stream_encoder_init_*()` to see if all settings
    /// comply. If `false`, the settings may take advantage of the full
    /// range that the format allows.
    ///
    /// Make sure you know what it entails before setting this to `false`.
    ///
    /// **Default**: `true`
    pub fn streamable_subset(&mut self, value: bool) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_streamable_subset(self.0, value as FLAC__bool) };
        self
    }

    /// Set the number of channels to be encoded.
    ///
    /// **Default**: `2`
    pub fn channels(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_channels(self.0, value) };
        self
    }

    /// Set the sample resolution of the input to be encoded.
    ///
    /// **Warning**:<br />
    /// Do not feed the encoder data that is wider than the value you
    /// set here or you will generate an invalid stream.
    ///
    /// **Default**: `16`
    pub fn bits_per_sample(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_bits_per_sample(self.0, value) };
        self
    }

    /// Set the sample rate (in Hz) of the input to be encoded.
    ///
    /// **Default**: `44100`
    pub fn sample_rate(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_sample_rate(self.0, value) };
        self
    }

    /// Set the compression level
    ///
    /// The compression level is roughly proportional to the amount of effort
    /// the encoder expends to compress the file. A higher level usually
    /// means more computation but higher compression. The default level is
    /// suitable for most applications.
    ///
    /// Currently the levels range from `0` (fastest, least compression) to
    /// `8` (slowest, most compression). A value larger than `8` will be
    /// treated as `8`.
    ///
    /// This function automatically calls the following other *`set`*
    /// functions with appropriate values, so the client does not need to
    /// unless it specifically wants to override them:
    ///   * `FLAC__stream_encoder_set_do_mid_side_stereo()`
    ///   * `FLAC__stream_encoder_set_loose_mid_side_stereo()`
    ///   * `FLAC__stream_encoder_set_apodization()`
    ///   * `FLAC__stream_encoder_set_max_lpc_order()`
    ///   * `FLAC__stream_encoder_set_qlp_coeff_precision()`
    ///   * `FLAC__stream_encoder_set_do_qlp_coeff_prec_search()`
    ///   * `FLAC__stream_encoder_set_do_escape_coding()`
    ///   * `FLAC__stream_encoder_set_do_exhaustive_model_search()`
    ///   * `FLAC__stream_encoder_set_min_residual_partition_order()`
    ///   * `FLAC__stream_encoder_set_max_residual_partition_order()`
    ///   * `FLAC__stream_encoder_set_rice_parameter_search_dist()`
    ///
    /// The actual values set for each level are:
    /// <table>
    /// <tr>
    ///  <td><b>level</b></td>
    ///  <td>do mid-side stereo</td>
    ///  <td>loose mid-side stereo</td>
    ///  <td>apodization</td>
    ///  <td>max lpc order</td>
    ///  <td>qlp coeff precision</td>
    ///  <td>qlp coeff prec search</td>
    ///  <td>escape coding</td>
    ///  <td>exhaustive model search</td>
    ///  <td>min residual partition order</td>
    ///  <td>max residual partition order</td>
    ///  <td>rice parameter search dist</td>
    /// </tr>
    /// <tr><td><b>0</b></td> <td>false</td> <td>false</td> <td>tukey(0.5)<td>
    ///     <td>0</td>        <td>0</td>     <td>false</td> <td>false</td>
    ///     <td>false</td>    <td>0</td>     <td>3</td>     <td>0</td></tr>
    /// <tr><td><b>1</b></td> <td>true</td>  <td>true</td>  <td>tukey(0.5)<td>
    ///     <td>0</td>        <td>0</td>     <td>false</td> <td>false</td>
    ///     <td>false</td>    <td>0</td>     <td>3</td>     <td>0</td></tr>
    /// <tr><td><b>2</b></td> <td>true</td>  <td>false</td> <td>tukey(0.5)<td>
    ///     <td>0</td>        <td>0</td>     <td>false</td> <td>false</td>
    ///     <td>false</td>    <td>0</td>     <td>3</td>     <td>0</td></tr>
    /// <tr><td><b>3</b></td> <td>false</td> <td>false</td> <td>tukey(0.5)<td>
    ///     <td>6</td>        <td>0</td>     <td>false</td> <td>false</td>
    ///     <td>false</td>    <td>0</td>     <td>4</td>     <td>0</td></tr>
    /// <tr><td><b>4</b></td> <td>true</td>  <td>true</td>  <td>tukey(0.5)<td>
    ///     <td>8</td>        <td>0</td>     <td>false</td> <td>false</td>
    ///     <td>false</td>    <td>0</td>     <td>4</td>     <td>0</td></tr>
    /// <tr><td><b>5</b></td> <td>true</td>  <td>false</td> <td>tukey(0.5)<td>
    ///     <td>8</td>        <td>0</td>     <td>false</td> <td>false</td>
    ///     <td>false</td>    <td>0</td>     <td>5</td>     <td>0</td></tr>
    /// <tr><td><b>6</b></td> <td>true</td>  <td>false</td> <td>tukey(0.5);partial_tukey(2)<td>
    ///     <td>8</td>        <td>0</td>     <td>false</td> <td>false</td>
    ///     <td>false</td>    <td>0</td>     <td>6</td>     <td>0</td></tr>
    /// <tr><td><b>7</b></td> <td>true</td>  <td>false</td> <td>tukey(0.5);partial_tukey(2)<td>
    ///     <td>12</td>       <td>0</td>     <td>false</td> <td>false</td>
    ///     <td>false</td>    <td>0</td>     <td>6</td>     <td>0</td></tr>
    /// <tr><td><b>8</b></td> <td>true</td>  <td>false</td> <td>tukey(0.5);partial_tukey(2);punchout_tukey(3)</td>
    ///     <td>12</td>       <td>0</td>     <td>false</td> <td>false</td>
    ///     <td>false</td>    <td>0</td>     <td>6</td>     <td>0</td></tr>
    /// </table>
    ///
    /// **Default**: `5`
    pub fn compression_level(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_compression_level(self.0, value) };
        self
    }

    /// Set the blocksize to use while encoding.
    ///
    /// The number of samples to use per frame. Use `0` to let the encoder
    /// estimate a blocksize; this is usually best.
    ///
    /// **Default**: `0`
    pub fn blocksize(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_blocksize(self.0, value) };
        self
    }

    /// Set to `true` to enable mid-side encoding on stereo input.
    ///
    /// The number of channels must be 2 for this to have any effect.
    /// Set to `false` to use only independent channel coding.
    ///
    /// **Default**: `true`
    pub fn do_mid_side_stereo(&mut self, value: bool) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_do_mid_side_stereo(self.0, value as FLAC__bool) };
        self
    }

    /// Set to `true` to enable adaptive switching between mid-side and left-right encoding on stereo input.
    ///
    /// Set to `false` to use exhaustive searching. Setting this to `true` requires
    /// FLAC__stream_encoder_set_do_mid_side_stereo() to also be set to `true` in order to have any effect.
    ///
    /// **Default**: `false`
    pub fn loose_mid_side_stereo(&mut self, value: bool) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_loose_mid_side_stereo(self.0, value as FLAC__bool) };
        self
    }

    /// Sets the apodization function(s) the encoder will use when windowing audio data for LPC analysis.
    ///
    /// The *specification* is a plain ASCII string which specifies exactly
    /// which functions to use. There may be more than one (up to 32),
    /// separated by `';'` characters. Some functions take one or more
    /// comma-separated arguments in parentheses.
    ///
    /// The available functions are `bartlett`, `bartlett_hann`,
    /// `blackman`, `blackman_harris_4term_92db`, `connes`, `flattop`,
    /// `gauss`(STDDEV), `hamming`, `hann`, `kaiser_bessel`, `nuttall`,
    /// `rectangle`, `triangle`, `tukey`(P), `partial_tukey(n[/ov[/P]])`,
    /// `punchout_tukey(n[/ov[/P]])`, `welch`.
    ///
    /// For `gauss(STDDEV)`, STDDEV specifies the standard deviation
    /// (0<STDDEV<=0.5).
    ///
    /// For `tukey(P)`, P specifies the fraction of the window that is
    /// tapered (0<=P<=1). P=0 corresponds to `rectangle` and P=1
    /// corresponds to `hann`.
    ///
    /// Specifying `partial_tukey` or `punchout_tukey` works a little
    /// different. These do not specify a single apodization function, but
    /// a series of them with some overlap. partial_tukey specifies a series
    /// of small windows (all treated separately) while punchout_tukey
    /// specifies a series of windows that have a hole in them. In this way,
    /// the predictor is constructed with only a part of the block, which
    /// helps in case a block consists of dissimilar parts.
    ///
    /// The three parameters that can be specified for the functions are
    /// n, ov and P. n is the number of functions to add, ov is the overlap
    /// of the windows in case of partial_tukey and the overlap in the gaps
    /// in case of punchout_tukey. P is the fraction of the window that is
    /// tapered, like with a regular tukey window. The function can be
    /// specified with only a number, a number and an overlap, or a number
    /// an overlap and a P, for example, partial_tukey(3), partial_tukey(3/0.3)
    /// and partial_tukey(3/0.3/0.5) are all valid. ov should be smaller than 1
    /// and can be negative.
    ///
    /// Example specifications are `"blackman"` or
    /// `"hann;triangle;tukey(0.5);tukey(0.25);tukey(0.125)"`
    ///
    /// Any function that is specified erroneously is silently dropped. Up
    /// to 32 functions are kept, the rest are dropped. If the specification
    /// is empty the encoder defaults to `"tukey(0.5)"`.
    ///
    /// When more than one function is specified, then for every subframe the
    /// encoder will try each of them separately and choose the window that
    /// results in the smallest compressed subframe.
    ///
    /// Note that each function specified causes the encoder to occupy a
    /// floating point array in which to store the window. Also note that the
    /// values of P, STDDEV and ov are locale-specific, so if the comma
    /// separator specified by the locale is a comma, a comma should be used.
    ///
    /// **Default**: `"tukey(0.5)"`
    pub fn apodization(&mut self, specification: &CStr) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_apodization(self.0, specification.as_ptr()) };
        self
    }

    /// Set the maximum LPC order, or `0` to use only the fixed predictors.
    ///
    /// **Default**: `8`
    pub fn max_lpc_order(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_max_lpc_order(self.0, value) };
        self
    }

    /// Set the precision, in bits, of the quantized linear predictor
    /// coefficients, or `0` to let the encoder select it based on the
    /// blocksize.
    ///
    /// **Note**:<br />
    /// In the current implementation, qlp_coeff_precision + bits_per_sample must
    /// be less than 32.
    ///
    /// **Default**: `0`
    pub fn qlp_coeff_precision(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_qlp_coeff_precision(self.0, value) };
        self
    }

    /// Set to `false` to use only the specified quantized linear predictor
    /// coefficient precision, or `true` to search neighboring precision
    /// values and use the best one.
    ///
    /// **Default**: `false`
    pub fn do_qlp_coeff_prec_search(&mut self, value: bool) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_do_qlp_coeff_prec_search(self.0, value as FLAC__bool) };
        self
    }

    /// Deprecated. Setting this value has no effect.
    ///
    /// **Default**: `false`
    pub fn do_escape_coding(&mut self, value: bool) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_do_escape_coding(self.0, value as FLAC__bool) };
        self
    }

    /// Set to `false` to let the encoder estimate the best model order
    /// based on the residual signal energy, or `true` to force the
    /// encoder to evaluate all order models and select the best.
    ///
    /// **Default**: `false`
    pub fn do_exhaustive_model_search(&mut self, value: bool) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_do_exhaustive_model_search(self.0, value as FLAC__bool) };
        self
    }

    /// Set the minimum partition order to search when coding the residual.
    ///
    /// This is used in tandem with
    /// `FLAC__stream_encoder_set_max_residual_partition_order()`.
    ///
    /// The partition order determines the context size in the residual.
    ///
    /// The context size will be approximately `blocksize / (2 ^ order)`.
    ///
    /// Set both min and max values to `0` to force a single context,
    /// whose Rice parameter is based on the residual signal variance.
    /// Otherwise, set a min and max order, and the encoder will search
    /// all orders, using the mean of each context for its Rice parameter,
    /// and use the best.
    ///
    /// **Default**: `0`
    pub fn min_residual_partition_order(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_min_residual_partition_order(self.0, value) };
        self
    }

    /// Set the maximum partition order to search when coding the residual.
    ///
    /// This is used in tandem with
    /// `FLAC__stream_encoder_set_min_residual_partition_order()`.
    ///
    /// The partition order determines the context size in the residual.
    /// The context size will be approximately `blocksize / (2 ^ order)`.
    ///
    /// Set both min and max values to `0` to force a single context,
    /// whose Rice parameter is based on the residual signal variance.
    /// Otherwise, set a min and max order, and the encoder will search
    /// all orders, using the mean of each context for its Rice parameter,
    /// and use the best.
    ///
    /// **Default**: `5`
    pub fn max_residual_partition_order(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_max_residual_partition_order(self.0, value) };
        self
    }

    /// Deprecated. Setting this value has no effect.
    ///
    /// **Default**: `0`
    pub fn rice_parameter_search_dist(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_rice_parameter_search_dist(self.0, value) };
        self
    }

    /// Set an estimate of the total samples that will be encoded.
    ///
    /// This is merely an estimate and may be set to `0` if unknown.
    /// This value will be written to the STREAMINFO block before encoding,
    /// and can remove the need for the caller to rewrite the value later
    /// if the value is known before encoding.
    ///
    /// **Default**: `0`
    pub fn total_samples_estimate(&mut self, value: u64) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_total_samples_estimate(self.0, value) };
        self
    }

    /// Set the metadata blocks to be emitted to the stream before encoding.
    ///
    /// A value of `NULL`, `0` implies no metadata; otherwise, supply an
    /// array of pointers to metadata blocks. The array is non-const since
    /// the encoder may need to change the *is_last* flag inside them, and
    /// in some cases update seek point offsets. Otherwise, the encoder will
    /// not modify or free the blocks. It is up to the caller to free the
    /// metadata blocks after encoding finishes.
    ///
    /// **Note**:<br />
    /// The encoder stores only copies of the pointers in the *metadata* array;
    /// the metadata blocks themselves must survive at least until after
    /// `FLAC__stream_encoder_finish()` returns. Do not free the blocks until then.
    ///
    /// **Note**:<br />
    /// The STREAMINFO block is always written and no STREAMINFO block may
    /// occur in the supplied array.
    ///
    /// **Note**:<br />
    /// By default the encoder does not create a SEEKTABLE. If one is supplied
    /// in the *metadata* array, but the client has specified that it does not
    /// support seeking, then the SEEKTABLE will be written verbatim. However
    /// by itself this is not very useful as the client will not know the stream
    /// offsets for the seekpoints ahead of time. In order to get a proper
    /// seektable the client must support seeking. See next note.
    ///
    /// **Note**:<br />
    /// SEEKTABLE blocks are handled specially. Since you will not know
    /// the values for the seek point stream offsets, you should pass in
    /// a SEEKTABLE 'template', that is, a SEEKTABLE object with the
    /// required sample numbers (or placeholder points), with `0` for the
    /// *frame_samples* and *stream_offset* fields for each point. If the
    /// client has specified that it supports seeking by providing a seek
    /// callback to `FLAC__stream_encoder_init_stream()` or both seek AND read
    /// callback to `FLAC__stream_encoder_init_ogg_stream()` (or by using
    /// `FLAC__stream_encoder_init*_file()` or `FLAC__stream_encoder_init*_FILE()`),
    /// then while it is encoding the encoder will fill the stream offsets in
    /// for you and when encoding is finished, it will seek back and write the
    /// real values into the SEEKTABLE block in the stream. There are helper
    /// routines for manipulating seektable template blocks; see metadata.h:
    /// `FLAC__metadata_object_seektable_template_*()`. If the client does
    /// not support seeking, the SEEKTABLE will have inaccurate offsets which
    /// will slow down or remove the ability to seek in the FLAC stream.
    ///
    /// **Note**:<br />
    /// The encoder instance \b will modify the first `SEEKTABLE` block
    /// as it transforms the template to a valid seektable while encoding,
    /// but it is still up to the caller to free all metadata blocks after
    /// encoding.
    ///
    /// **Note**:<br />
    /// A VORBIS_COMMENT block may be supplied. The vendor string in it
    /// will be ignored. libFLAC will use it's own vendor string. libFLAC
    /// will not modify the passed-in VORBIS_COMMENT's vendor string, it
    /// will simply write it's own into the stream. If no VORBIS_COMMENT
    /// block is present in the *metadata* array, libFLAC will write an
    /// empty one, containing only the vendor string.
    ///
    /// **Note**:<br /> The Ogg FLAC mapping requires that the VORBIS_COMMENT block be
    /// the second metadata block of the stream. The encoder already supplies
    /// the STREAMINFO block automatically. If *metadata* does not contain a
    /// VORBIS_COMMENT block, the encoder will supply that too. Otherwise, if
    /// *metadata* does contain a VORBIS_COMMENT block and it is not the
    /// first, the init function will reorder *metadata* by moving the
    /// VORBIS_COMMENT block to the front; the relative ordering of the other
    /// blocks will remain as they were.
    ///
    /// **Note**:<br />
    /// The Ogg FLAC mapping limits the number of metadata blocks per
    /// stream to `65535`. If *num_blocks* exceeds this the function will
    /// return `false`.
    ///
    /// **Default**: `NULL, 0`
    ///
    /// Requires, that *num_blocks* > 65535 if encoding to Ogg FLAC.
    pub fn metadata(&mut self) -> &mut FlacEncoderConfig {
        unimplemented!();
        // FLAC__stream_encoder_set_metadata(self.0);
        // self
    }
}
