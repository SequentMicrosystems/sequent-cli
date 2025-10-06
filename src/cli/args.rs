use clap::{Arg, Command, builder::BoolishValueParser};
use clap::{
    builder::TypedValueParser,
    error::{Error, ErrorKind},
};

use std::ffi::OsStr;

pub fn channel(no_channels: u8) -> Arg {
    Arg::new("channel")
        .value_name("CHANNEL")
        .required(true)
        .value_parser(clap::value_parser!(u8).range(1..=no_channels as i64))
        .help(format!("Channel number (1-{})", no_channels))
}

pub fn state_on_off() -> Arg {
    Arg::new("state")
        .value_name("STATE")
        .required(true)
        .help(format!("State: on/off, true/false, yes/no, 1/0"))
        // accepts true/false, yes/no, on/off, 1/0
        .value_parser(BoolishValueParser::new())
}

pub fn led_blink_modes(number_of_modes: u8) -> Arg {
    if number_of_modes == 2 {
        return Arg::new("mode")
            .value_name("MODE")
            .required(true)
            .help("LED mode (0=auto, 1=manual)")
            .value_parser(clap::value_parser!(u8).range(0..=(number_of_modes - 1) as i64));
    } else {
        unimplemented!("Only 2 LED modes supported");
    }
}

pub fn interval(max_interval: u64) -> Arg {
    Arg::new("interval")
        .value_name("INTERVAL")
        .required(true)
        .help(format!("Interval in seconds (1-{})", max_interval))
        .value_parser(clap::value_parser!(u64).range(1..=max_interval))
}

#[derive(Clone)]
struct FloatRangeParser {
    min: f32,
    max: f32,
}

impl TypedValueParser for FloatRangeParser {
    type Value = f32;

    fn parse_ref(&self, _cmd: &Command, _arg: Option<&clap::Arg>, value: &OsStr) -> Result<f32, Error> {
        let s = value.to_str().ok_or_else(|| clap::error::Error::new(ErrorKind::InvalidUtf8))?;
        let v: f32 = s
            .parse()
            .map_err(|_| Error::raw(ErrorKind::InvalidValue, format!("not a number: {s}")))?;
        if v.is_finite() && v >= self.min && v <= self.max {
            Ok(v)
        } else {
            Err(Error::raw(
                ErrorKind::ValueValidation,
                format!("must be between {} and {}", self.min, self.max),
            ))
        }
    }
}

pub fn resistance(min_value: f32, max_value: f32) -> Arg {
    Arg::new("resistance")
        .value_name("RESISTANCE")
        .required(true)
        .help("Resistance in Ohms (e.g. 100.0)")
        .value_parser(FloatRangeParser {
            min: min_value,
            max: max_value,
        })
}
