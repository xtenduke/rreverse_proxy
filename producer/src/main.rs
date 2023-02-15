use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread::spawn;

fn handle_client(client_stream: TcpStream) {
    let server_stream = TcpStream::connect("localhost:3000").expect("Failed to open target connection");

    let client_read = client_stream.try_clone().unwrap();
    let server_read = server_stream.try_clone().unwrap(); // todo: error handle
    spawn(move|| {
        cross_streams(client_read, server_stream);
    });

    spawn(move|| {
        cross_streams(server_read, client_stream)
    });
}

fn cross_streams(mut source: TcpStream, mut destination: TcpStream) {
    // writes from source to destination
    loop {
        let mut buffer = [0 as u8; 1024*1024];
        let n = source.read(&mut buffer);
        match n {
            Ok(0) => {
                return;
            }
            Ok(_) => {
                destination.write_all(&buffer).unwrap();
            }
            Err(_) => {
                return;
            }
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
                handle_client(stream);
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
