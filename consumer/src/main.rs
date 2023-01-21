use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:8000") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3001");

            // valid http/1.1 request
            let msg = b"GET /index.html HTTP/1.1

                ";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 1024]; // using 1024 byte buffer
            match stream.read(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("got response: {}", text);
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
