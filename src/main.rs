use std::fs;
use gpio::{GpioIn, GpioOut, sysfs::{SysFsGpioInput, SysFsGpioOutput}};
use rppal::gpio::Gpio;
use std::{time, thread};

use fsr_integration::{self, FSR_INTEGRATION};

fn main() {
    //println!("Hello, world!");
    let mut fsr = FSR_INTEGRATION::new().unwrap();
    // loop {
    //     fsr.run().unwrap();
    //     thread::sleep(time::Duration::from_millis(1000));
    // }
    loop{
        // for i in 0..10 {
        //     fsr.setRow(i);
        //     println!("{}", i);
        //     thread::sleep(time::Duration::from_millis(30000));
        // }
        fsr.setRow(5);
        println!("{}", 5);
        thread::sleep(time::Duration::from_millis(30000));
        // fsr.digitalWrite(22, 1);
        // thread::sleep(time::Duration::from_millis(10000));
        // fsr.digitalWrite(22, 0);
        // thread::sleep(time::Duration::from_millis(10000));
        
    }
    // let gpio = Gpio::new()?;
    // let mut pin = gpio.get(23)?.into_output();
    // let mut pin = gpio.get(23)?.into_input();

}
