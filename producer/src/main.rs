use std::str::from_utf8;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut client_stream: TcpStream) {
    let mut request_buffer = [0 as u8; 1024];
    let mut target_stream = TcpStream::connect("localhost:3000").expect("Failed connecting to dest");
    loop {
        let n = client_stream.read(&mut request_buffer).expect("Socket read failed");
        println!("Connected to target");
        // forward the original request to the target
        target_stream.write(&request_buffer[0..n]).unwrap();
        let client_request = from_utf8(&request_buffer).unwrap();
        println!("Forwarded request to target: {}", client_request);
        let mut data = Vec::new();
        match target_stream.read_to_end(&mut data) {
            Ok(_) => {
                client_stream.write_all(&data).expect("TODO: panic message");
                println!("Write data to client stream, size {} bytes", data.len());
            },
            Err(e) => {
                println!("Target failed: {}", e);
            }
        }

        if n == 0 {
            println!("Done");
            return;
        }
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
                    // need to reference these threads?
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
