use clap::Command;

use crate::cli::args;
use crate::traits::Card;
use crate::{Error, Result};

pub trait RelayInfo {
    const RELAY: u8;
    const RELAY_SET: u8;
    const RELAY_CLR: u8;

    const RELAY_CH_NO: u8;
}

pub trait Relay {
    fn get_relay(&self, channel: u8) -> Result<bool>;
    fn get_relay_cmd(&self) -> Command;
    fn set_relay(&self, channel: u8, state: bool) -> Result<()>;
    fn set_relay_cmd(&self) -> Command;
}

impl<T> Relay for T
where
    T: RelayInfo + Card,
{
    fn get_relay_cmd(&self) -> Command {
        Command::new("get-relay")
            .about("Get relay state for specified channel")
            .arg(args::channel(Self::RELAY_CH_NO))
    }

    fn get_relay(&self, channel: u8) -> Result<bool> {
        let val = self.read_u8(Self::RELAY + channel / 8)?;
        Ok(val & (1 << channel) != 0)
    }

    fn set_relay_cmd(&self) -> Command {
        Command::new("set-relay")
            .about("Set relay state for specified channel")
            .arg(args::channel(Self::RELAY_CH_NO))
            .arg(args::state_on_off())
    }

    fn set_relay(&self, channel: u8, state: bool) -> Result<()> {
        /*
        let val = self.read_u8(Self::RELAY + channel / 8)?;
        if (val & (1 << channel) != 0) == state {
            return Ok(());
        }
        */
        if state {
            self.write_u8(Self::RELAY_SET, channel)?;
        } else {
            self.write_u8(Self::RELAY_CLR, channel)?;
        }
        Ok(())
    }
}
