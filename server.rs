use std::io::{Listener, Acceptor, IoResult, IoError, IoErrorKind, InvalidInput};
use std::io::net::tcp::{TcpListener, TcpStream};
use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io;
use std::str;
use std::ascii;
use self::protocol::{RequestVoteRequest, AppendEntriesRequest, read_request_vote, read_append_entries};

mod protocol;

#[deriving(Clone)]
#[deriving(Show)]
pub struct ServerSpec {
    host: &'static str,
    port: u16
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
    let mut acceptor = TcpListener::bind(SocketAddr {
                                        ip: Ipv4Addr(127, 0, 0, 1),
                                        port: me.port
                                     }).listen();
    match acceptor {
        Err(e) => { println!("Error listening to {} ", me.port) }
        Ok(_) => {
            for stream in acceptor.incoming() {
                match stream {
                    Err(e) => { println!("Error handling client connection!") }
                    Ok(stream) => handle_client(stream, state, others.clone())
                }
            }
            drop(acceptor);
        }
    }
    //    stream.write(bytes!("Hello World\r\n"));
    //}
}

fn handle_client(mut stream: TcpStream, mut state: &ServerState, others:Vec<ServerSpec>) {
    let cmd = read_rpc_command(stream);
    println!("Command {}", cmd);
}

#[deriving(Clone)]
#[deriving(Show)]
enum RpcRequest {
    RequestVoteRequest,
    AppendEntriesRequest
}

fn read_rpc_command(mut stream: TcpStream) -> IoResult<RpcRequest> {
    let input = stream.read_byte();
    static REQUEST_VOTE: u8 = '1' as u8;
    static APPEND_ENTRIES: u8 = '2' as u8;
    match input {
        Ok(REQUEST_VOTE) => {
            return read_request_vote(stream);
        }
        Ok(APPEND_ENTRIES) => {
            return read_append_entries(stream);
        }
        Ok(_) => {
            return Err(IoError {
                kind: InvalidInput,
                desc: "Unknown command",
                detail: None
            })
        }
        Err(e) => {
            return Err(e)
        }
    }
}


fn handle_rpc_command(cmd: RpcRequest) {
    match cmd {
    }
}
