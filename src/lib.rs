#![allow(dead_code)]

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
    mapping_table: Vec<(gpio_cdev::Line, Box<dyn Fn(bool)>)>, // Collection of keys
    states: Vec<bool>,       // Last state of the keys to compare against
    mini: bool,              // Whether it is the keybow mini (3 key) or full (12 key)
    _chip: gpio_cdev::Chip
}

impl Keybow {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        Ok(Self {
            mapping_table: Vec::with_capacity(3),
            states: Vec::with_capacity(3),
            mini: false,
            _chip: match gpio_cdev::Chip::new(path) {
                Ok(c)  => c,
                Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e))
            }
        })
    }

    pub fn new_mini<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        Ok(Self {
            mapping_table: Vec::with_capacity(3),
            states: Vec::with_capacity(3),
            mini: true,
            _chip: match gpio_cdev::Chip::new(path) {
                Ok(c)  => c,
                Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e))
            }
        })
    }

    pub fn add_key(&mut self, index: usize, function: Box<dyn Fn(bool)>) -> Result<(), std::io::Error> {
        let pin = {
            if self.mini {
                match index {
                    0 => GpioV2Pins::P1_38 as u32,
                    1 => GpioV2Pins::P1_11 as u32,
                    2 => GpioV2Pins::P1_18 as u32,
                    _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid key index for device (Keybow mini)"))
                }
            } else {
                match index {
                    0  => GpioV2Pins::P1_38 as u32,
                    1  => GpioV2Pins::P1_31 as u32,
                    2  => GpioV2Pins::P1_15 as u32,
                    3  => GpioV2Pins::P1_11 as u32,
                    4  => GpioV2Pins::P1_36 as u32,
                    5  => GpioV2Pins::P1_32 as u32,
                    6  => GpioV2Pins::P1_18 as u32,
                    7  => GpioV2Pins::P1_13 as u32,
                    8  => GpioV2Pins::P1_37 as u32,
                    9  => GpioV2Pins::P1_33 as u32,
                    10 => GpioV2Pins::P1_29 as u32,
                    11 => GpioV2Pins::P1_16 as u32,
                    _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid key index for device (Keybow mini)"))
                }
            }
        };
        let line = match self._chip.get_line(pin) {
            Ok(l)  => l,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e))
        };
        self.mapping_table.push((line, function));
        self.states.push(false);
        Ok(())
    }

    pub fn update_keys(&mut self) -> Result<(), std::io::Error> {
        for (index, (line, function)) in self.mapping_table.iter().enumerate() {
            let handle = match line.request(gpio_cdev::LineRequestFlags::INPUT, 0, &format!("Keybow{}", if self.mini { " mini" } else { "" })) {
                Ok(h)  => h,
                Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e))
            };
            match handle.get_value() {
                Ok(v) => {
                    let state = v == 0;
                    if state != self.states[index] {
                        function(state);
                        self.states[index] = state;
                    }
                },
                Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e))
            }
        }
        Ok(())
    }
}

#[test]
fn full() {
    let mut k = crate::Keybow::new_mini("/dev/gpiochip0").unwrap();
    let cb = |b| println!("{}", b);
    k.add_key(0, Box::new(cb)).unwrap();
    k.update_keys().unwrap();
}
