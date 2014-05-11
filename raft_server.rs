extern crate sync;

use std::io;
use std::os;
use std::num;
use std::str;
use std::vec;
use std::rc::Rc;

struct RequestVoteRequest {
    term: uint,
    candidateId: uint,
    lastLogIndex: uint,
    lastLogTerm: uint
}

struct ServerState {
    currentTerm: uint,
    votedFor: Option<uint>,
    log: Vec<Vec<u8>>,

    commitIndex: uint,
    lastApplied: uint,

    nextIndex: Vec<uint>,
    matchIndex: Vec<uint>
}

fn start_server(serverId:uint, servers:&Vec<ServerSpec>) {
    let mut state:ServerState = ServerState {
        currentTerm: 0,
        votedFor: None,
        log: Vec::new(),
        commitIndex: 0,
        lastApplied: 0,
        nextIndex: Vec::new(),
        matchIndex: Vec::new()
    };
    let mut others = servers.clone();
    let mySpec = others.remove(serverId).unwrap();
    spawn(proc() {
        run_server(state, mySpec, others);
    });
}

fn run_server(state: ServerState, me:ServerSpec, others:Vec<ServerSpec>) {
    println!("Starting server on port {}. Others: {}", me.port, others);
    loop {
    }
}

#[deriving(Clone)]
#[deriving(Show)]
struct ServerSpec {
    host: &'static str,
    port: uint
}

fn main() {
    let args = os::args();
    if args.len() < 2 {
        println!("Usage: {} <port...>", args[0]);
        return;
    }
    let num_servers:uint = args.len() - 1;
    let mut servers:Vec<ServerSpec> = Vec::new();
    for i in range(1, num_servers + 1) {
        let server_port:uint = from_str(args[i]).unwrap();
        let server_host = "localhost";
        servers.push(ServerSpec{host: "localhost", port:server_port});
    }
    for i in range(0, num_servers) {
        start_server(i, &servers);
    }
    loop {}
    println!("Launching {} servers", num_servers);
}

