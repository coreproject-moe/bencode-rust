use super::_bencode::encode_bencode;
use crate::enums::bencode::BencodeValue;

pub fn encode_list(data: Vec<BencodeValue>) -> Result<String, &'static str> {
    let mut encoded = String::from("l");
    for item in data {
        // Encode each item based on its type.
        let item_str = encode_bencode(item)?;
        encoded.push_str(&item_str);
    }
    encoded.push('e');

    Ok(encoded)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_simple_list() {
        let list = vec![
            BencodeValue::Int(42),
            BencodeValue::Str("hello".to_string().into_bytes()),
        ];

        let encoded = encode_list(list).unwrap();
        assert_eq!(encoded, "li42e5:helloe");
    }

    #[test]
    fn test_encode_nested_list() {
        let nested_list = vec![
            BencodeValue::Int(1),
            BencodeValue::List(vec![
                BencodeValue::Int(2),
                BencodeValue::Str("nested".to_string().into_bytes()),
            ]),
            BencodeValue::Str("end".to_string().into_bytes()),
        ];

        let encoded = encode_list(nested_list).unwrap();
        assert_eq!(encoded, "li1eli2e6:nestede3:ende");
    }

    #[test]
    fn test_empty_list() {
        let empty: Vec<BencodeValue> = vec![];
        let encoded = encode_list(empty).unwrap();
        assert_eq!(encoded, "le");
    }

    #[test]
    fn test_list_with_only_strings() {
        let list = vec![
            BencodeValue::Str("bencode".to_string().into_bytes()),
            BencodeValue::Int(-20),
        ];
        let encoded = encode_list(list).unwrap();
        assert_eq!(encoded, "l7:bencodei-20ee");
    }
}
