// This is used to listen to a specific TCP port.
use std::{
    // This is used to read from a file.
    fs,
    //
    io::{prelude::*, BufReader},
    // Tcplistener is used to listen to a port and Tcpstream is used as a datatyp for data sent from an address.
    net::{TcpListener, TcpStream}, 
    thread, time::Duration,
};

use mulitthreaded_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let pool = ThreadPool::new(4);

        pool.execute(|| {
            handle_connection(stream);
        })
    }
    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    let buffer_reader = BufReader::new(&mut stream);
    let request_line: String = match buffer_reader.lines().next() {
        Some(Ok(val)) => val,
        Some(Err(_)) => "".to_string(),
        None => "".to_string(),
    };

    // The slash in the middle is for the path.
    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200", "hello.html")
        },
        _ => ("HTTP/1.1 404", "404.html")
    };
    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\n\
    Content-Length: {length}\r\n\r\n\
    {contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}
