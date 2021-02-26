use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::sleep as thread_sleep;
use std::time::Duration;

fn sleep(secs: u64) {
    thread_sleep(Duration::new(secs, 0));
}

fn main() {
    println!("Hello! Welcome to Rust client!");
    println!("Let's start by typing your naming and expecting some cool stuff:");
    let mut string = String::new();
    std::io::stdin()
        .read_line(&mut string)
        .expect("Failed reading name x_x");
    string.push('\n');
    let address = "localhost";
    let port = 3000;
    let address_port = format!("{}:{}", address, port);
    let mut stream = TcpStream::connect(address_port).expect("Couldn't connect to server x_x");
    let name_as_bytes = string.as_bytes();
    // name_as_bytes.concat(EOF);
    for byte in name_as_bytes {
        println!("Byte: {}", byte);
    }
    stream.write(&name_as_bytes).expect("Couldn't write ");
    println!("Wrote {} to stream, gonna wait for reply...", string);
    let mut stream_value = String::new();
    stream
        .read_to_string(&mut stream_value)
        .expect("Couldn't read to string x_x");
    while stream_value.len() == 0 {
        println!("Reading...");
        stream.read_to_string(&mut stream_value).unwrap();
        sleep(3);
    }
    println!("{}", stream_value);
}
