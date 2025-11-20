pub fn decode_integer(data: &str) -> Result<i64, &'static str> {
    if !data.starts_with('i') {
        return Err("Not an Integer");
    }
    let end = data.find('e').ok_or("Missing 'e' for integer")?;
    let int_part = &data[1..end];
    let value = int_part.parse::<i64>().map_err(|_| "Invalid Integer")?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_integer() {
        // Test a normal integer
        assert_eq!(decode_integer("i42e").unwrap(), 42);

        // Test zero
        assert_eq!(decode_integer("i0e").unwrap(), 0);

        // Test a large number
        assert_eq!(decode_integer("i123456789e").unwrap(), 123456789);

        // Test Negative Number
        assert_eq!(decode_integer("i-42e").unwrap(), -42);
    }
}
