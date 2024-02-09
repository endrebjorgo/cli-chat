use std::str;
use std::io::Read;
use std::net::{TcpStream, TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    let mut m_buf: [u8; 128]  = [0; 128];

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                stream.read(&mut m_buf);
                let message = match str::from_utf8(&m_buf) {
                    Ok(m) => m,
                    Err(e) => "UNABLE TO RECEIVE MESSAGE",
                };
                println!("{}", message);
                for elem in m_buf.iter_mut() { *elem = 0 }
            }
            Err(e) => {
                println!("{e:?}");
            }
        }
    }
}
