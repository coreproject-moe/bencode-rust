use super::bencode::decode_bencode;
use crate::enums::bencode::BencodeValue;
use std::collections::BTreeMap;

pub fn decode_dictionary(
    data: &[u8],
) -> Result<(BTreeMap<String, BencodeValue>, &[u8]), &'static str> {
    if data.is_empty() || data[0] != b'd' {
        return Err("Not a dictionary");
    }

    let mut rest = &data[1..]; // skip 'd'
    let mut dict = BTreeMap::new();

    while !rest.is_empty() && rest[0] != b'e' {
        // Decode key
        let (key_value, new_rest) = decode_bencode(rest)?;
        let key_bytes = match key_value {
            BencodeValue::Str(bytes) => bytes,
            _ => return Err("Dictionary key must be a string"),
        };

        // Convert key to String (UTF-8)
        let key = match String::from_utf8(key_bytes) {
            Ok(s) => s,
            Err(_) => return Err("Dictionary key is not valid UTF-8"),
        };

        rest = new_rest;

        // Decode value
        let (value, new_rest) = decode_bencode(rest)?;
        rest = new_rest;

        // Insert into map (optionally check for duplicate keys)
        dict.insert(key, value);
    }

    if rest.is_empty() {
        return Err("Missing 'e' to terminate dictionary");
    }

    // Skip the terminating 'e'
    let rest = &rest[1..];
    Ok((dict, rest))
}
