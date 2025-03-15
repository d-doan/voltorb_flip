use rand::{seq::SliceRandom, rng};


#[derive(Clone)]
pub struct Board {
    tiles: Vec<Vec<TileValue>>,
    board_dim: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileValue {
    Hidden = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Voltorb = 66,   // arbitrary constant for easy debugging
    ERR = 255,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SumData {
    pub value_sum : u8,
    pub voltorb_count : u8
}

#[derive(Clone)]
pub struct PremadeBoard {
    pub solution: Board,
    pub initial: Board,
    board_dim: usize
}

impl TileValue {
    pub fn to_value(self) -> u8 {
        match self {
            TileValue::Hidden => 0,
            TileValue::One => 1,
            TileValue::Two => 2,
            TileValue::Three => 3,
            TileValue::Voltorb => 66,
            TileValue::ERR => 255,
        }
    }
    pub fn to_enum(value: u8) -> TileValue {
        match value {
            0 => TileValue::Hidden,
            1 => TileValue::One,
            2 => TileValue::Two,
            3 => TileValue::Three,
            66 => TileValue::Voltorb,
            _ => TileValue::ERR,
        }
    }
}

impl PremadeBoard{
    pub fn get_board_dim(&self) -> usize {
        self.board_dim
    }
}

impl Board {

    pub fn new(board_dim: usize, default_value: TileValue) -> Board {
        Board {
            tiles: vec![vec![default_value; board_dim]; board_dim],
            board_dim
        }
    }

    pub fn get_tiles(&self) -> &Vec<Vec<TileValue>> {
        &self.tiles
    }

    pub fn get_board_dim(&self) -> usize {
        self.board_dim
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
        for row in 0..self.board_dim {
            for col in 0..self.board_dim {
                positions.push((row, col));
            }
        }
        positions.shuffle(&mut rng());
        positions.truncate(total_non_ones);

        self.tiles = vec![vec![TileValue::One; self.board_dim]; self.board_dim];
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

    pub fn get_row_sums(&self) -> Vec<SumData> {
        let mut row_sums = vec![SumData { value_sum: 0, voltorb_count: 0 }; self.board_dim];

        for row in 0..self.board_dim {
            for col in 0..self.board_dim {
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

    pub fn get_col_sums(&self) -> Vec<SumData> {
        let mut col_sums = vec![SumData { value_sum: 0, voltorb_count: 0 }; self.board_dim];

        for col in 0..self.board_dim {
            for row in 0..self.board_dim {
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
        for i in 0..self.board_dim {
            for j in 0..self.board_dim {
                if self.tiles[i][j] == TileValue::Hidden {
                    hidden_tiles.push((i, j));
                }
            }
        }
        hidden_tiles
    }

    pub fn from_tiles(tiles: Vec<Vec<TileValue>>) -> Board {
        let board_dim = tiles.len();
        Board {tiles, board_dim}
    }

    pub fn premade_boards() -> Vec<PremadeBoard> {
        vec![
            PremadeBoard {
                solution: Board::from_tiles(vec![
                    vec![TileValue::Two,  TileValue::Three,  TileValue::One,  TileValue::Two,    TileValue::One],
                    vec![TileValue::One,  TileValue::Voltorb,TileValue::Two,  TileValue::One,    TileValue::One],
                    vec![TileValue::Two,  TileValue::One,    TileValue::Three,TileValue::Three,  TileValue::One],
                    vec![TileValue::Three,TileValue::Voltorb,TileValue::One,  TileValue::One,    TileValue::One],
                    vec![TileValue::One,  TileValue::Two,    TileValue::Two,  TileValue::Voltorb,TileValue::Voltorb]
                ]),
                initial: Board::from_tiles(vec![
                    vec![TileValue::Two,  TileValue::Three, TileValue::One,  TileValue::Two,   TileValue::One],
                    vec![TileValue::One,  TileValue::Hidden,TileValue::Two,  TileValue::One,   TileValue::Hidden],
                    vec![TileValue::Two,  TileValue::One,   TileValue::Three,TileValue::Three, TileValue::One],
                    vec![TileValue::Three,TileValue::Hidden,TileValue::One,  TileValue::Hidden,TileValue::Hidden],
                    vec![TileValue::One,  TileValue::Hidden,TileValue::Two,  TileValue::Hidden,TileValue::Hidden]
                ]),
                board_dim: 5
            },
            PremadeBoard {
                solution: Board::from_tiles(vec![
                    vec![1, 1, 1, 1, 1],
                    vec![1, 2, 1, 66, 1],
                    vec![1, 1, 1, 1, 1],
                    vec![1, 66, 1, 2, 1],
                    vec![1, 1, 1, 1, 1]
                ].into_iter().map(|row| row.into_iter().map(TileValue::to_enum).collect()).collect()),
                initial: Board::from_tiles(vec![
                    vec![1, 1, 1, 1, 1],
                    vec![1, 0, 1, 0, 1],
                    vec![1, 1, 1, 1, 1],
                    vec![1, 0, 1, 0, 1],
                    vec![1, 1, 1, 1, 1]
                ].into_iter().map(|row| row.into_iter().map(TileValue::to_enum).collect()).collect()),
                board_dim: 5
            },
            PremadeBoard {
                solution: Board::from_tiles(vec![
                    vec![66, 1, 1],
                    vec![1, 66, 1],
                    vec![1, 1, 66]
                ].into_iter().map(|row| row.into_iter().map(TileValue::to_enum).collect()).collect()),
                initial: Board::from_tiles(vec![
                    vec![0, 0, 0],
                    vec![0, 0, 0],
                    vec![0, 0, 0]
                ].into_iter().map(|row| row.into_iter().map(TileValue::to_enum).collect()).collect()),
                board_dim: 3
            },
        ]
    }
}
