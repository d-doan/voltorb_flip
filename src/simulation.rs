use std::fs::File;
use std::io::{Write, stdout};
use std::time::Instant;
use serde::{Serialize, Deserialize};
use crate::board::Board;
use crate::game::Game;
use crate::solver::exhaustive;

#[derive(Serialize, Deserialize, Debug)]
struct SimulationResult {
    board_index: usize,
    best_move: (usize, usize),
    probability: f32,
    execution_time_ms: f64,
}

pub fn run_simulation() {
    println!("Running premade boards...");
    run_premade_boards();
    println!("Running random boards...");
    run_random_boards(100, 3); // Run 100 random 3x3 boards
}

// Runs simulation on premade boards
fn run_premade_boards() {
    let premade_boards = Board::premade_boards();
    let mut results = Vec::new();

    print!("Running Premade Board at index: ");
    stdout().flush().unwrap();

    for (board_index, premade_board) in premade_boards.iter().enumerate() {

        print!("{}... ", board_index);
        stdout().flush().unwrap();

        let mut game = Game::new(premade_board.get_board_dim(), Some(premade_board.clone()));

        // Start timing
        let start_time = Instant::now();
        let best_move = exhaustive(&mut game);
        let elapsed_time = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds

        // Prepare result
        let result = SimulationResult {
            board_index,
            best_move: best_move.0,
            probability: best_move.1,
            execution_time_ms: elapsed_time,
        };

        results.push(result);
    }

    save_to_json("ex_premade.json", &results);
}

// Runs simulation on random boards
fn run_random_boards(num_boards: usize, board_size: usize) {
    let mut results = Vec::new();

    print!("Running Random Board #: ");
    stdout().flush().unwrap();

    for i in 0..num_boards {
        // let mut random_board = Board::new(board_size, crate::board::TileValue::Hidden);
        // random_board.create_solution();

        print!("{}...", i);
        stdout().flush().unwrap();

        let mut game = Game::new(board_size, None);

        // Start timing
        let start_time = Instant::now();
        let best_move = exhaustive(&mut game);
        let elapsed_time = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds

        // Prepare result
        let result = SimulationResult {
            board_index: i,
            best_move: best_move.0,
            probability: best_move.1,
            execution_time_ms: elapsed_time,
        };

        results.push(result);
    }

    save_to_json("ex_random.json", &results);
}

// Helper function to save results to JSON
fn save_to_json(filename: &str, results: &Vec<SimulationResult>) {
    let path = format!("data/{}", filename); // Save inside `data/`
    let json_data = serde_json::to_string_pretty(results).expect("Failed to serialize JSON");
    let mut file = File::create(&path).expect("Failed to create file");
    file.write_all(json_data.as_bytes()).expect("Failed to write JSON");

    println!("Simulation complete. Data saved to {}", path);
}
