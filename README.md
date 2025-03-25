# ČOVJEČE NE LJUTI SE - RUST IMPLEMENTATION

This is a Rust implementation of the classic board game "Čoveče ne ljuti se". The game supports 2-4 players and follows the traditional gameplay rules.

## Features
- **Turn-based gameplay**: Players take turns rolling a die.
- **Color selection**: Players choose from red, blue, green, or yellow.
- **Pawn movement**: Move your pawns across the board to reach the final destination.
- **Safehouse logic**: Pawns must move strategically to avoid being sent back to the start.
- **Victory screen**: A celebration screen when a player wins.
- **Save & Load**: Uses Serde to serialize and deserialize game state.

## Installation
To run this game, you need to have Rust installed. If you haven't installed Rust yet, follow the instructions at [Rust's official website](https://www.rust-lang.org/).

Clone this repository:
```sh
$ git clone https://github.com/your-repository-url.git
$ cd your-repository-folder
```

## How to Play
1. Run the game using:
   ```sh
   $ cargo run
   ```
2. Choose the number of players (2-4).
3. Each player selects a color.
4. Players take turns rolling the die by pressing Enter.
5. Players must roll a 6 to place a pawn on the board.
6. Move pawns strategically to reach the final safe zone.
7. The first player to move all their pawns to the goal wins!

## Dependencies
The project uses the following Rust crates:
- `rand` - For dice rolling.
- `serde` & `serde_json` - For saving and loading game state.
- `inline_colorization` - For colored console output.

To install dependencies, use:
```sh
$ cargo build
```
