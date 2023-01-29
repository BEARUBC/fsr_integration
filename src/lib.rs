use std::io::Error as StdError;
use std::io::{ErrorKind};
use std::collections::{HashMap};
use std::pin;
use std::vec::Vec;
use gpio::{GpioIn, GpioOut, sysfs::{SysFsGpioInput, SysFsGpioOutput}};



const BAUD_RATE: u64 = 115200;
const ROW_COUNT: u16 = 10;
const COLUMN_COUNT: u16 = 16;

const ROWS_PER_MUX: u16 = 8;
const MUX_COUNT: u16 = 2;
const CHANNEL_PINS_PER_MUX: u16 = 3;

//TODO 
//Assign Pins
const PIN_ADC_INPUT: u16 = 1;
const PIN_SHIFT_REGISTER_CLOCK: u16 = 3;
const PIN_SHIFT_REGISTER_DATA: u16 = 2;
const PIN_MUX_CHANNEL_0: u16 = 2;
const PIN_MUX_CHANNEL_1: u16 = 5;
const PIN_MUX_CHANNEL_2: u16 = 6;
const PIN_MUX_INHIBIT_0: u16 = 3;
const PIN_MUX_INHIBIT_1: u16 = 8;



pub struct FSR_INTEGRATION {
    adcInputGpio: SysFsGpioInput,
    outputPinMap: HashMap<u16, Box<SysFsGpioOutput>>,
    currentEnabledMux: u16
}


impl FSR_INTEGRATION {
    pub fn new() -> Result<Self, StdError> {
        Ok(FSR_INTEGRATION{
            adcInputGpio: SysFsGpioInput::open(PIN_ADC_INPUT)?,
            outputPinMap: FSR_INTEGRATION::setupPins()?,
            currentEnabledMux: MUX_COUNT - 1
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

    pub fn setPinHigh(&mut self, pinNo: u16) -> Result<(), StdError> {
        let pin = self.outputPinMap.get(&pinNo).unwrap();
        
        Ok(())
    }

    pub fn setPinLow(&self, pinNo: u16) -> Result<(), StdError> {
        self.outputPinMap.get(&pinNo).unwrap().set_low();
        Ok(())
    }



    pub fn setRow(&self, rowNumber: u16) -> Result<(), StdError> {
    
        if rowNumber % ROWS_PER_MUX == 0 {
            //self.setPinHigh(PIN_MUX_INHIBIT_0 + self.currentEnabledMux);

            self.currentEnabledMux += 1;
            
            if self.currentEnabledMux >= MUX_COUNT {
                self.currentEnabledMux = 0;
            }

            match self.outputPinMap.get(&(PIN_MUX_INHIBIT_0 + self.currentEnabledMux)) {
                Some(&pin) => pin.set_low()?,
                _ => return Err(StdError::new(ErrorKind::Other, "Failed")),
            }
        }


        for i in 0..CHANNEL_PINS_PER_MUX {
            let bit = FSR_INTEGRATION::bitRead(rowNumber, i).unwrap();

            if bit == 1 {
                match self.outputPinMap.get(&(PIN_MUX_CHANNEL_0 + i)) {
                    Some(&pin) => pin.set_high()?,
                    _ => return Err(StdError::new(ErrorKind::Other, "Failed")),
                }
            } else {
                match self.outputPinMap.get(&(PIN_MUX_CHANNEL_0 + i)) {
                    Some(&pin) => pin.set_low()?,
                    _ => return Err(StdError::new(ErrorKind::Other, "Failed")),
                }
            }
        }

        Ok(())
    }



    pub fn shiftColumn(&self, isFirst: bool) -> Result<(), StdError> {
        if isFirst {
            match self.outputPinMap.get(&(PIN_SHIFT_REGISTER_DATA)) {
                Some(&pin) => pin.set_high()?,
                _ => return Err(StdError::new(ErrorKind::Other, "Failed")),
            }
        }

        match self.outputPinMap.get(&(PIN_SHIFT_REGISTER_CLOCK)) {
            Some(&pin) => pin.set_high()?,
            _ => return Err(StdError::new(ErrorKind::Other, "Failed")),
        }

        match self.outputPinMap.get(&(PIN_SHIFT_REGISTER_CLOCK)) {
            Some(&pin) => pin.set_low()?,
            _ => return Err(StdError::new(ErrorKind::Other, "Failed")),
        }

        if isFirst {
            match self.outputPinMap.get(&(PIN_SHIFT_REGISTER_DATA)) {
                Some(&pin) => pin.set_low()?,
                _ => return Err(StdError::new(ErrorKind::Other, "Failed")),
            }
        }

        Ok(())
    }

    
}