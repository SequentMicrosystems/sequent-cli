use clap::{Arg, Command, builder::BoolishValueParser};

pub fn channel(no_channels: u8) -> Arg {
    Arg::new("channel")
        .value_name("CHANNEL")
        .required(true)
        .value_parser(clap::value_parser!(u8).range(0..(no_channels - 1) as i64))
        .help(format!("Channel number (0-{})", no_channels - 1))
}

pub fn state_on_off() -> Arg {
    Arg::new("state")
        .value_name("STATE")
        .required(true)
        .help(format!("State: on/off, true/false, yes/no, 1/0"))
        // accepts true/false, yes/no, on/off, 1/0
        .value_parser(BoolishValueParser::new())
}

pub fn interval(max_interval: u64) -> Arg {
    Arg::new("interval")
        .value_name("INTERVAL")
        .required(true)
        .help(format!("Interval in seconds (1-{})", max_interval))
        .value_parser(clap::value_parser!(u64).range(1..=max_interval))
}
