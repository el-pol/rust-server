use std::fs;
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

        let get = b"GET HTTP/1.1";

        if buffer.starts_with(get) {
            send_index(stream);
        } else {
            send_not_found(stream);
        }

        // send_to_client(stream);
        fn build_response(content: String) -> String {
            format!(
                "HTTP/1.1 200 OK\r\n Content-Length: {}\r\n\r\n{}",
                content.len(),
                content
            )
        }

        fn send_index(mut stream: TcpStream) {
            let contents = fs::read_to_string("index.html").unwrap();

            stream.write(build_response(contents).as_bytes()).unwrap();
            stream.flush().unwrap();
        }

        fn send_not_found(mut stream: TcpStream) {
            let contents = fs::read_to_string("404.html").unwrap();

            stream.write(build_response(contents).as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
