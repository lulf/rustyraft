use std::io::{IoResult, IoError, IoErrorKind, InvalidInput};
use std::io::net::tcp::{TcpListener, TcpStream};
use std::io;

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

pub fn read_request_vote(mut stream: TcpStream) -> IoResult<RequestVoteRequest> {
    let ret = RequestVoteRequest {
        term: 0,
        candidateId: 0,
        lastLogIndex: 0,
        lastLogTerm: 0
    };
    return Ok(ret);
}

pub fn read_append_entries(mut stream: TcpStream) -> IoResult<AppendEntriesRequest> {
    let ret = AppendEntriesRequest {
        term: 0,
        leaderId: 0,
        prevLogIndex: 0,
        prevLogTerm: 0,
        entries: Vec::new(),
        leaderCommit: 0
    };
    return Ok(ret);
}
