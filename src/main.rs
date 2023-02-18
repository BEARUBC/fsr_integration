use std::fs;
use gpio::{GpioIn, GpioOut, sysfs::{SysFsGpioInput, SysFsGpioOutput}};
use rppal::gpio::Gpio;
use std::{time, thread};

use fsr_integration::{self, FSR_INTEGRATION};

fn main() {
    //println!("Hello, world!");
    let mut fsr = FSR_INTEGRATION::new().unwrap();
    loop {
        fsr.run().unwrap();
        thread::sleep(time::Duration::from_millis(5000));
    }
    // let gpio = Gpio::new()?;
    // let mut pin = gpio.get(23)?.into_output();
    // let mut pin = gpio.get(23)?.into_input();

}
