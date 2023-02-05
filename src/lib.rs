use std::io::Error as StdError;
use std::io::{ErrorKind};
use std::collections::{HashMap};
use std::pin;
use std::vec::Vec;
use gpio::{GpioIn, GpioOut, sysfs::{SysFsGpioInput, SysFsGpioOutput}};
extern crate mcp3008;
use mcp3008::Mcp3008;


const BAUD_RATE: u64 = 115200;
const ROW_COUNT: u16 = 10;
const COLUMN_COUNT: u16 = 16;

const ROWS_PER_MUX: u16 = 8;
const MUX_COUNT: u16 = 2;
const CHANNEL_PINS_PER_MUX: u16 = 3;

//TODO 
//Assign Pins
const PIN_ADC_INPUT: u16 = 1;
const PIN_SHIFT_REGISTER_CLOCK: u16 = 5;
const PIN_SHIFT_REGISTER_DATA: u16 = 6;
const PIN_MUX_CHANNEL_0: u16 = 22;
const PIN_MUX_CHANNEL_1: u16 = 27;
const PIN_MUX_CHANNEL_2: u16 = 17;
const PIN_MUX_INHIBIT_0: u16 = 24;
const PIN_MUX_INHIBIT_1: u16 = 23;



pub struct FSR_INTEGRATION {
    adcInputGpio: SysFsGpioInput,
    outputPinMap: HashMap<u16, Box<SysFsGpioOutput>>,
    currentEnabledMux: u16,
    mcp3008: Mcp3008
}


impl FSR_INTEGRATION {
    pub fn new() -> Result<Self, StdError> {
        Ok(FSR_INTEGRATION{
            adcInputGpio: SysFsGpioInput::open(PIN_ADC_INPUT)?,
            outputPinMap: FSR_INTEGRATION::setupPins()?,
            currentEnabledMux: MUX_COUNT - 1,
            mcp3008: Mcp3008::new("/dev/spidev0.0")?
        })
    }

    
    pub fn setupPins() -> Result<HashMap<u16, Box<SysFsGpioOutput>>, StdError> {
        let mut outputPinMap = HashMap::new();

        outputPinMap.insert(
            PIN_SHIFT_REGISTER_DATA,
            Box::new(SysFsGpioOutput::open(PIN_SHIFT_REGISTER_DATA)?)
        );

        outputPinMap.insert(
            PIN_SHIFT_REGISTER_CLOCK,
            Box::new(SysFsGpioOutput::open(PIN_SHIFT_REGISTER_CLOCK)?)
        );

        outputPinMap.insert(
            PIN_MUX_CHANNEL_0,
            Box::new(SysFsGpioOutput::open(PIN_MUX_CHANNEL_0)?)
        );

        outputPinMap.insert(
            PIN_MUX_CHANNEL_1,
            Box::new(SysFsGpioOutput::open(PIN_MUX_CHANNEL_1)?)
        );

        outputPinMap.insert(
            PIN_MUX_CHANNEL_2,
            Box::new(SysFsGpioOutput::open(PIN_MUX_CHANNEL_2)?)
        );

        outputPinMap.insert(
            PIN_MUX_INHIBIT_0,
            Box::new(SysFsGpioOutput::open(PIN_MUX_INHIBIT_0)?)
        );

        outputPinMap.insert(
            PIN_MUX_INHIBIT_1,
            Box::new(SysFsGpioOutput::open(PIN_MUX_INHIBIT_1)?)
        );

        return Ok(outputPinMap);
    }


    pub fn bitRead(x: u16, i: u16) -> Result<u16, std::io::Error> {
        let s = format!("{x:b}");
        
        return Ok(s.chars().nth(s.len() - (i + 1) as usize).unwrap() as u16 - '0' as  u16);
        
    }


    pub fn digitalWrite(&mut self, pinNo: u16, value: u16) -> Result<(), StdError> {
        let pin = self.outputPinMap.get_mut(&pinNo);
        let p = &mut (*pin.unwrap());
        if value == 0 {
            p.set_low();
        } else if value == 1 {
            p.set_high();
        }
        
        Ok(())
    }


    pub fn setRow(&mut self, rowNumber: u16) -> Result<(), StdError> {
    
        if rowNumber % ROWS_PER_MUX == 0 {
            
            self.digitalWrite(PIN_MUX_INHIBIT_0 + self.currentEnabledMux, 1)?;

            self.currentEnabledMux += 1;
            
            if self.currentEnabledMux >= MUX_COUNT {
                self.currentEnabledMux = 0;
            }

            self.digitalWrite(PIN_MUX_INHIBIT_0 + self.currentEnabledMux, 0)?;
        }


        for i in 0..CHANNEL_PINS_PER_MUX {
            let bit = FSR_INTEGRATION::bitRead(rowNumber, i).unwrap();

            if bit == 1 {
                self.digitalWrite(PIN_MUX_CHANNEL_0 + i, 1)?;
            } else {
                self.digitalWrite(PIN_MUX_CHANNEL_0 + i, 0)?;
            }
        }

        Ok(())
    }



    pub fn shiftColumn(&mut self, isFirst: bool) -> Result<(), StdError> {
        if isFirst {
            self.digitalWrite(PIN_SHIFT_REGISTER_DATA, 1)?;
        }

        self.digitalWrite(PIN_SHIFT_REGISTER_CLOCK, 1)?;
        self.digitalWrite(PIN_SHIFT_REGISTER_CLOCK, 0)?;

        if isFirst {
            self.digitalWrite(PIN_SHIFT_REGISTER_DATA, 0)?;
        }

        Ok(())
    }

    pub fn readADCValue(&mut self) -> Result<u16, StdError> {
        // match self.adcInputGpio.read_value() {
        //     Ok(val) => match val {
        //         gpio::GpioValue::Low => return Ok(0),
        //         gpio::GpioValue::High => return Ok(1)
        //     },
        //     _ => Err(StdError::new(ErrorKind::Other, "Failed to Read ADC"))
        // }
        self.mcp3008.read_adc(0).unwrap() as u16
    }


    pub fn run(&mut self) -> Result<(), StdError> {
        for i in 0..ROW_COUNT {
            self.setRow(i)?;
            self.shiftColumn(true)?;
            self.shiftColumn(false)?;

            for j in 0..COLUMN_COUNT {
                let reading = self.readADCValue()?;
                self.shiftColumn(false)?;
                print!("{} ", reading);
            }
            println!();
        }
        println!();
        Ok(())
    }

    
}