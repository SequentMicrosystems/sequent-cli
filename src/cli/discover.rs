use clap::{Arg, Command};

pub fn build_discover_command() -> Command {
    Command::new("discover")
        .about("Discover connected Sequent Microsystems HATs")
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .value_name("SECONDS")
                .help("Timeout in seconds for discovery")
                .default_value("5")
                .value_parser(clap::value_parser!(u64)),
        )
}
