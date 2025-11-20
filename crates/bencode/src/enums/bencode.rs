use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub enum BencodeValue {
    Int(i64),
    Str(String),
    List(Vec<BencodeValue>),
    Dict(BTreeMap<String, BencodeValue>),
}
