
// defines all of the potential algorithms we have to get the next move
// prioritizes correctness first until non-deterministic
use crate::board::{SumData, TileValue};
use crate::game::{Game, GameState};


pub fn exhaustive(game: &mut Game) -> ((usize, usize), f32) {
    let n = game.curr_board.get_board_dim();
    let tiles = game.curr_board.get_tiles();
    let rows = &game.row_sums;
    let cols = &game.col_sums;

    let mut safe_tiles = vec![vec![0.0; n]; n];
    let solved_boards = get_possible_solutions(tiles, rows, cols);
    let num_boards = solved_boards.len();
    let unflipped_tiles = get_unflipped_tiles(tiles);

    for board in solved_boards {
        for (i, row) in board.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if unflipped_tiles.contains(&(i,j)) && val != TileValue::Voltorb {
                    safe_tiles[i][j] += 1.0;
                }
            }
        }
    }

    for row in safe_tiles.iter_mut() {
        for val in row.iter_mut() {
            *val /= num_boards as f32;
        }
    }

    let mut safest = 0.0;
    let mut flip = (0, 0);
    for (i, row) in safe_tiles.iter().enumerate() {
        for (j, &safe_prob) in row.iter().enumerate() {
            if safe_prob > safest {
                safest = safe_prob;
                flip = (i, j);
            }
        }
    }
    return (flip, safest);
}

pub fn get_unflipped_tiles(tiles: &Vec<Vec<TileValue>>) -> Vec<(usize, usize)> {
    let mut unflipped: Vec<(usize, usize)> = Vec::new();
    for (r, row) in tiles.iter().enumerate() {
        for (c, &tile) in row.iter().enumerate() {
            if tile == TileValue::Hidden {
                unflipped.push((r, c));
            }
        }
    }
    return unflipped;
}

pub fn is_board_valid(tiles: &Vec<Vec<TileValue>>, rows: &Vec<SumData>, cols: &Vec<SumData>)
                        -> bool {
    let n = rows.len();

    for r in 0..n {
        let mut val_sum = 0;
        let mut voltorb_cnt = 0;
        for &tile in &tiles[r] {
            match tile {
                TileValue::Voltorb => voltorb_cnt += 1,
                _ => val_sum += tile.to_value()
            }
        }
        if val_sum != rows[r].value_sum || voltorb_cnt != rows[r].voltorb_count {
            return false;
        }
    }
    for c in 0..cols.len() {
        let mut val_sum = 0;
        let mut voltorb_cnt = 0;

        for r in 0..n {
            match tiles[r][c] {
                TileValue::Voltorb => voltorb_cnt += 1,
                _ => val_sum += tiles[r][c].to_value(),
            }
        }

        if val_sum != cols[c].value_sum || voltorb_cnt != cols[c].voltorb_count {
            return false;
        }
    }
    return true;
}

pub fn get_possible_solutions(tiles: &Vec<Vec<TileValue>>,
                                rows: &Vec<SumData>, cols: &Vec<SumData>)
                                    -> Vec<Vec<Vec<TileValue>>> {
    let unflipped_tiles = get_unflipped_tiles(tiles);
    if unflipped_tiles.is_empty() {
        if is_board_valid(tiles, rows, cols) {
            return vec![tiles.clone()];
        }
        else {
            return vec![];
        }
    }
    let mut result = Vec::new();
    let first_unflipped_tile = unflipped_tiles[0];
    for guess in [TileValue::Voltorb, TileValue::One, TileValue::Two, TileValue::Three] {
        let mut new_tiles = tiles.clone();
        new_tiles[first_unflipped_tile.0][first_unflipped_tile.1] = guess;
        result.extend(get_possible_solutions(&new_tiles, rows, cols))
    }
    return result;
}

