use self::super::{StreamEncoderContainer, FlacEncoderConfig};
use flac_sys::FLAC__stream_encoder_new;


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
pub struct FlacEncoder(pub(super) StreamEncoderContainer);

impl FlacEncoder {
    /// Create a new stream encoder, in a configuration wrapper, or `None` if one couldn't be allocated.
    pub fn new() -> Option<FlacEncoderConfig> {
        let enc = unsafe { FLAC__stream_encoder_new() };
        if !enc.is_null() {
            Some(FlacEncoderConfig(StreamEncoderContainer(enc)))
        } else {
            None
        }
    }
}
