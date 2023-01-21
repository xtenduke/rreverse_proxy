use std::str::from_utf8;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut client_stream: TcpStream) {
    let mut buffer = [0 as u8; 1024]; // using 1024 byte buffer
    loop {
        let n = client_stream.read(&mut buffer).expect("Socket read failed");

        // open a tcp connection to the target - send data back over original TCP connection
        match TcpStream::connect("localhost:8000") {
            Ok(mut target_stream) => {
                println!("Connected to target");
                // forward the original request to the target
                target_stream.write(&buffer[0..n]).unwrap();
                let client_request = from_utf8(&buffer).unwrap();
                println!("Forwarded request to target: {}", client_request);
                let mut data = [0 as u8; 1024]; // 1024 byte buff
                match target_stream.read(&mut data) {
                    Ok(_) => {
                        let target_response = from_utf8(&data).unwrap();
                        println!("Received target data: {}", target_response);
                        // return the response from the target back to the original client
                        client_stream.write_all(&data[0..n]).expect("Socket write failed");
                        println!("Write data to client stream");
                    },
                    Err(e) => {
                        println!("Target failed: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("Target connection failed: {}", e);
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
