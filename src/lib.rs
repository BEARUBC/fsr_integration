use std::io::Error as StdError;
use std::io::{ErrorKind};
use std::string::String;
use std::collections::{HashMap};
use std::pin;
use std::vec::Vec;
use rppal::gpio::{Gpio, InputPin, OutputPin};
extern crate mcp3008;
use mcp3008::Mcp3008;
use std::{thread, time};


const BAUD_RATE: u64 = 115200;
const ROW_COUNT: u8 = 10;
const COLUMN_COUNT: u8 = 16;

const ROWS_PER_MUX: u8 = 8;
const MUX_COUNT: u8 = 2;
const CHANNEL_PINS_PER_MUX: u8 = 3;

//TODO 
//Assign Pins
const PIN_ADC_INPUT: u8 = 1;
const PIN_SHIFT_REGISTER_CLOCK: u8 = 25;
const PIN_SHIFT_REGISTER_DATA: u8 = 5;
const PIN_MUX_CHANNEL_0: u8 = 17;
const PIN_MUX_CHANNEL_1: u8 = 27;
const PIN_MUX_CHANNEL_2: u8 = 22;
const PIN_MUX_INHIBIT_0: u8 = 23;
const PIN_MUX_INHIBIT_1: u8 = 24;

 const mux_mapping: [u8; 10] = [3, 0, 1, 5, 7, 2, 6, 4, 3, 0];
 const col_mapping: [u8; 16] = [7, 0, 1, 2, 3, 4, 5, 6, 7, 15, 8, 9, 10, 11, 12, 13, 14];
//const mux_mapping: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 0, 1];




pub struct FSR_INTEGRATION {
    adcInputGpio: InputPin,
    outputPinMap: HashMap<u8, Box<OutputPin>>,
    currentEnabledMux: u8,
    mcp3008: Mcp3008
    // printGranularity: u8
}


impl FSR_INTEGRATION {
    pub fn new() -> Result<Self, StdError> {
        let gpio = Gpio::new().unwrap();

        Ok(FSR_INTEGRATION{
            adcInputGpio: gpio.get(PIN_ADC_INPUT).unwrap().into_input(),
            outputPinMap: FSR_INTEGRATION::setupPins().unwrap(),
            currentEnabledMux: 1,
            mcp3008: Mcp3008::new("/dev/spidev0.0").unwrap()
        })
    }

    
    pub fn setupPins() -> Result<HashMap<u8, Box<OutputPin>>, StdError> {
        let mut outputPinMap = HashMap::new();
        let gpio = Gpio::new().unwrap();

        outputPinMap.insert(
            PIN_SHIFT_REGISTER_DATA,
            Box::new(gpio.get(PIN_SHIFT_REGISTER_DATA).unwrap().into_output())
        );

        outputPinMap.insert(
            PIN_SHIFT_REGISTER_CLOCK,
            Box::new(gpio.get(PIN_SHIFT_REGISTER_CLOCK).unwrap().into_output())
        );

        outputPinMap.insert(
            PIN_MUX_CHANNEL_0,
            Box::new(gpio.get(PIN_MUX_CHANNEL_0).unwrap().into_output())
        );

        outputPinMap.insert(
            PIN_MUX_CHANNEL_1,
            Box::new(gpio.get(PIN_MUX_CHANNEL_1).unwrap().into_output())
        );

        outputPinMap.insert(
            PIN_MUX_CHANNEL_2,
            Box::new(gpio.get(PIN_MUX_CHANNEL_2).unwrap().into_output())
        );

        outputPinMap.insert(
            PIN_MUX_INHIBIT_0,
            Box::new(gpio.get(PIN_MUX_INHIBIT_0).unwrap().into_output())
        );

        outputPinMap.insert(
            PIN_MUX_INHIBIT_1,
            Box::new(gpio.get(PIN_MUX_INHIBIT_1).unwrap().into_output())
        );


        // for (key, value) in outputPinMap {
        //     println!("{} / {:?}", key, value);
        // }

        return Ok(outputPinMap);
    }


    pub fn bitRead(x: u8, i: u8) -> Result<u8, std::io::Error> {
        let s = format!("{x:b}");
        //println!("{} - {} - i = {}", x, s, i);
        if usize::from(i) >= s.len(){
            return Ok(0);
        }
        return Ok(s.chars().nth(s.len() - (i + 1) as usize).unwrap() as u8 - '0' as  u8);
        
    }


    pub fn digitalWrite(&mut self, pinNo: u8, value: u8) -> Result<(), StdError> {
        let pin = self.outputPinMap.get_mut(&pinNo);
        //println!("{}", pinNo);
        let p = &mut (*pin.unwrap());
        // println!("PIN {} to {}", pinNo, value);
        if value == 0 {
            p.set_low();
        } else if value == 1 {
            p.set_high();
        }
        
        Ok(())
    }


    pub fn setRow(&mut self, rowNumber: u8) -> Result<(), StdError> {
    
        if rowNumber % ROWS_PER_MUX == 0 {
            
            self.digitalWrite(PIN_MUX_INHIBIT_0 + self.currentEnabledMux, 1).unwrap();

            self.currentEnabledMux += 1;
            
            if self.currentEnabledMux >= MUX_COUNT {
                self.currentEnabledMux = 0;
            }

            self.digitalWrite(PIN_MUX_INHIBIT_0 + self.currentEnabledMux, 0).unwrap();
        }

        let mux_channel_no: u8;

        mux_channel_no = mux_mapping[rowNumber as usize];
	print!("{}-> ", mux_channel_no);
        // print!("SET ROW: {} ->", rowNumber);
        for i in 0..CHANNEL_PINS_PER_MUX {
            let bit = FSR_INTEGRATION::bitRead(mux_channel_no, i).unwrap();
            // print!("{}", bit);
            if bit == 1 {
                match (i) {
                    0 => self.digitalWrite(PIN_MUX_CHANNEL_0, 1).unwrap(),
                    1 => self.digitalWrite(PIN_MUX_CHANNEL_1, 1).unwrap(),
                    2 => self.digitalWrite(PIN_MUX_CHANNEL_2, 1).unwrap(),
                    _ => ()
                };
                //self.digitalWrite(PIN_MUX_CHANNEL_0 + i, 1).unwrap();
            } else {
                match (i) {
                    0 => self.digitalWrite(PIN_MUX_CHANNEL_0, 0).unwrap(),
                    1 => self.digitalWrite(PIN_MUX_CHANNEL_1, 0).unwrap(),
                    2 => self.digitalWrite(PIN_MUX_CHANNEL_2, 0).unwrap(),
                    _ => ()
                };
                //self.digitalWrite(PIN_MUX_CHANNEL_0 + i, 0).unwrap();
            }
        }
        // println!();

        Ok(())
    }



    pub fn shiftColumn(&mut self, isFirst: bool) -> Result<(), StdError> {
        if isFirst {
            self.digitalWrite(PIN_SHIFT_REGISTER_DATA, 1).unwrap();
        }

        self.digitalWrite(PIN_SHIFT_REGISTER_CLOCK, 1).unwrap();
        self.digitalWrite(PIN_SHIFT_REGISTER_CLOCK, 0).unwrap();

        if isFirst {
            self.digitalWrite(PIN_SHIFT_REGISTER_DATA, 0).unwrap();
        }

        Ok(())
    }

    pub fn readADCValue(&mut self) -> Result<u8, StdError> {
        // match self.adcInputGpio.read_value() {
        //     Ok(val) => match val {
        //         gpio::GpioValue::Low => return Ok(0),
        //         gpio::GpioValue::High => return Ok(1)
        //     },
        //     _ => Err(StdError::new(ErrorKind::Other, "Failed to Read ADC"))
        // }
        Ok(self.mcp3008.read_adc(0).unwrap() as u8)
    }


    pub fn run(&mut self) -> Result<(), StdError> {

        

        for i in 0..ROW_COUNT {
            
            let mut cells_array: [u8; 16] = Default::default();

            print!("[");
            
            self.setRow(i).unwrap();
            self.shiftColumn(true).unwrap();
            //self.shiftColumn(false).unwrap();
            

            for j in 0..COLUMN_COUNT {
                

                let reading = self.readADCValue().unwrap();
                self.shiftColumn(false).unwrap();


                cells_array[col_mapping[j]] = reading;

		// //print!("[{0: <2},{1: <2}],", i, j);
        //         if j == COLUMN_COUNT-1 {
        //             // print!("{0: <4}", reading);
        //             print!("{}", reading);
        //         } else {
        //             // print!("{0: <4},", reading);
        //             print!("{},",reading);

        //         }
            }

            print!("{:?}", cells_array);

            if i == ROW_COUNT-1 {
                print!("]");
            } else {
                print!("],");
                println!();
                //print!(" ");
            }
            
            
        }
        print!("]");
        let ten_millis = time::Duration::from_millis(100);
        print!("\n");

        print!("\n");
        thread::sleep(ten_millis);

        Ok(())
    }

    // fn printGranularity(&mut self, reading: u8) {
    //     if reading < 255 / 3 {
    //         print!{""}
    //     } 
    //     else if reading < (255 * 2) / 3 {
    //         print!{"|"}
    //     } else {
    //         print!{"B"}
    //     }
    // }

    
}
