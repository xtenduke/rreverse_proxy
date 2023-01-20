use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0 as u8; 1024]; // using 1024 byte buffer
    loop {
        let n = stream.read(&mut buffer).expect("Socket read failed");
        if n == 0 {
            println!("Done");
            return;
        }

        stream.write_all(&buffer[0..n]).expect("Socket write failed");
   }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3001").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3001");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
