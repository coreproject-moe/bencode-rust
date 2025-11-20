use super::_bencode::encode_bencode;
use super::string::encode_string;
use crate::enums::bencode::BencodeValue;

use std::collections::BTreeMap;

pub fn encode_dict(data: BTreeMap<String, BencodeValue>) -> Result<String, &'static str> {
    let mut encoded = String::from("d");

    for (key, value) in data.into_iter() {
        let item_str = encode_bencode(value)?;
        let encoded_key = encode_string(&key)?;
        encoded.push_str(&format!("{}{}", encoded_key, item_str));
    }

    encoded.push('e');
    Ok(encoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::bencode::BencodeValue;
    use std::collections::BTreeMap;

    #[test]
    fn test_encode_empty_dict() {
        let dict: BTreeMap<String, BencodeValue> = BTreeMap::new();
        let encoded = encode_dict(dict).unwrap();
        assert_eq!(encoded, "de");
    }

    #[test]
    fn test_encode_dict_with_int() {
        let mut dict: BTreeMap<String, BencodeValue> = BTreeMap::new();
        dict.insert("wiki".to_string(), BencodeValue::Str("bencode".to_string()));
        dict.insert("meaning".to_string(), BencodeValue::Int(42));
        let encoded = encode_dict(dict).unwrap();
        assert_eq!(encoded, "d7:meaningi42e4:wiki7:bencodee")
    }

    #[test]
    fn test_encode_dict_with_string() {
        let mut dict = BTreeMap::new();
        dict.insert("name".to_string(), BencodeValue::Str("Alice".to_string()));
        let encoded = encode_dict(dict).unwrap();
        assert_eq!(encoded, "d4:name5:Alicee");
    }

    #[test]
    fn test_encode_dict_with_list() {
        let mut dict = BTreeMap::new();
        let list = vec![BencodeValue::Int(1), BencodeValue::Str("two".to_string())];
        dict.insert("numbers".to_string(), BencodeValue::List(list));
        let encoded = encode_dict(dict).unwrap();
        assert_eq!(encoded, "d7:numbersli1e3:twoee");
    }

    #[test]
    fn test_nested_dict() {
        let mut dict: BTreeMap<String, BencodeValue> = BTreeMap::new();
        let mut nested_dict: BTreeMap<String, BencodeValue> = BTreeMap::new();
        nested_dict.insert("hello".to_string(), BencodeValue::Str("world".to_string()));
        dict.insert("hello".to_string(), BencodeValue::Dict(nested_dict));
        let encoded = encode_dict(dict).unwrap();
        assert_eq!(encoded, "d5:hellod5:hello5:worldee")
    }
}
