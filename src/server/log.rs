use std::io::{IoResult, IoError, IoErrorKind, InvalidInput};
use std::collections::TreeMap;

//pub trait Operation {
//    fn apply() -> IoResult<()>;
//    fn unapply() -> IoResult<()>;
//}

#[deriving(Clone,Show)]
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
    fn write(&mut self, term: uint, operation: Operation) -> IoResult<uint>;
    fn read(&self, term: uint, index: uint) -> IoResult<Operation>;
    fn last_log_index(&self) -> IoResult<uint>;
    fn last_log_term(&self) -> IoResult<uint>;
}

pub struct MemoryLog {
    entries: Vec<LogEntry>
}

impl MemoryLog {
    pub fn new() -> MemoryLog {
        MemoryLog {
            entries: Vec::new()
        }
    }
}

struct LogEntry {
    term: uint,
    entry: Operation
}

impl Log for MemoryLog {
    fn write(&mut self, term: uint, operation: Operation) -> IoResult<uint> {
        self.entries.push(LogEntry{term: term, entry: operation});
        Ok(self.entries.len())
    }
    fn read(&self, term: uint, index: uint) -> IoResult<Operation> {
        // TODO: What should we do with term?
        assert!(index > 0);
        let entry = &self.entries[index - 1];
        Ok(entry.entry.clone())
    }
    fn last_log_index(&self) -> IoResult<uint> {
        Ok(self.entries.len())
    }
    fn last_log_term(&self) -> IoResult<uint> {
        Err(IoError{ kind: InvalidInput, desc: "Not yet implemented", detail: None})
    }
}

#[test]
fn test_that_put_operation_is_written_to_memory_log() {
    let mut log = MemoryLog::new();
    let data = vec![1, 2, 3];
    let result = log.write(1, Put(3, data));
    assert!(result.is_ok());
    let last_index = log.last_log_index();
    assert!(last_index.is_ok());
    let output = log.read(1, last_index.unwrap());
    assert!(output.is_ok());
    let put = output.unwrap();
    assert_eq!(3, put.get_key());
}
