use sequent_cli::Result;
use sequent_cli::cards::Industrial;
use sequent_cli::cli;

fn main() -> Result<()> {
    let mut industrial = Industrial::new(0);
    let cmd = cli::build_command(&industrial);
    let matches = cmd.get_matches();
    cli::run_command(&mut industrial, matches)?;
    Ok(())
}
