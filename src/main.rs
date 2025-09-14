mod cards;
mod error;
mod traits;
#[macro_use]
mod cli;

use cards::{Industrial, MultiIo};
use clap::Command;
use error::{Error, Result};

use crate::{cards::EightInputs, cli::build_command::Capabilities};

fn main() -> Result<()> {
    let mut multiio = MultiIo::new(0);
    let mut industrial = Industrial::new(0);
    let mut eight_inputs = EightInputs::new(0);
    let cmd1 = cli::build_command(&multiio);
    let cmd2 = cli::build_command(&industrial);
    let cmd3 = cli::build_command(&eight_inputs);
    let cmd4 = cli::build_discover_command();
    let cmd = Command::new("sequent-cli")
        .about("Universal Sequent Microsystems CLI tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(cmd1)
        .subcommand(cmd2)
        .subcommand(cmd3)
        .subcommand(cmd4);
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("multiio", sub_m)) => cli::run_command(&mut multiio, sub_m.clone()),
        Some(("industrial", sub_m)) => cli::run_command(&mut industrial, sub_m.clone()),
        Some(("8inputs", sub_m)) => cli::run_command(&mut eight_inputs, sub_m.clone()),
        _ => panic!("No valid subcommand was used"),
    }?;
    Ok(())
    //cli::run_command(&mut device, matches).unwrap();
}
