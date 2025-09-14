use crate::impl_capabilities;
use crate::traits::{Card, CardInfo, Relay, RelayInfo, Watchdog, WatchdogInfo};

impl_capabilities!(Industrial; Relay, Watchdog);
pub struct Industrial {
    stack_level: u8,
}

impl Industrial {
    pub fn new(stack_level: u8) -> Self {
        if stack_level > Self::MAX_STACK_LEVEL {
            panic!("Invalid stack level: {}", stack_level);
        }
        Industrial { stack_level }
    }
}

impl CardInfo for Industrial {
    const BASE_ADDR: u8 = 0x50;
    const CARD_NAME: &'static str = "Industrial Automation";
    const PROGRAM_NAME: &'static str = "industrial";
    const VERSION: &'static str = "1.0.0";

    fn stack_level(&self) -> u8 {
        self.stack_level
    }
    fn set_stack_level(&mut self, stack_level: u8) {
        self.stack_level = stack_level;
    }
}

impl RelayInfo for Industrial {
    const RELAY: u8 = 0x00;
    const RELAY_SET: u8 = 0x01;
    const RELAY_CLR: u8 = 0x02;

    const RELAY_CH_NO: u8 = 8;
}

impl WatchdogInfo for Industrial {
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
