pub mod dispatcher;
pub mod encoders;
pub mod enums;
pub mod parser;

// Re-export top-level API for convenience
pub use dispatcher::bdecode::decode_bencode;
pub use dispatcher::bencode::encode_bencode;
