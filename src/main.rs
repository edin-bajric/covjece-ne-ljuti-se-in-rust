extern crate rand;

use rand::Rng;

fn roll() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}
fn main() {
    
}
