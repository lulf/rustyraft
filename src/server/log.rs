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
    fn write(&mut self, entry: LogEntry) -> IoResult<uint>;
    fn read(&self, index: uint) -> IoResult<LogEntry>;
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

#[deriving(Clone,Show)]
struct LogEntry {
    term: uint,
    operation: Operation
}

impl LogEntry {
    fn new(term: uint, operation: Operation) -> LogEntry {
        LogEntry {
            term: term,
            operation: operation
        }
    }
}

impl Log for MemoryLog {
    fn write(&mut self, entry: LogEntry) -> IoResult<uint> {
        self.entries.push(entry);
        Ok(self.entries.len())
    }
    fn read(&self, index: uint) -> IoResult<LogEntry> {
        // TODO: What should we do with term?
        assert!(index > 0);
        let entry = &self.entries[index - 1];
        Ok(entry.clone())
    }
    fn last_log_index(&self) -> IoResult<uint> {
        Ok(self.entries.len())
    }
    fn last_log_term(&self) -> IoResult<uint> {
        let idx = try!(self.last_log_index());
        let entry = try!(self.read(idx));
        Ok(entry.term)
    }
}

#[test]
fn test_that_put_operation_is_written_to_memory_log() {
    let mut log = MemoryLog::new();
    let data = vec![1, 2, 3];
    let result = log.write(LogEntry::new(1, Put(3, data)));
    assert!(result.is_ok());
    let last_index = log.last_log_index();
    assert!(last_index.is_ok());
    let output = log.read(last_index.unwrap());
    assert!(output.is_ok());
    let entry = output.unwrap();
    assert_eq!(3, entry.operation.get_key());
    assert_eq!(1, entry.term);
    let term = log.last_log_term();
    assert!(term.is_ok());
    assert_eq!(1, term.unwrap());
}
