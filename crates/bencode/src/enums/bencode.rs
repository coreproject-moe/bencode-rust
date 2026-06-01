use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BencodeValue {
    Int(i128),
    Str(Vec<u8>),
    List(Vec<BencodeValue>),
    Dict(BTreeMap<Vec<u8>, BencodeValue>),
}
