use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
fn main() {
    let listerner = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listerner.incoming(){
        let stream = stream.unwrap();
        println!("Connection established");
        handle_stream(stream);
    }


}

fn handle_stream(mut stream: TcpStream){
    let mut buffer = [0; 1024];

    let get = b"GET / HTTP/1.1\r\n";

    
    stream.read(&mut buffer).unwrap();
    let (status, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n","index.html")
    } else{
        ("HTTP/1.1 404 OK\r\n\r\n","not_found.html")
    };
    let file = fs::read_to_string(filename).unwrap();
    let response = format!("{} {}",status,file);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
