#![allow(dead_code)]
use bcm283x_linux_gpio as bcm283;
use std::{ thread, time };

pub struct Key {
    pin: usize, // GPIO pin that key is connected to
    // led_index: usize, // Led support to be added in the future
    function: Option<Box<dyn Fn(bool)>> // The function called on the key on a state change
}

enum GpioV2Pins {
    P1_03 = 2, P1_05 = 3, P1_07 = 4,
    P1_08 = 14, P1_10 = 15, P1_11 = 17, P1_12 = 18,
    P1_13 = 27, P1_15 = 22, P1_16 = 23, P1_18 = 24,
    P1_19 = 10, P1_21 = 9, P1_22 = 25, P1_23 = 11,
    P1_24 = 8, P1_26 = 7, P1_29 = 5, P1_31 = 6,
    P1_32 = 12, P1_33 = 13, P1_35 = 19, P1_36 = 16,
    P1_37 = 26, P1_38 = 20, P1_40 = 21, P5_03 = 28,
    P5_04 = 29, P5_05 = 30, P5_06 = 31
}

pub struct Keybow {
    mapping_table: Vec<Key>, // Collection of keys
    states: Vec<bool>,       // Last state of the keys to compare against
    mini: bool,              // Whether it is the keybow mini (3 key) or full (12 key)
    _gpio: bcm283::Gpio      // Internal gpio connection
}

impl Keybow {
    pub fn new() -> Result<Self, bcm283::Error> {
        bcm283::check_bcm283x_gpio()?;
        let mut s = Self {
            mapping_table: Vec::with_capacity(3),
            states: Vec::with_capacity(3),
            mini: false,
            _gpio: bcm283::Gpio::new()
        };
        Ok(s)
    }

    pub fn new_mini() -> Result<Self, bcm283::Error> {
        bcm283::check_bcm283x_gpio()?;
        let mut s = Self {
            mapping_table: Vec::with_capacity(3),
            states: Vec::with_capacity(3),
            mini: true,
            _gpio: bcm283::Gpio::new()
        };
        Ok(s)
    }

    pub fn add_key(&mut self, index: usize, /*led_index: usize,*/ function: Option<Box<dyn Fn(bool)>>) {
        // adjust key indexes to pins
        if self.mini {
            let pin: usize = match index {
                0 => GpioV2Pins::P1_38,
                1 => GpioV2Pins::P1_11,
                2 => GpioV2Pins::P1_18
            };
        } else {
            let pin: usize = match index {
                0  => GpioV2Pins::P1_38,
                1  => GpioV2Pins::P1_31,
                2  => GpioV2Pins::P1_15,
                3  => GpioV2Pins::P1_11,
                4  => GpioV2Pins::P1_36,
                5  => GpioV2Pins::P1_32,
                6  => GpioV2Pins::P1_18,
                7  => GpioV2Pins::P1_13,
                8  => GpioV2Pins::P1_37,
                9  => GpioV2Pins::P1_33,
                10 => GpioV2Pins::P1_29,
                11 => GpioV2Pins::P1_16
            };
        }
        self.mapping_table.push(Key {
            pin,
            // led_index,
            function
        });
        self.states.push(false);
    }

    fn init_gpio(&mut self) {
        let mut config = bcm283::GpioConfig::new();
        let mut config_pull = bcm283::GpioPullConfig::new();
        for Key { pin, led_index: _, function: _ } in self.mapping_table.iter() {
            config.set_function(pin, bcm283::PinFunction::Input);
            config_pull.set_pull_mode(pin, bcm283::PullMode::PullUp);
        }
        config.apply(&mut self._gpio);
        unsafe { config_pull.apply(&mut self._gpio); }
    }

    fn update_keys(&mut self) {
        for (i, key) in self.mapping_table.iter().enumerate() {
            let state = self._gpio.read_level(key.pin) == 0;
            if state != self.states[i] {
                if let Some(function) = key.function {
                    function(state);
                    self.states[i] = state;
                }
            }
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.update_keys();
            thread::sleep(time::Duration::from_micros(1000));
        }
    }
}
