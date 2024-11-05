//! FLAC encoding via libFLAC FFI
//!
//! # Building <!-- also update README -->
//!
//! There are two supported libFLAC back-ends:
//!   * [`flac-sys`](https://crates.io/crates/flac-sys), under the `"flac"` feature, the default, and
//!   * [`libflac-sys`](https://crates.io/crates/libflac-sys), under the `"libflac"` feature group
//!     (better-maintained, [`FlacEncoderConfig::set_limit_min_bitrate()`] is only available here).
//!
//! `flac-sys` tries to link to a libFLAC already present on your system, but it doesn't do a very good job, and might need some help by copying
//! `/usr/lib/x86_64-linux-gnu/libFLAC.so` (Debian), `$MSYSROOT\mingw64\lib\libflac.dll.a` (msys2), or equivalent
//! to `target/{debug,release}/deps` as `libflac.so`/`libflac.dll.a`/&c. (note the lowercase).
//!
//! `libflac-sys` tries to build libFLAC; this is a problem because it (a) doesn't work all that well (at all) under GNU/NT,
//! and (b) requires the host system to have both CMake and a C toolchain funxional.
//!
//! The `"libflac-noogg"` feature will build libFLAC without OGG support.
//!
//! The `"libflac-nobuild"` feature will still use `libflac-sys` but instruct it to link to the system libFLAC.
//!
//! Downstreams are encouraged to expose these features to the user.
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
//! #       #[cfg(not(feature="libflac-noogg"))] {
//!         enc = conf.compression_level(0).channels(1).init_stdout_ogg().unwrap();
//! #       }
//! #       #[cfg(feature="libflac-noogg")] {
//! #       enc = conf.compression_level(0).channels(1).init_stdout().unwrap();
//! #       }
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
//!   * Jasper Bekkers


#[cfg(feature="flac")]
extern crate flac_sys;
#[cfg(feature="libflac-nobuild")]
extern crate libflac_sys;

mod encoder;

pub use encoder::{FlacEncoderInitError, FlacEncoderConfig, FlacEncoderState, WriteWrapper, FlacEncoder};
