use super::dictionary::decode_dict;
use super::integer::decode_integer;
use super::list::decode_list;
use super::string::decode_string;

use crate::enums::bencode::BencodeValue;

pub fn decode_bencode(data: &str) -> Result<(BencodeValue, &str), &'static str> {
    let first = data.chars().next().ok_or("Empty input")?;
    match first {
        'i' => {
            let (val, rest) = decode_integer(data)?;
            Ok((BencodeValue::Int(val), rest))
        }
        'l' => {
            let (val, rest) = decode_list(data)?;
            Ok((BencodeValue::List(val), rest))
        }
        'd' => {
            let (val, rest) = decode_dict(data)?;
            Ok((BencodeValue::Dict(val), rest))
        }
        '0'..='9' => {
            let (val, rest) = decode_string(data)?;
            Ok((BencodeValue::Str(val.into_bytes()), rest))
        }
        _ => Err("Invalid bencode format"),
    }
}
