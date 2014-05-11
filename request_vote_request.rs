// This file is generated. Do not edit

use protobuf::*;
use protobuf::rt;
use protobuf::descriptor;

static file_descriptor_proto_data: &'static [u8] = &[0x0a, 0x1a, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x76, 0x6f, 0x74, 0x65, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x62, 0x0a, 0x12, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x56, 0x6f, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x13, 0x0a, 0x0b, 0x63, 0x61, 0x6e, 0x64, 0x69, 0x64, 0x61, 0x74, 0x65, 0x49, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x14, 0x0a, 0x0c, 0x6c, 0x61, 0x73, 0x74, 0x4c, 0x6f, 0x67, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x02, 0x28, 0x04, 0x12, 0x13, 0x0a, 0x0b, 0x6c, 0x61, 0x73, 0x74, 0x4c, 0x6f, 0x67, 0x54, 0x65, 0x72, 0x6d, 0x18, 0x04, 0x20, 0x02, 0x28, 0x04, 0x4a, 0xb4, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x00, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x00, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x01, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x01, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x01, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x01, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x01, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x02, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x02, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x02, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x02, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x02, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x03, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x03, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x03, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x03, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x03, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x04, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x04, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x04, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x04, 0x22, 0x23];

pub fn file_descriptor_proto() -> descriptor::FileDescriptorProto {
    parse_from_bytes(file_descriptor_proto_data)
}

#[deriving(Clone,Eq)]
pub struct RequestVoteRequest {
    term: Option<~[u8]>,
    candidateId: Option<u64>,
    lastLogIndex: Option<u64>,
    lastLogTerm: Option<u64>,
}

impl<'a> RequestVoteRequest {
    pub fn new() -> RequestVoteRequest {
        RequestVoteRequest {
            term: None,
            candidateId: None,
            lastLogIndex: None,
            lastLogTerm: None,
        }
    }

    pub fn default_instance() -> &'static RequestVoteRequest {
        static instance: RequestVoteRequest = RequestVoteRequest {
            term: None,
            candidateId: None,
            lastLogIndex: None,
            lastLogTerm: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.term {
            Some(ref v) => {
                os.write_bytes(1, *v);
            },
            None => {},
        };
        match self.candidateId {
            Some(ref v) => {
                os.write_uint64(2, *v);
            },
            None => {},
        };
        match self.lastLogIndex {
            Some(ref v) => {
                os.write_uint64(3, *v);
            },
            None => {},
        };
        match self.lastLogTerm {
            Some(ref v) => {
                os.write_uint64(4, *v);
            },
            None => {},
        };
    }

    pub fn clear_term(&mut self) {
        self.term = None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: ~[u8]) {
        self.term = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_term(&'a mut self) -> &'a mut ~[u8] {
        if self.term.is_none() {
            self.term = Some(~[]);
        };
        self.term.get_mut_ref()
    }

    pub fn get_term(&'a self) -> &'a [u8] {
        match self.term {
            Some(ref v) => rt::as_slice_tmp(v),
            None => &'a [],
        }
    }

    pub fn clear_candidateId(&mut self) {
        self.candidateId = None;
    }

    pub fn has_candidateId(&self) -> bool {
        self.candidateId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_candidateId(&mut self, v: u64) {
        self.candidateId = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_candidateId(&'a mut self) -> &'a mut u64 {
        if self.candidateId.is_none() {
            self.candidateId = Some(0);
        };
        self.candidateId.get_mut_ref()
    }

    pub fn get_candidateId(&self) -> u64 {
        self.candidateId.unwrap_or_else(|| 0)
    }

    pub fn clear_lastLogIndex(&mut self) {
        self.lastLogIndex = None;
    }

    pub fn has_lastLogIndex(&self) -> bool {
        self.lastLogIndex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastLogIndex(&mut self, v: u64) {
        self.lastLogIndex = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lastLogIndex(&'a mut self) -> &'a mut u64 {
        if self.lastLogIndex.is_none() {
            self.lastLogIndex = Some(0);
        };
        self.lastLogIndex.get_mut_ref()
    }

    pub fn get_lastLogIndex(&self) -> u64 {
        self.lastLogIndex.unwrap_or_else(|| 0)
    }

    pub fn clear_lastLogTerm(&mut self) {
        self.lastLogTerm = None;
    }

    pub fn has_lastLogTerm(&self) -> bool {
        self.lastLogTerm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastLogTerm(&mut self, v: u64) {
        self.lastLogTerm = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lastLogTerm(&'a mut self) -> &'a mut u64 {
        if self.lastLogTerm.is_none() {
            self.lastLogTerm = Some(0);
        };
        self.lastLogTerm.get_mut_ref()
    }

    pub fn get_lastLogTerm(&self) -> u64 {
        self.lastLogTerm.unwrap_or_else(|| 0)
    }
}

impl Message for RequestVoteRequest {
    fn new() -> RequestVoteRequest {
        RequestVoteRequest::new()
    }

    fn clear(&mut self) {
        self.clear_term();
        self.clear_candidateId();
        self.clear_lastLogIndex();
        self.clear_lastLogTerm();
    }

    fn is_initialized(&self) -> bool {
        if self.term.is_none() {
            return false;
        };
        if self.candidateId.is_none() {
            return false;
        };
        if self.lastLogIndex.is_none() {
            return false;
        };
        if self.lastLogTerm.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_bytes();
                    self.term = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.candidateId = Some(tmp);
                },
                3 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.lastLogIndex = Some(tmp);
                },
                4 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.lastLogTerm = Some(tmp);
                },
                _ => {
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.term.iter() {
            my_size += rt::bytes_size(1, *value);
        };
        for value in self.candidateId.iter() {
            my_size += rt::value_size(2, *value, wire_format::WireTypeVarint);
        };
        for value in self.lastLogIndex.iter() {
            my_size += rt::value_size(3, *value, wire_format::WireTypeVarint);
        };
        for value in self.lastLogTerm.iter() {
            my_size += rt::value_size(4, *value, wire_format::WireTypeVarint);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}
