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

    pub fn send_message_i32(&mut self, message: i32) -> GameState {
        self.stream.write(&i32_to_array_of_u8(message)).expect("there was an error sending message to server");
        let mut buffer = [0; 9*4];
        let bytes_read = self.stream.read(&mut buffer).expect("could not read from stream");
        println!("received from server: [");
        for i in 0..bytes_read {
        print!("{},", buffer[i]);
        }
        println!("]");
        protocol::deserialize(buffer)
    }
}

fn connect() -> io::Result<TcpStream> {
    TcpStream::connect(SERVER_ADDR)
}  