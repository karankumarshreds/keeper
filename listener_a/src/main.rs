use std::net::{TcpListener, TcpStream};

use std::io::prelude::*;
use serde_json as json;
use serde::Deserialize;

const PORT: u16 = 8080;

#[derive(Deserialize)]
pub struct User {
    pub name: String,
    pub age: i32,
}

fn main() {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        let stream = stream.expect("Should get stream");
        println!("Accepted connection from {}", stream.peer_addr().expect("should get peer address"));
        handle_stream(stream);
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let n_bytes = stream.read(&mut buf).expect("should read");
    // let s = String::from_utf8((&buf[..n_bytes]).to_vec()).expect("should get string");
    let u = json::from_slice::<User>(&buf[..n_bytes]).expect("should get user");
    println!("Got user: {:?}, {:?}", u.name, u.age);
}
