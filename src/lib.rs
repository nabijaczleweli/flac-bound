extern crate flac_sys;

mod encoder;

pub use encoder::{FlacEncoderInitError, FlacEncoderConfig, FlacEncoderState, WriteWrapper, FlacEncoder};
