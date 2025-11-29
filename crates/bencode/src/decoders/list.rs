use super::bencode::decode_bencode;
use crate::enums::bencode::BencodeValue;

pub fn decode_list(data: &[u8]) -> Result<(Vec<BencodeValue>, &[u8]), &'static str> {
    if data.is_empty() || data[0] != b'l' {
        return Err("Not a list");
    }

    let mut rest = &data[1..]; // Skip leading 'l'
    let mut items = Vec::new();

    while !rest.is_empty() && rest[0] != b'e' {
        let (value, new_rest) = decode_bencode(rest)?;
        items.push(value);
        rest = new_rest;
    }

    if rest.is_empty() {
        return Err("Missing 'e' to terminate list");
    }

    // Skip the terminating 'e'
    Ok((items, &rest[1..]))
}
