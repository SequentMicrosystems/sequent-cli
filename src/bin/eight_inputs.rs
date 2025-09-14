use sequent_cli::Result;
use sequent_cli::cards::EightInputs;
use sequent_cli::cli;

fn main() -> Result<()> {
    let mut eight_inputs = EightInputs::new(0);
    let cmd = cli::build_command(&eight_inputs);
    let matches = cmd.get_matches();
    cli::run_command(&mut eight_inputs, matches)?;
    Ok(())
}
