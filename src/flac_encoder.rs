use flac_sys::{FLAC__StreamEncoder, FLAC__bool, FLAC__stream_encoder_new, FLAC__stream_encoder_delete, FLAC__stream_encoder_set_ogg_serial_number,
               FLAC__stream_encoder_set_verify, FLAC__stream_encoder_set_streamable_subset, FLAC__stream_encoder_set_channels,
               FLAC__stream_encoder_set_bits_per_sample, FLAC__stream_encoder_set_sample_rate, FLAC__stream_encoder_set_compression_level,
               FLAC__stream_encoder_set_blocksize, FLAC__stream_encoder_set_do_mid_side_stereo, FLAC__stream_encoder_set_loose_mid_side_stereo,
               FLAC__stream_encoder_set_apodization, FLAC__stream_encoder_set_max_lpc_order, FLAC__stream_encoder_set_qlp_coeff_precision,
               FLAC__stream_encoder_set_do_qlp_coeff_prec_search, FLAC__stream_encoder_set_do_escape_coding,
               FLAC__stream_encoder_set_do_exhaustive_model_search, FLAC__stream_encoder_set_min_residual_partition_order,
               FLAC__stream_encoder_set_max_residual_partition_order, FLAC__stream_encoder_set_rice_parameter_search_dist,
               FLAC__stream_encoder_set_total_samples_estimate /* , FLAC__stream_encoder_set_metadata */};
use std::os::raw::c_long;
use std::{mem, ptr};
use std::ffi::CStr;


pub struct FlacEncoder(*mut FLAC__StreamEncoder);

impl FlacEncoder {
    /// Create a new stream encoder instance.  The instance is created with
    ///  default settings; see the individual FLAC__stream_encoder_set_*()
    ///  functions for each setting's default.
    ///
    /// \retval FLAC__StreamEncoder*
    ///    \c NULL if there was an error allocating memory, else the new instance.
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


pub struct FlacEncoderConfig(*mut FLAC__StreamEncoder);

impl Drop for FlacEncoderConfig {
    fn drop(&mut self) {
        drop_stream_encoder(&mut self.0)
    }
}


fn drop_stream_encoder(enc: &mut *mut FLAC__StreamEncoder) {
    let ptr = mem::replace(enc, ptr::null_mut());
    if !ptr.is_null() {
        FLAC__stream_encoder_delete(ptr);
    }
}


impl FlacEncoderConfig {
    /// Set the serial number for the FLAC stream to use in the Ogg container.
    ///
    /// \note
    /// This does not need to be set for native FLAC encoding.
    ///
    /// \note
    /// It is recommended to set a serial number explicitly as the default of '0'
    /// may collide with other streams.
    ///
    /// \default \c 0
    /// \param  encoder        An encoder instance to set.
    /// \param  serial_number  See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn ogg_serial_number(&mut self, serial_number: c_long) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_ogg_serial_number(self.0, serial_number) };
        self
    }

    /// Set the "verify" flag.  If \c true, the encoder will verify it's own
    ///  encoded output by feeding it through an internal decoder and comparing
    ///  the original signal against the decoded signal.  If a mismatch occurs,
    ///  the process call will return \c false.  Note that this will slow the
    ///  encoding process by the extra time required for decoding and comparison.
    ///
    /// \default \c false
    /// \param  encoder  An encoder instance to set.
    /// \param  value    Flag value (see above).
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn verify(&mut self, value: bool) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_verify(self.0, value as FLAC__bool) };
        self
    }

    /// Set the <A HREF="../format.html#subset">Subset</A> flag.  If \c true,
    ///  the encoder will comply with the Subset and will check the
    ///  settings during FLAC__stream_encoder_init_*() to see if all settings
    ///  comply.  If \c false, the settings may take advantage of the full
    ///  range that the format allows.
    ///
    ///  Make sure you know what it entails before setting this to \c false.
    ///
    /// \default \c true
    /// \param  encoder  An encoder instance to set.
    /// \param  value    Flag value (see above).
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn streamable_subset(&mut self, value: bool) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_streamable_subset(self.0, value as FLAC__bool) };
        self
    }

    /// Set the number of channels to be encoded.
    ///
    /// \default \c 2
    /// \param  encoder  An encoder instance to set.
    /// \param  value    See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn channels(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_channels(self.0, value) };
        self
    }

    /// Set the sample resolution of the input to be encoded.
    ///
    /// \warning
    /// Do not feed the encoder data that is wider than the value you
    /// set here or you will generate an invalid stream.
    ///
    /// \default \c 16
    /// \param  encoder  An encoder instance to set.
    /// \param  value    See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn bits_per_sample(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_bits_per_sample(self.0, value) };
        self
    }

    /// Set the sample rate (in Hz) of the input to be encoded.
    ///
    /// \default \c 44100
    /// \param  encoder  An encoder instance to set.
    /// \param  value    See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn sample_rate(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_sample_rate(self.0, value) };
        self
    }

    /// Set the compression level
    ///
    /// The compression level is roughly proportional to the amount of effort
    /// the encoder expends to compress the file.  A higher level usually
    /// means more computation but higher compression.  The default level is
    /// suitable for most applications.
    ///
    /// Currently the levels range from \c 0 (fastest, least compression) to
    /// \c 8 (slowest, most compression).  A value larger than \c 8 will be
    /// treated as \c 8.
    ///
    /// This function automatically calls the following other \c _set_
    /// functions with appropriate values, so the client does not need to
    /// unless it specifically wants to override them:
    /// - FLAC__stream_encoder_set_do_mid_side_stereo()
    /// - FLAC__stream_encoder_set_loose_mid_side_stereo()
    /// - FLAC__stream_encoder_set_apodization()
    /// - FLAC__stream_encoder_set_max_lpc_order()
    /// - FLAC__stream_encoder_set_qlp_coeff_precision()
    /// - FLAC__stream_encoder_set_do_qlp_coeff_prec_search()
    /// - FLAC__stream_encoder_set_do_escape_coding()
    /// - FLAC__stream_encoder_set_do_exhaustive_model_search()
    /// - FLAC__stream_encoder_set_min_residual_partition_order()
    /// - FLAC__stream_encoder_set_max_residual_partition_order()
    /// - FLAC__stream_encoder_set_rice_parameter_search_dist()
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
    /// <tr>  <td><b>0</b></td> <td>false</td> <td>false</td> <td>tukey(0.5)<td>
    ///       <td>0</td>  <td>0</td> <td>false</td> <td>false</td> <td>false</td> <td>0</td> <td>3</td> <td>0</td> </tr>
    /// <tr>  <td><b>1</b></td> <td>true</td>  <td>true</td>  <td>tukey(0.5)<td>
    ///       <td>0</td>  <td>0</td> <td>false</td> <td>false</td> <td>false</td> <td>0</td> <td>3</td> <td>0</td> </tr>
    /// <tr>  <td><b>2</b></td> <td>true</td>  <td>false</td> <td>tukey(0.5)<td>
    ///       <td>0</td>  <td>0</td> <td>false</td> <td>false</td> <td>false</td> <td>0</td> <td>3</td> <td>0</td> </tr>
    /// <tr>  <td><b>3</b></td> <td>false</td> <td>false</td> <td>tukey(0.5)<td>
    ///       <td>6</td>  <td>0</td> <td>false</td> <td>false</td> <td>false</td> <td>0</td> <td>4</td> <td>0</td> </tr>
    /// <tr>  <td><b>4</b></td> <td>true</td>  <td>true</td>  <td>tukey(0.5)<td>
    ///       <td>8</td>  <td>0</td> <td>false</td> <td>false</td> <td>false</td> <td>0</td> <td>4</td> <td>0</td> </tr>
    /// <tr>  <td><b>5</b></td> <td>true</td>  <td>false</td> <td>tukey(0.5)<td>
    ///       <td>8</td>  <td>0</td> <td>false</td> <td>false</td> <td>false</td> <td>0</td> <td>5</td> <td>0</td> </tr>
    /// <tr>  <td><b>6</b></td> <td>true</td>  <td>false</td> <td>tukey(0.5);partial_tukey(2)<td>
    ///       <td>8</td>  <td>0</td> <td>false</td> <td>false</td> <td>false</td> <td>0</td> <td>6</td> <td>0</td> </tr>
    /// <tr>  <td><b>7</b></td> <td>true</td>  <td>false</td> <td>tukey(0.5);partial_tukey(2)<td>
    ///       <td>12</td> <td>0</td> <td>false</td> <td>false</td> <td>false</td> <td>0</td> <td>6</td> <td>0</td> </tr>
    /// <tr>  <td><b>8</b></td> <td>true</td>  <td>false</td> <td>tukey(0.5);partial_tukey(2);punchout_tukey(3)</td>
    ///       <td>12</td> <td>0</td> <td>false</td> <td>false</td> <td>false</td> <td>0</td> <td>6</td> <td>0</td> </tr>
    /// </table>
    ///
    /// \default \c 5
    /// \param  encoder  An encoder instance to set.
    /// \param  value    See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn compression_level(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_compression_level(self.0, value) };
        self
    }

    /// Set the blocksize to use while encoding.
    ///
    /// The number of samples to use per frame.  Use \c 0 to let the encoder
    /// estimate a blocksize; this is usually best.
    ///
    /// \default \c 0
    /// \param  encoder  An encoder instance to set.
    /// \param  value    See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn blocksize(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_blocksize(self.0, value) };
        self
    }

    /// Set to \c true to enable mid-side encoding on stereo input.  The
    ///  number of channels must be 2 for this to have any effect.  Set to
    ///  \c false to use only independent channel coding.
    ///
    /// \default \c true
    /// \param  encoder  An encoder instance to set.
    /// \param  value    Flag value (see above).
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn do_mid_side_stereo(&mut self, value: bool) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_do_mid_side_stereo(self.0, value as FLAC__bool) };
        self
    }

    /// Set to \c true to enable adaptive switching between mid-side and
    ///  left-right encoding on stereo input.  Set to \c false to use
    ///  exhaustive searching.  Setting this to \c true requires
    ///  FLAC__stream_encoder_set_do_mid_side_stereo() to also be set to
    ///  \c true in order to have any effect.
    ///
    /// \default \c false
    /// \param  encoder  An encoder instance to set.
    /// \param  value    Flag value (see above).
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn loose_mid_side_stereo(&mut self, value: bool) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_loose_mid_side_stereo(self.0, value as FLAC__bool) };
        self
    }

    /// Sets the apodization function(s) the encoder will use when windowing
    ///  audio data for LPC analysis.
    ///
    /// The \a specification is a plain ASCII string which specifies exactly
    /// which functions to use.  There may be more than one (up to 32),
    /// separated by \c ';' characters.  Some functions take one or more
    /// comma-separated arguments in parentheses.
    ///
    /// The available functions are \c bartlett, \c bartlett_hann,
    /// \c blackman, \c blackman_harris_4term_92db, \c connes, \c flattop,
    /// \c gauss(STDDEV), \c hamming, \c hann, \c kaiser_bessel, \c nuttall,
    /// \c rectangle, \c triangle, \c tukey(P), \c partial_tukey(n[/ov[/P]]),
    /// \c punchout_tukey(n[/ov[/P]]), \c welch.
    ///
    /// For \c gauss(STDDEV), STDDEV specifies the standard deviation
    /// (0<STDDEV<=0.5).
    ///
    /// For \c tukey(P), P specifies the fraction of the window that is
    /// tapered (0<=P<=1).  P=0 corresponds to \c rectangle and P=1
    /// corresponds to \c hann.
    ///
    /// Specifying \c partial_tukey or \c punchout_tukey works a little
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
    /// Example specifications are \c "blackman" or
    /// \c "hann;triangle;tukey(0.5);tukey(0.25);tukey(0.125)"
    ///
    /// Any function that is specified erroneously is silently dropped.  Up
    /// to 32 functions are kept, the rest are dropped.  If the specification
    /// is empty the encoder defaults to \c "tukey(0.5)".
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
    /// \default \c "tukey(0.5)"
    /// \param  encoder        An encoder instance to set.
    /// \param  specification  See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    ///    \code specification != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn apodization(&mut self, specification: &CStr) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_apodization(self.0, specification.as_ptr()) };
        self
    }

    /// Set the maximum LPC order, or \c 0 to use only the fixed predictors.
    ///
    /// \default \c 8
    /// \param  encoder  An encoder instance to set.
    /// \param  value    See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn max_lpc_order(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_max_lpc_order(self.0, value) };
        self
    }

    /// Set the precision, in bits, of the quantized linear predictor
    ///  coefficients, or \c 0 to let the encoder select it based on the
    ///  blocksize.
    ///
    /// \note
    /// In the current implementation, qlp_coeff_precision + bits_per_sample must
    /// be less than 32.
    ///
    /// \default \c 0
    /// \param  encoder  An encoder instance to set.
    /// \param  value    See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn qlp_coeff_precision(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_qlp_coeff_precision(self.0, value) };
        self
    }

    /// Set to \c false to use only the specified quantized linear predictor
    ///  coefficient precision, or \c true to search neighboring precision
    ///  values and use the best one.
    ///
    /// \default \c false
    /// \param  encoder  An encoder instance to set.
    /// \param  value    See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn do_qlp_coeff_prec_search(&mut self, value: bool) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_do_qlp_coeff_prec_search(self.0, value as FLAC__bool) };
        self
    }

    /// Deprecated.  Setting this value has no effect.
    ///
    /// \default \c false
    /// \param  encoder  An encoder instance to set.
    /// \param  value    See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn do_escape_coding(&mut self, value: bool) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_do_escape_coding(self.0, value as FLAC__bool) };
        self
    }

    /// Set to \c false to let the encoder estimate the best model order
    ///  based on the residual signal energy, or \c true to force the
    ///  encoder to evaluate all order models and select the best.
    ///
    /// \default \c false
    /// \param  encoder  An encoder instance to set.
    /// \param  value    See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn do_exhaustive_model_search(&mut self, value: bool) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_do_exhaustive_model_search(self.0, value as FLAC__bool) };
        self
    }

    /// Set the minimum partition order to search when coding the residual.
    ///  This is used in tandem with
    ///  FLAC__stream_encoder_set_max_residual_partition_order().
    ///
    ///  The partition order determines the context size in the residual.
    ///  The context size will be approximately <tt>blocksize / (2 ^ order)</tt>.
    ///
    ///  Set both min and max values to \c 0 to force a single context,
    ///  whose Rice parameter is based on the residual signal variance.
    ///  Otherwise, set a min and max order, and the encoder will search
    ///  all orders, using the mean of each context for its Rice parameter,
    ///  and use the best.
    ///
    /// \default \c 0
    /// \param  encoder  An encoder instance to set.
    /// \param  value    See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn min_residual_partition_order(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_min_residual_partition_order(self.0, value) };
        self
    }

    /// Set the maximum partition order to search when coding the residual.
    ///  This is used in tandem with
    ///  FLAC__stream_encoder_set_min_residual_partition_order().
    ///
    ///  The partition order determines the context size in the residual.
    ///  The context size will be approximately <tt>blocksize / (2 ^ order)</tt>.
    ///
    ///  Set both min and max values to \c 0 to force a single context,
    ///  whose Rice parameter is based on the residual signal variance.
    ///  Otherwise, set a min and max order, and the encoder will search
    ///  all orders, using the mean of each context for its Rice parameter,
    ///  and use the best.
    ///
    /// \default \c 5
    /// \param  encoder  An encoder instance to set.
    /// \param  value    See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn max_residual_partition_order(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_max_residual_partition_order(self.0, value) };
        self
    }

    /// Deprecated.  Setting this value has no effect.
    ///
    /// \default \c 0
    /// \param  encoder  An encoder instance to set.
    /// \param  value    See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn rice_parameter_search_dist(&mut self, value: u32) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_rice_parameter_search_dist(self.0, value) };
        self
    }

    /// Set an estimate of the total samples that will be encoded.
    ///  This is merely an estimate and may be set to \c 0 if unknown.
    ///  This value will be written to the STREAMINFO block before encoding,
    ///  and can remove the need for the caller to rewrite the value later
    ///  if the value is known before encoding.
    ///
    /// \default \c 0
    /// \param  encoder  An encoder instance to set.
    /// \param  value    See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    pub fn total_samples_estimate(&mut self, value: u64) -> &mut FlacEncoderConfig {
        unsafe { FLAC__stream_encoder_set_total_samples_estimate(self.0, value) };
        self
    }

    /// Set the metadata blocks to be emitted to the stream before encoding.
    ///  A value of \c NULL, \c 0 implies no metadata; otherwise, supply an
    ///  array of pointers to metadata blocks.  The array is non-const since
    ///  the encoder may need to change the \a is_last flag inside them, and
    ///  in some cases update seek point offsets.  Otherwise, the encoder will
    ///  not modify or free the blocks.  It is up to the caller to free the
    ///  metadata blocks after encoding finishes.
    ///
    /// \note
    /// The encoder stores only copies of the pointers in the \a metadata array;
    /// the metadata blocks themselves must survive at least until after
    /// FLAC__stream_encoder_finish() returns.  Do not free the blocks until then.
    ///
    /// \note
    /// The STREAMINFO block is always written and no STREAMINFO block may
    /// occur in the supplied array.
    ///
    /// \note
    /// By default the encoder does not create a SEEKTABLE.  If one is supplied
    /// in the \a metadata array, but the client has specified that it does not
    /// support seeking, then the SEEKTABLE will be written verbatim.  However
    /// by itself this is not very useful as the client will not know the stream
    /// offsets for the seekpoints ahead of time.  In order to get a proper
    /// seektable the client must support seeking.  See next note.
    ///
    /// \note
    /// SEEKTABLE blocks are handled specially.  Since you will not know
    /// the values for the seek point stream offsets, you should pass in
    /// a SEEKTABLE 'template', that is, a SEEKTABLE object with the
    /// required sample numbers (or placeholder points), with \c 0 for the
    /// \a frame_samples and \a stream_offset fields for each point.  If the
    /// client has specified that it supports seeking by providing a seek
    /// callback to FLAC__stream_encoder_init_stream() or both seek AND read
    /// callback to FLAC__stream_encoder_init_ogg_stream() (or by using
    /// FLAC__stream_encoder_init*_file() or FLAC__stream_encoder_init*_FILE()),
    /// then while it is encoding the encoder will fill the stream offsets in
    /// for you and when encoding is finished, it will seek back and write the
    /// real values into the SEEKTABLE block in the stream.  There are helper
    /// routines for manipulating seektable template blocks; see metadata.h:
    /// FLAC__metadata_object_seektable_template_*().  If the client does
    /// not support seeking, the SEEKTABLE will have inaccurate offsets which
    /// will slow down or remove the ability to seek in the FLAC stream.
    ///
    /// \note
    /// The encoder instance \b will modify the first \c SEEKTABLE block
    /// as it transforms the template to a valid seektable while encoding,
    /// but it is still up to the caller to free all metadata blocks after
    /// encoding.
    ///
    /// \note
    /// A VORBIS_COMMENT block may be supplied.  The vendor string in it
    /// will be ignored.  libFLAC will use it's own vendor string. libFLAC
    /// will not modify the passed-in VORBIS_COMMENT's vendor string, it
    /// will simply write it's own into the stream.  If no VORBIS_COMMENT
    /// block is present in the \a metadata array, libFLAC will write an
    /// empty one, containing only the vendor string.
    ///
    /// \note The Ogg FLAC mapping requires that the VORBIS_COMMENT block be
    /// the second metadata block of the stream.  The encoder already supplies
    /// the STREAMINFO block automatically.  If \a metadata does not contain a
    /// VORBIS_COMMENT block, the encoder will supply that too.  Otherwise, if
    /// \a metadata does contain a VORBIS_COMMENT block and it is not the
    /// first, the init function will reorder \a metadata by moving the
    /// VORBIS_COMMENT block to the front; the relative ordering of the other
    /// blocks will remain as they were.
    ///
    /// \note The Ogg FLAC mapping limits the number of metadata blocks per
    /// stream to \c 65535.  If \a num_blocks exceeds this the function will
    /// return \c false.
    ///
    /// \default \c NULL, 0
    /// \param  encoder     An encoder instance to set.
    /// \param  metadata    See above.
    /// \param  num_blocks  See above.
    /// \assert
    ///    \code encoder != NULL \endcode
    /// \retval FLAC__bool
    ///    \c false if the encoder is already initialized, else \c true.
    ///    \c false if the encoder is already initialized, or if
    ///    \a num_blocks > 65535 if encoding to Ogg FLAC, else \c true.
    pub fn metadata(&mut self) -> &mut FlacEncoderConfig {
        unimplemented!();
        // FLAC__stream_encoder_set_metadata(self.0);
        // self
    }
}
