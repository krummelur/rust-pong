pub struct GameState{
    pub player1_score: i32,
    pub player2_score: i32,
    pub player1_pos_x: i32,
    pub player2_pos_x: i32,
    pub player1_pos_y: i32,
    pub player2_pos_y: i32, 
}

impl GameState {
    pub fn new() -> GameState {
        GameState { player1_score: 0, player2_score: 0, player1_pos_x: 0, player2_pos_x: 0, player1_pos_y: 0, player2_pos_y: 0}
    }
    pub fn testFunction(self) {
        println!("called function in GameState");
    }
}