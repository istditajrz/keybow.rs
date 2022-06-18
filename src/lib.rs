#![allow(dead_code)]
use gpio::{self, GpioIn};

struct DummyPin<F> {
    value: F
}

impl<F> DummyPin<F> {
    fn new(value: F) -> DummyPin<F> {
        Self { value }
    }
}

impl<V, F> gpio::GpioIn for DummyPin<F>
where
    V: Into<gpio::GpioValue>,
    F: Fn() -> V
{
    type Error = std::io::Error;

    fn read_value(&mut self) -> Result<gpio::GpioValue, Self::Error> {
        Ok((self.value)().into())
    }
}

pub struct Key {
    pin: Box<dyn gpio::GpioIn<Error = std::io::Error>>, // GPIO connection to be read
    // led_index: usize,                                // Led support to be added in the future
    function: Option<Box<dyn Fn(bool)>>                 // The function called on the key on a state change
}

impl Key {
    pub fn new_from_index(index: usize, mini: bool, function: Option<Box<dyn Fn(bool)>>) -> Result<Self, std::io::Error> {
        let pin = {
            if mini {
                match index {
                    0 => GpioV2Pins::P1_38 as u16,
                    1 => GpioV2Pins::P1_11 as u16,
                    2 => GpioV2Pins::P1_18 as u16,
                    _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid key index for device (Keybow mini)"))
                }
            } else {
                match index {
                    0  => GpioV2Pins::P1_38 as u16,
                    1  => GpioV2Pins::P1_31 as u16,
                    2  => GpioV2Pins::P1_15 as u16,
                    3  => GpioV2Pins::P1_11 as u16,
                    4  => GpioV2Pins::P1_36 as u16,
                    5  => GpioV2Pins::P1_32 as u16,
                    6  => GpioV2Pins::P1_18 as u16,
                    7  => GpioV2Pins::P1_13 as u16,
                    8  => GpioV2Pins::P1_37 as u16,
                    9  => GpioV2Pins::P1_33 as u16,
                    10 => GpioV2Pins::P1_29 as u16,
                    11 => GpioV2Pins::P1_16 as u16,
                    _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid key index for device (Keybow mini)"))
                }
            }
        };
        Ok(Self {
            pin: Box::new(gpio::sysfs::SysFsGpioInput::open(pin)?),
            function
        })
    }
}

impl gpio::GpioIn for Key {
    type Error = std::io::Error;

    fn read_value(&mut self) -> Result<gpio::GpioValue, Self::Error> {
        self.pin.read_value()
    }
}

enum GpioV2Pins {
    P1_03 = 2,  P1_05 = 3,  P1_07 = 4,  P1_08 = 14,
    P1_10 = 15, P1_11 = 17, P1_12 = 18, P1_13 = 27,
    P1_15 = 22, P1_16 = 23, P1_18 = 24, P1_19 = 10,
    P1_21 = 9,  P1_22 = 25, P1_23 = 11, P1_24 = 8,
    P1_26 = 7,  P1_29 = 5,  P1_31 = 6,  P1_32 = 12,
    P1_33 = 13, P1_35 = 19, P1_36 = 16, P1_37 = 26,
    P1_38 = 20, P1_40 = 21, P5_03 = 28, P5_04 = 29,
    P5_05 = 30, P5_06 = 31
}

pub struct Keybow {
    mapping_table: Vec<Key>, // Collection of keys
    states: Vec<bool>,       // Last state of the keys to compare against
    mini: bool,              // Whether it is the keybow mini (3 key) or full (12 key)
}

impl Keybow {
    pub fn new() -> Self {
        Self {
            mapping_table: Vec::with_capacity(3),
            states: Vec::with_capacity(3),
            mini: false
        }
    }

    pub fn new_mini() -> Self {
        Self {
            mapping_table: Vec::with_capacity(3),
            states: Vec::with_capacity(3),
            mini: true
        }
    }

    pub fn add_key(&mut self, key: Key) {
        self.mapping_table.push(key);
        self.states.push(false);
    }

    fn update_keys(&mut self) -> Result<(), std::io::Error> {
        for (i, key) in self.mapping_table.iter_mut().enumerate() {
            let state: bool = key.read_value()?.into();
            if state != self.states[i] {
                if let Some(function) = &key.function {
                    function(state);
                    self.states[i] = state;
                }
            }
        }
        Ok(())
    }
}

#[test]
fn full() -> Result<(), std::io::Error> {
    let mut k = crate::Keybow::new_mini();
    let dummy = DummyPin::new(|| true);
    k.add_key(
        Key {
            pin: Box::new(dummy),
            function: Some(Box::new(|state: bool| println!("Prev Key: {}", state)))
        }
    );
    k.update_keys().unwrap();
}
