mod board;
mod game;
mod solver;

use std::io;
use game::Game;
use game::GameState;
use solver::exhaustive;

fn main() {

    println!("Welcome to Dan's Casino!");

    println!("Enter the board dimension for this game");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let board_dim_input: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! Please enter a valid number.");
            return;
        }
    };

    let mut game = Game::new(board_dim_input);
    let board_dim = game.curr_board.get_board_dim();

    println!("\nCurrent Board:");
    game.display_board();

    loop {
        println!("\nEnter row and column to flip (e.g., '1 2') or type 'q' to quit:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        if input.eq_ignore_ascii_case("q") {
            println!("99% of gamblers quit before they strike it big 💎🚀🔥");
            break;
        }

        let mut parts = input.trim().split_whitespace();
        let row: usize = match parts.next().and_then(|r| r.parse().ok()) {
            Some(num) if num < board_dim => num,
            _ => {
                println!("Invalid row! Enter a number between 0 and {}.", board_dim - 1);
                continue;
            }
        };

        let col: usize = match parts.next().and_then(|c| c.parse().ok()) {
            Some(num) if num < board_dim => num,
            _ => {
                println!("Invalid column! Enter a number between 0 and {}.", board_dim - 1);
                continue;
            }
        };

        let result = game.click(row, col);

        println!("\nCurrent Board:");
        game.display_board();

        match result {
            GameState::Won => {
                println!("\nRun it back double or nothing 🤑");
                game.display_score();
                break;
            }
            GameState::Lost => {
                println!("\nbig boooooooom");
                game.display_score();
                break;
            }
            GameState::InProgress => {
                println!("Keep going!");
            }
        }
    }
}
