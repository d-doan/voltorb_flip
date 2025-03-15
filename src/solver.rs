use crate::board::{SumData, TileValue};
use crate::game::Game;

use std::collections::{HashMap, HashSet};


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

fn get_unflipped_tiles(tiles: &Vec<Vec<TileValue>>) -> Vec<(usize, usize)> {
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

fn is_board_valid(tiles: &Vec<Vec<TileValue>>, rows: &Vec<SumData>, cols: &Vec<SumData>)
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

fn get_possible_solutions(tiles: &Vec<Vec<TileValue>>,
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

// ------------OPTIMIZED ALGO------------

pub fn optimized_solver(game: &mut Game) -> ((usize, usize), f32) {
    let tiles = game.curr_board.get_tiles();
    let rows = &game.row_sums;
    let cols = &game.col_sums;

    let unflipped_tiles = get_unflipped_tiles(tiles);
    let mut possible_values: HashMap<(usize, usize), HashSet<TileValue>> = HashMap::new();

    // Initialize possible values for each unflipped tile
    for &tile in &unflipped_tiles {
        possible_values.insert(tile, HashSet::from([
            TileValue::Voltorb, TileValue::One, TileValue::Two, TileValue::Three,
        ]));
    }

    // Initialize row and column sums
    let mut curr_row_sums = get_row_sums(tiles);
    let mut curr_col_sums = get_col_sums(tiles);

    // Iteratively prune until no further changes occur
    loop {
        let (is_pruned, safe_tile) = prune(tiles, rows, &mut curr_row_sums, cols, &mut curr_col_sums, &unflipped_tiles, &mut possible_values);
        // println!("SAFE TILE: {}", safe_tile.0 );
        if is_pruned {
            if safe_tile != (usize::MAX, usize::MAX) {
                // println!("dfyuiasdolfjnjksdfkslafklsnfklsanflksnflksnklfsnflknslfknsadlkfk");
                return (safe_tile, 1.0);
            }
        }
        else {
            // println!("ENTERING EXHAUSTIVE");
            return optimized_exhaustive(tiles, rows, &mut curr_row_sums, cols, &mut curr_col_sums, &mut possible_values);
        }
    }
}

fn prune(
    tiles: &Vec<Vec<TileValue>>,
    rows: &Vec<SumData>, curr_row_sums: &Vec<SumData>,
    cols: &Vec<SumData>, curr_col_sums: &Vec<SumData>,
    unflipped_tiles: &Vec<(usize, usize)>,
    possible_values: &mut HashMap<(usize, usize), HashSet<TileValue>>,
) -> (bool, (usize, usize)) {
    let mut safe_tile = (usize::MAX, usize::MAX);
    let mut is_pruned = false;

    for &tile in unflipped_tiles {
        let old_vals = possible_values.get(&tile).cloned().unwrap_or_default();

        apply_rules(tile, tiles, rows, curr_row_sums, cols, curr_col_sums, possible_values);

        let updated_vals = possible_values.get(&tile).cloned().unwrap_or_default();

        if updated_vals != old_vals {
            is_pruned = true;
            possible_values.insert(tile, updated_vals.clone());
        }

        if !updated_vals.contains(&TileValue::Voltorb) && safe_tile == (usize::MAX, usize::MAX) {
            safe_tile = tile;
        }
    }
    return (is_pruned, safe_tile);
}

fn optimized_exhaustive(
    tiles: &Vec<Vec<TileValue>>,
    rows: &Vec<SumData>, curr_row_sums: &mut Vec<SumData>,
    cols: &Vec<SumData>, curr_col_sums: &mut Vec<SumData>,
    possible_values: &mut HashMap<(usize, usize), HashSet<TileValue>>,
) -> ((usize, usize), f32) {
    let mut safe_tiles: HashMap<(usize, usize), usize> = HashMap::new();

    let solved_boards = opt_get_solutions(tiles, rows, curr_row_sums, cols, curr_col_sums, possible_values);
    let num_boards = solved_boards.len();

    // If no solutions exist, return default
    if num_boards == 0 {
        return ((usize::MAX, usize::MAX), 0.0);
    }

    let unflipped_tiles = get_unflipped_tiles(tiles);

    for board in &solved_boards {
        for (r, row) in board.iter().enumerate() {
            for (c, &val) in row.iter().enumerate() {
                let tile = (r, c);

                if unflipped_tiles.contains(&tile) && val != TileValue::Voltorb {
                    *safe_tiles.entry(tile).or_insert(0) += 1;
                }
            }
        }
    }

    let mut safest = 0;
    let mut flip = (usize::MAX, usize::MAX);

    // print_safe_tiles(&safe_tiles, num_boards);

    for (&tile, &safe_count) in &safe_tiles {
        if safe_count > safest {
            safest = safe_count;
            flip = tile;
        }
    }
    return (flip, safest as f32 / num_boards as f32);
}

fn opt_get_solutions(
    tiles: &Vec<Vec<TileValue>>,
    rows: &Vec<SumData>, curr_row_sums: &mut Vec<SumData>,
    cols: &Vec<SumData>, curr_col_sums: &mut Vec<SumData>,
    possible_values: &mut HashMap<(usize, usize), HashSet<TileValue>>,
) -> Vec<Vec<Vec<TileValue>>> {
    let unflipped_tiles = get_unflipped_tiles(tiles);

    if unflipped_tiles.is_empty() {
        if is_board_valid(tiles, rows, cols) {
            return vec![tiles.clone()];
        } else {
            return vec![];
        }
    }

    while prune(tiles, rows, curr_row_sums, cols, curr_col_sums, &unflipped_tiles, possible_values).0 {
        // Prune modifies possible_values in place, so we don't need to update it manually
    }

    let mut result = Vec::new();
    let first_unflipped_tile = unflipped_tiles[0];

    if let Some(guesses) = possible_values.get(&first_unflipped_tile).cloned() {
        for guess in guesses {
            let mut new_tiles = tiles.clone();
            new_tiles[first_unflipped_tile.0][first_unflipped_tile.1] = guess;

            let mut next_possible_values = possible_values.clone();
            // Clear possible values for guessed tile
            next_possible_values.insert(first_unflipped_tile, HashSet::new());

            let mut new_row_sums = curr_row_sums.clone();
            let mut new_col_sums = curr_col_sums.clone();

            if guess != TileValue::Voltorb {
                new_row_sums[first_unflipped_tile.0].value_sum += guess.to_value();
                new_col_sums[first_unflipped_tile.1].value_sum += guess.to_value();
            } else {
                new_row_sums[first_unflipped_tile.0].voltorb_count += 1;
                new_col_sums[first_unflipped_tile.1].voltorb_count += 1;
            }

            result.extend(opt_get_solutions(
                &new_tiles,
                rows,
                &mut new_row_sums,
                cols,
                &mut new_col_sums,
                &mut next_possible_values,
            ));
        }
    }
    return result;
}


fn apply_rules(
    tile: (usize, usize), tiles: &Vec<Vec<TileValue>>,
    rows: &Vec<SumData>, curr_row_sums: &Vec<SumData>,
    cols: &Vec<SumData>, curr_col_sums: &Vec<SumData>,
    possible_values: &mut HashMap<(usize, usize), HashSet<TileValue>>,
) {
    rule1(tile, rows, curr_row_sums, cols, curr_col_sums, possible_values);
    rule2(tile, rows, curr_row_sums, cols, curr_col_sums, possible_values);
    rule3(tile, tiles, rows, curr_row_sums, cols, curr_col_sums, possible_values);
    rule4(tile, tiles, rows, cols, possible_values);
}

// Rule 1: Remove Voltorb if all Voltorbs are already accounted for
fn rule1(
    (r, c): (usize, usize),
    rows: &Vec<SumData>,
    curr_row_sums: &Vec<SumData>,
    cols: &Vec<SumData>,
    curr_col_sums: &Vec<SumData>,
    possible_values: &mut HashMap<(usize, usize), HashSet<TileValue>>,
) {
    if rows[r].voltorb_count == curr_row_sums[r].voltorb_count
        || cols[c].voltorb_count == curr_col_sums[c].voltorb_count
    {
        if let Some(set) = possible_values.get_mut(&(r, c)) {
            set.remove(&TileValue::Voltorb);
        }
    }
}

// Rule 2: If all sum values match, enforce Voltorb-only or clear invalid boards
fn rule2(
    (r, c): (usize, usize),
    rows: &Vec<SumData>, curr_row_sums: &Vec<SumData>,
    cols: &Vec<SumData>, curr_col_sums: &Vec<SumData>,
    possible_values: &mut HashMap<(usize, usize), HashSet<TileValue>>,
) {
    if rows[r].value_sum == curr_row_sums[r].value_sum
        || cols[c].value_sum == curr_col_sums[c].value_sum
    {
        if let Some(set) = possible_values.get_mut(&(r, c)) {
            if set.contains(&TileValue::Voltorb) {
                set.clear();
                set.insert(TileValue::Voltorb);
            } else {
                set.clear(); // Invalid board state
            }
        }
    }
}

// Rule 3: If no more unflipped tiles remain in row or column, enforce exact sum
fn rule3(
    (r, c): (usize, usize),
    tiles: &Vec<Vec<TileValue>>,
    rows: &Vec<SumData>, curr_row_sums: &Vec<SumData>,
    cols: &Vec<SumData>, curr_col_sums: &Vec<SumData>,
    possible_values: &mut HashMap<(usize, usize), HashSet<TileValue>>,
) {
    let mut row_unflipped_cnt = 0;
    let mut col_unflipped_cnt = 0;

    for i in 0..tiles.len() {
        if tiles[i][c] == TileValue::Hidden {
            col_unflipped_cnt += 1;
        }
    }

    for j in 0..tiles[r].len() {
        if tiles[r][j] == TileValue::Hidden {
            row_unflipped_cnt += 1;
        }
    }

    if row_unflipped_cnt == 0 {
        if let Some(set) = possible_values.get_mut(&(r, c)) {
            let val = rows[r].value_sum.saturating_sub(curr_row_sums[r].value_sum);
            let enum_val = TileValue::to_enum(val);
            set.clear();
            set.insert(enum_val);
        }
    }

    if col_unflipped_cnt == 0 {
        if let Some(set) = possible_values.get_mut(&(r, c)) {
            let val = cols[c].value_sum.saturating_sub(curr_col_sums[c].value_sum);
            let enum_val = TileValue::to_enum(val);
            set.clear();
            set.insert(enum_val);
        }
    }
}


// Rule 4: Remove impossible values based on calculated reward
fn rule4(
    (r, c): (usize, usize),
    tiles: &Vec<Vec<TileValue>>,
    rows: &Vec<SumData>, cols: &Vec<SumData>,
    possible_values: &mut HashMap<(usize, usize), HashSet<TileValue>>,
) {
    for &is_row in &[true, false] {
        let (line_sum, voltorb_count, line_len) = if is_row {
            (rows[r].value_sum, rows[r].voltorb_count, rows.len())
        } else {
            (cols[c].value_sum, cols[c].voltorb_count, cols.len())
        };

        let mut num_2s = 0;
        let mut num_3s = 0;

        for i in 0..line_len {
            let tile = if is_row { tiles[r][i] } else { tiles[i][c] };

            match tile {
                TileValue::Two => num_2s += 1,
                TileValue::Three => num_3s += 1,
                _ => {}
            }
        }

        let reward = (line_sum + voltorb_count).saturating_sub(line_len as u8 + num_2s + 2 * num_3s);

        if reward == 0 {
            for i in 0..line_len {
                let key = if is_row { (r, i) } else { (i, c) };

                if let Some(set) = possible_values.get_mut(&key) {
                    set.remove(&TileValue::Two);
                    set.remove(&TileValue::Three);
                }
            }
        }
    }
}

pub fn get_row_sums(tiles: &Vec<Vec<TileValue>>) -> Vec<SumData> {
    let mut row_sums = vec![SumData { value_sum: 0, voltorb_count: 0 }; tiles.len()];

    for (r, row) in tiles.iter().enumerate() {
        for &tile in row {
            match tile {
                TileValue::One | TileValue::Two | TileValue::Three => {
                    row_sums[r].value_sum += tile.to_value();
                }
                TileValue::Voltorb => {
                    row_sums[r].voltorb_count += 1;
                }
                _ => {}
            }
        }
    }
    return row_sums;
}

pub fn get_col_sums(tiles: &Vec<Vec<TileValue>>) -> Vec<SumData> {
    let n = tiles.len();
    let mut col_sums = vec![SumData { value_sum: 0, voltorb_count: 0 }; n];

    for c in 0..n {
        for r in 0..n {
            match tiles[r][c] {
                TileValue::One | TileValue::Two | TileValue::Three => {
                    col_sums[c].value_sum += tiles[r][c].to_value();
                }
                TileValue::Voltorb => {
                    col_sums[c].voltorb_count += 1;
                }
                _ => {}
            }
        }
    }
    return col_sums;
}

