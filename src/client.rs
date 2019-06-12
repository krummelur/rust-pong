use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{self, Read, Error, Write};
use networkGame::GameState;

const SERVER_ADDR: &'static str = "127.0.0.1:4545";


pub fn start() {
    let mut connection_stream = connect().expect("could not connect to server");
    loop {
    let message = String::from("this is my test message");
    send_message(&mut connection_stream, message);
    thread::sleep_ms(100);
    }
}

fn connect() -> io::Result<TcpStream> {
    TcpStream::connect(SERVER_ADDR)
}  

pub fn send_message(stream: &mut TcpStream, message: String) {
    //println!("sending message: {:?}", message.as_bytes());
    stream.write(message.as_bytes()).expect("there was an error sending message");
}