use flac_sys::{FLAC__StreamEncoder, FLAC__StreamEncoderWriteStatus, FLAC__StreamEncoderSeekStatus, FLAC__StreamEncoderSeekStatus_FLAC__STREAM_ENCODER_SEEK_STATUS_OK,
               FLAC__StreamEncoderSeekStatus_FLAC__STREAM_ENCODER_SEEK_STATUS_ERROR, FLAC__StreamEncoderWriteStatus_FLAC__STREAM_ENCODER_WRITE_STATUS_OK,
               FLAC__StreamEncoderWriteStatus_FLAC__STREAM_ENCODER_WRITE_STATUS_FATAL_ERROR};
use std::io::{SeekFrom, Write, Seek};
use std::os::raw::{c_uint, c_void};
use std::slice;


/// This wrapper is necessary due to [fat pointers](https://chat.stackoverflow.com/transcript/message/47940937#47940937).
///
/// # Examples
///
/// ```
/// # use flac_bound::{WriteWrapper, FlacEncoder};
/// # use std::fs::File;
/// let mut outf = File::create("ЦшЦ.flac").unwrap();
/// let mut outw = WriteWrapper(&mut outf);
/// let mut outs = FlacEncoder::new().unwrap().init_write(&mut outw).unwrap();
///
/// outs.process_interleaved(&[0xA1, 0xF3], 1).unwrap();
/// ```
pub struct WriteWrapper<'out>(pub &'out mut dyn Write);


pub unsafe extern "C" fn flac_encoder_write_write_callback(_: *const FLAC__StreamEncoder, buffer: *const u8, bytes: usize, _: c_uint, _: c_uint,
                                                           client_data: *mut c_void)
                                                           -> FLAC__StreamEncoderWriteStatus {
    let out = &mut (*(client_data as *mut WriteWrapper<'static>)).0;

    match out.write_all(slice::from_raw_parts(buffer, bytes)) {
        Ok(_) => FLAC__StreamEncoderWriteStatus_FLAC__STREAM_ENCODER_WRITE_STATUS_OK,
        Err(_) => FLAC__StreamEncoderWriteStatus_FLAC__STREAM_ENCODER_WRITE_STATUS_FATAL_ERROR,
    }
}
