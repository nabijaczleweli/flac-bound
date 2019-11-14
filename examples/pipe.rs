extern crate flac_bound;

use flac_bound::FlacEncoder;


fn main() {
    let enc = FlacEncoder::new().unwrap().channels(1).bits_per_sample(24).compression_level(8).init_stdout().unwrap();
    println!("{:?}", enc);
}
