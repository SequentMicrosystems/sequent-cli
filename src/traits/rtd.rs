use crate::cli::args;
use crate::{Error, Result};

use crate::traits::{Calib, Card};
use clap::Command;

pub trait RtdInfo {
    const RTD_TEMP: u8;
    const RTD_RES: u8;

    const RTD_CH_NO: u8;
    const RTD_TEMP_SIZE: u8;
    const RTD_RES_SIZE: u8;
}

pub trait RtdCalibInfo {
    const RTD_CALIB: u8;

    const MIN_RESISTANCE: f32 = 1.0;
    const MAX_RESISTANCE: f32 = 1000000.0;
}

pub trait Rtd {
    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()>;

    fn rtd_cmd(&self) -> Command;

    fn get_rtd_cmd(&self) -> Command;
    fn get_rtd(&self, channel: u8) -> Result<f32>;

    fn get_rtd_resistance_cmd(&self) -> Command;
    fn get_rtd_resistance(&self, channel: u8) -> Result<f32>;
}

pub trait RtdCalib {
    fn calib_rtd_cmd(&self) -> Command;
    fn calib_rtd(&self, channel: u8, resistance: f32) -> Result<()>;

    fn reset_calib_rtd_cmd(&self) -> Command;
    fn reset_calib_rtd(&self, channel: u8) -> Result<()>;
}

impl<T> Rtd for T
where
    T: RtdInfo + Card,
{
    fn rtd_cmd(&self) -> Command {
        Command::new("rtd")
            .about("RTD input commands")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommands([self.get_rtd_cmd(), self.get_rtd_resistance_cmd()])
    }

    fn get_rtd_cmd(&self) -> Command {
        Command::new("get").about("Get RTD value").arg(args::channel(Self::RTD_CH_NO))
    }
    fn get_rtd(&self, channel: u8) -> Result<f32> {
        self.read_f_n(Self::RTD_TEMP + (channel - 1) * Self::RTD_TEMP_SIZE, Self::RTD_TEMP_SIZE)
            .map(|v| v as f32)
    }

    fn get_rtd_resistance_cmd(&self) -> Command {
        Command::new("get-resistance")
            .about("Get RTD resistance calibration value")
            .arg(args::channel(Self::RTD_CH_NO))
    }
    fn get_rtd_resistance(&self, channel: u8) -> Result<f32> {
        self.read_f_n(Self::RTD_RES + (channel - 1) * Self::RTD_RES_SIZE, Self::RTD_RES_SIZE)
            .map(|v| v as f32)
    }

    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("get", sub_m)) => {
                let channel = *sub_m.get_one::<u8>("channel").unwrap();
                let value = self.get_rtd(channel)?;
                println!("RTD[{}]: {:.3} °C", channel, value);
                Ok(())
            }
            Some(("get-resistance", sub_m)) => {
                let channel = *sub_m.get_one::<u8>("channel").unwrap();
                let value = self.get_rtd_resistance(channel)?;
                println!("RTD Resistance[{}]: {:.1} Ω", channel, value);
                Ok(())
            }
            Some((unimplemented, _)) => unimplemented!("Unimplemented opto command: {}", unimplemented),
            None => Err(Error::UnknownCommand {
                command: "opto: Subcommand required or else help!!".to_string(),
            }),
        }
    }
}

impl<T> RtdCalib for T
where
    T: RtdInfo + RtdCalibInfo + Card + Calib,
{
    fn calib_rtd_cmd(&self) -> Command {
        Command::new("calib")
            .about("Calibrate RTD input with specified resistance value. Must be done in 2 points, as far aparts as possible")
            .arg(args::channel(Self::RTD_CH_NO))
            .arg(args::resistance(Self::MIN_RESISTANCE, Self::MAX_RESISTANCE))
    }
    fn calib_rtd(&self, channel: u8, resistance: f32) -> Result<()> {
        self.calib_set(Self::RTD_CALIB + (channel - 1), resistance)
    }

    fn reset_calib_rtd_cmd(&self) -> Command {
        Command::new("reset-calib")
            .about("Reset RTD calibration for specified channel")
            .arg(args::channel(Self::RTD_CH_NO))
    }
    fn reset_calib_rtd(&self, channel: u8) -> Result<()> {
        self.calib_reset(Self::RTD_CALIB + (channel - 1))
    }
}
