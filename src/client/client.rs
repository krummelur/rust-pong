use std::net::{TcpStream};
use std::io::{self, Read, Write};
use protocol::{self, GameState};
use number_helpers::{i32_to_array_of_u8};

pub struct Client {
    stream : TcpStream
}

impl Client {
    /// Returns a new instance of Client
    pub fn new(address: &str) -> Client {
        Client { stream: connect(address).expect("could not connect to server") }
    }

    /// Sends update message to the server
    /// 
    /// Returns the new gamestate from the server
    ///
    /// # Arguments
    ///
    /// * `message` - an 32-bit integer containing the direction the player wants to move
    pub fn send_message_i32(&mut self, message: i32) -> GameState {
        self.stream.write(&i32_to_array_of_u8(message)).expect("there was an error sending message to server");
        let mut buffer = [0; 9*4];
        self.stream.read(&mut buffer).expect("could not read from stream");
        protocol::deserialize(buffer)
    }
}

fn connect(address: &str) -> io::Result<TcpStream> {
    TcpStream::connect(address)
}