use std::net::TcpListener;

pub fn listen(){
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established");
    }
}