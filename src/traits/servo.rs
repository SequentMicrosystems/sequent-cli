use crate::{Error, Result};
use clap::Command;

use crate::cli::args;
use crate::traits::Card;

pub trait ServoInfo {
    const SERVO: u8;

    const SERVO_CH_NO: u8;
    const SERVO_SIZE: u8;

    const SERVO_SCALE: f32 = 10.0;
}

pub trait Servo {
    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()>;
    fn servo_cmd(&self) -> Command;

    fn get_servo_cmd(&self) -> Command;
    fn get_servo(&self, channel: u8) -> Result<f32>;

    fn set_servo_cmd(&self) -> Command;
    fn set_servo(&self, channel: u8, value: f32) -> Result<()>;
}

impl<T> Servo for T
where
    T: ServoInfo + Card,
{
    fn servo_cmd(&self) -> Command {
        Command::new("servo")
            .about("Servo control commands")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(self.get_servo_cmd())
            .subcommand(self.set_servo_cmd())
    }

    fn get_servo_cmd(&self) -> Command {
        Command::new("get")
            .about("Get servo value")
            .arg(args::channel(Self::SERVO_CH_NO))
    }

    fn get_servo(&self, channel: u8) -> Result<f32> {
        self.read_i_n(Self::SERVO + (channel - 1) * Self::SERVO_SIZE, Self::SERVO_SIZE)
            .map(|v| (v as f32) / Self::SERVO_SCALE)
    }

    fn set_servo_cmd(&self) -> Command {
        Command::new("set")
            .about("Set servo position (-100..100) for standard and (-120..120) for extended range servos")
            .arg(args::channel(Self::SERVO_CH_NO))
            .arg(args::value_percentage(-120.0, 120.0))
    }

    fn set_servo(&self, channel: u8, value: f32) -> Result<()> {
        let scaled_value = (value * Self::SERVO_SCALE) as i32;
        self.write_i_n(
            Self::SERVO + (channel - 1) * Self::SERVO_SIZE,
            Self::SERVO_SIZE,
            scaled_value,
        )
    }

    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("get", sub_m)) => {
                let channel = *sub_m.get_one::<u8>("channel").unwrap();
                let value = self.get_servo(channel)?;
                println!("Servo[{}]: {:.1}%", channel, value);
                Ok(())
            }
            Some(("set", sub_m)) => {
                let channel = *sub_m.get_one::<u8>("channel").unwrap();
                let value = *sub_m.get_one::<f32>("value").unwrap();
                self.set_servo(channel, value)?;
                println!("Servo[{}] set to {:.1}%", channel, value);
                Ok(())
            }
            // TODO: Change this ugly catch all
            Some((unimplemented, _)) => unimplemented!("Unimplemented servo command: {}", unimplemented),
            None => Err(Error::UnknownCommand {
                command: "servo: Subcommand required or else help!!".to_string(),
            }),
        }
    }
}
