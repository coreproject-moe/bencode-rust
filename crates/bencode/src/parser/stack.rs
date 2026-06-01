use crate::enums::bencode::BencodeValue;
use std::collections::BTreeMap;

/// A single frame on the parse stack, tracking one open container.
/// Each frame holds the container being built and its current state.
#[derive(Debug)]
pub enum ParseFrame {
    /// Collecting items into a list (terminated by 'e').
    List { items: Vec<BencodeValue> },
    /// Collecting key-value pairs into a dictionary (terminated by 'e').
    Dict {
        entries: BTreeMap<Vec<u8>, BencodeValue>,
        /// Temporarily holds the decoded key before its value arrives.
        pending_key: Option<Vec<u8>>,
    },
}

/// A heap-allocated stack of parse frames.
/// Every container (list/dict) pushes a frame; completing it pops and produces a value.
pub type ParseStack = Vec<ParseFrame>;
