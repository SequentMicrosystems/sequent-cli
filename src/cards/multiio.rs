use crate::impl_capabilities;
use crate::traits::{
    Card, CardInfo, Led, LedInfo, LedModeInfo, Opto, OptoInfo, Relay, RelayInfo, Watchdog, WatchdogInfo,
};

impl_capabilities!(MultiIo; Opto, Relay, Led, Watchdog);
pub struct MultiIo {
    stack_level: u8,
}

impl MultiIo {
    pub fn new(stack_level: u8) -> Self {
        if stack_level > Self::MAX_STACK_LEVEL {
            panic!("Invalid stack level: {}", stack_level);
        }
        MultiIo { stack_level }
    }
}

impl CardInfo for MultiIo {
    const BASE_ADDR: u8 = 0x06; // Example base address
    const CARD_NAME: &'static str = "Multi-IO";
    const PROGRAM_NAME: &'static str = "multiio";
    const VERSION: &'static str = "1.0.0";

    fn stack_level(&self) -> u8 {
        self.stack_level
    }
    fn set_stack_level(&mut self, stack_level: u8) {
        self.stack_level = stack_level;
    }
}

impl OptoInfo for MultiIo {
    const OPTO: u8 = 0x06;
    const RISING: u8 = 49;
    const FALLING: u8 = 50;
    const ENCODER_ENABLE: u8 = 51;
    const COUNTER_RESET: u8 = 52;
    const ENCODER_COUNTER_RESET: u8 = 53;
    const EDGE_COUNT: u8 = 54;
    const ENCODER_COUNT: u8 = 70;

    const COUNTER_SIZE: u8 = 4;
    const OPTO_CH_NO: u8 = 4;
}

impl RelayInfo for MultiIo {
    const RELAY: u8 = 0x00;
    const RELAY_SET: u8 = 0x01;
    const RELAY_CLR: u8 = 0x02;

    const RELAY_CH_NO: u8 = 2;
}

impl LedInfo for MultiIo {
    const LED: u8 = 0x03;
    const LED_SET: u8 = 0x04;
    const LED_CLR: u8 = 0x05;

    const LED_CH_NO: u8 = 6;
}
impl LedModeInfo for MultiIo {
    const LED_MODE: u8 = 0x20;
}

impl WatchdogInfo for MultiIo {
    const RESET: u8 = 98;
    const INTERVAL_SET: u8 = 99;
    const INTERVAL_GET: u8 = 101;
    const INIT_INTERVAL_SET: u8 = 103;
    const INIT_INTERVAL_GET: u8 = 105;
    const RESET_COUNT: u8 = 107;
    const CLEAR_RESET_COUNT: u8 = 109;
    const POWER_OFF_INTERVAL_SET: u8 = 110;
    const POWER_OFF_INTERVAL_GET: u8 = 114;
}
