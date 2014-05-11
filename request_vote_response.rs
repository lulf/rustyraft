// This file is generated. Do not edit

use protobuf::*;
use protobuf::rt;
use protobuf::descriptor;

static file_descriptor_proto_data: &'static [u8] = &[0x0a, 0x1b, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x76, 0x6f, 0x74, 0x65, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x38, 0x0a, 0x13, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x56, 0x6f, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x13, 0x0a, 0x0b, 0x76, 0x6f, 0x74, 0x65, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x08, 0x4a, 0xaa, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x03, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x00, 0x00, 0x03, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x00, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x01, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x01, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x01, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x01, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x01, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x02, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x02, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x02, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x02, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x02, 0x20, 0x21];

pub fn file_descriptor_proto() -> descriptor::FileDescriptorProto {
    parse_from_bytes(file_descriptor_proto_data)
}

#[deriving(Clone,Eq)]
pub struct RequestVoteResponse {
    term: Option<u64>,
    voteGranted: Option<bool>,
}

impl<'a> RequestVoteResponse {
    pub fn new() -> RequestVoteResponse {
        RequestVoteResponse {
            term: None,
            voteGranted: None,
        }
    }

    pub fn default_instance() -> &'static RequestVoteResponse {
        static instance: RequestVoteResponse = RequestVoteResponse {
            term: None,
            voteGranted: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.term {
            Some(ref v) => {
                os.write_uint64(1, *v);
            },
            None => {},
        };
        match self.voteGranted {
            Some(ref v) => {
                os.write_bool(2, *v);
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
    pub fn set_term(&mut self, v: u64) {
        self.term = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_term(&'a mut self) -> &'a mut u64 {
        if self.term.is_none() {
            self.term = Some(0);
        };
        self.term.get_mut_ref()
    }

    pub fn get_term(&self) -> u64 {
        self.term.unwrap_or_else(|| 0)
    }

    pub fn clear_voteGranted(&mut self) {
        self.voteGranted = None;
    }

    pub fn has_voteGranted(&self) -> bool {
        self.voteGranted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_voteGranted(&mut self, v: bool) {
        self.voteGranted = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_voteGranted(&'a mut self) -> &'a mut bool {
        if self.voteGranted.is_none() {
            self.voteGranted = Some(false);
        };
        self.voteGranted.get_mut_ref()
    }

    pub fn get_voteGranted(&self) -> bool {
        self.voteGranted.unwrap_or_else(|| false)
    }
}

impl Message for RequestVoteResponse {
    fn new() -> RequestVoteResponse {
        RequestVoteResponse::new()
    }

    fn clear(&mut self) {
        self.clear_term();
        self.clear_voteGranted();
    }

    fn is_initialized(&self) -> bool {
        if self.term.is_none() {
            return false;
        };
        if self.voteGranted.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.term = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.voteGranted = Some(tmp);
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
            my_size += rt::value_size(1, *value, wire_format::WireTypeVarint);
        };
        if self.voteGranted.is_some() {
            my_size += 2;
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
