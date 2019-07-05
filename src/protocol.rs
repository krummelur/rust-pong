const MESSAGE_LEN: usize = (9*4);
use std::vec::Vec;
use number_helpers::{as_i32, i32_to_array_of_u8};
use constants;

#[derive(Copy, Clone)]
pub struct GameState{
    pub ball_position: [i32; 2],
    pub player_x_positions: [i32; 2],
    pub player_y_positions: [i32; 2],
    pub scores: [i32; 2]
}

impl GameState {
    /// Returns a reference to a new GameState instance with default values 
    pub fn new() -> GameState {
        GameState { ball_position: [200,200], player_x_positions: [20, 380],  player_y_positions: [0,0], scores: [0,0]}
    }
}

/// Returns a new GameState instance deserialized from a u8 Array
/// 
/// # Arguments
///
/// * `message` - The message containing the serialized GameState
pub fn deserialize(message: [u8; MESSAGE_LEN]) -> GameState {
    let player_x_positions_ = match as_i32(&message[32..36]) {
            0 => [as_i32(&message[0..4]),as_i32(&message[4..8])],
            1 => [as_i32(&message[4..8]),as_i32(&message[0..4])],
            _ => panic!("no player index received from server")
        };

    let player_y_positions_ = match as_i32(&message[32..36]) { 
        0 => [as_i32(&message[8..12]),as_i32(&message[12..16])],
        1 => [as_i32(&message[12..16]),as_i32(&message[8..12])],
        _ => panic!("no player index received from server")
    };

    let ball_position = [as_i32(&message[16..20]),as_i32(&message[20..24])];
    GameState {
        ball_position: ball_position,
        player_x_positions: player_x_positions_,
        player_y_positions: player_y_positions_,
        scores: [0,0]
    }
}
/// Returns a serialized gamestate from GameState as array of u8  
/// 
/// # Arguments
///
/// * `game_state` - The GameState to serialize
pub fn serialize(game_state: GameState, player_index: i32) -> [u8; MESSAGE_LEN] {
    let mut arr_game_state: [u8; 9*4] = [0; 9*4];
    let arr_game_state_unflat = [            
        i32_to_array_of_u8(game_state.player_x_positions[0]),
        i32_to_array_of_u8(game_state.player_x_positions[1]),
        i32_to_array_of_u8(game_state.player_y_positions[0]),
        i32_to_array_of_u8(game_state.player_y_positions[1]),
        i32_to_array_of_u8(game_state.ball_position[0]),
        i32_to_array_of_u8(game_state.ball_position[1]),
        i32_to_array_of_u8(game_state.scores[0]),
        i32_to_array_of_u8(game_state.scores[1]),
        i32_to_array_of_u8(player_index),
        ]; 
        for i in 0..4 {
            arr_game_state[i] = arr_game_state_unflat[0][i];
            arr_game_state[4+i] = arr_game_state_unflat[1][i];
            arr_game_state[8+i] = arr_game_state_unflat[2][i];
            arr_game_state[12+i] = arr_game_state_unflat[3][i];
            arr_game_state[16+i] = arr_game_state_unflat[4][i];
            arr_game_state[20+i] = arr_game_state_unflat[5][i];
            arr_game_state[24+i] = arr_game_state_unflat[6][i];
            arr_game_state[28+i] = arr_game_state_unflat[7][i];
            arr_game_state[32+i] = arr_game_state_unflat[8][i];
        }
        arr_game_state
}