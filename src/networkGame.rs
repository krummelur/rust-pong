use std::vec::Vec;

pub struct GameState{
    pub ball_position: [i32; 2],
    pub player_x_positions: [i32; 2],
    pub player_y_positions: [i32; 2],
  
}

impl GameState {
    pub fn new() -> GameState {
        GameState { ball_position: [200,200], player_x_positions: [20, 380],  player_y_positions: [0,0]}
    }
    pub fn testFunction(self) {
        println!("called function in GameState");
    }
}