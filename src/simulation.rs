use std::fs::File;
use std::io::Write;
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
    let premade_boards = Board::premade_boards();
    let mut results = Vec::new();

    for (board_index, premade_board) in premade_boards.iter().enumerate() {
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

    // Convert results to JSON and save to a file
    let json_data = serde_json::to_string_pretty(&results).expect("Failed to serialize JSON");
    let mut file = File::create("results.json").expect("Failed to create file");
    file.write_all(json_data.as_bytes()).expect("Failed to write JSON");

    println!("Simulation complete. Data saved to simulation_results.json");
}
