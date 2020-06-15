use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use serde::{Serialize, Deserialize};

use bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            // let msg = b"Hello!";
            let msg = Point{ x: 1, y: 2};
            // Convert the Point to a bincode.
            let serialized = serialize(&msg).unwrap();
            println!("Serialized Point: {:?}", serialized.as_slice());
            stream.write(serialized.as_slice()).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 8]; // using 8 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == serialized.as_slice() {
                        println!("Reply is ok!");
                        let decoded: Point = deserialize(&data).unwrap();
                        println!("Reply is...{:?}", decoded);
                    } else {
                        // let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {:?}", data);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
