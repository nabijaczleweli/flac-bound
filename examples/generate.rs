extern crate flac_bound;

use flac_bound::{WriteWrapper, FlacEncoder};
use std::fs::File;


fn main() {
    let mut outf = File::create("a.flac").unwrap();
    let mut outw = WriteWrapper(&mut outf);
    let mut enc = FlacEncoder::new().unwrap().channels(1).bits_per_sample(24).compression_level(8).init_write(&mut outw).unwrap();
    eprintln!("{:?}", enc);

    for i in 0..44100 {
        eprintln!("{}", i * 380 - 0xFFFFFF / 2);
        if enc.process(&[&[i * 380 - 0xFFFFFF / 2]]).is_err() {
            eprintln!("err {}", i);
        }
    }
    eprintln!("{:?}", enc.finish());
}
