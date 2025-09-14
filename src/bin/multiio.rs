use sequent_cli::Result;
use sequent_cli::cards::MultiIo;
use sequent_cli::cli;

fn main() -> Result<()> {
    let mut multiio = MultiIo::new(0);
    let cmd = cli::build_command(&multiio);
    let matches = cmd.get_matches();
    cli::run_command(&mut multiio, matches)?;
    Ok(())
}
