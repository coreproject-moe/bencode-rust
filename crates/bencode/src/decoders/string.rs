pub fn decode_string(data: &str) -> Result<(String, &str), &'static str> {
    let colon = data.find(':').ok_or("Invalid string format")?;
    let len: usize = data[..colon].parse().map_err(|_| "Invalid string length")?;
    let start = colon + 1;
    let end = start + len;
    if end > data.len() {
        return Err("String length out of bounds");
    }
    let value = data[start..end].to_string();
    Ok((value, &data[end..]))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_string() {
        // Normal string
        assert_eq!(
            decode_string("7:bencode").unwrap(),
            ("bencode".to_string(), "")
        );

        // Zero-length string
        assert_eq!(decode_string("0:").unwrap(), ("".to_string(), ""));

        // Unicode string
        assert_eq!(
            decode_string("46:আসস\u{9be}ল\u{9be}ম\u{9c1} আল\u{9be}ইক\u{9c1}ম").unwrap(),
            ("আসসালামু আলাইকুম".to_string(), "")
        );
    }

    #[test]
    fn test_decode_string_with_remaining_data() {
        // Remaining data after string
        assert_eq!(
            decode_string("5:helloXYZ").unwrap(),
            ("hello".to_string(), "XYZ")
        );
    }
}
