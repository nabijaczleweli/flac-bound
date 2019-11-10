#[macro_use]
extern crate clap;
extern crate flac_sys;

mod options;
mod flac_encoder;

pub use options::Options;
pub use flac_encoder::{FlacEncoderConfig, FlacEncoder};
