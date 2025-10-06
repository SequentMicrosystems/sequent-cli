use crate::{Error, Result};
use clap::Command;

use crate::cli::args;
use crate::traits::Card;

pub trait MotorInfo {
    const MOTOR: u8;

    const MOTOR_CH_NO: u8;
    const MOTOR_SIZE: u8;

    const MOTOR_SCALE: f32 = 10.0;
}

pub trait Motor {
    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()>;
    fn motor_cmd(&self) -> Command;

    fn get_motor_cmd(&self) -> Command;
    fn get_motor(&self, channel: u8) -> Result<f32>;

    fn set_motor_cmd(&self) -> Command;
    fn set_motor(&self, channel: u8, value: f32) -> Result<()>;
}

impl<T> Motor for T
where
    T: MotorInfo + Card,
{
    fn motor_cmd(&self) -> Command {
        Command::new("motor")
            .about("Motor control commands")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(self.get_motor_cmd())
            .subcommand(self.set_motor_cmd())
    }

    fn get_motor_cmd(&self) -> Command {
        Command::new("get")
            .about("Get motor value")
            .arg(args::channel(Self::MOTOR_CH_NO))
    }

    fn get_motor(&self, channel: u8) -> Result<f32> {
        self.read_i_n(Self::MOTOR + (channel - 1) * Self::MOTOR_SIZE, Self::MOTOR_SIZE)
            .map(|v| (v as f32) / Self::MOTOR_SCALE)
    }

    fn set_motor_cmd(&self) -> Command {
        Command::new("set")
            .about("Set motor PWM fill factor (-100..100)%")
            .arg(args::channel(Self::MOTOR_CH_NO))
            .arg(args::value_percentage(-100.0, 100.0))
    }

    fn set_motor(&self, channel: u8, value: f32) -> Result<()> {
        let scaled_value = (value * Self::MOTOR_SCALE) as i32;
        self.write_i_n(
            Self::MOTOR + (channel - 1) * Self::MOTOR_SIZE,
            Self::MOTOR_SIZE,
            scaled_value,
        )
    }

    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("get", sub_m)) => {
                // TODO: Consider having an args::get_channel(sub_m, ...)
                let channel = *sub_m.get_one::<u8>("channel").unwrap();
                let value = self.get_motor(channel)?;
                println!("Motor[{}]: {:.1}%", channel, value);
                Ok(())
            }
            Some(("set", sub_m)) => {
                let channel = *sub_m.get_one::<u8>("channel").unwrap();
                let value = *sub_m.get_one::<f32>("value").unwrap();
                self.set_motor(channel, value)?;
                println!("Motor[{}] set to {:.1}%", channel, value);
                Ok(())
            }
            // TODO: Change this ugly catch all
            Some((unimplemented, _)) => unimplemented!("Unimplemented led command: {}", unimplemented),
            None => Err(Error::UnknownCommand {
                command: "led: Subcommand required or else help!!".to_string(),
            }),
        }
    }
}
