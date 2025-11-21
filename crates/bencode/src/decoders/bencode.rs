use super::integer::decode_integer;
use super::list::decode_list;
use super::string::decode_string;
use crate::enums::bencode::BencodeValue;

pub fn decode_bencode(data: &[u8]) -> Result<(BencodeValue, &[u8]), &'static str> {
    if data.is_empty() {
        return Err("Empty input");
    }

    match data[0] {
        b if (b'0'..=b'9').contains(&b) => {
            let (bytes, rest) = decode_string(data)?;
            Ok((BencodeValue::Str(bytes), rest))
        }
        b'i' => {
            let (num, rest) = decode_integer(data)?;
            Ok((BencodeValue::Int(num), rest))
        }
        b'l' => {
            let (list, rest) = decode_list(data)?;
            Ok((BencodeValue::List(list), rest))
        }
        _ => Err("Unknown type prefix"),
    }
}
