use crate::cli::args;
use crate::{Error, Result};

use crate::traits::Card;
use clap::Command;

pub trait OptoInfo {
    const OPTO: u8;
    const RISING: u8;
    const FALLING: u8;
    const ENCODER_ENABLE: u8;
    const COUNTER_RESET: u8;
    const ENCODER_COUNTER_RESET: u8;
    const EDGE_COUNT: u8;
    const ENCODER_COUNT: u8;

    // board parameters
    const OPTO_CH_NO: u8;
    const COUNTER_SIZE: u8;
}

pub trait Opto {
    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()>;
    fn opto_cmd(&self) -> Command;

    fn get_opto_cmd(&self) -> Command;
    fn get_opto(&self, channel: u8) -> Result<bool>;

    fn get_counter_cmd(&self) -> Command;
    fn get_counter(&self, channel: u8) -> Result<u32>;

    fn get_encoder_count_cmd(&self) -> Command;
    fn get_encoder_count(&self, channel: u8) -> Result<i32>;

    fn reset_counter_cmd(&self) -> Command;
    fn reset_counter(&self, channel: u8) -> Result<()>;

    fn reset_encoder_counter_cmd(&self) -> Command;
    fn reset_encoder_counter(&self, channel: u8) -> Result<()>;

    fn cfg_edge_cmd(&self) -> Command;
    fn cfg_edge(&self, channel: u8, rising: bool, falling: bool) -> Result<()>;

    fn cfg_encoder_cmd(&self) -> Command;
    fn cfg_encoder(&self, channel: u8, enable: bool) -> Result<()>;
}

impl<T> Opto for T
where
    T: OptoInfo + Card,
{
    fn opto_cmd(&self) -> Command {
        Command::new("opto")
            .about("Opto input and encoder commands")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommands([
                self.get_opto_cmd(),
                self.get_counter_cmd(),
                self.get_encoder_count_cmd(),
                self.reset_counter_cmd(),
                self.reset_encoder_counter_cmd(),
                self.cfg_edge_cmd(),
                self.cfg_encoder_cmd(),
            ])
    }

    fn get_opto_cmd(&self) -> Command {
        Command::new("get")
            .about("Get opto input state for specified channel")
            .arg(args::channel(Self::OPTO_CH_NO))
    }
    fn get_opto(&self, channel: u8) -> Result<bool> {
        let val = self.read_u8(Self::OPTO + (channel - 1) / 8)?;
        Ok(val & (1 << (channel - 1)) != 0)
    }

    fn get_counter_cmd(&self) -> Command {
        Command::new("get-counter")
            .about("Get opto edge counter value for specified channel")
            .visible_alias("getc")
            .arg(args::channel(Self::OPTO_CH_NO))
    }
    fn get_counter(&self, channel: u8) -> Result<u32> {
        let val = self.read_u_n(
            Self::EDGE_COUNT + (channel - 1) * Self::COUNTER_SIZE,
            Self::COUNTER_SIZE,
        )?;
        Ok(val)
    }

    fn get_encoder_count_cmd(&self) -> Command {
        Command::new("get-encoder")
            .about("Get encoder counter value for specified channel")
            .arg(args::channel(Self::OPTO_CH_NO))
    }
    fn get_encoder_count(&self, channel: u8) -> Result<i32> {
        let val = self.read_i_n(
            Self::ENCODER_COUNT + (channel - 1) * Self::COUNTER_SIZE,
            Self::COUNTER_SIZE,
        )?;
        Ok(val)
    }

    fn reset_counter_cmd(&self) -> Command {
        Command::new("reset-counter")
            .about("Reset opto edge counter for specified channel")
            .arg(args::channel(Self::OPTO_CH_NO))
    }
    fn reset_counter(&self, channel: u8) -> Result<()> {
        self.write_u8(Self::COUNTER_RESET, channel)?;
        Ok(())
    }

    fn reset_encoder_counter_cmd(&self) -> Command {
        Command::new("reset-encoder")
            .about("Reset encoder counter for specified channel")
            .arg(args::channel(Self::OPTO_CH_NO))
    }
    fn reset_encoder_counter(&self, channel: u8) -> Result<()> {
        self.write_u8(Self::ENCODER_COUNTER_RESET, channel)?;
        Ok(())
    }

    fn cfg_edge_cmd(&self) -> Command {
        Command::new("cfg-edge")
            .about("Configure opto edge detection for specified channel")
            .arg(args::channel(Self::OPTO_CH_NO))
            .arg(
                clap::arg!(--rising "Enable rising edge detection")
                    .required(false)
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                clap::arg!(--falling "Enable falling edge detection")
                    .required(false)
                    .action(clap::ArgAction::SetTrue),
            )
    }
    fn cfg_edge(&self, channel: u8, rising: bool, falling: bool) -> Result<()> {
        let byte = (channel - 1) / 8;
        let bit = (channel - 1) % 8;
        self.write_bit(Self::RISING + byte, bit, rising)?;
        self.write_bit(Self::FALLING + byte, bit, falling)?;
        Ok(())
    }

    fn cfg_encoder_cmd(&self) -> Command {
        Command::new("cfg-encoder")
            .about("Configure encoder mode for specified channel")
            .arg(args::channel(Self::OPTO_CH_NO))
            .arg(
                clap::arg!(--enable "Enable encoder mode")
                    .required(false)
                    .action(clap::ArgAction::SetTrue),
            )
    }
    fn cfg_encoder(&self, channel: u8, enable: bool) -> Result<()> {
        self.write_bit(Self::ENCODER_ENABLE + (channel - 1) / 8, (channel - 1) % 8, enable)?;
        Ok(())
    }

    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("get", sub_m)) => {
                let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
                let state = self.get_opto(channel)?;
                println!("Opto channel {} state: {}", channel, state);
                Ok(())
            }
            Some(("get-counter", sub_m)) => {
                let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
                let count = self.get_counter(channel)?;
                println!("Opto channel {} counter: {}", channel, count);
                Ok(())
            }
            Some(("get-encoder", sub_m)) => {
                let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
                let count = self.get_encoder_count(channel)?;
                println!("Encoder channel {} count: {}", channel, count);
                Ok(())
            }
            Some(("reset-counter", sub_m)) => {
                let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
                self.reset_counter(channel)?;
                println!("Opto channel {} counter reset", channel);
                Ok(())
            }
            Some(("reset-encoder", sub_m)) => {
                let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
                self.reset_encoder_counter(channel)?;
                println!("Encoder channel {} counter reset", channel);
                Ok(())
            }
            Some(("cfg-edge", sub_m)) => {
                let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
                let rising: bool = *sub_m.get_one::<bool>("rising").unwrap_or(&false);
                let falling: bool = *sub_m.get_one::<bool>("falling").unwrap_or(&false);
                self.cfg_edge(channel, rising, falling)?;
                println!(
                    "Opto channel {} edge config updated (rising: {}, falling: {})",
                    channel, rising, falling
                );
                Ok(())
            }
            Some(("cfg-encoder", sub_m)) => {
                let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
                let enable: bool = *sub_m.get_one::<bool>("enable").unwrap_or(&false);
                self.cfg_encoder(channel, enable)?;
                println!(
                    "Encoder channel {} encoder mode {}",
                    channel,
                    if enable { "enabled" } else { "disabled" }
                );
                Ok(())
            }
            Some((unimplemented, _)) => unimplemented!("Unimplemented opto command: {}", unimplemented),
            None => Err(Error::UnknownCommand {
                command: "opto: Subcommand required or else help!!".to_string(),
            }),
        }
    }
}
