// Import Opto trait
mod traits;
mod cards;

use traits::Opto;
use cards::MultiIo;

fn main() {
    let multiio = MultiIo::new();
    println!("{}", multiio.read_opto(1).unwrap());
}
