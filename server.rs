#![feature(globs)]
extern crate protobuf;

use std::io::net::tcp::{TcpListener, TcpStream};
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use std::io::{Acceptor, Listener};

mod request_vote_request;
// mod request_vote_response;

fn handle_client(stream: TcpStream) {
    println!("In handle_client!");
}

fn main() {
    let addr = SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8080 };
    let listener = TcpListener::bind(addr);
    let mut acceptor = listener.listen();

    for opt_stream in acceptor.incoming() {
        println!("Got connection!");
        match opt_stream {
            Ok(stream) => { handle_client(stream); }
            Err(_) => { println!("Error opening stream"); }
        }
    }
    drop(acceptor);
}
