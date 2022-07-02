use std::io::Write;
use std::str;
use std::thread;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    // Set a buffer for incoming bytes
    let mut buffer = [0; 1024];

    // Reading data continuously
    loop {
        // Transfer the incoming bytes into the buffer
        let n = stream.read(&mut buffer).unwrap();
        // &[u8] to &str
        let msg = str::from_utf8(&buffer[..n]).unwrap();

        // When the message start with "exit", close the communication
        if msg.starts_with("exit") {
            stream
                .write(b"[Attention!] The communication is over.\n")
                .unwrap();
            break;
        }

        // Response: "Get the message:" + incoming data
        stream
            .write(format!("Get the message: {}", msg).as_bytes())
            .unwrap();
    }
}

fn main() {
    // Creates a new TcpListener which will be bound to the address "127.0.0.1:7000"
    // Use 'unwrap()' to handle the error implicitly
    let listener = TcpListener::bind("127.0.0.1:7000").unwrap();

    // -----------------------------------------------------
    // Or use 'match' explicitly
    // let listener = TcpListener::bind("127.0.0.1:7000");
    // let listener = match listener {
    //     Ok(l) => l,
    //     Err(e) => panic!("Error: {}", e),
    // };
    // -----------------------------------------------------

    // Get the incoming TCP stream from the listener iteratively
    for stream in listener.incoming() {
        // Get the inner value, and pattern matching
        match stream {
            // When get a TcpStream
            Ok(stream) => {
                // Spawn a new thread, and move the 'stream' into the 'handle_connection()'
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            // When get an Error, panic.
            Err(e) => panic!("Error: {}", e),
        }
    }
}
