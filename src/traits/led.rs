use crate::{Error, Result};
use clap::Command;

use crate::cli::args;
use crate::traits::Card;

pub trait LedCapabilities {
    fn as_led(&self) -> Option<&dyn Led> {
        None
    }
    fn as_led_mode(&self) -> Option<&dyn LedMode> {
        None
    }
}

pub trait LedInfo {
    const LED: u8;
    const LED_SET: u8;
    const LED_CLR: u8;

    const LED_CH_NO: u8;
}

pub trait LedModeInfo {
    const LED_MODE: u8;
}

pub trait Led {
    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()>;
    fn led_cmd(&self) -> Command;

    fn get_led_cmd(&self) -> Command;
    fn get_led(&self, channel: u8) -> Result<bool>;

    fn set_led_cmd(&self) -> Command;
    fn set_led(&self, channel: u8, state: bool) -> Result<()>;
}

impl<T> Led for T
where
    T: LedInfo + Card,
{
    fn led_cmd(&self) -> Command {
        Command::new("led")
            .about("LED control commands")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(self.get_led_cmd())
            .subcommand(self.set_led_cmd())
    }

    fn get_led_cmd(&self) -> Command {
        Command::new("get")
            .about("Get LED state for specified channel")
            .arg(args::channel(Self::LED_CH_NO))
    }

    fn get_led(&self, channel: u8) -> Result<bool> {
        let val = self.read_u8(Self::LED + (channel - 1) / 8)?;
        Ok(val & (1 << (channel - 1)) != 0)
    }

    fn set_led_cmd(&self) -> Command {
        Command::new("set")
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

    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("get", sub_m)) => {
                let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
                let state = self.get_led(channel)?;
                println!("LED channel {} state: {}", channel, state);
                Ok(())
            }
            Some(("set", sub_m)) => {
                let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
                let state: bool = *sub_m.get_one::<bool>("state").unwrap();
                self.set_led(channel, state)?;
                println!("LED channel {} set to {}", channel, state);
                Ok(())
            }
            Some((unimplemented, _)) => unimplemented!("Unimplemented led command: {}", unimplemented),
            None => Err(Error::UnknownCommand {
                command: "led: Subcommand required or else help!!".to_string(),
            }),
        }
    }
}

pub trait LedMode {
    fn get_led_mode_cmd(&self) -> Command;
    fn get_led_mode(&self, channel: u8) -> Result<u8>;

    fn set_led_mode_cmd(&self) -> Command;
    fn set_led_mode(&self, channel: u8, mode: u8) -> Result<()>;
}

/*

impl<T> LedMode for T
where
    T: LedModeInfo + Card,
{
    fn get_led_mode_cmd(&self) -> Command {
        Command::new("get-mode")
            .about("Get LED mode for specified channel")
            .arg(args::channel(Self::LED_MODE))
    }

    fn get_led_mode(&self, channel: u8) -> Result<u8> {
        let val = self.read_u8(Self::LED_MODE + (channel - 1))?;
        Ok(val)
    }

    fn set_led_mode_cmd(&self) -> Command {
        Command::new("set-mode")
            .about("Set LED mode for specified channel")
            .arg(args::channel(Self::LED_MODE))
            .arg(args::mode().help("LED mode (0: Off, 1: On, 2: Heartbeat, 3: Blink)"))
    }

    fn set_led_mode(&self, channel: u8, mode: u8) -> Result<()> {
        self.write_u8(Self::LED_MODE + (channel - 1), mode)?;
        Ok(())
    }
}

*/
