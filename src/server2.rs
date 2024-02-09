use std::str;
use std::io::{self, Read, Error};
use std::net::{SocketAddr, TcpStream, TcpListener};
use tokio;

async fn listen_to_stream(s: &mut TcpStream) {
    let mut m_buf: [u8; 128]  = [0; 128];
    println!("Now listening to the new user...");
    loop {
        println!("Starting to read");
        s.read(&mut m_buf);
        println!("Read it :)");
        let mut message = match str::from_utf8(&m_buf) {
            Ok(m) => String::from(m),
            Err(_) => String::from("Error"),
        };
        message.pop();
        println!("{}", message);
        for elem in m_buf.iter_mut() { *elem = 0 }
    }
}

async fn receive_connection(listener: &TcpListener) -> Result<(TcpStream, SocketAddr), Error> {
        let stream = listener.accept();
        if let Ok(_) = stream {
            println!("Welcome to the new user :-)");
        }
        stream  
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    loop {
        let (mut stream, _): (TcpStream, SocketAddr) = receive_connection(&listener).await.unwrap();
        listen_to_stream(&mut stream).await;

    }
}
