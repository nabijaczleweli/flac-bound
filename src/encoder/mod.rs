mod callbacks;
#[allow(clippy::module_inception)]
mod encoder;
mod config;
mod state;
mod error;

#[cfg(feature = "flac")]
use flac_sys::{FLAC__StreamEncoder, FLAC__stream_encoder_delete};

#[cfg(feature = "libflac-nobuild")]
use libflac_sys::{FLAC__StreamEncoder, FLAC__stream_encoder_delete};

use std::{mem, ptr};

pub use self::callbacks::{WriteWrapper, flac_encoder_write_write_callback};
pub use self::error::FlacEncoderInitError;
pub use self::config::FlacEncoderConfig;
pub use self::state::FlacEncoderState;
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
