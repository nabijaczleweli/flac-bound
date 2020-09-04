# flac-bound [![TravisCI build status](https://travis-ci.com/nabijaczleweli/flac-bound.svg?branch=master)](https://travis-ci.com/nabijaczleweli/flac-bound) [![AppVeyorCI build status](https://ci.appveyor.com/api/projects/status/5ku5wl0xux9gyhk9?svg=true)](https://ci.appveyor.com/project/nabijaczleweli/flac-bound/branch/master) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE) [![Crates.io version](https://meritbadge.herokuapp.com/flac-bound)](https://crates.io/crates/flac-bound)
FLAC encoding via libFLAC FFI

## [Documentation](https://rawcdn.githack.com/nabijaczleweli/flac-bound/doc/flac_bound/index.html)

## Building <!-- also update toplevel doc -->

There are two supported libFLAC back-ends:
  * [`flac-sys`](https://crates.io/crates/flac-sys), under the `"flac"` feature, the default, and
  * [`libflac-sys`](https://crates.io/crates/libflac-sys), under the `"libflac"` feature.

`flac-sys` tries to link to a libFLAC already present on your system, but it doesn't do a very good job, and might need some help by copying
`/usr/lib/x86_64-linux-gnu/libFLAC.so` (Debian), `$MSYSROOT\mingw64\lib\libflac.dll.a` (msys2), or equivalent
to `target/{debug,release}/deps` as `libflac.so`/`libflac.dll.a`/&c. (note the lowercase).

`libflac-sys` tries to build libFLAC; this is a problem because it (a) doesn't work all that well (at all) under GNU/NT,
and (b) requires the host system to have both CMake and a C toolchain funxional.

Downstreams are encouraged to expose these features to the user.

## Special thanks

To all who support further development on Patreon, in particular:

  * ThePhD
  * Embark Studios
