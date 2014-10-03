use std::io::net::tcp::{TcpStream};
use std::io::{IoResult};

pub struct RequestVoteRequest {
    pub term: uint,
    pub candidate_id: uint,
    pub last_log_index: uint,
    pub last_log_term: uint
}

pub struct AppendEntriesRequest {
    pub term: uint,
    pub leader_id: uint,
    pub prev_log_index: uint,
    pub prev_log_term: uint,
    pub entries: Vec<Vec<u8>>,
    pub leader_commit: uint
}

pub struct RequestVoteResponse {
    pub term: uint,
    pub vote_granted: bool
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
            candidate_id: try!(stream.read_le_uint()),
            last_log_index: try!(stream.read_le_uint()),
            last_log_term: try!(stream.read_le_uint())
        };
        return Ok(ret);
    }
}

impl RpcRequest for AppendEntriesRequest {

    fn decode(stream: &mut TcpStream) -> IoResult<AppendEntriesRequest> {
        let ret = AppendEntriesRequest {
            term: try!(stream.read_le_uint()),
            leader_id: try!(stream.read_le_uint()),
            prev_log_index: try!(stream.read_le_uint()),
            prev_log_term: try!(stream.read_le_uint()),
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
            leader_commit: try!(stream.read_le_uint())
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
        if (self.vote_granted) {
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
            vote_granted: (b == 1)})
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

