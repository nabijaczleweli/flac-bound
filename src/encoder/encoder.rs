#[cfg(feature = "default")]
use flac_sys::{FLAC__stream_encoder_new, FLAC__stream_encoder_get_state, FLAC__stream_encoder_get_verify_decoder_state, FLAC__stream_encoder_finish,
               FLAC__stream_encoder_process, FLAC__stream_encoder_process_interleaved};

#[cfg(feature = "libflac")]
use libflac_sys::{FLAC__stream_encoder_new, FLAC__stream_encoder_get_state, FLAC__stream_encoder_get_verify_decoder_state, FLAC__stream_encoder_finish,
    FLAC__stream_encoder_process, FLAC__stream_encoder_process_interleaved};

use super::{StreamEncoderContainer, FlacEncoderConfig, FlacEncoderState};
use std::marker::PhantomData;
use std::convert::TryFrom;
use std::os::raw::c_uint;
use std::{mem, ptr};


/// The [stream encoder](https://xiph.org/flac/api/group__flac__stream__encoder.html) can encode to native FLAC,
/// and optionally Ogg FLAC (check FLAC_API_SUPPORTS_OGG_FLAC) streams and files.
///
/// The basic usage of this encoder is as follows:
///   * The program creates an instance of an encoder using
///     [`FlacEncoder::new()`](#method.new).
///   * The program overrides the default settings using functions in
///     [`FlacEncoderConfig`](struct.FlacEncoderConfig.html). At a minimum, the following
///     functions should be called:
///       * [`FlacEncoderConfig::channels()`](struct.FlacEncoderConfig.html#method.channels)
///       * [`FlacEncoderConfig::bits_per_sample()`](struct.FlacEncoderConfig.html#method.bits_per_sample)
///       * [`FlacEncoderConfig::sample_rate()`](struct.FlacEncoderConfig.html#method.sample_rate)
///       * [`FlacEncoderConfig::ogg_serial_number()`](struct.FlacEncoderConfig.html#method.ogg_serial_number)
///         (if encoding to Ogg FLAC)
///       * [`FlacEncoderConfig::total_samples_estimate()`](struct.FlacEncoderConfig.html#method.total_samples_estimate)
///         (if known)
///   * If the application wants to control the compression level or set its own
///     metadata, then the following should also be called:
///     * [`FlacEncoderConfig::compression_level()`](struct.FlacEncoderConfig.html#method.compression_level)
///     * [`FlacEncoderConfig::verify()`](struct.FlacEncoderConfig.html#method.verify)
///     * [`FlacEncoderConfig::metadata()`](struct.FlacEncoderConfig.html#method.metadata)
///   * The rest of the set functions should only be called if the client needs
///     exact control over how the audio is compressed; thorough understanding
///     of the FLAC format is necessary to achieve good results.
///   * The program initializes the instance to validate the settings and
///     prepare for encoding using
///       * [`FlacEncoderConfig::init_write()`](struct.FlacEncoderConfig.html#method.init_write), or
///         [`FlacEncoderConfig::init_file()`](struct.FlacEncoderConfig.html#method.init_file), or
///         [`FlacEncoderConfig::init_stdout()`](struct.FlacEncoderConfig.html#method.init_stdout) for native FLAC
///       * [`FlacEncoderConfig::init_write_ogg()`](struct.FlacEncoderConfig.html#method.init_write_ogg), or
///         [`FlacEncoderConfig::init_file_ogg()`](struct.FlacEncoderConfig.html#method.init_file_ogg), or
///         [`FlacEncoderConfig::init_stdout_ogg()`](struct.FlacEncoderConfig.html#method.init_stdout_ogg) for Ogg FLAC
///   * The program calls [`FlacEncoder::process()`](#method.process) or
///     [`FlacEncoder::process_interleaved()`](#method.process_interleaved) to encode data, which
///     subsequently calls the callbacks when there is encoder data ready
///     to be written.
///   * The program finishes the encoding with [`FlacEncoder::finish()`](#method.finish),
///     which causes the encoder to encode any data still in its input pipe,
///     update the metadata with the final encoding statistics if output
///     seeking is possible, and finally reset the encoder to the
///     uninitialized state.
///     Note: the stream is `finish()`ed when it's dropped, and any potential error is ignored.
///   * The instance may be used again or deleted with
///     [`FlacEncoder::delete()`](#method.delete).
///     Note: the stream is `delete()`ed when it's dropped.
///
/// In more detail, the stream encoder functions similarly to the
/// stream decoder, but has fewer
/// callbacks and more options. Typically the client will create a new
/// instance by calling [`FlacEncoder::new()`](#method.new), then set the necessary
/// parameters with functions on [`FlacEncoderConfig`](struct.FlacEncoderConfig.html), and initialize it by
/// calling one of the [`FlacEncoderConfig::init_*()`](struct.FlacEncoderConfig.html#method.init_write) functions.
///
/// Unlike the decoders, the stream encoder has many options that can
/// affect the speed and compression ratio. When setting these parameters
/// you should have some basic knowledge of the format (see the
/// user-level documentation or the formal description). The functions on
/// [`FlacEncoderConfig`](struct.FlacEncoderConfig.html) themselves do not validate the
/// values as many are interdependent. The [`FlacEncoderConfig::init_*()`](struct.FlacEncoderConfig.html#method.init_write)
/// functions will do this, so make sure to pay attention to the result
/// returned by [`FlacEncoderConfig::init_*()`](struct.FlacEncoderConfig.html#method.init_write) to make sure that it is
/// `Ok()`. Any parameters that are not set
/// before [`FlacEncoderConfig::init_*()`](struct.FlacEncoderConfig.html#method.init_write) will take on the defaults from
/// the constructor.
///
/// There are three initialization functions for native FLAC, one for
/// setting up the encoder to encode FLAC data to the client via
/// a `Write` stream, and two for encoding directly to a file.
///
/// For encoding via a `Write` stream, use [`FlacEncoderConfig::init_write()`](struct.FlacEncoderConfig.html#method.init_write).
/// You must also supply a `std::io::Write` stream which will be called anytime
/// there is raw encoded data to write. The client cannot seek the output due to
/// [RFC 2035](https://github.com/rust-lang/rfcs/issues/2035), so the
/// encoder cannot go back after encoding is finished to write back
/// information that was collected while encoding, like seek point offsets,
/// frame sizes, etc.
///
/// For encoding directly to a file, use [`FlacEncoderConfig::init_file()`](struct.FlacEncoderConfig.html#method.init_file).
/// Then you must only supply a UTF-8 filename; the encoder will handle all the callbacks
/// internally. You may also supply a progress callback for periodic
/// notification of the encoding progress.
///
/// There are three similarly-named init functions for encoding to Ogg
/// FLAC streams.
///
/// The call to [`FlacEncoderConfig::init_*()`](struct.FlacEncoderConfig.html#method.init_write) currently will also immediately
/// call write to the sink several times, once with the `fLaC` signature,
/// and once for each encoded metadata block. Note that for Ogg FLAC
/// encoding you will usually get at least twice the number of callbacks than
/// with native FLAC, one for the Ogg page header and one for the page body.
///
/// After initializing the instance, the client may feed audio data to the
/// encoder in one of two ways:
///
///   * Channel separate, through [`FlacEncoder::process()`](#method.process) - The client
///     will pass an slice of buffer slices, one for each channel, to
///     the encoder, each of the same length. The samples need not be
///     block-aligned, but each channel should have the same number of samples.
///     This function will allocate if the user supplies more than 8 channels.
///   * Channel interleaved, through
///     [`FlacEncoder::process_interleaved()`](#method.process_interleaved) - The client will pass a single
///     slice to data that is channel-interleaved (i.e. `channel0_sample0`,
///     `channel1_sample0`, ... , `channelN_sample0`, `channel0_sample1`, ...).
///     Again, the samples need not be block-aligned but they must be
///     sample-aligned, i.e. the first value should be `channel0_sample0` and
///     the last value `channelN_sampleM`.
///
/// Note that for either process call, each sample in the buffers should be a
/// signed integer, right-justified to the resolution set by
/// [`FlacEncoderConfig::bits_per_sample()`](struct.FlacEncoderConfig.html#method.bits_per_sample).
/// For example, if the resolution is 16 bits per sample, the samples should all be in the range [-32768,32767].
///
/// When the client is finished encoding data, it calls
/// [`FlacEncoder::finish()`](#method.finish), either explicitly or by dropping the encoder,
/// which causes the encoder to encode any
/// data still in its input pipe, and call the metadata callback with the
/// final encoding statistics. Then the instance may be deleted with
/// [`FlacEncoder::delete()`](#method.delete) by dropping the encoder, or initialized again to encode another
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
/// there. (If using [`FlacEncoderConfig::init_file()`](struct.FlacEncoderConfig.html#method.init_file), the file is managed internally.)
///
/// **Note**:<br />
/// [`FlacEncoder::finish()`](#method.finish) resets all settings to the constructor defaults.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct FlacEncoder<'out>(pub(super) StreamEncoderContainer, pub(super) PhantomData<&'out mut ()>);

impl<'out> FlacEncoder<'out> {
    /// Create a new stream encoder, in a configuration wrapper, or `None` if one couldn't be allocated.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Option<FlacEncoderConfig> {
        let enc = unsafe { FLAC__stream_encoder_new() };
        if !enc.is_null() {
            Some(FlacEncoderConfig(StreamEncoderContainer(enc)))
        } else {
            None
        }
    }

    /// Get the current encoder state.
    pub fn state(&self) -> FlacEncoderState {
        FlacEncoderState::try_from(unsafe { FLAC__stream_encoder_get_state((self.0).0) }).unwrap()
    }

    /// Get the state of the verify stream decoder.
    ///
    /// Useful when the stream encoder state is
    /// [`VerifyDecoderError`](enum.FlacEncoderState.html#variant.VerifyDecoderError).
    pub fn verify_decoder_state(&self) -> FlacEncoderState {
        FlacEncoderState::try_from(unsafe { FLAC__stream_encoder_get_verify_decoder_state((self.0).0) }).unwrap()
    }

    /// Submit data for encoding.
    ///
    /// This version allows you to supply the input data via a slice of
    /// slices, each slice consisting of the same amount of samples as the first one,
    /// representing one channel. The samples need not be block-aligned,
    /// but each channel should have the same number of samples. Each sample
    /// should be a signed integer, right-justified to the resolution set by
    /// [`FlacEncoderConfig::bits_per_sample()`](struct.FlacEncoderConfig.html#method.bits_per_sample). For example, if the
    /// resolution is 16 bits per sample, the samples should all be in the
    /// range [-32768,32767].
    ///
    /// For applications where channel order is important, channels must
    /// follow the order as described in the
    /// [frame header](https://xiph.org/flac/format.html#frame_header).
    ///
    /// Requires encoder instance to be in OK state.
    pub fn process(&mut self, buffers: &[&[i32]]) -> Result<(), ()> {
        if buffers.len() <= 8 {
            let mut buffer = [ptr::null(); 8];
            self.process_impl(&mut buffer, buffers)
        } else {
            let mut buffer = vec![ptr::null(); buffers.len()];
            self.process_impl(&mut buffer, buffers)
        }
    }

    fn process_impl(&mut self, buffer: &mut [*const i32], buffers: &[&[i32]]) -> Result<(), ()> {
        let samples = buffers.iter().next().map(|b| b.len()).unwrap_or(0) as c_uint;

        for (pbfr, sbfr) in buffer.iter_mut().zip(buffers) {
            *pbfr = sbfr.as_ptr();
        }

        if unsafe { FLAC__stream_encoder_process((self.0).0, buffer.as_ptr(), samples) } != 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Submit data for encoding.
    ///
    /// This version allows you to supply the input data where the channels
    /// are interleaved into a single array (i.e. `channel0_sample0`,
    /// `channel1_sample0`, ... , `channelN_sample0`, `channel0_sample1`, ...).
    /// The samples need not be block-aligned but they must be
    /// sample-aligned, i.e. the first value should be `channel0_sample0`
    /// and the last value `channelN_sampleM`. Each sample should be a signed
    /// integer, right-justified to the resolution set by
    /// [`FlacEncoderConfig::bits_per_sample()`](struct.FlacEncoderConfig.html#method.bits_per_sample).
    /// For example, if the resolution is 16 bits per sample, the samples should all be in the
    /// range [-32768,32767].
    ///
    /// For applications where channel order is important, channels must
    /// follow the order as described in the
    /// [frame header](https://xiph.org/flac/format.html#frame_header).
    ///
    /// Requires encoder instance to be in OK state.
    pub fn process_interleaved(&mut self, buffer: &[i32], samples_per_channel: u32) -> Result<(), ()> {
        if unsafe { FLAC__stream_encoder_process_interleaved((self.0).0, buffer.as_ptr(), samples_per_channel) } != 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Finish the encoding process.
    ///
    /// Flushes the encoding buffer, releases resources, resets the encoder
    /// settings to their defaults, and returns the encoder state to
    /// `FLAC__STREAM_ENCODER_UNINITIALIZED`. Note that this can generate
    /// one or more write callbacks before returning, and will generate
    /// a metadata callback.
    ///
    /// Note that in the course of processing the last frame, errors can
    /// occur, so the caller should be sure to check the return value to
    /// ensure the file was encoded properly.
    ///
    /// In the event of a prematurely-terminated encode, it is not strictly
    /// necessary to call this immediately before [`FlacEncoder::delete()`](#method.delete)
    /// but it is good practice to match every [`FlacEncoderConfig::init_*()`](struct.FlacEncoderConfig.html#method.init_write)
    /// with a [`FlacEncoder::finish()`](#method.finish).
    ///
    /// This is also called by `drop()`.
    ///
    /// Returns `Err(self)` if an error occurred processing the last frame, or, if verify
    /// mode is set (see [`FlacEncoderConfig::verify()`](struct.FlacEncoderConfig.html#method.verify)), there was a
    /// verify mismatch; else the config wrapper.
    ///
    /// If `Err()`, caller should check the state with [`state()`](#method.state) for more information about the error.
    pub fn finish(mut self) -> Result<FlacEncoderConfig, FlacEncoder<'out>> {
        if unsafe { FLAC__stream_encoder_finish((self.0).0) } != 0 {
            Ok(FlacEncoderConfig(mem::replace(&mut self.0, StreamEncoderContainer(ptr::null_mut()))))
        } else {
            Err(self)
        }
    }
}

impl<'out> Drop for FlacEncoder<'out> {
    fn drop(&mut self) {
        if !(self.0).0.is_null() {
            unsafe { FLAC__stream_encoder_finish((self.0).0) };
        }
    }
}
