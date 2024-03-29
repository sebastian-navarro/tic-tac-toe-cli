use std::collections::VecDeque;

use super::board::Board;
use super::player::Player;

#[derive(Debug)]
pub struct Game {
    pub status: String,
    pub players: VecDeque<Player>,
    pub board: Board,
}

impl Game {
    pub fn new( players: VecDeque<Player>, board: Board) -> Self {
        Self { 
            players, 
            board,
            status: "NOT_STARTED".to_string(),
         }
    }
}