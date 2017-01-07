use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    let response = "<html><h1>I'm Sorry Dave I can't do that</h1></html>";
    let content_length = response.to_string().capacity();
    println!("listening on port http://127.0.0.1:5000");
    println!("HTTP/1.1 200 OK Content-Type: text/html Content-Length: {}", content_length);
    for stream in listener.incoming() { match stream {
        Ok(stream) => {
            let mut stream = stream;
            handle_client(&mut stream);
        }
        Err(e) => {
            println!("Failed to connect on port\n{}", e);
            return;
        }
    } }
}

fn handle_client(stream: &mut TcpStream) {
    let response = "<html><h1>I'm Sorry Dave I can't do that</h1></html>";
    let content_length = response.to_string().capacity();
    let _ = stream.write(format!("HTTP/1.1 200 OK Content-Type: text/html Content-Length: {}", content_length).as_bytes());
    let _ = stream.write(response.as_bytes());
    let _ = stream.flush();
}

