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
        fsr.shiftColumn(true).unwrap();
            fsr.shiftColumn(false).unwrap();
            let mut i = 0;
         while true {
            fsr.shiftColumn(false).unwrap();

             // fsr.setRow(i);
            println!("{}", i);

            let mut line = String::new();
            let b1 = std::io::stdin().read_line(&mut line).unwrap();
            i += 1;
            if i > 15 {
                i = 0;
            }
         }
    //     fsr.setRow(5);
    //     println!("{}", 5);
    //     thread::sleep(time::Duration::from_millis(30000));
    //     // fsr.digitalWrite(22, 1);
    //     // thread::sleep(time::Duration::from_millis(10000));
    //     // fsr.digitalWrite(22, 0);
    //     // thread::sleep(time::Duration::from_millis(10000));
        
    }
    // let gpio = Gpio::new()?;
    // let mut pin = gpio.get(23)?.into_output();
    // let mut pin = gpio.get(23)?.into_input();

}
