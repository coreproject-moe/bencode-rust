pub fn encode_string(data: &str) -> Result<String, &'static str> {
    let encoded = format!("{}:{}", data.len(), data);
    Ok(encoded.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_string() {
        //
        assert_eq!(encode_string("bencode").unwrap(), "7:bencode");

        // Zero string
        assert_eq!(encode_string("").unwrap(), "0:");

        // Test an unicode
        assert_eq!(
            encode_string("আসসালামু আলাইকুম").unwrap(),
            "46:আসস\u{9be}ল\u{9be}ম\u{9c1} আল\u{9be}ইক\u{9c1}ম"
        );
    }
}
