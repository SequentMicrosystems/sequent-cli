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
    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()>;
    fn relay_cmd(&self) -> Command;

    fn get_relay_cmd(&self) -> Command;
    fn get_relay(&self, channel: u8) -> Result<bool>;

    fn set_relay_cmd(&self) -> Command;
    fn set_relay(&self, channel: u8, state: bool) -> Result<()>;
}

impl<T> Relay for T
where
    T: RelayInfo + Card,
{
    fn relay_cmd(&self) -> Command {
        Command::new("relay")
            .about("Relay control commands")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(self.get_relay_cmd())
            .subcommand(self.set_relay_cmd())
    }

    fn get_relay_cmd(&self) -> Command {
        Command::new("get")
            .about("Get relay state for specified channel")
            .arg(args::channel(Self::RELAY_CH_NO))
    }

    fn get_relay(&self, channel: u8) -> Result<bool> {
        let val = self.read_u8(Self::RELAY + (channel - 1) / 8)?;
        Ok(val & (1 << (channel - 1)) != 0)
    }

    fn set_relay_cmd(&self) -> Command {
        Command::new("set")
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

    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("get", sub_m)) => {
                let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
                let state = self.get_relay(channel)?;
                println!("Relay channel {} state: {}", channel, state);
                Ok(())
            }
            Some(("set", sub_m)) => {
                let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
                let state: bool = *sub_m.get_one::<bool>("state").unwrap();
                self.set_relay(channel, state)?;
                println!("Relay channel {} set to {}", channel, state);
                Ok(())
            }
            Some((unimplemented, _)) => unimplemented!("Unimplemented relay command: {}", unimplemented),
            None => Err(Error::UnknownCommand {
                command: "relay: Subcommand required or else help!!".to_string(),
            }),
        }
    }
}
