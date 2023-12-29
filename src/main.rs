use rand::Rng;
use std::io;
use inline_colorization::*;

struct Player {
    color: String,
    pawns: [u32; 4],
}

impl Player {
    fn new(color: String) -> Self {
        Player {
            color,
            pawns: [0; 4],
        }
    }
    fn roll(&mut self) -> u32 {
        println!("\nPress 'Enter' to roll the dice for player {}...", self.color);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=6)
    }
}

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
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

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

fn choose_player_colors(number_of_players: u32) -> Vec<Player> {
    let mut players = Vec::new();
    let available_colors = vec!["red", "blue", "green", "yellow"];

    for player in 1..=number_of_players {
        println!(
            "player {}: Choose your color ({color_red}red, {color_blue}blue, {color_green}green, {color_reset}or {color_yellow}yellow{color_reset}):",
            player
        );

        let chosen_color = loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let chosen_color = input.trim().to_string();

            if available_colors.contains(&chosen_color.as_str()) {
                if players.iter().any(|p: &Player| p.color == chosen_color) {
                    println!("Color already chosen. Please choose a different color.");
                } else {
                    break chosen_color;
                }
            } else {
                println!("Invalid color. Choose from red, blue, green, or yellow.");
            }
        };

        let player = Player::new(chosen_color);
        players.push(player);
    }

    players
}


fn initial_rolls(players: &mut Vec<Player>) {
    for player in players.iter_mut() {
        let mut counter = 0;

        while counter < 3 {
            let roll_result = player.roll();
            println!("{} rolled: {}", player.color, roll_result);

            if roll_result == 6 {
                println!("{} put a pawn on the board", player.color);
                player.pawns[0] = 1;
                break;
            }

            counter += 1;
        }
        println!("pawn positions on the board - {:?}", player.pawns);
    }
}

fn run_game() {
    clearscreen::clear().expect("failed to clear screen");
    print_logo();
    let number_of_players = choose_number_of_players();
    let mut players = choose_player_colors(number_of_players);

    println!("Players and their chosen colors:");
    for (i, player) in players.iter_mut().enumerate() {
        println!("Player {}: {} | pawn positions on the board - {:?}", i + 1, player.color, player.pawns);
    }

    initial_rolls(&mut players);
}

fn main() {
    run_game();
}

