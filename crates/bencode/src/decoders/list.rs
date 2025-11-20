use super::_bencode::decode_bencode;
use crate::enums::bencode::BencodeValue;

pub fn decode_list(mut data: &str) -> Result<(Vec<BencodeValue>, &str), &'static str> {
    if !data.starts_with('l') {
        return Err("Not a list");
    }
    data = &data[1..];
    let mut items = Vec::new();
    while !data.starts_with('e') {
        let (item, rest) = decode_bencode(data)?;
        items.push(item);
        data = rest;
    }
    Ok((items, &data[1..]))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::bencode::BencodeValue;

    #[test]
    fn test_decode_empty_list() {
        let data = "le";
        let result = decode_list(data).unwrap();
        assert_eq!(result.0, Vec::<BencodeValue>::new());
        assert_eq!(result.1, "");
    }

    #[test]
    fn test_decode_integer_list() {
        let data = "li1ei2ei3ee";
        let result = decode_list(data).unwrap();
        assert_eq!(
            result.0,
            vec![
                BencodeValue::Int(1),
                BencodeValue::Int(2),
                BencodeValue::Int(3)
            ]
        );
        assert_eq!(result.1, "");
    }

    #[test]
    fn test_decode_string_list() {
        let data = "l3:foo3:bare";
        let result = decode_list(data).unwrap();
        assert_eq!(
            result.0,
            vec![
                BencodeValue::Str("foo".to_string()),
                BencodeValue::Str("bar".to_string())
            ]
        );
        assert_eq!(result.1, "");
    }

    #[test]
    fn test_decode_mixed_list() {
        let data = "li42e3:fooe";
        let result = decode_list(data).unwrap();
        assert_eq!(
            result.0,
            vec![BencodeValue::Int(42), BencodeValue::Str("foo".to_string())]
        );
        assert_eq!(result.1, "");
    }

    #[test]
    fn test_decode_nested_list() {
        let data = "lli1ei2ee3:fooe";
        let result = decode_list(data).unwrap();
        assert_eq!(
            result.0,
            vec![
                BencodeValue::List(vec![BencodeValue::Int(1), BencodeValue::Int(2)]),
                BencodeValue::Str("foo".to_string())
            ]
        );
        assert_eq!(result.1, "");
    }

    #[test]
    fn test_decode_invalid_start() {
        let data = "i123e"; // integer instead of list
        let result = decode_list(data);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Not a list");
    }

    #[test]
    fn test_decode_list_with_extra_data() {
        let data = "li1ei2eeextra";
        let result = decode_list(data).unwrap();
        assert_eq!(result.0, vec![BencodeValue::Int(1), BencodeValue::Int(2)]);
        assert_eq!(result.1, "extra");
    }
}
