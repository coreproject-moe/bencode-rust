#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::enums::bencode::BencodeValue;
    use crate::parser::{parse_bencode, Cursor};

    // --- Strings ---

    #[test]
    fn parse_simple_string() {
        let (val, rest) = parse_bencode(b"5:hello").unwrap();
        assert_eq!(val, BencodeValue::Str(b"hello".to_vec()));
        assert!(rest.is_empty());
    }

    #[test]
    fn parse_empty_string() {
        let (val, rest) = parse_bencode(b"0:").unwrap();
        assert_eq!(val, BencodeValue::Str(Vec::new()));
        assert!(rest.is_empty());
    }

    #[test]
    fn parse_string_with_trailing_data() {
        let (val, rest) = parse_bencode(b"3:abc123").unwrap();
        assert_eq!(val, BencodeValue::Str(b"abc".to_vec()));
        assert_eq!(rest, b"123".as_slice());
    }

    #[test]
    fn parse_binary_string() {
        let (val, _rest) = parse_bencode(b"4:\xff\x00\xaa\xbb").unwrap();
        assert_eq!(val, BencodeValue::Str(vec![0xff, 0x00, 0xaa, 0xbb]));
    }

    // --- Integers ---

    #[test]
    fn parse_positive_integer() {
        let (val, rest) = parse_bencode(b"i42e").unwrap();
        assert_eq!(val, BencodeValue::Int(42));
        assert!(rest.is_empty());
    }

    #[test]
    fn parse_zero() {
        let (val, rest) = parse_bencode(b"i0e").unwrap();
        assert_eq!(val, BencodeValue::Int(0));
        assert!(rest.is_empty());
    }

    #[test]
    fn parse_negative_integer() {
        let (val, _rest) = parse_bencode(b"i-42e").unwrap();
        assert_eq!(val, BencodeValue::Int(-42));
    }

    #[test]
    fn parse_large_integer() {
        let (val, _rest) = parse_bencode(b"i12345678901234567890e").unwrap();
        assert_eq!(val, BencodeValue::Int(12_345_678_901_234_567_890));
    }

    #[test]
    fn parse_integer_with_trailing() {
        let (val, rest) = parse_bencode(b"i123ei456e").unwrap();
        assert_eq!(val, BencodeValue::Int(123));
        assert_eq!(rest, b"i456e".as_slice());
    }

    // --- Lists ---

    #[test]
    fn parse_empty_list() {
        let (val, rest) = parse_bencode(b"le").unwrap();
        assert_eq!(val, BencodeValue::List(Vec::new()));
        assert!(rest.is_empty());
    }

    #[test]
    fn parse_simple_list() {
        let (val, _rest) = parse_bencode(b"li42ei99ee").unwrap();
        match val {
            BencodeValue::List(items) => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[0], BencodeValue::Int(42));
                assert_eq!(items[1], BencodeValue::Int(99));
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn parse_nested_list() {
        let (val, _rest) = parse_bencode(b"lli42eee").unwrap();
        match val {
            BencodeValue::List(outer) => {
                assert_eq!(outer.len(), 1);
                match &outer[0] {
                    BencodeValue::List(inner) => {
                        assert_eq!(inner.len(), 1);
                        assert_eq!(inner[0], BencodeValue::Int(42));
                    }
                    _ => panic!("Expected inner list"),
                }
            }
            _ => panic!("Expected outer list"),
        }
    }

    #[test]
    fn parse_mixed_list() {
        let (val, _rest) = parse_bencode(b"l5:helloi42ei-7e3:byee").unwrap();
        match val {
            BencodeValue::List(items) => {
                assert_eq!(items.len(), 4);
                assert_eq!(items[0], BencodeValue::Str(b"hello".to_vec()));
                assert_eq!(items[1], BencodeValue::Int(42));
                assert_eq!(items[2], BencodeValue::Int(-7));
                assert_eq!(items[3], BencodeValue::Str(b"bye".to_vec()));
            }
            _ => panic!("Expected list"),
        }
    }

    // --- Dictionaries ---

    #[test]
    fn parse_empty_dict() {
        let (val, rest) = parse_bencode(b"de").unwrap();
        match val {
            BencodeValue::Dict(map) => assert!(map.is_empty()),
            _ => panic!("Expected dict"),
        }
        assert!(rest.is_empty());
    }

    #[test]
    fn parse_simple_dict() {
        let (val, _rest) = parse_bencode(b"d4:name5:alicee").unwrap();
        match val {
            BencodeValue::Dict(map) => {
                assert_eq!(map.len(), 1);
                assert_eq!(*map.get(b"name".as_slice()).unwrap(), BencodeValue::Str(b"alice".to_vec()));
            }
            _ => panic!("Expected dict"),
        }
    }

    #[test]
    fn parse_dict_with_multiple_keys() {
        let (val, _rest) = parse_bencode(b"d3:agei25e4:name5:alicee").unwrap();
        match val {
            BencodeValue::Dict(map) => {
                assert_eq!(map.len(), 2);
                assert_eq!(*map.get(b"age".as_slice()).unwrap(), BencodeValue::Int(25));
                assert_eq!(*map.get(b"name".as_slice()).unwrap(), BencodeValue::Str(b"alice".to_vec()));
            }
            _ => panic!("Expected dict"),
        }
    }

    #[test]
    fn parse_nested_dict() {
        let (val, _rest) = parse_bencode(b"d4:addrd7:zipcodei90210eee").unwrap();
        match val {
            BencodeValue::Dict(map) => {
                assert_eq!(map.len(), 1);
                match map.get(b"addr".as_slice()) {
                    Some(BencodeValue::Dict(inner)) => {
                        assert_eq!(inner.len(), 1);
                        assert_eq!(*inner.get(b"zipcode".as_slice()).unwrap(), BencodeValue::Int(90210));
                    }
                    _ => panic!("Expected nested dict"),
                }
            }
            _ => panic!("Expected outer dict"),
        }
    }

    // --- Error cases ---

    #[test]
    fn parse_empty_input() {
        assert!(parse_bencode(b"").is_err());
    }

    #[test]
    fn parse_unknown_prefix() {
        assert!(parse_bencode(b"x42e").is_err());
    }

    #[test]
    fn parse_missing_terminator() {
        assert!(parse_bencode(b"li42e").is_err());
    }

    #[test]
    fn parse_leading_zero_integer() {
        assert!(parse_bencode(b"i042e").is_err());
    }

    #[test]
    fn parse_empty_integer() {
        assert!(parse_bencode(b"ie").is_err());
    }

    #[test]
    fn parse_truncated_string() {
        assert!(parse_bencode(b"10:short").is_err());
    }

    #[test]
    fn parse_dict_non_string_key() {
        assert!(parse_bencode(b"di42ei99ee").is_err());
    }

    #[test]
    fn parse_unmatched_terminator() {
        assert!(parse_bencode(b"e").is_err());
    }

    // --- Round-trip ---

    #[test]
    fn roundtrip_complex_structure() {
        use crate::dispatcher::bencode::encode_bencode;

        let original = BencodeValue::List(vec![
            BencodeValue::Int(42),
            BencodeValue::Str(b"hello".to_vec()),
            BencodeValue::Dict({
                let mut map = std::collections::BTreeMap::new();
                map.insert(b"key".to_vec(), BencodeValue::Int(-99));
                map
            }),
            BencodeValue::List(vec![
                BencodeValue::Str(b"nested".to_vec()),
            ]),
        ]);

        let encoded = encode_bencode(original.clone()).unwrap();
        let (decoded, rest) = parse_bencode(&encoded).unwrap();
        assert!(rest.is_empty());
        assert_eq!(original, decoded);
    }

    // --- Deep nesting (no stack overflow) ---

    #[test]
    fn parse_deeply_nested_list() {
        // Build a list nested 10,000 levels deep using a flat heap buffer.
        // Format: lli...i1eee...e  (10k 'l's, i1e, 10k 'e's)
        let depth = 2_000;
        let inner = b"i1e";
        let needed = depth + inner.len() + depth;
        let mut payload = Vec::with_capacity(needed);
        payload.extend(std::iter::repeat_n(b'l', depth));
        payload.extend(inner);
        payload.extend(std::iter::repeat_n(b'e', depth));

        let (val, _rest) = parse_bencode(&payload).unwrap();
        assert!(matches!(val, BencodeValue::List(_)));
    }

    // --- Cursor-specific ---

    #[test]
    fn cursor_position_advances_correctly() {
        let cursor = Cursor::new(b"hello");
        assert_eq!(cursor.peek(), Some(b'h'));
        let mut c = cursor;
        c.advance(3);
        assert_eq!(c.peek(), Some(b'l'));
        assert_eq!(c.remaining(), b"lo".as_slice());
    }

    #[test]
    fn cursor_read_bytes() {
        let mut cursor = Cursor::new(b"hello world");
        let slice = cursor.read_bytes(5).unwrap();
        assert_eq!(slice, b"hello".as_slice());
        assert_eq!(cursor.peek(), Some(b' '));
    }
}
