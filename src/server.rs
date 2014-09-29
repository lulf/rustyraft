use std::io::{Listener, Acceptor, IoResult, IoError, IoErrorKind, InvalidInput};
use std::io::net::tcp::{TcpListener, TcpStream};
use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io;
use std::str;
use std::ascii;

trait RpcRequest {
    fn handle(&self, mut state: &ServerState) -> Box<RpcResponse>;
    fn decode(mut stream: &TcpStream) -> IoResult<Self>;
}

pub struct RequestVoteRequest {
    term: uint,
    candidateId: uint,
    lastLogIndex: uint,
    lastLogTerm: uint
}

pub struct AppendEntriesRequest {
    term: uint,
    leaderId: uint,
    prevLogIndex: uint,
    prevLogTerm: uint,
    entries: Vec<Vec<u8>>,
    leaderCommit: uint
}

trait RpcResponse {
    fn encode(&self, mut stream: &TcpStream) -> IoResult<()>;
    fn decode(mut stream: &TcpStream) -> IoResult<Self>;
}

pub struct RequestVoteResponse {
    term: uint,
    voteGranted: bool
}

pub struct AppendEntriesResponse {
    term: uint,
    success: bool
}

impl RpcResponse for RequestVoteResponse {
    fn encode(&self, mut stream: &TcpStream) -> IoResult<()> {
        try!(stream.write_le_uint(self.term));
        if (self.voteGranted) {
            try!(stream.write([1]));
        } else {
            try!(stream.write([0]));
        }
        Ok(())
    }
    fn decode(mut stream: &TcpStream) -> IoResult<RequestVoteResponse> {
        let term = try!(stream.read_le_uint());
        let b = try!(stream.read_byte());
        Ok(RequestVoteResponse {
            term: term,
            voteGranted: (b == 1)})
    }
}

impl RpcResponse for AppendEntriesResponse {
    fn encode(&self, mut stream: &TcpStream) -> IoResult<()> {
        try!(stream.write_le_uint(self.term));
        if (self.success) {
            try!(stream.write([1]));
        } else {
            try!(stream.write([0]));
        }
        Ok(())
    }
    fn decode(mut stream: &TcpStream) -> IoResult<AppendEntriesResponse> {
        let term = try!(stream.read_le_uint());
        let b = try!(stream.read_byte());
        Ok(AppendEntriesResponse {
            term: term,
            success: (b == 1)
        })
    }
}

impl RpcRequest for RequestVoteRequest {
    fn handle(&self, mut state: &ServerState) -> Box<RpcResponse> {
        let ret = RequestVoteResponse {
            term: 0,
            voteGranted: false
        };
        box ret as Box<RpcResponse>
    }
    fn decode(mut stream: TcpStream) -> IoResult<RequestVoteRequest> {
        let ret =  RequestVoteRequest {
            term: try!(stream.read_le_uint()),
            candidateId: try!(stream.read_le_uint()),
            lastLogIndex: try!(stream.read_le_uint()),
            lastLogTerm: try!(stream.read_le_uint())
        };
        return Ok(ret);
    }
}

impl RpcRequest for AppendEntriesRequest {
    fn handle(&self, mut state: &ServerState) -> Box<RpcResponse> {
        let ret = AppendEntriesResponse {
            term: 0,
            success: false
        };
        box ret as Box<RpcResponse>
    }
    fn decode(mut stream: TcpStream) -> IoResult<AppendEntriesRequest> {
        let ret = AppendEntriesRequest {
            term: try!(stream.read_le_uint()),
            leaderId: try!(stream.read_le_uint()),
            prevLogIndex: try!(stream.read_le_uint()),
            prevLogTerm: try!(stream.read_le_uint()),
            entries: {
                let vec_length = try!(stream.read_le_uint());
                let mut entries:Vec<Vec<u8>> = Vec::new();
                // Perhaps this can be done simpler...
                for entry in range(0, vec_length) {
                    let entry_length = try!(stream.read_le_uint());
                    let mut entry:Vec<u8> = Vec::new();
                    for c in range(0, entry_length) {
                        entry.push(try!(stream.read_byte()));
                    }
                    entries.push(entry)
                }
                entries
            },
            leaderCommit: try!(stream.read_le_uint())
        };
        return Ok(ret);
    }
}

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
    match cmd {
    	Ok(command) => {
		command.handle(state);
	}
	Err(e) => {
		println!("Error reading rpc command: {}", e);
	}
    }
    //println!("Command {}", cmd);
}

fn read_rpc_command(mut stream: TcpStream) -> IoResult<Box<RpcRequest+'static>> {
    let input = stream.read_byte();
    static REQUEST_VOTE: u8 = '1' as u8;
    static APPEND_ENTRIES: u8 = '2' as u8;
    match input {
        Ok(REQUEST_VOTE) => {
            let request: RequestVoteRequest = try!(RpcRequest::decode(stream));
            return Ok(box request as Box<RpcRequest>);
        }
        Ok(APPEND_ENTRIES) => {
            let request: IoResult<AppendEntriesRequest> = RpcRequest::decode(stream);
            return request.map(|ret| { box ret as Box<RpcRequest> });
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
