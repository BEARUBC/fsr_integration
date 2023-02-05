use std::fs;

use fsr_integration::{self, FSR_INTEGRATION};

fn main() {
    println!("Hello, world!");
    let mut fsr = FSR_INTEGRATION::new().unwrap();
    fsr.run().unwrap();
}
