use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread::spawn;

fn handle_client(client_stream: TcpStream, proxy_link: TcpStream) {
    // read from the client_stream.. write directly to the proxy_link
    // read from the proxy_link.. write directly to the client_stream

    let client_read = client_stream.try_clone().expect("Failed to clone client");
    let proxy_read = proxy_link.try_clone().expect("Failed to clone server"); // todo: error handle
    spawn(move|| {
        cross_streams(client_read, proxy_link);
    });

    spawn(move|| {
        cross_streams(proxy_read, client_stream)
    });
}

fn cross_streams(mut source: TcpStream, mut destination: TcpStream) {
    // writes from source to destination
    loop {
        // support 1MB...  probably not supporting much of the HTTP spec..
        // should implement growable buffer
        let mut temp_buffer = [0 as u8; 1024*1024];
        let read_result = source.read(&mut temp_buffer);
        match read_result {
            Ok(0) => {
                return;
            }
            Ok(size) => {
                println!("Reading.....");
                // write only what was read from 
                destination.write(&temp_buffer[0..size]).unwrap();
            }
            Err(error) => {
                println!("Error reading from stream {}", error);
                return;
            }
        }
    }
}

fn main() {
    let proxy_server = TcpListener::bind("0.0.0.0:4000").unwrap();
    let proxy_link = proxy_server.incoming().next().unwrap().expect(""); // TODO: error and backoff handle

    let listener = TcpListener::bind("0.0.0.0:3001").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3001");
    for stream in listener.incoming() {
        let temp_proxy_link = proxy_link.try_clone().unwrap();
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream, temp_proxy_link);
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
