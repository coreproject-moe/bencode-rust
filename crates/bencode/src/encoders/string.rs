pub fn encode_string(data: Vec<u8>) -> Result<String, &'static str> {
    let len = data.len();
    let content = String::from_utf8(data).map_err(|_| "Invalid UTF-8")?;
    Ok(format!("{}:{}", len, content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_string() {
        //
        assert_eq!(
            encode_string("bencode".to_string().into_bytes()).unwrap(),
            "7:bencode"
        );

        // Zero string
        assert_eq!(encode_string("".to_string().into_bytes()).unwrap(), "0:");

        // Test an unicode
        assert_eq!(
            encode_string("আসসালামু আলাইকুম".to_string().into_bytes()).unwrap(),
            "46:আসস\u{9be}ল\u{9be}ম\u{9c1} আল\u{9be}ইক\u{9c1}ম"
        );
    }
}
