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
        Self { game, moves:0 }
    }
}