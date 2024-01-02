use rand::Rng;
use std::io;
use inline_colorization::*;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Player {
    color: String,
    pawns: [u32; 4],
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GameState {
    players: Vec<Player>,
    turn_count: usize,
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
        rng.gen_range(1..=1)
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
            let current_position = player.pawns[pawn_number - 1];
            let target_position = current_position + roll_result;

            if target_position > 44 {
                println!("Moving this pawn will exceed the limit of 44. Do you want to place a new pawn? (y/n): ");
                let mut choice = String::new();
                io::stdin().read_line(&mut choice).expect("Failed to read line");
                choice = choice.trim().to_lowercase();

                if choice == "y" {
                    place_new_pawn(player);
                    break;
                } else {
                    println!("Choose another pawn or enter 'y' to place a new pawn.");
                    continue;
                }
            }

            if !is_position_occupied(&player.pawns, target_position) {
                player.pawns[pawn_number - 1] = target_position;
                break;
            } else {
                println!("Another pawn occupies the target position. Choose another pawn.");
            }
        } else {
            println!("Invalid pawn number or pawn not at the target position. Try again.");
        }
    }
}

fn is_position_occupied(pawns: &[u32; 4], target_position: u32) -> bool {
    pawns.iter().any(|&position| position == target_position)
}

fn place_new_pawn(player: &mut Player) {
    if let Some(empty_index) = player.pawns.iter().position(|&pawn| pawn == 0) {
        player.pawns[empty_index] = 1;
    } else {
        println!("Player has no empty space for a new pawn.");
    }
}

fn is_winning_condition(pawns: &[u32; 4]) -> bool {
    let required_numbers = [41, 42, 43, 44];
    required_numbers.iter().all(|&num| pawns.contains(&num))
}

fn turns(game_state: &mut GameState) {
    let mut current_player = 0;

    loop {
        let cloned_game_state = game_state.clone();
        let player = &mut game_state.players[current_player];

        if save_game(&cloned_game_state).is_err() {
            println!("Error saving game.");
        }

        while handle_roll(player, game_state.turn_count) {}

        if is_winning_condition(&player.pawns) {
            display_victory_screen(&player.color);
            break;
        }

        println!(
            "Turn {}: pawn positions on the board - {:?}",
            game_state.turn_count,
            player.pawns
        );

        current_player = (current_player + 1) % game_state.players.len();
        game_state.turn_count += 1;
    }
}

fn display_victory_screen(winning_color: &str) {
    let victory_text = r#"                                                                                                                                             
VVVVVVVV           VVVVVVVV iiii                               tttt                                                                      !!!      
V::::::V           V::::::Vi::::i                           ttt:::t                                                                     !!:!!     
V::::::V           V::::::V iiii                            t:::::t                                                                     !:::!     
V::::::V           V::::::V                                 t:::::t                                                                     !:::!     
 V:::::V           V:::::Viiiiiii     ccccccccccccccccttttttt:::::ttttttt       ooooooooooo   rrrrr   rrrrrrrrryyyyyyy           yyyyyyy!:::!     
  V:::::V         V:::::V i:::::i   cc:::::::::::::::ct:::::::::::::::::t     oo:::::::::::oo r::::rrr:::::::::ry:::::y         y:::::y !:::!     
   V:::::V       V:::::V   i::::i  c:::::::::::::::::ct:::::::::::::::::t    o:::::::::::::::or:::::::::::::::::ry:::::y       y:::::y  !:::!     
    V:::::V     V:::::V    i::::i c:::::::cccccc:::::ctttttt:::::::tttttt    o:::::ooooo:::::orr::::::rrrrr::::::ry:::::y     y:::::y   !:::!     
     V:::::V   V:::::V     i::::i c::::::c     ccccccc      t:::::t          o::::o     o::::o r:::::r     r:::::r y:::::y   y:::::y    !:::!     
      V:::::V V:::::V      i::::i c:::::c                   t:::::t          o::::o     o::::o r:::::r     rrrrrrr  y:::::y y:::::y     !:::!     
       V:::::V:::::V       i::::i c:::::c                   t:::::t          o::::o     o::::o r:::::r               y:::::y:::::y      !!:!!     
        V:::::::::V        i::::i c::::::c     ccccccc      t:::::t    tttttto::::o     o::::o r:::::r                y:::::::::y        !!!      
         V:::::::V        i::::::ic:::::::cccccc:::::c      t::::::tttt:::::to:::::ooooo:::::o r:::::r                 y:::::::y                  
          V:::::V         i::::::i c:::::::::::::::::c      tt::::::::::::::to:::::::::::::::o r:::::r                  y:::::y          !!!      
           V:::V          i::::::i  cc:::::::::::::::c        tt:::::::::::tt oo:::::::::::oo  r:::::r                 y:::::y          !!:!!     
            VVV           iiiiiiii    cccccccccccccccc          ttttttttttt     ooooooooooo    rrrrrrr                y:::::y            !!!      
                                                                                                                     y:::::y                      
                                                                                                                    y:::::y                       
                                                                                                                   y:::::y                        
                                                                                                                  y:::::y                         
                                                                                                                 yyyyyyy 
    "#;
    println!("Congratulations, Player {}! You are the winner!\n {}", winning_color, victory_text)
}


fn handle_roll(player: &mut Player, turn_count: usize) -> bool {
    let roll_result = player.roll();
    println!("{} rolled: {}", player.color, roll_result);

    if roll_result == 6 {
        if player.pawns[0] == 0 {
            player.pawns[0] = 1;
        } else {
            handle_six_roll(player, turn_count);
        }
        println!("{} got an extra roll!", player.color);
        true
    } else {
        let empty_count = player.pawns.iter().filter(|&&pawn| pawn == 0).count();
        if empty_count == 4 {
            false
        } else if empty_count == 3 {
            if let Some(non_empty_index) = player.pawns.iter().position(|&pawn| pawn != 0) {
                let target_position = player.pawns[non_empty_index] + roll_result;
                if target_position <= 44 {
                    player.pawns[non_empty_index] += roll_result;
                } else {
                    println!("Invalid move. Target position cannot exceed 44. Turn skipped.");
                }
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

fn handle_six_roll(player: &mut Player, turn_count: usize) {
    loop {
        println!("Turn {}: Press 'm' to move an existing pawn or 'p' to place another pawn on the board", turn_count);
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Failed to read line");
        action = action.trim().to_lowercase();

        match action.as_str() {
            "m" => {
                move_existing_pawn(player, 6);
                break;
            }
            "p" => {
                if player.pawns.iter().any(|&pawn| pawn == 0) {
                    place_new_pawn(player);
                } else {
                    println!("Player has no empty space for a new pawn. Moving an existing pawn instead.");
                    move_existing_pawn(player, 6);
                }
                break;
            }
            _ => {
                println!("Invalid option. Please enter 'm' or 'p'.");
            }
        }
    }
}

fn get_user_input(prompt: &str) -> u32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(number) => return number,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn save_game(game_state: &GameState) -> Result<(), Box<dyn std::error::Error>> {
    let serialized = serde_json::to_string(game_state)?;
    std::fs::write("saved_game.json", serialized)?;
    println!("Game saved successfully.");
    Ok(())
}

fn load_game() -> Result<GameState, Box<dyn std::error::Error>> {
    let serialized = std::fs::read_to_string("saved_game.json")?;
    let game_state = serde_json::from_str(&serialized)?;
    Ok(game_state)
}

fn game_loop(mut game_state: GameState, is_new_game: bool) {
    let mut _turn_count = if is_new_game { 2 } else { game_state.turn_count };

    loop {
        clearscreen::clear().expect("failed to clear screen");
        print_logo();

        println!("Players and their chosen colors:");
        for (i, player) in game_state.players.iter_mut().enumerate() {
            println!(
                "Player {}: {} | pawn positions on the board - {:?}",
                i + 1,
                player.color,
                player.pawns
            );
        }

        if is_new_game {
            initial_rolls(&mut game_state.players);
        }

        turns(&mut game_state);

        if game_state.players.iter().any(|player| is_winning_condition(&player.pawns)) {
            if let Err(err) = fs::remove_file("saved_game.json") {
                eprintln!("Error deleting saved game file: {}", err);
            }
            println!("Save game deleted. Player wins!");

           std::thread::sleep(std::time::Duration::from_secs(5));

            clearscreen::clear().expect("failed to clear screen");
            print_logo();
            break;
        }

        if save_game(&game_state).is_err() {
            println!("Error saving game.");
        }

        _turn_count += 1;
    }
}

fn start_new_game() {
    clearscreen::clear().expect("failed to clear screen");
    print_logo();
    let num_players = choose_number_of_players();
    let players = choose_player_colors(num_players);
    let game_state = GameState {
        players,
        turn_count: 2,
    };
    game_loop(game_state, true);
}

fn load_saved_game() {
    match load_game() {
        Ok(loaded_game_state) => {
            println!("Loaded players: {:?}", loaded_game_state.players);
            game_loop(loaded_game_state, false);
        }
        Err(err) => println!("Error loading game: {}", err),
    }
}

fn main() {
    clearscreen::clear().expect("failed to clear screen");
    print_logo();
    loop {
        println!("1. Start a new game");
        println!("2. Load a saved game");
        println!("3. Quit");

        let choice: u32 = get_user_input("Enter your choice: ");

        match choice {
            1 => start_new_game(),
            2 => load_saved_game(),
            3 => return,
            _ => println!("Invalid choice. Please enter a number between 1 and 3."),
        }
    }
}