use std::net::{TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello");
    let mut proxy_link= TcpStream::connect("localhost:4000").unwrap();
    proxy_link.write(b"whatthefuck").unwrap();

    let target = TcpStream::connect("localhost:3000").unwrap();

    let proxy_read = proxy_link.try_clone().expect("Failed to clone proxy conn");
    let target_write = target.try_clone().expect("Failed to clone target conn");

    thread::spawn(move|| {
        cross_streams(proxy_read, target_write);
    });

    thread::spawn(move|| {
        cross_streams(target, proxy_link);
    });

    println!("Done...");
}

fn cross_streams(mut source: TcpStream, mut destination: TcpStream) {
    println!("Cross streams");
    // writes from source to destination
    loop {
        println!("Looping...");
        // support 1MB...  probably not supporting much of the HTTP spec..
        // should implement growable buffer
        let mut temp_buffer = [0 as u8; 1024*1024];
        let read_result = source.read(&mut temp_buffer);
        match read_result {
            Ok(0) => {
                // return;
            }
            Ok(size) => {
                // write only what was read from
                destination.write(&temp_buffer[0..size]).unwrap();
            }
            Err(error) => {
                println!("Error reading from stream {}", error);
                // return;
            }
        }
    }
}