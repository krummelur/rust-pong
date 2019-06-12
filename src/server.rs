use std::net::{TcpListener, TcpStream};
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Read, Error};
use networkGame::GameState;
use std::ops::Deref;

///starts listening on port 4545 handles incoming connections in new threads
pub fn start() {
    let game_state_obj = GameState::new();
    let game_state = Arc::new(Mutex::new(game_state_obj));
    let listener = TcpListener::bind("0.0.0.0:4545").expect("could not bind tcpListener");
    for stream in listener.incoming() {
         match stream {
            Err(e) => { println!("Error in tcp stream {}", e) },
            Ok(stream) => {   
                let game_state = game_state.clone();

            thread::spawn(move || { handle_incoming(stream, game_state) });
            }
        }
        {
            let game_state = game_state.clone();
        thread::spawn(move || { 
            loop {
            let guard = game_state.lock().unwrap();
            let new_player1_pos_x = guard.player1_pos_x;
            println!("new state of pos_x: {}", new_player1_pos_x);
            drop(guard);
            thread::sleep_ms(100);
        }});
        }
    }
}

fn handle_incoming(mut stream: TcpStream, game_state: Arc<Mutex<GameState>>) -> Result<([u8; 1024], usize), Error> {
    println!("A connection from: {}", stream.peer_addr()?);
    loop {    
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read != 0 {
        let my_string = std::str::from_utf8(&buffer).expect("could not parse as utf8");
        println!("MESSAGE RECEIVED: {},message length: {}", my_string, bytes_read);
        let mut mutable_game_state = game_state.lock().unwrap();
        //println!("state: {}", mutable_game_state.player1_pos_x);
        mutable_game_state.player1_pos_x += 1;
        drop(mutable_game_state);
        }
    }
}