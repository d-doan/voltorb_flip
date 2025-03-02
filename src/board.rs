use rand::{seq::SliceRandom, rng};

pub const BOARD_DIM: usize = 5;

#[derive(Clone)]
pub struct Board {
    tiles: [[TileValue; BOARD_DIM]; BOARD_DIM],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileValue {
    Hidden = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Voltorb = 66,   // arbitrary constant for easy debugging
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SumData {
    pub value_sum : u8,
    pub voltorb_count : u8
}

impl Board {

    pub fn new(default_value: TileValue) -> Board {
        Board {
            tiles: [[default_value; BOARD_DIM]; BOARD_DIM],
        }
    }

    pub fn get_val(&self, i: usize, j: usize) -> TileValue {
        self.tiles[i][j]
    }

    pub fn set_val(&mut self, i: usize, j: usize, val: TileValue) {
        self.tiles[i][j] = val;
    }

    pub fn create_solution(&mut self) {
        // dummy fixed values for testing
        // randomly gen values later
        let mut num_twos = 6;
        let mut num_threes = 4;
        let mut num_voltorbs = 3;
        let total_non_ones = num_twos + num_threes + num_voltorbs;

        // get all board positions and randomize
        let mut positions: Vec<(usize, usize)> = Vec::new();
        for row in 0..BOARD_DIM {
            for col in 0..BOARD_DIM {
                positions.push((row, col));
            }
        }
        positions.shuffle(&mut rng());
        positions.truncate(total_non_ones);

        self.tiles = [[TileValue::One; BOARD_DIM]; BOARD_DIM];
        for (row, col) in positions {
            let to_place = if num_twos > 0 {
                num_twos -= 1;
                TileValue::Two
            } else if num_threes > 0 {
                num_threes -= 1;
                TileValue::Three
            } else {
                num_voltorbs -= 1;
                TileValue::Voltorb
            };
            self.tiles[row][col] = to_place;
        }
    }

    pub fn get_row_sums(&self) -> [SumData; BOARD_DIM] {
        let mut row_sums = [SumData { value_sum: 0, voltorb_count: 0 }; BOARD_DIM];

        for row in 0..BOARD_DIM {
            for col in 0..BOARD_DIM {
                match self.tiles[row][col] {
                    TileValue::One => row_sums[row].value_sum += 1,
                    TileValue::Two => row_sums[row].value_sum += 2,
                    TileValue::Three => row_sums[row].value_sum += 3,
                    TileValue::Voltorb => row_sums[row].voltorb_count += 1,
                    _ => {}
                }
            }
        }
        row_sums
    }

    pub fn get_col_sums(&self) -> [SumData; BOARD_DIM] {
        let mut col_sums = [SumData { value_sum: 0, voltorb_count: 0 }; BOARD_DIM];

        for col in 0..BOARD_DIM {
            for row in 0..BOARD_DIM {
                match self.tiles[row][col] {
                    TileValue::One => col_sums[col].value_sum += 1,
                    TileValue::Two => col_sums[col].value_sum += 2,
                    TileValue::Three => col_sums[col].value_sum += 3,
                    TileValue::Voltorb => col_sums[col].voltorb_count += 1,
                    _ => {}
                }
            }
        }
        col_sums
    }

    pub fn get_hidden_tile_indices(&self) -> Vec<(usize, usize)> {
        let mut hidden_tiles = Vec::new();
        for i in 0..BOARD_DIM {
            for j in 0..BOARD_DIM {
                if self.tiles[i][j] == TileValue::Hidden {
                    hidden_tiles.push((i, j));
                }
            }
        }
        hidden_tiles
    }

    // pub fn get_tile_counts(&self) -> (usize, usize, usize) {
    //     let mut num_twos = 0;
    //     let mut num_threes = 0;
    //     let mut num_voltorbs = 0;

    //     for i in 0..BOARD_DIM {
    //         for j in 0..BOARD_DIM {
    //             match self.tiles[i][j] {
    //                 TileValue::Two => num_twos += 1,
    //                 TileValue::Three => num_threes += 1,
    //                 TileValue::Voltorb => num_voltorbs += 1,
    //                 _ => {}
    //             }
    //         }
    //     }
    //     (num_twos, num_threes, num_voltorbs)
    // }
}
