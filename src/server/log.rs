use std::io::{IoResult, IoError, IoErrorKind, InvalidInput};

//pub trait Operation {
//    fn apply() -> IoResult<()>;
//    fn unapply() -> IoResult<()>;
//}

pub enum Operation {
    Put(uint, Vec<u8>),
    Delete(uint)
}

impl Operation {
    fn get_key(&self) -> uint {
        match *self {
            Put(key, _) => key,
            Delete(key) => key
        }
    }
}


pub trait Log {
    fn write(&self, term: uint, operation: Operation) -> IoResult<uint>;
    fn read(&self, term: uint, index: uint) -> IoResult<Operation>;
    fn last_index(&self) -> IoResult<uint>;
    fn log_term(&self) -> IoResult<uint>;
}

pub struct MemoryLog;

impl MemoryLog {
    pub fn new() -> MemoryLog {
        MemoryLog
    }
}

impl Log for MemoryLog {
    fn write(&self, term: uint, operation: Operation) -> IoResult<uint> {
        Err(IoError{ kind: InvalidInput, desc: "Not yet implemented", detail: None})
    }
    fn read(&self, term: uint, index: uint) -> IoResult<Operation> {
        Err(IoError{ kind: InvalidInput, desc: "Not yet implemented", detail: None})
    }
    fn last_index(&self) -> IoResult<uint> {
        Err(IoError{ kind: InvalidInput, desc: "Not yet implemented", detail: None})
    }
    fn log_term(&self) -> IoResult<uint> {
        Err(IoError{ kind: InvalidInput, desc: "Not yet implemented", detail: None})
    }
}

#[test]
fn test_that_operation_is_written() {
    let log = MemoryLog::new();
    let data = vec![1, 2, 3];
    let result = log.write(1, Put(3, data));
    assert!(result.is_ok());
    let last_index = log.last_index();
    assert!(last_index.is_ok());
    let output = log.read(1, last_index.unwrap());
    assert!(output.is_ok());
    let put = output.unwrap();
    assert_eq!(3, put.get_key());
}
