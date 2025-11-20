pub fn decode_integer(data: &str) -> Result<(i64, &str), &'static str> {
    if !data.starts_with('i') {
        return Err("Not an Integer");
    }
    let end = data.find('e').ok_or("Missing 'e' for integer")?;
    let int_part = &data[1..end];

    let value = int_part.parse::<i64>().map_err(|_| "Invalid Integer")?;
    Ok((value, &data[end + 1..]))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_integer() {
        // Test a normal integer
        assert_eq!(decode_integer("i42e").unwrap(), (42, ""));

        // Test zero
        assert_eq!(decode_integer("i0e").unwrap(), (0, ""));

        // Test a large number
        assert_eq!(decode_integer("i123456789e").unwrap(), (123456789, ""));

        // Test negative number
        assert_eq!(decode_integer("i-42e").unwrap(), (-42, ""));
    }

    #[test]
    fn test_decode_integer_with_extra_data() {
        // Integer followed by extra data
        assert_eq!(decode_integer("i42ehello").unwrap(), (42, "hello"));
    }

    #[test]
    fn test_decode_integer_errors() {
        // Not starting with 'i'
        assert!(decode_integer("42e").is_err());

        // Missing 'e'
        assert!(decode_integer("i42").is_err());

        // Invalid integer
        assert!(decode_integer("i4a2e").is_err());
    }
}
