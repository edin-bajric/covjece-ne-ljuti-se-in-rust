extern crate rand;

use rand::Rng;
use std::io;
use inline_colorization::*;

fn print_logo() {
    let logo = r#"
     ____           _                              _  _       _   _            
    / ___|_____   _(_) ___  ___ ___   _ __   ___  | |(_)_   _| |_(_)  ___  ___ 
   | |   / _ \ \ / / |/ _ \/ __/ _ \ | '_ \ / _ \ | || | | | | __| | / __|/ _ \
   | |__| (_) \ V /| |  __/ (_|  __/ | | | |  __/ | || | |_| | |_| | \__ \  __/
    \____\___/ \_/_/ |\___|\___\___| |_| |_|\___| |_|/ |\__,_|\__|_| |___/\___|
                 |__/                              |__/                        

    "#;
    
    println!("{color_cyan}{}{color_reset}", logo);
}

fn choose_number_of_players() -> u32 {
    loop {
        let mut number_of_players = String::new();
        println!("Choose number of players (must be between 2 and 4): ");
        io::stdin().read_line(&mut number_of_players).expect("Failed to read line");

        let number: u32 = match number_of_players.trim().parse() {
            Ok(parsed) => parsed,
            Err(_) => {
                println!("Invalid input. Please enter a valid u32 number.");
                continue;
            }
        };

        if (2..=4).contains(&number) {
            return number;
        } else {
            println!("Number out of range. Please enter a number between 2 and 4.");
        }
    }
}


fn roll() -> u32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..=6);
}

fn main() {
    print_logo();
    choose_number_of_players();

}
