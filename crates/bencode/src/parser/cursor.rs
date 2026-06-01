/// A zero-copy cursor over the input byte slice.
/// Advances through input without cloning, only adjusting an offset.
#[derive(Debug, Clone, Copy)]
pub struct Cursor<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    #[inline]
    pub fn new(input: &'a [u8]) -> Self {
        Self { input, position: 0 }
    }

    #[inline]
    pub fn remaining(&self) -> &'a [u8] {
        &self.input[self.position..]
    }

    #[inline]
    pub fn at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    #[inline]
    pub fn peek(&self) -> Option<u8> {
        self.input.get(self.position).copied()
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        self.position = self.position.saturating_add(n);
    }

    #[inline]
    pub fn read_bytes(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.position.checked_add(n)?;
        if end > self.input.len() {
            return None;
        }
        let slice = &self.input[self.position..end];
        self.position = end;
        Some(slice)
    }

    /// Parse an ASCII decimal length prefix, e.g. the "5" in "5:hello".
    /// Returns (parsed_length, bytes_consumed_including_colon).
    #[inline]
    pub fn parse_length_prefix(&mut self) -> Result<(usize, usize), &'static str> {
        let start = self.position;
        let mut len: usize = 0;

        while let Some(&b) = self.input.get(self.position) {
            if b == b':' {
                self.position += 1;
                return Ok((len, self.position - start));
            }
            let digit = (b - b'0') as usize;
            if digit < 10 {
                len = len
                    .checked_mul(10)
                    .ok_or("String length overflow")?
                    .checked_add(digit)
                    .ok_or("String length overflow")?;
                self.position += 1;
            } else {
                return Err("Invalid character in string length");
            }
        }

        Err("Missing colon in string length prefix")
    }

    /// Parse an integer value: skip 'i', accumulate digits to 'e', advance cursor.
    /// Single-pass: no full-slice scan, no UTF-8 conversion, no string allocation.
    #[inline]
    pub fn parse_integer(&mut self) -> Result<i128, &'static str> {
        self.position += 1; // skip 'i'

        let mut negative = false;
        let mut value: i128 = 0;
        let mut seen_digit = false;

        // Handle optional leading '-'
        if let Some(&b) = self.input.get(self.position)
            && b == b'-'
        {
            negative = true;
            self.position += 1;
        }

        // Leading-zero check: if next byte is '0' and followed by another digit, reject
        if let Some(&b) = self.input.get(self.position)
            && b == b'0'
        {
            if let Some(&b2) = self.input.get(self.position + 1)
                && b2.is_ascii_digit()
            {
                return Err("Leading zeros in integer");
            }
            // Bare '0' (possibly followed by 'e') — value stays 0
            self.position += 1;
            seen_digit = true;
        }

        // Accumulate remaining digits, require 'e' terminator
        let mut terminated = false;
        while let Some(&b) = self.input.get(self.position) {
            if b == b'e' {
                self.position += 1;
                terminated = true;
                break;
            }
            if b.is_ascii_digit() {
                let digit = (b - b'0') as i128;
                value = value
                    .checked_mul(10)
                    .ok_or("Integer overflow")?
                    .checked_add(digit)
                    .ok_or("Integer overflow")?;
                seen_digit = true;
                self.position += 1;
            } else {
                return Err("Invalid character in integer");
            }
        }

        if !terminated {
            return Err("Missing integer terminator");
        }
        if !seen_digit {
            return Err("Empty integer");
        }
        if negative && value == 0 {
            return Err("Negative zero");
        }

        Ok(if negative { -value } else { value })
    }
}
