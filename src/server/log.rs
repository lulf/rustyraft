use std::io::{IoResult};

pub trait Operation {
    fn apply() -> IoResult<()>;
    fn unapply() -> IoResult<()>;
}

pub trait Log {
    fn write(term: uint, operation: Operation) -> IoResult<uint>;
    fn read(term: uint, index: uint) -> IoResult<Box<Operation+'static>>;
    fn last_index() -> IoResult<uint>;
    fn log_term() -> IoResult<uint>;
}
