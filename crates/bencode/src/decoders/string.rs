pub fn decode_string(data: &str) -> Result<String, &'static str> {
    let colon = data.find(':').ok_or("Invalid string format")?;
    let size = data[..colon]
        .parse::<usize>()
        .map_err(|_| "Invalid Length")?;

    let string_start = colon + 1;
    let string_end = string_start + size;
    let value = data[string_start..string_end].to_string();

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_string() {
        //
        assert_eq!(decode_string("7:bencode").unwrap(), "bencode");

        // Zero string
        assert_eq!(decode_string("0:").unwrap(), "");

        // Test an unicode
        assert_eq!(
            decode_string("46:আসস\u{9be}ল\u{9be}ম\u{9c1} আল\u{9be}ইক\u{9c1}ম").unwrap(),
            "আসসালামু আলাইকুম"
        );
    }
}
