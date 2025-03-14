// defines all of the potential algorithms we have to get the next move
// prioritizes correctness first until non-deterministic
use crate::board::TileValue;
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


// pub fn exhaustive(n, tiles, rows, cols):
/*
exhaustive(n, tiles, rows, cols):
    safe_tiles[n][n] // nxn grid with values initialized at 0
    solved_boards = get_possible_solutions(tiles, rows, cols)
    num_boards = len(solved_boards)
    unflipped_tiles = get_unflipped_tiles(tiles)

    for board in solved_boards:
        for (i,j, val) in board.enumerate():
            if (i,j) in unflipped_tiles and val.not_voltorb:
                safe_tiles[i][j] += 1

    for (i,j, val) in safe_tiles.enumerate():
        safe_tiles[i][j] = val / num_boards

    safest = 0
    flip = (-1,-1)
    for (i,j, safe_prob) in safe_tiles.enumerate():
        if safe_prob > safest:
            safest = safe_prob
            flip = (i,j)
    return (flip, safest)
*/
