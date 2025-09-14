use crate::impl_capabilities;
use crate::traits::{
    Card, CardInfo, Led, LedInfo, LedMode, LedModeInfo, Opto, OptoInfo, Relay, RelayInfo, Watchdog, WatchdogInfo,
};

impl_capabilities!(EightInputs; Led, LedMode);
pub struct EightInputs {
    stack_level: u8,
}

impl EightInputs {
    pub fn new(stack_level: u8) -> Self {
        if stack_level > Self::MAX_STACK_LEVEL {
            panic!("Invalid stack level: {}", stack_level);
        }
        EightInputs { stack_level }
    }
}

impl CardInfo for EightInputs {
    const BASE_ADDR: u8 = 0x06; // Example base address
    const CARD_NAME: &'static str = "Eight Inputs";
    const PROGRAM_NAME: &'static str = "8inputs";
    const VERSION: &'static str = "1.0.0";

    fn stack_level(&self) -> u8 {
        self.stack_level
    }
    fn set_stack_level(&mut self, stack_level: u8) {
        self.stack_level = stack_level;
    }
}

/*
impl OptoInfo for EightInputs {
    const OPTO: u8 = 4;
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
*/

impl LedInfo for EightInputs {
    const LED: u8 = 0x06;
    const LED_SET: u8 = 0x07;
    const LED_CLR: u8 = 0x08;

    const LED_CH_NO: u8 = 8;
}
impl LedModeInfo for EightInputs {
    const LED_MODE: u8 = 9;
}

/*
impl WatchdogInfo for EightInputs {
    const RESET: u8 = 98; // TODO: COrrect these addreses
    const INTERVAL_SET: u8 = 99;
    const INTERVAL_GET: u8 = 101;
    const INIT_INTERVAL_SET: u8 = 103;
    const INIT_INTERVAL_GET: u8 = 105;
    const RESET_COUNT: u8 = 107;
    const CLEAR_RESET_COUNT: u8 = 109;
    const POWER_OFF_INTERVAL_SET: u8 = 110;
    const POWER_OFF_INTERVAL_GET: u8 = 114;
}
*/
