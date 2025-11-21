pub fn encode_integer(data: isize) -> Result<String, &'static str> {
    let encoded = format!("i{}e", data);
    Ok(encoded.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_integer() {
        // Test a normal integer
        assert_eq!(encode_integer(42).unwrap(), "i42e");

        // Test zero
        assert_eq!(encode_integer(0).unwrap(), "i0e");

        // Test a large number
        assert_eq!(encode_integer(123456789).unwrap(), "i123456789e");

        // Test Negative Number
        assert_eq!(encode_integer(-42).unwrap(), "i-42e");
    }
}
