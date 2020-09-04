//! FLAC encoding via libflac FFI
//!
//! # Examples
//!
//! ```
//! # use flac_bound::{WriteWrapper, FlacEncoder};
//! # use std::fs::File;
//! let mut outf = File::create("ЦшЦ.flac").unwrap();
//! let mut outw = WriteWrapper(&mut outf);
//! let mut enc = FlacEncoder::new().unwrap().compression_level(8).init_write(&mut outw).unwrap();
//!
//! // The following two calls are equivalent for a two-channel encoder
//! enc.process(&[&[0xA1], &[0xF3]]).unwrap();
//! enc.process_interleaved(&[0xA1, 0xF3], 1).unwrap();
//!
//! // If you don't care about errors that may arise when writing the final frames,
//! // you can just drop the encoder; or you can inspect them:
//! match enc.finish() {
//!     Ok(mut conf) => {
//!         // Encoding succeeded, a new encoder can be initialised in the same place and memory
//!         enc = conf.compression_level(0).channels(1).init_stdout_ogg().unwrap();
//!         // &c.
//!     }
//!     Err(enc) => {
//!         eprintln!("Encoding failed: {:?}", enc.state());
//!     }
//! };
//! ```
//!
//! # Special thanks
//!
//! To all who support further development on [Patreon](https://patreon.com/nabijaczleweli), in particular:
//!
//!   * ThePhD
//!   * Embark Studios


#[cfg(feature="flac")]
extern crate flac_sys;
#[cfg(feature="libflac")]
extern crate libflac_sys;

mod encoder;

pub use encoder::{FlacEncoderInitError, FlacEncoderConfig, FlacEncoderState, WriteWrapper, FlacEncoder};
