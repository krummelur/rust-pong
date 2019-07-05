const MESSAGE_LEN: usize = (9*4);
use std::vec::Vec;
use number_helpers::as_i32;

pub struct GameState{
    pub ball_position: [i32; 2],
    pub player_x_positions: [i32; 2],
    pub player_y_positions: [i32; 2],
    pub scores: [i32; 2]
}

impl GameState {
    pub fn new() -> GameState {
        GameState { ball_position: [200,200], player_x_positions: [20, 380],  player_y_positions: [0,0], scores: [0,0]}
    }
}

pub fn deserialize(message: [u8; MESSAGE_LEN]) -> GameState {
    let player_y_positions = match as_i32(&message[32..36]) {
            0 => [as_i32(&message[8..12]),as_i32(&message[12..16])],
            1 => [as_i32(&message[12..16]),as_i32(&message[8..12])],
            _ => panic!("no player index received from server")
        };

    let player_x_positions = [as_i32(&message[0..4]),as_i32(&message[4..8])];
    let ball_position = [as_i32(&message[16..20]),as_i32(&message[20..24])];
    GameState {
        ball_position: ball_position,
        player_x_positions: player_x_positions,
        player_y_positions: player_y_positions,
        scores: [0,0]
    }
}