use flac_sys::{FLAC__StreamEncoderInitStatus, FLAC__bool, FLAC__stream_encoder_set_ogg_serial_number, FLAC__stream_encoder_set_verify,
               FLAC__stream_encoder_set_streamable_subset, FLAC__stream_encoder_set_channels, FLAC__stream_encoder_set_bits_per_sample,
               FLAC__stream_encoder_set_sample_rate, FLAC__stream_encoder_set_compression_level, FLAC__stream_encoder_set_blocksize,
               FLAC__stream_encoder_set_do_mid_side_stereo, FLAC__stream_encoder_set_loose_mid_side_stereo, FLAC__stream_encoder_set_apodization,
               FLAC__stream_encoder_set_max_lpc_order, FLAC__stream_encoder_set_qlp_coeff_precision, FLAC__stream_encoder_set_do_qlp_coeff_prec_search,
               FLAC__stream_encoder_set_do_escape_coding, FLAC__stream_encoder_set_do_exhaustive_model_search,
               FLAC__stream_encoder_set_min_residual_partition_order, FLAC__stream_encoder_set_max_residual_partition_order,
               FLAC__stream_encoder_set_rice_parameter_search_dist,
               FLAC__stream_encoder_set_total_samples_estimate /* , FLAC__stream_encoder_set_metadata */, FLAC__stream_encoder_init_stream,
               FLAC__stream_encoder_init_ogg_stream, FLAC__stream_encoder_init_file, FLAC__stream_encoder_init_ogg_file,
               FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_OK};
use self::super::{StreamEncoderContainer, FlacEncoderInitError, WriteWrapper, FlacEncoder, flac_encoder_write_write_callback};
use std::os::raw::{c_long, c_void};
use std::ffi::{CString, CStr};
use std::marker::PhantomData;
use std::convert::TryFrom;
use std::path::Path;
use std::ptr;



/// Wrapper around a FLAC encoder for configuring the output settings.
///
/// `FILE*` constructors unsupported, Write+Seek constructors unsupportable due to https://github.com/rust-lang/rfcs/issues/2035
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct FlacEncoderConfig(pub(super) StreamEncoderContainer);

impl FlacEncoderConfig {
    /// Initialize the encoder instance to encode native FLAC streams.
    ///
    /// This flavor of initialization sets up the encoder to encode to a
    /// native FLAC stream. I/O is performed via callbacks to the client.
    /// For encoding to a plain file via filename or open `FILE*`,
    /// `FLAC__stream_encoder_init_file()` and `FLAC__stream_encoder_init_FILE()`
    /// provide a simpler interface.
    ///
    /// This function should be called after `FLAC__stream_encoder_new()` and
    /// `FLAC__stream_encoder_set_*(`) but before `FLAC__stream_encoder_process()`
    /// or `FLAC__stream_encoder_process_interleaved()`.
    /// initialization succeeded.
    ///
    /// The call to `FLAC__stream_encoder_init_stream()` currently will also
    /// immediately call the write callback several times, once with the `fLaC`
    /// signature, and once for each encoded metadata block.
    pub fn init_write<'out>(self, out: &'out mut WriteWrapper<'out>) -> Result<FlacEncoder<'out>, FlacEncoderInitError> {
        let result = unsafe {
            FLAC__stream_encoder_init_stream((self.0).0,
                                             Some(flac_encoder_write_write_callback),
                                             None,
                                             None,
                                             None,
                                             out as *mut WriteWrapper as *mut c_void)
        };
        self.do_init(result)
    }

    /// Initialize the encoder instance to encode Ogg FLAC streams.
    ///
    /// This flavor of initialization sets up the encoder to encode to a FLAC
    /// stream in an Ogg container.  I/O is performed via callbacks to the
    /// client.  For encoding to a plain file via filename or open `FILE*`,
    /// `FLAC__stream_encoder_init_ogg_file()` and `FLAC__stream_encoder_init_ogg_FILE()`
    /// provide a simpler interface.
    ///
    /// This function should be called after `FLAC__stream_encoder_new()` and
    /// `FLAC__stream_encoder_set_*()` but before `FLAC__stream_encoder_process()`
    /// or `FLAC__stream_encoder_process_interleaved()`.
    /// initialization succeeded.
    ///
    /// The call to `FLAC__stream_encoder_init_ogg_stream()` currently will also
    /// immediately call the write callback several times, once with the `fLaC`
    /// signature, and once for each encoded metadata block.
    pub fn init_write_ogg<'out>(self, out: &'out mut WriteWrapper<'out>) -> Result<FlacEncoder<'out>, FlacEncoderInitError> {
        let result = unsafe {
            FLAC__stream_encoder_init_ogg_stream((self.0).0,
                                                 None,
                                                 Some(flac_encoder_write_write_callback),
                                                 None,
                                                 None,
                                                 None,
                                                 out as *mut WriteWrapper as *mut c_void)
        };
        self.do_init(result)
    }

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
                                     -> Result<FlacEncoder<'static>, FlacEncoderInitError> {
        self.init_file_impl(filename.as_ref())
    }

    fn init_file_impl(self, filename: &Path /* FLAC__StreamEncoderProgressCallback progress_callback, void *client_data */)
                      -> Result<FlacEncoder<'static>, FlacEncoderInitError> {
        let result = unsafe { FLAC__stream_encoder_init_file((self.0).0, FlacEncoderConfig::convert_path(filename).as_ptr(), None, ptr::null_mut()) };
        self.do_init(result)
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
                                         -> Result<FlacEncoder<'static>, FlacEncoderInitError> {
        self.init_file_ogg_impl(filename.as_ref())
    }

    fn init_file_ogg_impl(self, filename: &Path /* FLAC__StreamEncoderProgressCallback progress_callback, void *client_data */)
                          -> Result<FlacEncoder<'static>, FlacEncoderInitError> {
        let result = unsafe { FLAC__stream_encoder_init_ogg_file((self.0).0, FlacEncoderConfig::convert_path(filename).as_ptr(), None, ptr::null_mut()) };
        self.do_init(result)
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
    pub fn init_stdout(self) -> Result<FlacEncoder<'static>, FlacEncoderInitError> {
        let result = unsafe { FLAC__stream_encoder_init_file((self.0).0, ptr::null(), None, ptr::null_mut()) };
        self.do_init(result)
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
    pub fn init_stdout_ogg(self) -> Result<FlacEncoder<'static>, FlacEncoderInitError> {
        let result = unsafe { FLAC__stream_encoder_init_ogg_file((self.0).0, ptr::null(), None, ptr::null_mut()) };
        self.do_init(result)
    }

    fn convert_path(path: &Path) -> CString {
        CString::new(path.to_str().expect("non-UTF-8 filename")).expect("filename has internal NULs")
    }

    fn do_init<'out>(self, init_result: FLAC__StreamEncoderInitStatus) -> Result<FlacEncoder<'out>, FlacEncoderInitError> {
        if init_result == FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_OK {
            Ok(FlacEncoder(self.0, PhantomData))
        } else {
            Err(FlacEncoderInitError::try_from(init_result).unwrap())
        }
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
    pub fn ogg_serial_number(self, serial_number: c_long) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_ogg_serial_number((self.0).0, serial_number) };
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
    pub fn verify(self, value: bool) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_verify((self.0).0, value as FLAC__bool) };
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
    pub fn streamable_subset(self, value: bool) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_streamable_subset((self.0).0, value as FLAC__bool) };
        self
    }

    /// Set the number of channels to be encoded.
    ///
    /// **Default**: `2`
    pub fn channels(self, value: u32) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_channels((self.0).0, value) };
        self
    }

    /// Set the sample resolution of the input to be encoded.
    ///
    /// **Warning**:<br />
    /// Do not feed the encoder data that is wider than the value you
    /// set here or you will generate an invalid stream.
    ///
    /// **Default**: `16`
    pub fn bits_per_sample(self, value: u32) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_bits_per_sample((self.0).0, value) };
        self
    }

    /// Set the sample rate (in Hz) of the input to be encoded.
    ///
    /// **Default**: `44100`
    pub fn sample_rate(self, value: u32) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_sample_rate((self.0).0, value) };
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
    pub fn compression_level(self, value: u32) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_compression_level((self.0).0, value) };
        self
    }

    /// Set the blocksize to use while encoding.
    ///
    /// The number of samples to use per frame. Use `0` to let the encoder
    /// estimate a blocksize; this is usually best.
    ///
    /// **Default**: `0`
    pub fn blocksize(self, value: u32) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_blocksize((self.0).0, value) };
        self
    }

    /// Set to `true` to enable mid-side encoding on stereo input.
    ///
    /// The number of channels must be 2 for this to have any effect.
    /// Set to `false` to use only independent channel coding.
    ///
    /// **Default**: `true`
    pub fn do_mid_side_stereo(self, value: bool) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_do_mid_side_stereo((self.0).0, value as FLAC__bool) };
        self
    }

    /// Set to `true` to enable adaptive switching between mid-side and left-right encoding on stereo input.
    ///
    /// Set to `false` to use exhaustive searching. Setting this to `true` requires
    /// FLAC__stream_encoder_set_do_mid_side_stereo() to also be set to `true` in order to have any effect.
    ///
    /// **Default**: `false`
    pub fn loose_mid_side_stereo(self, value: bool) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_loose_mid_side_stereo((self.0).0, value as FLAC__bool) };
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
    pub fn apodization(self, specification: &CStr) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_apodization((self.0).0, specification.as_ptr()) };
        self
    }

    /// Set the maximum LPC order, or `0` to use only the fixed predictors.
    ///
    /// **Default**: `8`
    pub fn max_lpc_order(self, value: u32) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_max_lpc_order((self.0).0, value) };
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
    pub fn qlp_coeff_precision(self, value: u32) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_qlp_coeff_precision((self.0).0, value) };
        self
    }

    /// Set to `false` to use only the specified quantized linear predictor
    /// coefficient precision, or `true` to search neighboring precision
    /// values and use the best one.
    ///
    /// **Default**: `false`
    pub fn do_qlp_coeff_prec_search(self, value: bool) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_do_qlp_coeff_prec_search((self.0).0, value as FLAC__bool) };
        self
    }

    /// Deprecated. Setting this value has no effect.
    ///
    /// **Default**: `false`
    pub fn do_escape_coding(self, value: bool) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_do_escape_coding((self.0).0, value as FLAC__bool) };
        self
    }

    /// Set to `false` to let the encoder estimate the best model order
    /// based on the residual signal energy, or `true` to force the
    /// encoder to evaluate all order models and select the best.
    ///
    /// **Default**: `false`
    pub fn do_exhaustive_model_search(self, value: bool) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_do_exhaustive_model_search((self.0).0, value as FLAC__bool) };
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
    pub fn min_residual_partition_order(self, value: u32) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_min_residual_partition_order((self.0).0, value) };
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
    pub fn max_residual_partition_order(self, value: u32) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_max_residual_partition_order((self.0).0, value) };
        self
    }

    /// Deprecated. Setting this value has no effect.
    ///
    /// **Default**: `0`
    pub fn rice_parameter_search_dist(self, value: u32) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_rice_parameter_search_dist((self.0).0, value) };
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
    pub fn total_samples_estimate(self, value: u64) -> FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_total_samples_estimate((self.0).0, value) };
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
    pub fn metadata(self) -> FlacEncoderConfig {
        unimplemented!();
        // FLAC__stream_encoder_set_metadata(self.0);
        // self
    }
}
