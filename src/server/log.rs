use std::io::{IoResult, IoError, IoErrorKind, InvalidInput};

//pub trait Operation {
//    fn apply() -> IoResult<()>;
//    fn unapply() -> IoResult<()>;
//}

pub enum Operation {
    PUT(uint, Vec<u8>),
    DELETE(uint)
}


pub trait Log {
    fn write(term: uint, operation: Operation) -> IoResult<uint>;
    fn read(term: uint, index: uint) -> IoResult<Operation>;
    fn last_index() -> IoResult<uint>;
    fn log_term() -> IoResult<uint>;
}

pub struct MemoryLog;

impl MemoryLog {
    pub fn new() -> MemoryLog {
        MemoryLog
    }
}

impl Log for MemoryLog {
    fn write(term: uint, operation: Operation) -> IoResult<uint> {
        Err(IoError{ kind: InvalidInput, desc: "Not yet implemented", detail: None})
    }
    fn read(term: uint, index: uint) -> IoResult<Operation> {
        Err(IoError{ kind: InvalidInput, desc: "Not yet implemented", detail: None})
    }
    fn last_index() -> IoResult<uint> {
        Err(IoError{ kind: InvalidInput, desc: "Not yet implemented", detail: None})
    }
    fn log_term() -> IoResult<uint> {
        Err(IoError{ kind: InvalidInput, desc: "Not yet implemented", detail: None})
    }
}


