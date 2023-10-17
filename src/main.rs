use rand::Rng;
use std::io;
use inline_colorization::*;
use clearscreen::*;

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
        let mut input = String::new();
        println!("Choose number of players (must be between 2 and 4): ");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let number_of_players: u32 = match input.trim().parse() {
            Ok(parsed) => parsed,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        if (2..=4).contains(&number_of_players) {
            return number_of_players;
        } else {
            println!("Number out of range. Please enter a number between 2 and 4.");
        }
    }
}

fn choose_player_colors(number_of_players: u32) -> Vec<String> {
    let mut player_colors = Vec::new();
    let available_colors = vec!["Red", "Blue", "Green", "Yellow"];

    for player in 1..=number_of_players {
        println!("Player {}: Choose your color ({color_red}Red, {color_blue}Blue, {color_green}Green, or {color_yellow}Yellow{color_reset}):", player);

        let chosen_color = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let chosen_color = input.trim().to_string();

            if available_colors.contains(&chosen_color.as_str()) {
                if player_colors.contains(&chosen_color) {
                    println!("Color already chosen. Please choose a different color.");
                } else {
                    break chosen_color;
                }
            } else {
                println!("Invalid color. Choose from Red, Blue, Green, or Yellow.");
            }
        };

        player_colors.push(chosen_color);
    }

    player_colors
}

fn roll() -> u32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..=6);
}

fn main() {
    clearscreen::clear().expect("failed to clear screen");
    print_logo();
    let number_of_players = choose_number_of_players();
    let player_colors = choose_player_colors(number_of_players);

    println!("Players and their chosen colors:");
    for (i, color) in player_colors.iter().enumerate() {
        println!("Player {}: {}", i + 1, color);
    }

    roll();

}
