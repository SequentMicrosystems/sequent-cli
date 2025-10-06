use crate::{Error, Result};
use clap::Command;

use crate::cli::args;
use crate::traits::Card;

pub trait Analog {
    fn read_analog(&self, channel: u8) -> Result<f32>;
    fn write_analog(&self, channel: u8, value: f32) -> Result<()>;
}

pub trait UnitSpec {
    const NAME: &'static str; // "voltage", "miliampere"
    const SYMBOL: &'static str; // "V", "mA"
    const SCALE: i32; // 100 for two decimal places
    const MIN: i32;
    const MAX: i32;
}

pub struct Volt<const MIN: i32, const MAX: i32, const SCALE: i32>;
impl<const MIN: i32, const MAX: i32, const SCALE: i32> UnitSpec for Volt<MIN, MAX, SCALE> {
    const NAME: &'static str = "voltage";
    const SYMBOL: &'static str = "V";
    const SCALE: i32 = SCALE;
    const MIN: i32 = MIN;
    const MAX: i32 = MAX;
}

pub type V0_10 = Volt<0, { 10 * 100 }, 100>;

pub trait OutputInfo<U: UnitSpec> {
    const CMD_NAME: &'static str;
    const VALUE_ADDR: u8;
    const VALUE_SIZE: u8;
    const VALUE_SCALE: f32 = 100.0;
    const CH_NO: u8;
}

// TODO: Trait alliases are experimental
pub trait OutputV0_10: Output<V0_10> {}

pub trait Output<U: UnitSpec> {
    fn output_cmd(&self) -> Command;
    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()>;

    fn get_output_cmd(&self) -> Command;
    fn get_output(&self, channel: u8) -> Result<f32>;

    fn set_output_cmd(&self) -> Command;
    fn set_output(&self, channel: u8, value: f32) -> Result<()>;
}

impl<T, U> Output<U> for T
where
    T: OutputInfo<U> + Card,
    U: UnitSpec,
{
    // TODO: Change everywhere to parent_cmd or smthing
    fn output_cmd(&self) -> Command {
        Command::new(T::CMD_NAME)
            .about("Output control commands")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(self.get_output_cmd())
            .subcommand(self.set_output_cmd())
    }
    fn get_output_cmd(&self) -> Command {
        Command::new("get")
            // TODO: Fix the description to include the Unit range
            .about("Get analog output value")
            .arg(args::channel(Self::CH_NO))
    }

    fn get_output(&self, channel: u8) -> Result<f32> {
        self.read_i_n(Self::VALUE_ADDR + (channel - 1) * Self::VALUE_SIZE, Self::VALUE_SIZE)
            .map(|v| (v as f32) / Self::VALUE_SCALE)
    }

    fn set_output_cmd(&self) -> Command {
        Command::new("set")
            .about("Set analog output value")
            .arg(args::channel(Self::CH_NO))
            .arg(args::value_range_with_unit(
                U::MIN as f32 / U::SCALE as f32,
                U::MAX as f32 / U::SCALE as f32,
                U::NAME,
            ))
    }
    fn set_output(&self, channel: u8, value: f32) -> Result<()> {
        let scaled_value = (value * Self::VALUE_SCALE) as i32;
        self.write_i_n(
            Self::VALUE_ADDR + (channel - 1) * Self::VALUE_SIZE,
            Self::VALUE_SIZE,
            scaled_value,
        )
    }

    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("get", sub_m)) => {
                let channel = *sub_m.get_one::<u8>("channel").unwrap();
                let value = self.get_output(channel)?;
                println!("Channel {}: {:.2} {}", channel, value, U::SYMBOL);
                Ok(())
            }
            Some(("set", sub_m)) => {
                let channel = *sub_m.get_one::<u8>("channel").unwrap();
                let value = *sub_m.get_one::<f32>("value").unwrap();
                self.set_output(channel, value)?;
                println!("Channel {} set to {:.2} {}", channel, value, U::SYMBOL);
                Ok(())
            }
            Some((unimplemented, _)) => unimplemented!("Unimplemented led command: {}", unimplemented),
            None => Err(Error::UnknownCommand {
                command: "led: Subcommand required or else help!!".to_string(),
            }),
        }
    }
}
