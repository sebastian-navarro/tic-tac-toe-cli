mod domains;
use std::collections::VecDeque;

mod services;
use services::game_manager::GameManager;

use domains::player::Player;

fn main() {
    let player1 = Player::new("Seba".to_string(),'X');
    let player2 = Player::new("Dani".to_string(),'O');
    let mut players: VecDeque<Player> = VecDeque::new();
    players.push_back(player1);
    players.push_back(player2);
    
    let mut tic_tac_toe_game_manager = GameManager::new(players, 3);
    tic_tac_toe_game_manager.play_game();
}
