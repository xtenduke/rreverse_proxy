use std::net::{TcpListener, TcpStream};
use std::str::{from_utf8};
use std::io::{Read, Write, BufReader, BufRead};
use std::time::SystemTime;

fn handle_client(mut client_stream: &TcpStream) {
    //GET / HTTP/1.1 Host: localhost:3001 User-Agent: curl/7.85.0 Accept: */*
    let fake_request = b"GET / HTTP/1.1\n\nHost: localhost:3001\n\nUser-Agent: curl/7.85.0\n\nAccept: */*\n\n\n\n";

    // read_to_end seems to be stripping some required line endings.. causing extreme slowness as
    // buf only flushes when the stream times out..
    // 
    let mut request_buffer = [0 as u8; 1024];
    // todo: handle connection lifecycle
    let n = client_stream.read(&mut request_buffer).expect("Socket read failed");
    //println!("{}", from_utf8(&request_buffer[0..n]).expect("request was not a string"));
    // let mut target_stream = TcpStream::connect("neverssl.com:80").expect("Failed to open target connection");
    // let mut target_stream = TcpStream::connect("localhost:3002").expect("Failed to open target connection");
    let mut target_stream = TcpStream::connect("localhost:3000").expect("Failed to open target connection");
    println!("read request data from client");
    // forward the original request to the target
    target_stream.write(&request_buffer[0..n]).unwrap();
    //target_stream.write(fake_request).unwrap();

    println!("Pass request data to server");
    // let client_request = from_utf8(&request_buffer).unwrap();
    println!("Forwarded fake request to target: {:?}", from_utf8(fake_request).unwrap());
    println!("Forwarded real request to target: {:?}", from_utf8(&request_buffer[0..n]).unwrap());
    // let data = read_to_end(&target_stream);
    let data = try_read(&target_stream).unwrap();
    client_stream.write_all(&data).expect("TODO: panic message");
}

fn try_read(target_stream: &TcpStream) -> Result<Vec<u8>, std::io::Error> {
    let mut reader = BufReader::new(target_stream);
    let mut name = String::new();
    loop {
        let r = reader.read_line(&mut name).unwrap();
        if r < 3 { //detect empty line
            break;
        }
    }
    let mut size = 0;
    let linesplit = name.split("\n");
    for l in linesplit {
        if l.starts_with("Content-Length") {
            let sizeplit = l.split(":");
            for s in sizeplit {
                if !(s.starts_with("Content-Length")) {
                    size = s.trim().parse::<usize>().unwrap(); //Get Content-Length
                }
            }
        }
    }
    let mut buffer = vec![0; size]; //New Vector with size of Content   
    println!("READ {}bytes", size);
    reader.read_exact(&mut buffer).unwrap(); //Get the Body Content.
    return Ok(buffer.to_vec())
}

fn read_to_end(mut target_stream: &TcpStream) -> Vec<u8> {
    let mut data = Vec::new();
    println!("Stream read to end...");
    let start = SystemTime::now();
    match target_stream.read_to_end(&mut data) {
        Ok(_) => {
            let read_time = SystemTime::now().duration_since(start)
                .expect("Time is broken")
                .as_secs();
            println!("Begin write back to client after {}s", read_time);
            return data;
        },
        Err(e) => {
            println!("Target failed: {}", e);
            // TODO: option here
            return Vec::new();
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
                handle_client(&stream);
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
