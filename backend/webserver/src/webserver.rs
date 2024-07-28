use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

pub fn listen() {
    //Start the Server with the "bind" function
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    //iterate over every stream that is incoming
    //each connection has one stream and to handle all of them we need to iterate over each
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //call the function for each of those streams
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    //create a buffer to read the responses
    //the buffer is initialized as an array of size 1024 with all positions set to 0
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("index.html").unwrap();

    //return the http response as the browser needs to
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}"
                           , contents.len(),
                           contents
    );
    //send response
    stream.write(response.as_bytes()).unwrap();
    //flush the output to ensure everything reached its target
    stream.flush().unwrap();
}