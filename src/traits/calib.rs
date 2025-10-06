use crate::cli::args;
use crate::{Error, Result};

use crate::traits::Card;
use clap::Command;

pub trait CalibInfo {
    const CALIB_VALUE: u8;
    const CALIB_CHANNEL: u8;
    const CALIB_STATUS: u8;

    const CALIBRATION_KEY: u8;
    const RESET_CALIBRATION_KEY: u8;
}

pub trait Calib {
    fn calib_status_cmd(&self) -> Command;
    fn calib_status(&self) -> Result<u8>;

    fn calib_set(&self, channel: u8, value: f32) -> Result<()>;

    fn calib_reset(&self, channel: u8) -> Result<()>;
}

impl<T> Calib for T
where
    T: CalibInfo + Card,
{
    fn calib_status_cmd(&self) -> Command {
        Command::new("status").about("Display current calibration status of device")
    }
    fn calib_status(&self) -> Result<u8> {
        self.read_u8(Self::CALIB_STATUS)
    }

    fn calib_set(&self, channel: u8, value: f32) -> Result<()> {
        let bytes = value.to_le_bytes();
        self.write_bytes(
            Self::CALIB_VALUE,
            &[bytes[0], bytes[1], bytes[2], bytes[3], channel, Self::CALIBRATION_KEY],
        )
    }

    fn calib_reset(&self, channel: u8) -> Result<()> {
        self.write_bytes(Self::CALIB_CHANNEL, &[channel, Self::RESET_CALIBRATION_KEY])
    }
}
