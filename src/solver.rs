use std::iter::Sum;

// defines all of the potential algorithms we have to get the next move
// prioritizes correctness first until non-deterministic
use crate::board::{SumData, TileValue};
use crate::game::{Game, GameState};


// old baseline - prob need to change?
pub fn baseline(game: &mut Game)
                -> (usize, usize) {
    // copy board state before modification
    let old_board = game.curr_board.clone();
    let tile_values = vec![TileValue::Voltorb, TileValue::One, TileValue::Two, TileValue::Three];
    let hidden_tiles = game.curr_board.get_hidden_tile_indices();
    let num_unflipped = hidden_tiles.len();

    let mut indices = vec![0; num_unflipped];
    loop {
        for (idx, &(i, j)) in hidden_tiles.iter().enumerate() {
            game.curr_board.set_val(i, j, tile_values[indices[idx]]);
        }

        if game.check_sol() == GameState::Won {
            game.curr_board = old_board.clone();
            return hidden_tiles[0]; // return the first hidden tile
        }
        // check next permutation
        let mut carry = true;
        for idx in 0..num_unflipped {
            if carry {
                indices[idx] += 1;
                if indices[idx] == tile_values.len() {
                    indices[idx] = 0; // reset val + carry next
                } else {
                    carry = false;
                    break;
                }
            }
        }
        if carry {
            break;
        }
    }
    game.curr_board = old_board.clone();
    // broke it return dummy case
    return (100, 100);
}


// TODO: implement
pub fn exhaustive(n: usize, tiles: Vec<Vec<TileValue>>,
                    rows: Vec<SumData>, cols: Vec<SumData>)
                        -> ((usize, usize), f32) {


    return ((0,0), 0.0);
}

pub fn get_unflipped_tiles(tiles: &Vec<Vec<TileValue>>) -> Vec<(usize, usize)> {
    let n = tiles.len();
    let mut unflipped: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        for j in 0..tiles[i].len() {
            if tiles[i][j] == TileValue::Hidden {
                unflipped.push((i,j));
            }
        }
    }
    return unflipped;
}

// TODO: implement
pub fn is_board_valid(tiles: Vec<Vec<TileValue>>,
                        rows: Vec<SumData>, cols: Vec<SumData>)
                            -> bool {
    // jdfk
    true
}

// TODO: check return type
// TODO: implement
pub fn get_possible_solutions(tiles: Vec<Vec<TileValue>>,
                                rows: Vec<SumData>, cols: Vec<SumData>)
                                    -> Vec<Vec<Vec<TileValue>>> {
    return vec![vec![vec![TileValue::Two]]];
}

