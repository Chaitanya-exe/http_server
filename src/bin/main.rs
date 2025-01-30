use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use http_server::ThreadPool;

fn main() {
    let listerner = TcpListener::bind("127.0.0.1:8080").unwrap();

    let pool = ThreadPool::new(4);
    for stream in listerner.incoming(){
        let stream = stream.unwrap();
        println!("Connection established");
        pool.execute(||{
            handle_stream(stream);
        });
    }


}

fn handle_stream(mut stream: TcpStream){
    let mut buffer = [0; 1024];

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    
    stream.read(&mut buffer).unwrap();
    let (status, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n","index.html")
    }else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n","index.html")
    }
     else{
        ("HTTP/1.1 404 OK\r\n\r\n","not_found.html")
    };
    let file = fs::read_to_string(filename).unwrap();
    let response = format!("{} {}",status,file);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
