use super::_bencode::decode_bencode;
use super::string::decode_string;
use std::collections::BTreeMap;

use crate::enums::bencode::BencodeValue;

pub fn decode_dict(mut data: &str) -> Result<(BTreeMap<String, BencodeValue>, &str), &'static str> {
    if !data.starts_with('d') {
        return Err("Not a dictionary");
    }
    data = &data[1..];
    let mut map = BTreeMap::new();
    while !data.starts_with('e') {
        let (key, rest) = decode_string(data)?;
        let (value, rest2) = decode_bencode(rest)?;
        map.insert(key, value);
        data = rest2;
    }
    Ok((map, &data[1..]))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::bencode::BencodeValue;

    #[test]
    fn test_decode_simple_dict() {
        let data = "d3:foo3:bare";
        let (result, rest) = decode_dict(data).unwrap();

        assert!(rest.is_empty());
        assert_eq!(result.len(), 1);
        assert_eq!(
            result.get("foo"),
            Some(&BencodeValue::Str("bar".to_string()))
        );
    }

    #[test]
    fn test_decode_dict_with_integer() {
        let data = "d3:agei25ee";
        let (result, rest) = decode_dict(data).unwrap();

        assert!(rest.is_empty());
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("age"), Some(&BencodeValue::Int(25)));
    }

    #[test]
    fn test_decode_nested_dict() {
        let data = "d4:info d3:foo3:baree4:listli1ei2ee";
        let (result, rest) = decode_dict(data).unwrap();

        assert!(rest.is_empty());
        assert!(matches!(result.get("info"), Some(BencodeValue::Dict(_))));
        assert!(matches!(result.get("list"), Some(BencodeValue::List(_))));
    }

    #[test]
    fn test_decode_empty_dict() {
        let data = "de";
        let (result, rest) = decode_dict(data).unwrap();

        assert!(rest.is_empty());
        assert!(result.is_empty());
    }

    #[test]
    fn test_decode_invalid_dict() {
        let data = "l3:foo3:bare"; // Not a dictionary
        let result = decode_dict(data);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Not a dictionary");
    }
}
