use std::fs::File;
use std::io::{Write, stdout};
use std::time::Instant;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};
use crate::board::Board;
use crate::game::Game;
use crate::solver::{exhaustive, optimized_solver};

#[derive(Serialize, Deserialize, Debug)]
struct SimulationResult {
    board_index: usize,
    best_move: (usize, usize),
    probability: f32,
    execution_time_ms: f64,
}

pub fn run_simulation() {
    // println!("Running premade boards...");
    // run_premade_boards();
    // println!("Running long premade test...");
    // run_long_premade();
    // println!("Running random boards...");
    // run_random_boards(100, 3); // Run 100 random 3x3 boards
    // for i in 4..7 {
    //     println!("Running random {}-dimension boards on optimal solver...", i);
    //     run_rand_opt(100, i);
    // }

    println!("Running half-complete test...");
    run_half_completed_boards(1000, 5);
}

// Runs simulation on premade boards
fn run_premade_boards() {
    let premade_boards = Board::premade_boards();
    let mut ex_results = Vec::new();
    let mut opt_results = Vec::new();

    print!("Running Premade Board at index: ");
    stdout().flush().unwrap();

    for board_index in 0..=3 {

        print!("{}... ", board_index);
        stdout().flush().unwrap();

        let premade_board = premade_boards.get(board_index).unwrap();
        let mut ex_game = Game::new(premade_board.get_board_dim(), Some(premade_board.clone()), None, None, None);
        let mut opt_game = Game::new(premade_board.get_board_dim(), Some(premade_board.clone()), None, None, None);

        // Run exhaustive solver
        let start_time = Instant::now();
        let best_move_ex = exhaustive(&mut ex_game);
        let elapsed_time_ex = start_time.elapsed().as_secs_f64() * 1000.0;

        // Run optimized solver
        let start_time = Instant::now();
        let best_move_opt = optimized_solver(&mut opt_game);
        let elapsed_time_opt = start_time.elapsed().as_secs_f64() * 1000.0;

        // Prepare results
        ex_results.push(SimulationResult {
            board_index,
            best_move: best_move_ex.0,
            probability: best_move_ex.1,
            execution_time_ms: elapsed_time_ex,
        });

        opt_results.push(SimulationResult {
            board_index,
            best_move: best_move_opt.0,
            probability: best_move_opt.1,
            execution_time_ms: elapsed_time_opt,
        });
    }
    // Save results
    save_to_json("ex_premade.json", &ex_results);
    save_to_json("opt_premade.json", &opt_results);
}

// Runs optimized solver **only** on premade boards
fn run_long_premade() {
    let premade_boards = Board::premade_boards();
    let mut ex_results = Vec:: new();
    let mut opt_results = Vec::new();

    let board_index = 4;
    let premade_board = match premade_boards.get(board_index) {
        Some(board) => board.clone(),
        None => {
            println!("Error: Board index {} not found!", board_index);
            return;
        }
    };

    let mut ex_game = Game::new(premade_board.get_board_dim(), Some(premade_board.clone()), None, None, None);
    let mut opt_game = Game::new(premade_board.get_board_dim(), Some(premade_board.clone()), None, None, None);

    print!("Running Optimized on Unprunable 4x4 Board...");
    stdout().flush().unwrap();
    // Run Exhaustive Solver
    let start_time_ex = Instant::now();
    let best_move_ex = exhaustive(&mut ex_game);
    let elapsed_time_ex = start_time_ex.elapsed().as_secs_f64() * 1000.0;

    print!("Running Exhaustive on Unprunable 4x4 Board...");
    stdout().flush().unwrap();
    // Run Optimized Solver
    let start_time_opt = Instant::now();
    let best_move_opt = optimized_solver(&mut opt_game);
    let elapsed_time_opt = start_time_opt.elapsed().as_secs_f64() * 1000.0;

    // Store results
    ex_results.push(SimulationResult {
        board_index,
        best_move: best_move_ex.0,
        probability: best_move_ex.1,
        execution_time_ms: elapsed_time_ex,
    });

    opt_results.push(SimulationResult {
        board_index,
        best_move: best_move_opt.0,
        probability: best_move_opt.1,
        execution_time_ms: elapsed_time_opt,
    });

    // Save results to JSON
    save_to_json("data/long_ex_premade.json", &ex_results);
    save_to_json("data/long_opt_premade.json", &opt_results);

    println!("Done! Results saved to 'data/long_ex_premade.json' & 'data/long_opt_premade.json'.");
}

// Runs simulation on random boards
fn run_random_boards(num_boards: usize, board_size: usize) {
    let mut ex_results = Vec::new();
    let mut opt_results = Vec::new();

    print!("Running Random Board #: ");
    stdout().flush().unwrap();

    for i in 0..num_boards {
        // let mut random_board = Board::new(board_size, crate::board::TileValue::Hidden);
        // random_board.create_solution();

        print!("{}...", i);
        stdout().flush().unwrap();

        let mut ex_game = Game::new(board_size, None, None, None, None);
        let mut opt_game = Game::new(board_size, None, None, None, None);

        // Run exhaustive solver
        let start_time = Instant::now();
        let best_move_ex = exhaustive(&mut ex_game);
        let elapsed_time_ex = start_time.elapsed().as_secs_f64() * 1000.0;

        // Run optimized solver
        let start_time = Instant::now();
        let best_move_opt = optimized_solver(&mut opt_game);
        let elapsed_time_opt = start_time.elapsed().as_secs_f64() * 1000.0;

        // Prepare results
        ex_results.push(SimulationResult {
            board_index: i,
            best_move: best_move_ex.0,
            probability: best_move_ex.1,
            execution_time_ms: elapsed_time_ex,
        });

        opt_results.push(SimulationResult {
            board_index: i,
            best_move: best_move_opt.0,
            probability: best_move_opt.1,
            execution_time_ms: elapsed_time_opt,
        });
    }
    // Save results
    save_to_json("ex_random.json", &ex_results);
    save_to_json("opt_random.json", &opt_results);
}

// Runs optimized on random larger boards
fn run_rand_opt(num_boards: usize, board_size: usize) {
    let mut results = Vec::new();

    print!("Running Random Board #: ");
    stdout().flush().unwrap();

    for i in 0..num_boards {
        // let mut random_board = Board::new(board_size, crate::board::TileValue::Hidden);
        // random_board.create_solution();

        print!("{}...", i);
        stdout().flush().unwrap();

        let mut game = Game::new(board_size, None, None, None, None);

        // Run optimized solver
        let start_time = Instant::now();
        let best_move_opt = optimized_solver(&mut game);
        let elapsed_time_opt = start_time.elapsed().as_secs_f64() * 1000.0;

        // Prepare results
        results.push(SimulationResult {
            board_index: i,
            best_move: best_move_opt.0,
            probability: best_move_opt.1,
            execution_time_ms: elapsed_time_opt,
        });
    }
    // Save results
    let filename = format!("opt_random_{}x{}.json", board_size, board_size);
    save_to_json(&filename, &results);
}

// Run half-completed boards with optimized solver
fn run_half_completed_boards(num_boards: usize, board_size: usize) {
    let mut results = Vec::new();

    print!("Running Half-Completed Boards #: ");
    stdout().flush().unwrap();

    for i in 0..num_boards {
        print!("{}... ", i);
        stdout().flush().unwrap();

        // Step 1: Generate a random board using `Game::new`
        let mut game = Game::new(board_size, None, None, None, None);
        let sol_board = game.get_sol();

        // Step 2: Reveal half of the tiles in `curr_board` using `sol_board`
        let mut all_tiles: Vec<(usize, usize)> = Vec::new();

        for r in 0..board_size {
            for c in 0..board_size {
                all_tiles.push((r, c));
            }
        }

        // Randomly shuffle and reveal half of the tiles
        let mut rng = rand::rng();
        all_tiles.shuffle(&mut rng);
        let reveal_count = all_tiles.len() / 2;

        for &(r, c) in &all_tiles[..reveal_count] {
            let revealed_val = sol_board.get_val(r, c);
            game.curr_board.set_val(r, c, revealed_val);
        }

        // Step 3: Run the optimized solver on the half-completed board
        let start_time = Instant::now();
        let best_move_opt = optimized_solver(&mut game);
        let elapsed_time_opt = start_time.elapsed().as_secs_f64() * 1000.0;

        // Prepare results
        results.push(SimulationResult {
            board_index: i,
            best_move: best_move_opt.0,
            probability: best_move_opt.1,
            execution_time_ms: elapsed_time_opt,
        });
    }

    // Save results to JSON
    save_to_json("half_complete.json", &results);
    println!("Done! Results saved to 'data/rand_half_complete.json'.");
}

// Helper function to save results to JSON
fn save_to_json(filename: &str, results: &Vec<SimulationResult>) {
    let path = format!("data/{}", filename);
    let json_data = serde_json::to_string_pretty(results).expect("Failed to serialize JSON");
    let mut file = File::create(&path).expect("Failed to create file");
    file.write_all(json_data.as_bytes()).expect("Failed to write JSON");

    println!("Simulation complete. Data saved to {}", path);
}
