use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{self, Read, Error, Write, BufReader, BufRead};
use protocol::{self, GameState};
use std::mem::transmute;
use number_helpers::{i32_to_array_of_u8, as_i32};
const SERVER_ADDR: &'static str = "127.0.0.1:4545";

pub struct Client {
    stream : TcpStream
}

impl Client {
    pub fn new() -> Client {
        Client { stream: connect().expect("could not connect to server") }
    }

    pub fn send_message_string(&mut self, message: String) -> GameState {
        self.stream.write(message.as_bytes()).expect("there was an error sending message");
        let mut buffer: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&self.stream);
        reader.read(&mut buffer).expect("Could not read response from server");
        //let my_string = std::str::from_utf8(&buffer).expect("could not parse as message as utf8");
        println!("received from server: {:?}", buffer);
        let mut new_state = GameState::new();
        new_state.player_x_positions = [as_i32(&buffer[0..4]),as_i32(&buffer[4..8])];
        new_state.player_x_positions = [as_i32(&buffer[8..12]),as_i32(&buffer[12..16])];
        new_state.player_x_positions = [as_i32(&buffer[16..20]),as_i32(&buffer[20..24])];
        new_state
    }

    pub fn send_message_i32(&mut self, message: i32) -> GameState {
        self.stream.write(&i32_to_array_of_u8(message)).expect("there was an error sending message to server");
        let mut buffer = [0; 9*4];
        let bytes_read = self.stream.read(&mut buffer).expect("could not read from stream");
        //let my_string = std::str::from_utf8(&buffer).expect("could not parse as message as utf8");
        println!("received from server: [");
        for i in 0..bytes_read {
        print!("{},", buffer[i]);
        }
        println!("]");
        /*
        let mut new_state = GameState::new();
        new_state.player_y_positions = match as_i32(&buffer[32..36]) {
            0 => [as_i32(&buffer[8..12]),as_i32(&buffer[12..16])],
            1 => [as_i32(&buffer[12..16]),as_i32(&buffer[8..12])],
            _ => panic!("no player index received from server")
        };
        new_state.player_x_positions = [as_i32(&buffer[0..4]),as_i32(&buffer[4..8])];
        //new_state.ball_position = [as_i32(&buffer[16..20]),as_i32(&buffer[20..24])];
        new_state.ball_position = [as_i32(&buffer[24..28]),as_i32(&buffer[28..32])];
        new_state*/
        protocol::deserialize(buffer)
    }
}

fn connect() -> io::Result<TcpStream> {
    TcpStream::connect(SERVER_ADDR)
}  