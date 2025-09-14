use crate::{Error, Result};
use clap::Command;

use crate::cli::args;
use crate::traits::Card;

pub trait LedInfo {
    const LED: u8;
    const LED_SET: u8;
    const LED_CLR: u8;

    const LED_CH_NO: u8;
}

pub trait Led {
    fn get_led(&self, channel: u8) -> Result<bool>;
    fn get_led_cmd(&self) -> Command;
    fn set_led(&self, channel: u8, state: bool) -> Result<()>;
    fn set_led_cmd(&self) -> Command;
}

impl<T> Led for T
where
    T: LedInfo + Card,
{
    fn get_led_cmd(&self) -> Command {
        Command::new("get-led")
            .about("Get LED state for specified channel")
            .arg(args::channel(Self::LED_CH_NO))
    }

    fn get_led(&self, channel: u8) -> Result<bool> {
        let val = self.read_u8(Self::LED + channel / 8)?;
        Ok(val & (1 << channel) != 0)
    }

    fn set_led_cmd(&self) -> Command {
        Command::new("set-led")
            .about("Set LED state for specified channel")
            .arg(args::channel(Self::LED_CH_NO))
            .arg(args::state_on_off())
    }

    fn set_led(&self, channel: u8, state: bool) -> Result<()> {
        if state {
            self.write_u8(Self::LED_SET, channel)?;
        } else {
            self.write_u8(Self::LED_CLR, channel)?;
        }
        Ok(())
    }
}
