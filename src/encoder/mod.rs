mod encoder;
mod config;
mod error;

use flac_sys::{FLAC__StreamEncoder, FLAC__stream_encoder_delete};
use std::{mem, ptr};

pub use self::error::FlacEncoderInitError;
pub use self::config::FlacEncoderConfig;
pub use self::encoder::FlacEncoder;


#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct StreamEncoderContainer(pub *mut FLAC__StreamEncoder);

impl Drop for StreamEncoderContainer {
    fn drop(&mut self) {
        let ptr = mem::replace(&mut self.0, ptr::null_mut());
        if !ptr.is_null() {
            unsafe { FLAC__stream_encoder_delete(ptr) };
        }
    }
}
