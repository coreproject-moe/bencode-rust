use crate::encoders::integer::encode_integer;
use crate::encoders::list::encode_list;
use crate::encoders::string::encode_string;
use crate::enums::dictionary::DictionaryValue;

use std::collections::BTreeMap;

pub fn encode_dict(data: BTreeMap<String, DictionaryValue>) -> Result<String, &'static str> {
    let mut encoded = String::from("d");

    for (key, value) in data.into_iter() {
        let item_str = match value {
            DictionaryValue::Int(n) => encode_integer(n)?,
            DictionaryValue::Str(s) => encode_string(&s)?,
            DictionaryValue::List(l) => encode_list(l)?,
            DictionaryValue::Dict(d) => encode_dict(d)?,
        };
        let encoded_key = encode_string(&key)?;
        encoded.push_str(&format!("{}{}", encoded_key, item_str));
    }

    encoded.push('e');
    Ok(encoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::dictionary::DictionaryValue;
    use crate::enums::list::BencodeValue;
    use std::collections::BTreeMap;

    #[test]
    fn test_encode_empty_dict() {
        let dict: BTreeMap<String, DictionaryValue> = BTreeMap::new();
        let encoded = encode_dict(dict).unwrap();
        assert_eq!(encoded, "de");
    }

    #[test]
    fn test_encode_dict_with_int() {
        let mut dict: BTreeMap<String, DictionaryValue> = BTreeMap::new();
        dict.insert(
            "wiki".to_string(),
            DictionaryValue::Str("bencode".to_string()),
        );
        dict.insert("meaning".to_string(), DictionaryValue::Int(42));
        let encoded = encode_dict(dict).unwrap();
        assert_eq!(encoded, "d7:meaningi42e4:wiki7:bencodee")
    }

    #[test]
    fn test_encode_dict_with_string() {
        let mut dict = BTreeMap::new();
        dict.insert(
            "name".to_string(),
            DictionaryValue::Str("Alice".to_string()),
        );
        let encoded = encode_dict(dict).unwrap();
        assert_eq!(encoded, "d4:name5:Alicee");
    }

    #[test]
    fn test_encode_dict_with_list() {
        let mut dict = BTreeMap::new();
        let list = vec![BencodeValue::Int(1), BencodeValue::Str("two".to_string())];
        dict.insert("numbers".to_string(), DictionaryValue::List(list));
        let encoded = encode_dict(dict).unwrap();
        assert_eq!(encoded, "d7:numbersli1e3:twoee");
    }

    #[test]
    fn test_nested_dict() {
        let mut dict: BTreeMap<String, DictionaryValue> = BTreeMap::new();
        let mut nested_dict: BTreeMap<String, DictionaryValue> = BTreeMap::new();
        nested_dict.insert(
            "hello".to_string(),
            DictionaryValue::Str("world".to_string()),
        );
        dict.insert("hello".to_string(), DictionaryValue::Dict(nested_dict));
        let encoded = encode_dict(dict).unwrap();
        assert_eq!(encoded, "d5:hellod5:hello5:worldee")
    }
}
