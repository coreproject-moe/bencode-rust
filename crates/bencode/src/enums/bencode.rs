use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub enum BencodeValue {
    Int(i64),
    Str(Vec<u8>),
    List(Vec<BencodeValue>),
    Dict(BTreeMap<String, BencodeValue>),
}
