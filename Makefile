all:
	rustc raft_server.rs
	protoc --rust_out . request_vote_request.proto
#	protoc --rust_out . request_vote_response.proto
#	rustc -L /Users/lulf/dev/rust-protobuf/src server.rs
