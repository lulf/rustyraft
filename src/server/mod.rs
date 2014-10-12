use std::io::{Listener, Acceptor, IoResult, IoError, IoErrorKind, InvalidInput};
use std::io::net::tcp::{TcpListener, TcpStream};
use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io;
use std::str;
use std::ascii;
use self::protocol::{RequestVoteRequest, AppendEntriesRequest, RequestVoteResponse, AppendEntriesResponse, RpcResponse, RpcRequest};
use self::log::{Log, MemoryLog};

mod protocol;
mod log;

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
    current_term: uint,
    voted_for: Option<uint>,
    log: Box<Log+'static>,

    commit_index: uint,
    last_applied: uint,

    next_index: Vec<uint>,
    match_index: Vec<uint>,
    state: State,

    me: ServerSpec,
    peers: Vec<ServerSpec>
}

impl ServerState {
    fn initial(me:ServerSpec, peers:Vec<ServerSpec>) -> ServerState {
        let mut ret = ServerState {
            current_term: 0,
            voted_for: None,
            log: box MemoryLog::new(),
            commit_index: 0,
            last_applied: 0,
            next_index: Vec::new(),
            match_index: Vec::new(),
            state: Follower,
            me: me,
            peers: peers
        };
        ret
    }

    fn request_vote(&self, request: RequestVoteRequest) -> RequestVoteResponse {
        RequestVoteResponse {
            term: 0,
            vote_granted: false
        }
    }
    fn append_entries(&self, request: AppendEntriesRequest) -> AppendEntriesResponse {
        AppendEntriesResponse {
            term: 0,
            success: false
        }
    }
}

pub fn start_server(server_id:uint, servers:&Vec<ServerSpec>) {
    let mut others = servers.clone();
    let my_spec = others.remove(server_id).unwrap();
    spawn(proc() {
        let mut state = ServerState::initial(my_spec, others);
        run_raft_server(&mut state);
    });
}

fn run_raft_server(mut state: &ServerState) {
    let me = &state.me;
    let mut acceptor = TcpListener::bind("127.0.0.1", me.port).listen();
    match acceptor {
        Err(e) => { println!("Error listening to {} ", me.port) }
        Ok(_) => {
            for stream in acceptor.incoming() {
                match stream {
                    Err(e) => { println!("Error handling client connection!"); }
                    Ok(mut tcp_stream) => {
                        let status = handle_client(&mut tcp_stream, state);
                        match status {
                            Ok(_) => {}
                            Err(msg) => { println!("Encountered error: {}", msg); }
                        }
                    }
                }
            }
            drop(acceptor);
        }
    }
    //    stream.write(bytes!("Hello World\r\n"));
    //}
}

fn handle_client(stream: &mut TcpStream, mut state: &ServerState) -> IoResult<()> {
    let input: u8 = try!(stream.read_byte());
    const REQUEST_VOTE: u8 = '1' as u8;
    const APPEND_ENTRIES: u8 = '2' as u8;
    match input {
        REQUEST_VOTE => {
            let request: RequestVoteRequest = try!(RpcRequest::decode(stream));
            let response = state.request_vote(request);
        }
        APPEND_ENTRIES => {
            let request: AppendEntriesRequest = try!(RpcRequest::decode(stream));
            let response = state.append_entries(request);
        }
        n => {
            println!("Unknown command {}", n);
        }
    }
    Ok(())
}
