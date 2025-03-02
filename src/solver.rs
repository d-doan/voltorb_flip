// defines all of the potential algorithms we have to get the next move
// prioritizes correctness first until non-deterministic
use crate::board::{TileValue};
use crate::game::{Game, GameState};


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

