use super::dictionary::encode_dict;
use super::integer::encode_integer;
use super::list::encode_list;
use super::string::encode_string;
use crate::enums::bencode::BencodeValue;

pub fn encode_bencode(value: BencodeValue) -> Result<String, &'static str> {
    match value {
        BencodeValue::Int(n) => encode_integer(n),
        BencodeValue::Str(s) => encode_string(&s),
        BencodeValue::List(l) => encode_list(l),
        BencodeValue::Dict(d) => encode_dict(d),
    }
}
