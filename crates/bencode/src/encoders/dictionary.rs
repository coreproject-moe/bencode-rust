use super::bencode::encode_bencode;
use super::string::encode_string;
use crate::enums::bencode::BencodeValue;

use std::collections::BTreeMap;

pub fn encode_dict(data: BTreeMap<String, BencodeValue>) -> Result<Vec<u8>, &'static str> {
    let mut encoded = Vec::new();
    encoded.push(b'd');

    for (key, value) in data.into_iter() {
        // Encode key as byte string (bencode requires keys to be byte strings)
        let encoded_key = encode_string(key.into_bytes())?;
        encoded.extend(encoded_key);

        // Encode value based on its type
        let encoded_value = encode_bencode(value)?;
        encoded.extend(encoded_value);
    }

    encoded.push(b'e');
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
        assert_eq!(encoded, b"de".to_vec());
    }

    #[test]
    fn test_encode_dict_with_int() {
        let mut dict: BTreeMap<String, BencodeValue> = BTreeMap::new();
        dict.insert(
            "wiki".to_string(),
            BencodeValue::Str("bencode".as_bytes().to_vec()),
        );
        dict.insert("meaning".to_string(), BencodeValue::Int(42));
        let encoded = encode_dict(dict).unwrap();
        assert_eq!(encoded, b"d4:wiki7:bencode7:meaningi42ee".to_vec());
    }

    #[test]
    fn test_encode_dict_with_string() {
        let mut dict = BTreeMap::new();
        dict.insert(
            "name".to_string(),
            BencodeValue::Str("Alice".as_bytes().to_vec()),
        );
        let encoded = encode_dict(dict).unwrap();
        assert_eq!(encoded, b"d4:name5:Alicee".to_vec());
    }

    #[test]
    fn test_encode_dict_with_list() {
        let mut dict = BTreeMap::new();
        let list = vec![
            BencodeValue::Int(1),
            BencodeValue::Str("two".as_bytes().to_vec()),
        ];
        dict.insert("numbers".to_string(), BencodeValue::List(list));
        let encoded = encode_dict(dict).unwrap();
        assert_eq!(encoded, b"d7:numbersli1e3:twoee".to_vec());
    }

    #[test]
    fn test_nested_dict() {
        let mut dict: BTreeMap<String, BencodeValue> = BTreeMap::new();
        let mut nested_dict: BTreeMap<String, BencodeValue> = BTreeMap::new();
        nested_dict.insert(
            "hello".to_string(),
            BencodeValue::Str("world".as_bytes().to_vec()),
        );
        dict.insert("inner".to_string(), BencodeValue::Dict(nested_dict));
        let encoded = encode_dict(dict).unwrap();
        assert_eq!(encoded, b"d5:innerd5:hello5:worldee".to_vec());
    }
}
