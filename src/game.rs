use crate::board::{Board, SumData, TileValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    Won,
    Lost,
}

pub struct Game {
    score: u32,
    pub curr_board: Board,
    sol_board: Board,
    row_sums : Vec<SumData>,
    col_sums : Vec<SumData>,
    state : GameState
}

impl Game {
    pub fn new(board_dim: usize) -> Game {
        let mut sol_board = Board::new(board_dim, TileValue::Hidden);
        sol_board.create_solution();

        let row_sums = sol_board.get_row_sums();
        let col_sums = sol_board.get_col_sums();

        let curr_board = Board::new(board_dim, TileValue::Hidden);

        Game {
            score: 0,
            curr_board,
            sol_board,
            row_sums,
            col_sums,
            state: GameState::InProgress,
        }
    }

    // checks for all 2 and 3 uncovered to win
    // if 1s are covered that's fine
    pub fn check_sol(&mut self) -> GameState {
        let board_dim = self.curr_board.get_board_dim();
        for i in 0..board_dim {
            for j in 0..board_dim {
                let sol_val = self.sol_board.get_val(i, j);
                let curr_val = self.curr_board.get_val(i, j);

                if sol_val == TileValue::Two || sol_val == TileValue::Three {
                    if curr_val == TileValue::Hidden {
                        return GameState::InProgress;
                    }
                    else if sol_val != curr_val {
                        return GameState::Lost;
                    }
                }
            }
        }
        GameState::Won
    }

    pub fn click(&mut self, i: usize, j: usize) -> GameState {
        let new_val = self.sol_board.get_val(i, j);
        self.curr_board.set_val(i, j, new_val);
        match new_val {
            TileValue::Two => self.score *= 2,
            TileValue::Three => self.score *= 3,
            TileValue::Voltorb => {
                // don't set score to 0 for simplicity
                self.state = GameState::Lost;
                return self.state;
            }
            _ => {} // don't update score
        }
        self.state = self.check_sol();
        self.state
    }

    // pub fn reset(&mut self) {
    //     self.score = 0;
    //     self.curr_board = Board::new(TileValue::Hidden);
    //     self.sol_board = Board::new(TileValue::Hidden);
    //     self.sol_board.create_solution();
    //     self.row_sums = self.sol_board.get_row_sums();
    //     self.col_sums = self.sol_board.get_col_sums();
    //     self.state = GameState::InProgress;
    // }

    // formatting functions
    pub fn display_board(&self) {
        let board_dim = self.curr_board.get_board_dim();
        let green_square = "🟩";
        let numbers = [" ", "1️⃣", "2️⃣", "3️⃣", "💥"];

        // Offset for row labels
        print!("     ");
        for col in 0..board_dim {
            print!(" S:{:<2}  ", self.col_sums[col].value_sum);
        }
        println!();

        print!("     ");
        for col in 0..board_dim {
            print!(" V:{:<2}  ", self.col_sums[col].voltorb_count);
        }
        println!();

        print!("     ");
        for _ in 0..board_dim {
            print!("-------");
        }
        println!();

        for row in 0..board_dim {
            // rows
            print!("R{} |", row);
            for col in 0..board_dim {
                let tile = self.curr_board.get_val(row, col);
                let symbol = match tile {
                    TileValue::Hidden => green_square.to_string(),
                    TileValue::One => numbers[1].to_string(),
                    TileValue::Two => numbers[2].to_string(),
                    TileValue::Three => numbers[3].to_string(),
                    TileValue::Voltorb => numbers[4].to_string(),
                };
                print!(" {:^4} ", symbol);
            }
            println!(" | S:{:<2} V:{:<1}", self.row_sums[row].value_sum, self.row_sums[row].voltorb_count);
        }

        print!("     ");
        for _ in 0..board_dim {
            print!("-------");
        }
        println!();

        // column numbers at bottom
        print!("     ");
        for col in 0..board_dim {
            print!(" C{}    ", col);
        }
        println!();
    }

    pub fn display_score(&self) {
        println!("Your score is: {}", self.score);
    }

}
