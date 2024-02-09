use std::io::{stdin, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = if let Ok(stream) = TcpStream::connect("127.0.0.1:5000") {
        println!("Connected to server! Happy chatting :-)");
        stream
    } else {
        panic!("Could not connect to the server...")
    };

    let mut input = String::new();

    loop {
        match stdin().read_line(&mut input) {
            Ok(_) => { stream.write(&input.as_bytes()); },
            Err(e) => (),
        }
        input = String::new();
    }
}
