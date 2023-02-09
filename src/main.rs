use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn main() {
    // Start server
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(address).unwrap();

    // Listen for connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        println!("Stream received");
        println!("{}", String::from_utf8_lossy(&buffer[..]));

        let response = "HTTP/1.1 200 OK";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
