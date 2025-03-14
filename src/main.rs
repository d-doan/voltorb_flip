mod board;
mod game;
mod solver;

use std::io;
use board::Board;
use game::Game;
use game::GameState;
use solver::exhaustive;

fn main() {

    println!("Welcome to Dan's Casino!");

    println!("Enter a number (0, 1, 2, ...) to pick a predefined board, or 'r #' to generate a random board of size # by #:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
    
    let premade_boards = Board::premade_boards();
    let mut custom_board = None;
    let mut board_dim_input = 5; // default

    if input.starts_with('r') {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() == 2 {
            match parts[1].parse::<usize>() {
                Ok(dim) if dim > 0 => {
                    board_dim_input = dim;
                }
                _ => {
                    println!("Invalid board size! Using default dimension {}.", board_dim_input);
                }
            }
        } else {
            println!("Invalid format! Use 'r #' where # is the board size.");
            return;
        }
    } else {
        match input.parse::<usize>() {
            Ok(index) if index < premade_boards.len() => {
                custom_board = Some(premade_boards[index].clone());
                board_dim_input = custom_board.as_ref().unwrap().get_board_dim(); // Set board dimension to match premade board
            }
            _ => {
                println!("Invalid selection. Using a random board with default size {}.", board_dim_input);
            }
        }
    }

    let mut game = Game::new(board_dim_input, custom_board);
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
