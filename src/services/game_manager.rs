use std::collections::VecDeque;
use std::io;

use crate::domains::game::Game;
use crate::domains::board::Board;
use crate::domains::player::Player;
pub struct GameManager {
    pub game: Game,
    pub moves: i32,
}

impl GameManager {
    pub fn new(players: VecDeque<Player>, board_size:i32) -> Self {
        let board = Board::new(board_size);
        let game = Game::new(players, board);
        return Self { 
            game, 
            moves:0,
         }
    }

    pub fn play_game(&mut self) {
        while self.game.status != "COMPLETE" {

        let (mut row, mut col , is_success) = self.get_player_input();

        if !is_success {
            continue;
        }

        row = row - 1;
        col = col - 1;

        let game_board = &mut self.game.board;

        let is_inserted = game_board.insert_new_symbol(row, col, self.game.players[0].symbol);
        if !is_inserted {
            continue;
        }

        self.moves += 1;
        println!("{}", game_board);

        let has_won = self.is_winner(row as usize, col as usize, self.game.players[0].symbol);

        if has_won {
            println!("Winner is Player {0}", self.game.players[0].name);
            self.game.status = "COMPLETE".to_string();
            return
        }

        let n = self.game.board.size;
        if self.moves == (n * n) as i32 {
            println!("Game has ended in a draw");
            self.game.status = "COMPLETE".to_string();
        }

        self.change_player_turn();
            
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
            if self.game.board.cells[i as usize][(n - i - 1) as usize ] != symbol {
                break;
            }
            symbols_in_diagonal += 1;
        }

        return symbols_in_diagonal == n
    }

    pub fn is_winner(&self, row:usize, col: usize, symbol: char) -> bool {
        return self.is_row_complete(row, symbol) || 
        self.is_col_complete(col, symbol) ||
        self.is_diagonal_complete(symbol);
    }

    pub fn change_player_turn(&mut self) {
        let current_player_option = self.game.players.pop_front();
        match current_player_option {
            Some(current_player) => {
                self.game.players.push_back(current_player);
            },
            _ => {}
        }
    }

    pub fn get_player_input(&self) -> (i32, i32, bool) {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        user_input.truncate(user_input.len() -1); // remove last /n

        let inputs: Vec<i32> = user_input.split(" ")
        .map(|x| x.parse().expect("Not an integer!")).collect();
        
        if inputs.len() != 2 {
            println!("enter row and column with a space beetwen");
            return(0, 0, false);
        }

        return (inputs[0], inputs[1], true);
    }

}