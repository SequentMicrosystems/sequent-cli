use crate::traits::{Opto, OptoAddrs};
use crate::traits::{Card, CardAddrs};

pub struct MultiIo;

impl MultiIo {
    pub fn new() -> Self {
        MultiIo
    }
}

impl CardAddrs for MultiIo {
    const BASE_ADDR: u8 = 0x06; // Example base address
    const CARD_NAME: &'static str = "MultiIo";
    const PROGRAM_NAME: &'static str = "multiio";
    const VERSION: &'static str = "1.0.0";
}
impl Card for MultiIo {}

impl OptoAddrs for MultiIo {
    const OPTO: u8 = 69; // Example value
    const RISING:  u8 = 0x01;
    const FALLING: u8 = 0x02;
    const ENCODER_ENABLE: u8 = 0x04;
    const COUNTER_RESET:    u8 = 0x08;
    const ENCODER_COUNTER_RESET: u8 = 0x10;
    const EDGE_COUNT: u8 = 0x20;
    const ENCODER_COUNT: u8 = 0x40;

    const COUNTER_SIZE: usize = 4;
    const OPTO_CH_NO: usize = 8;
}
impl Opto for MultiIo {}