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

fn move_existing_pawn(player: &mut Player, roll_result: u32) {
    let empty_count = player.pawns.iter().filter(|&&pawn| pawn == 0).count();

    if empty_count == 3 {
        if let Some(non_empty_index) = player.pawns.iter().position(|&pawn| pawn != 0) {
            player.pawns[non_empty_index] += roll_result;
        }
    } else if empty_count < 3 {
        loop {
            println!("Choose which pawn to move (1, 2, 3, or 4): ");
            println!("pawn positions on the board - {:?}", player.pawns);
            let mut pawn_number_input = String::new();
            io::stdin().read_line(&mut pawn_number_input).expect("Failed to read line");
            let pawn_number: usize = match pawn_number_input.trim().parse() {
                Ok(parsed) => parsed,
                Err(_) => {
                    println!("Invalid input. Please enter a number between 1 and 4.");
                    continue;
                }
            };

            if pawn_number >= 1 && pawn_number <= 4 && player.pawns[pawn_number - 1] != 0 {
                player.pawns[pawn_number - 1] += roll_result;
                break;
            } else {
                println!("Invalid pawn number or pawn already at the target position. Try again.");
            }
        }
    }
}

fn place_new_pawn(player: &mut Player) {
    if let Some(empty_index) = player.pawns.iter().position(|&pawn| pawn == 0) {
        player.pawns[empty_index] = 1;
    } else {
        println!("Player has no empty space for a new pawn.");
    }
}

fn turns(players: &mut Vec<Player>) {
    let mut current_player = 0;

    loop {
        let player = &mut players[current_player];

        while handle_roll(player) {}

        println!("pawn positions on the board - {:?}", player.pawns);
        current_player = (current_player + 1) % players.len();
    }
}

fn handle_roll(player: &mut Player) -> bool {
    let roll_result = player.roll();
    println!("{} rolled: {}", player.color, roll_result);

    if roll_result == 6 {
        if player.pawns[0] == 0 {
            player.pawns[0] = 1;
        } else {
            handle_six_roll(player);
        }
        println!("{} got an extra roll!", player.color);
        true
    } else {
        let empty_count = player.pawns.iter().filter(|&&pawn| pawn == 0).count();
        if empty_count == 4 {
            false
        } else if empty_count == 3 {
            if let Some(non_empty_index) = player.pawns.iter().position(|&pawn| pawn != 0) {
                player.pawns[non_empty_index] += roll_result;
            }
            false
        } else if empty_count < 3 {
            move_existing_pawn(player, roll_result);
            false
        } else {
            false
        }
    }
}

fn handle_six_roll(player: &mut Player) {
    
    loop {
        println!("Press 'm' to move an existing pawn or 'p' to place another pawn on the board");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Failed to read line");
        action = action.trim().to_lowercase();

        match action.as_str() {
            "m" => {
                move_existing_pawn(player, 6);
                break;
            }
            "p" => {
                place_new_pawn(player);
                break;
            }
            _ => {
                println!("Invalid option. Please enter 'm' or 'p'.");
            }
        }
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
    turns(&mut players);
}

fn main() {
    run_game();
}

