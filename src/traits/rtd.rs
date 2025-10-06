use crate::cli::args;
use crate::{Error, Result};

use crate::traits::Card;
use clap::Command;

pub trait RtdInfo {
    const RTD: u8;
    const RTD_RESISTANCE: u8;

    const RTD_CH_NO: u8;
    const RTD_SIZE: u8;
}

pub trait Rtd {
    fn rtd_cmd(&self) -> Command;
    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()>;

    fn get_rtd_cmd(&self) -> Command;
    fn get_rtd(&self, channel: u8) -> Result<f32>;

    fn get_rtd_resistance_cmd(&self) -> Command;
    fn get_rtd_resistance(&self, channel: u8) -> Result<u32>;

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
            .subcommands([
                self.get_rtd_cmd(),
                self.set_rtd_cmd(),
                self.get_rtd_resistance_cmd(),
                self.set_rtd_resistance_cmd(),
            ])
    }

    fn get_rtd_cmd(&self) -> Command {
        Command::new("get")
            .about("Get RTD value")
            .arg(args::channel(Self::RTD_CH_NO))
    }
    fn get_rtd(&self, channel: u8) -> Result<f32> {
        self.read_f_n(Self::RTD, channel, Self::RTD_SIZE)
    }
    
    fn get_rtd_resistance_cmd(&self) -> Command {
        Command::new("get-resistance")
            .about("Get RTD resistance calibration value")
            .arg(args::channel(Self::RTD_CH_NO))
    }
    fn get_rtd_resistance(&self, channel: u8) -> Result<f32> {
        self.read_f_n(Self::RTD_RESISTANCE, channel, Self::RTD_SIZE)
    }
}