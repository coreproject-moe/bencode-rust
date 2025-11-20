use crate::enums::list::BencodeValue;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum DictionaryValue {
    Int(i64),
    Str(String),
    List(Vec<BencodeValue>),
    Dict(BTreeMap<String, DictionaryValue>),
}
