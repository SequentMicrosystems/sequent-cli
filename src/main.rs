// Import Opto trait
mod cards;
mod error;
mod traits;
#[macro_use]
mod cli;

use cards::MultiIo;
use error::{Error, Result};

use crate::cli::build_command::Capabilities;

fn main() {
    let device = MultiIo::new(0);
    let cmd = cli::build_command(&device);
    let matches = cmd.get_matches();
    cli::run_command(&device, matches).unwrap();
}
