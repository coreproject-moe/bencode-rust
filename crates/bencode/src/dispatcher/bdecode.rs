use crate::enums::bencode::BencodeValue;
use crate::parser::parse_bencode;

/// Decode bencode data using the cursor-based iterative parser.
pub fn decode_bencode(data: &[u8]) -> Result<(BencodeValue, &[u8]), &'static str> {
    parse_bencode(data)
}
