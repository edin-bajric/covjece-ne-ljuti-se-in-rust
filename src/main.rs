extern crate rand;

use rand::Rng;

fn print_logo() {
    let logo = r#"
     _               _       
    | |             | |      
    | |    _   _  __| | ___  
    | |   | | | |/ _` |/ _ \ 
    | |___| |_| | (_| | (_) |
    |______\__,_|\__,_|\___/ 

    "#;
    
    println!("{}", logo);
}

fn roll() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}
fn main() {
    print_logo();
}
