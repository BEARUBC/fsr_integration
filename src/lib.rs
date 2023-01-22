use std::io::Error as StdError;
use std::io::{ErrorKind};
use std::collections::{HashMap};
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
    outpuPinMap: HashMap<u16, SysFsGpioOutput>,
    currentEnabledMux: u16
}


impl FSR_INTEGRATION {
    pub fn new() -> Result<FSR_INTEGRATION, StdError> {
        
        Ok(FSR_INTEGRATION{})
    }

    
    pub fn setup(&self) -> Result<(), StdError> {
        self.outpuPinMap = HashMap::new();

        self.adcInputGpio = SysFsGpioInput::open(PIN_ADC_INPUT)?;

        self.outpuPinMap.insert(
            PIN_SHIFT_REGISTER_DATA,
            SysFsGpioOutput::open(PIN_SHIFT_REGISTER_DATA)?
        );

        self.outpuPinMap.insert(
            PIN_SHIFT_REGISTER_CLOCK,
            SysFsGpioOutput::open(PIN_SHIFT_REGISTER_CLOCK)?
        );

        self.outpuPinMap.insert(
            PIN_MUX_CHANNEL_0,
            SysFsGpioOutput::open(PIN_MUX_CHANNEL_0)?
        );

        self.outpuPinMap.insert(
            PIN_MUX_CHANNEL_1,
            SysFsGpioOutput::open(PIN_MUX_CHANNEL_1)?
        );

        self.outpuPinMap.insert(
            PIN_MUX_CHANNEL_2,
            SysFsGpioOutput::open(PIN_MUX_CHANNEL_2)?
        );

        self.outpuPinMap.insert(
            PIN_MUX_INHIBIT_0,
            SysFsGpioOutput::open(PIN_MUX_INHIBIT_0)?
        );

        self.outpuPinMap.insert(
            PIN_MUX_INHIBIT_1,
            SysFsGpioOutput::open(PIN_MUX_INHIBIT_1)?
        );

        Ok(())
    }

    pub fn readData() -> Result<Vec<Vec<u16>>, StdError> {

        Ok(())
    }


    pub fn setRow(&self, rowNumber: u16) -> Result<(), StdError> {

        if rowNumber % ROWS_PER_MUX == 0 {
            match self.outpuPinMap.get(&(PIN_MUX_CHANNEL_0 + self.currentEnabledMux)) {
                Some(&pin) => pin.set_high()?,
                _ => return Err(StdError::new(ErrorKind::Other, "Failed")),
            }
            self.currentEnabledMux += 1;
            
            if self.currentEnabledMux >= MUX_COUNT {
                self.currentEnabledMux = 0;
            }

            match self.outpuPinMap.get(&(PIN_MUX_CHANNEL_0 + self.currentEnabledMux)) {
                Some(&pin) => pin.set_low()?,
                _ => return Err(StdError::new(ErrorKind::Other, "Failed")),
            }
        }





        Ok(())
    }


}