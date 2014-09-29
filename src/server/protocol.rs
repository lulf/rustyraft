use std::io::net::tcp::{TcpStream};
use std::io::{IoResult};

pub struct RequestVoteRequest {
    pub term: uint,
    pub candidateId: uint,
    pub lastLogIndex: uint,
    pub lastLogTerm: uint
}

pub struct AppendEntriesRequest {
    pub term: uint,
    pub leaderId: uint,
    pub prevLogIndex: uint,
    pub prevLogTerm: uint,
    pub entries: Vec<Vec<u8>>,
    pub leaderCommit: uint
}

pub struct RequestVoteResponse {
    pub term: uint,
    pub voteGranted: bool
}

pub struct AppendEntriesResponse {
    pub term: uint,
    pub success: bool
}

pub trait RpcRequest {
    fn decode(stream: &mut TcpStream) -> IoResult<Self>;
}

impl RpcRequest for RequestVoteRequest {
    fn decode(stream: &mut TcpStream) -> IoResult<RequestVoteRequest> {
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

    fn decode(stream: &mut TcpStream) -> IoResult<AppendEntriesRequest> {
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

pub trait RpcResponse {
    fn encode(&self, stream: &mut TcpStream) -> IoResult<()>;
    fn decode(stream: &mut TcpStream) -> IoResult<Self>;
}

impl RpcResponse for RequestVoteResponse {
    fn encode(&self, stream: &mut TcpStream) -> IoResult<()> {
        try!(stream.write_le_uint(self.term));
        if (self.voteGranted) {
            try!(stream.write([1]));
        } else {
            try!(stream.write([0]));
        }
        Ok(())
    }
    fn decode(stream: &mut TcpStream) -> IoResult<RequestVoteResponse> {
        let term = try!(stream.read_le_uint());
        let b = try!(stream.read_byte());
        Ok(RequestVoteResponse {
            term: term,
            voteGranted: (b == 1)})
    }
}

impl RpcResponse for AppendEntriesResponse {
    fn encode(&self, stream: &mut TcpStream) -> IoResult<()> {
        try!(stream.write_le_uint(self.term));
        if (self.success) {
            try!(stream.write([1]));
        } else {
            try!(stream.write([0]));
        }
        Ok(())
    }
    fn decode(stream: &mut TcpStream) -> IoResult<AppendEntriesResponse> {
        let term = try!(stream.read_le_uint());
        let b = try!(stream.read_byte());
        Ok(AppendEntriesResponse {
            term: term,
            success: (b == 1)
        })
    }
}

