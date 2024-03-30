use std::collections::VecDeque;

use crate::domains::game::Game;
use crate::domains::board::Board;
use crate::domains::player::Player;
pub struct GameManager {
    pub game: Game,
    pub moves: i32,
}

impl GameManager {
    pub fn new(players: VecDeque<Player>, board_size:usize) -> Self {
        let board = Board::new(board_size);
        let game = Game::new(players, board);
        return Self { 
            game, 
            moves:0,
         }
    }

    pub fn play_game(&mut self) {
        while self.game.status != "COMPLETE" {


            // get_player_input

            // insert_symbol_in_cell

            // check if player has won

            // check if game is a draw

            // change turn to the next player
        }
    }

    // check if player has won
    pub fn is_row_complete(&self, row: usize, symbol: char) -> bool {

        for i in 0..self.game.board.size {
            if self.game.board.cells[row as usize][i as usize] != symbol {
                return false;
            }
        }

        return true
    }

    pub fn is_col_complete(&self, col:usize, symbol: char) -> bool {

        for i in 0..self.game.board.size {
            if self.game.board.cells[i as usize][col as usize] != symbol {
                return false;
            }
        }

        return true
    }

    pub fn is_diagonal_complete(&self, symbol: char) -> bool {
        let n = self.game.board.size;

        let mut symbols_in_diagonal = 0;

        for i in 0..n {
            if self.game.board.cells[i as usize][i as usize] != symbol {
                break;
            }
            symbols_in_diagonal += 1;
        }

        if symbols_in_diagonal == n {
            return true
        }

        // reverse diagonal
        let mut symbols_in_diagonal = 0;

        for i in 0..n {
            if self.game.board.cells[i as usize][n - i - 1 as usize] != symbol {
                break;
            }
            symbols_in_diagonal += 1;
        }

        return symbols_in_diagonal == n
    }

    pub fn is_winner(&self, row:usize, col: usize, symbol: char) -> bool {
        return self.is_row_complete(row, symbol) || 
        self.is_col_complete(col, symbol) ||
        self.is_diagonal_complete(symbol)
    }
}