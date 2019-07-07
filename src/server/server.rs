extern crate rand;
#[path = "../game_objects.rs"] mod game_objects;
#[path = "../constants.rs"] mod constants;  
#[path = "../protocol.rs"] mod protocol;
#[path = "../number_helpers.rs"] mod number_helpers;  
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::io::{Read, Error, Write};
use constants::{BALL_SIZE, PADDLE_HEIGHT, PADDLE_WIDTH, SIG_RESET};
use protocol::{GameState};
use number_helpers::{as_i32};
use std::sync::atomic::{AtomicBool};

///starts listening on port 4545 handles incoming connections in new threads
pub fn main() {
    let game_reset_requested = Arc::new(Mutex::new(AtomicBool::new(true)));
    let game_state = Arc::new(Mutex::new(GameState::new()));
    let listener = TcpListener::bind("0.0.0.0:4545").expect("could not bind tcpListener");
    let mut player_index = 0; 
    {
        let game_state = game_state.clone();
        let game_reset_requested = game_reset_requested.clone();
        thread::spawn(move || {master_loop(game_state, game_reset_requested)});
    }
    for stream in listener.incoming() {
         match stream {
            Err(e) => { println!("Error in tcp stream {}", e) },
            Ok(stream) => {   
                let game_state = game_state.clone();
                let game_reset_requested = game_reset_requested.clone();
                let this_player = player_index;
                thread::spawn(move || { handle_incoming(stream, game_state, this_player, game_reset_requested)});
                player_index += 1;
            }
        }
    }
}

fn master_loop(game_state: Arc<Mutex<GameState>>, should_reset: Arc<Mutex<AtomicBool>>) {
    let mut ball_vel: [f32;2] = [-2.0, -1.0];
    let mut_g_s = game_state.lock().unwrap();
    let mut ball_exact_pos: [f32;2] = [mut_g_s.ball_position[0] as f32, mut_g_s.ball_position[1] as f32];
    drop(mut_g_s);
    loop {
        let mut scored = false;
        let mut mut_g_s = game_state.lock().unwrap();
        let mut mut_should_reset = should_reset.lock().unwrap();
        match  *mut_should_reset.get_mut() {
            false => {
                let collision_p1 = game_objects::is_colliding(mut_g_s.ball_position, 
                                    [mut_g_s.player_x_positions[0], mut_g_s.player_y_positions[0]], 
                                    [BALL_SIZE, BALL_SIZE], 
                                    [PADDLE_WIDTH, PADDLE_HEIGHT]);
                let collision_p2 = game_objects::is_colliding(mut_g_s.ball_position,
                                    [mut_g_s.player_x_positions[1], mut_g_s.player_y_positions[1]],
                                    [BALL_SIZE, BALL_SIZE],
                                    [PADDLE_WIDTH, PADDLE_HEIGHT]);

                if collision_p1.0 {
                    ball_vel[0] = ball_vel[0]*-1.0;
                    ball_vel[1] = ball_vel[1]+((rand::random::<f32>()-1.0)*3.0);
                    ball_exact_pos[0] = ball_exact_pos[0] + collision_p1.1;
                }
                else if collision_p2.0 {
                    ball_vel[0] = ball_vel[0]*-1.0;
                    ball_vel[1] = ball_vel[1]+((rand::random::<f32>()-1.0)*3.0);
                    ball_exact_pos[0] = ball_exact_pos[0] - collision_p2.1;
                }
                for i in 0..2 {
                    ball_exact_pos[i] += ball_vel[i];
                    mut_g_s.ball_position[i] = ball_exact_pos[i] as i32;
                }

                if ball_exact_pos[0] < 0.0 {
                    mut_g_s.scores[1] = mut_g_s.scores[1]+1;
                    scored = true;
                    ball_vel = [rand::random::<f32>()*-2.0-3.0, (rand::random::<f32>()-1.0)*5.0+1.0];
                }

                if ball_exact_pos[0] > (constants::WINDOW_WIDTH - BALL_SIZE) as f32 {
                    mut_g_s.scores[0] = mut_g_s.scores[0]+1;
                    scored = true;
                    ball_vel = [rand::random::<f32>()*2.0+3.0, (rand::random::<f32>()-1.0)*5.0+1.0];
                }

                if ball_exact_pos[1] <= 0.0 || ball_exact_pos[1] > (constants::WINDOW_HEIGHT-BALL_SIZE) as f32 {
                    ball_vel[1] = ball_vel[1]*-1.0;
                }

                if scored {
                    ball_exact_pos = [(constants::WINDOW_WIDTH as f32)/2.0, (constants::WINDOW_HEIGHT as f32)/2.0];
                    for i in 0..2 {
                        ball_exact_pos[i] += ball_vel[i];
                        mut_g_s.ball_position[i] = ball_exact_pos[i] as i32;
                    }
                }
            },
            true => {
                mut_g_s.reset();
                for i in 0..2 {
                    ball_exact_pos[i] = mut_g_s.ball_position[i] as f32;
                }
                scored = true;
                *mut_should_reset.get_mut() = false;
            }
        }

        //mut_g_s.player_y_positions[0] = ball_exact_pos[1] as i32;
        drop(mut_should_reset);
        drop(mut_g_s);
        match scored {
        true => thread::sleep(time::Duration::from_millis(1000)),
        false => thread::sleep(time::Duration::from_millis(10)),
        }
    }
}

fn handle_incoming(mut stream: TcpStream, game_state: Arc<Mutex<GameState>>, player_index: i32, should_reset: Arc<Mutex<AtomicBool>>) -> Result<([u8; 1024], usize), Error> {
    println!("A connection from: {}", stream.peer_addr()?);
    loop {    
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read != 0 {
        let message_as_integer = as_i32(&buffer[0 .. 4]);
        let mut mutable_game_state = game_state.lock().unwrap();
        match message_as_integer {
            SIG_RESET => {let mut atomic_bool_lock = should_reset.lock().unwrap();
                *atomic_bool_lock.get_mut() = true;
                drop(atomic_bool_lock);}
            _ => {mutable_game_state.player_y_positions[player_index as usize] += message_as_integer}
        }
        let serialized_state = protocol::serialize(mutable_game_state.clone(), player_index);
        drop(mutable_game_state); 
        stream.write(&serialized_state).expect("Could not write response to stream");
        }
    }
}

