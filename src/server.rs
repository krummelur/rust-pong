use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::io::{Read, Error, Write};
use protocol::{self, GameState};
use number_helpers::{i32_to_array_of_u8, as_i32};

///starts listening on port 4545 handles incoming connections in new threads
pub fn start() {
    let game_state_obj = GameState::new();
    let game_state = Arc::new(Mutex::new(game_state_obj));
    let listener = TcpListener::bind("0.0.0.0:4545").expect("could not bind tcpListener");
    let mut player_index = 0; 
    {
        let game_state = game_state.clone();
        thread::spawn(move || {master_loop(game_state)});
    }
    for stream in listener.incoming() {
         match stream {
            Err(e) => { println!("Error in tcp stream {}", e) },
            Ok(stream) => {   
                let game_state = game_state.clone();
                let this_player = player_index;
                thread::spawn(move || { handle_incoming(stream, game_state, this_player) });
                player_index += 1;
            }
        }
    }
}

fn master_loop(mut game_state: Arc<Mutex<GameState>>) {
    let mut ball_vel: [f32;2] = [0.5;2];
    let mut_g_s = game_state.lock().unwrap();
    let mut ball_exact_pos: [f32;2] = [mut_g_s.ball_position[0] as f32, mut_g_s.ball_position[1] as f32];
    drop(mut_g_s);
    loop {
        let mut mut_g_s = game_state.lock().unwrap();
        for i in 0..2 {
            ball_exact_pos[i] += ball_vel[i];
            mut_g_s.ball_position[i] = ball_exact_pos[i] as i32;
        }
        drop(mut_g_s);
        thread::sleep(time::Duration::from_millis(10));
    }
}

fn handle_incoming(mut stream: TcpStream, game_state: Arc<Mutex<GameState>>, player_index: i32) -> Result<([u8; 1024], usize), Error> {
    println!("A connection from: {}", stream.peer_addr()?);
    loop {    
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read != 0 {
        let message_as_integer = as_i32(&buffer[0 .. 4]);
        let mut mutable_game_state = game_state.lock().unwrap();
        mutable_game_state.player_y_positions[player_index as usize] += message_as_integer;
        let serialized_state = protocol::serialize(mutable_game_state.clone(), player_index);
        drop(mutable_game_state); 
        stream.write(&serialized_state).expect("Could not write response to stream");
        }
    }
}

