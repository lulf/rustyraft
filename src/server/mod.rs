use std::io::{Listener, Acceptor, IoResult, IoError, IoErrorKind, InvalidInput};
use std::io::net::tcp::{TcpListener, TcpStream};
use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io;
use std::str;
use std::ascii;
use self::protocol::{RequestVoteRequest, AppendEntriesRequest, RequestVoteResponse, AppendEntriesResponse, RpcResponse, RpcRequest};

mod protocol;

#[deriving(Clone)]
#[deriving(Show)]
pub struct ServerSpec {
    host: &'static str,
    port: u16
}

impl ServerSpec {
    pub fn new(hostname: &'static str, port: u16) -> ServerSpec {
        ServerSpec { host: hostname, port: port }
    }
}

enum State {
    Candidate,
    Follower,
    Leader
}

struct ServerState {
    currentTerm: uint,
    votedFor: Option<uint>,
    log: Vec<Vec<u8>>,

    commitIndex: uint,
    lastApplied: uint,

    nextIndex: Vec<uint>,
    matchIndex: Vec<uint>,
    state: State
}

impl ServerState {
    pub fn handleRequestVote(&self, request: RequestVoteRequest) {
        let ret = RequestVoteResponse {
            term: 0,
            voteGranted: false
        };
        //box ret as Box<RpcResponse>
    }
    pub fn handleAppendEntries(&self, request: AppendEntriesRequest) {
        let ret = AppendEntriesResponse {
            term: 0,
            success: false
        };
        //box ret as Box<RpcResponse>
    }
}

pub fn start_server(serverId:uint, servers:&Vec<ServerSpec>) {
    let mut state:ServerState = ServerState {
        currentTerm: 0,
        votedFor: None,
        log: Vec::new(),
        commitIndex: 0,
        lastApplied: 0,
        nextIndex: Vec::new(),
        matchIndex: Vec::new(),
        state: Follower
    };
    let mut others = servers.clone();
    let mySpec = others.remove(serverId).unwrap();
    spawn(proc() {
        run_raft_server(&state, mySpec, others);
    });
}

fn run_raft_server(mut state: &ServerState, me:ServerSpec, others:Vec<ServerSpec>) {
    let mut acceptor = TcpListener::bind("127.0.0.1", me.port).listen();
    match acceptor {
        Err(e) => { println!("Error listening to {} ", me.port) }
        Ok(_) => {
            for stream in acceptor.incoming() {
                match stream  {
                    Err(e) => { println!("Error handling client connection!") }
                    Ok(mut tcpStream) => handle_client(&mut tcpStream, state, others.clone())
                }
            }
            drop(acceptor);
        }
    }
    //    stream.write(bytes!("Hello World\r\n"));
    //}
}

fn handle_client(stream: &mut TcpStream, mut state: &ServerState, others:Vec<ServerSpec>) {
    let input: IoResult<u8> = stream.read_byte();
    static REQUEST_VOTE: u8 = '1' as u8;
    static APPEND_ENTRIES: u8 = '2' as u8;
    match input {
        Ok(REQUEST_VOTE) => {
            let request: IoResult<RequestVoteRequest> = RpcRequest::decode(stream);
            request.map(|req| { state.handleRequestVote(req); });
        }
        Ok(APPEND_ENTRIES) => {
            let request: IoResult<AppendEntriesRequest> = RpcRequest::decode(stream);
            request.map(|req| { state.handleAppendEntries(req); });
        }
        Ok(_) => {
            println!("Unknown command");
        }
        Err(e) => {
            println!("Error reading: {}", e);
        }
    }
}
