use std::str::from_utf8;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut client_stream: TcpStream) {
    let mut request_buffer = [0 as u8; 1024];
    loop {
        let n = client_stream.read(&mut request_buffer).expect("Socket read failed");

        // open a tcp connection to the target - send data back over original TCP connection
        // reuse the connection?
        match TcpStream::connect("localhost:3000") {
            Ok(mut target_stream) => {
                println!("Connected to target");
                // forward the original request to the target
                target_stream.write(&request_buffer[0..n]).unwrap();
                // let client_request = from_utf8(&request_buffer).unwrap();
                // println!("Forwarded request to target: {}", client_request);
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
