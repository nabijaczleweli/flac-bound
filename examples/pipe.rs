extern crate flac_bound;

use flac_bound::FlacEncoder;


fn main() {
    let mut enc = FlacEncoder::new().unwrap().channels(1).bits_per_sample(24).compression_level(8).init_stdout().unwrap();
    eprintln!("{:?}", enc);
    for i in 0..44100 {
        eprintln!("{}", i * 380 - 0xFFFFFF / 2);
        if enc.process(&[&[i * 380 - 0xFFFFFF / 2]]).is_err() {
            eprintln!("err {}", i);
        }
    }
    // eprintln!("{:?}", enc.finish());
}
