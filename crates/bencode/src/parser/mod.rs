mod cursor;
mod stack;
#[cfg(test)]
mod tests;

pub use cursor::Cursor;
pub use stack::{ParseFrame, ParseStack};

use crate::enums::bencode::BencodeValue;

/// Parse a complete bencode value from `input`.
/// Returns the root `BencodeValue` and any trailing unconsumed bytes.
///
/// Iterative cursor-based parser with explicit heap-allocated stack.
/// No recursion — arbitrarily deep nesting will not overflow the call stack.
/// All containers (frames, lists, dicts, strings) are heap-allocated.
pub fn parse_bencode(input: &[u8]) -> Result<(BencodeValue, &[u8]), &'static str> {
    if input.is_empty() {
        return Err("Empty input");
    }

    // Pre-allocate stack capacity: at most input.len()/2 nested containers.
    let initial_capacity = (input.len() / 8).max(4);
    let mut cursor = Cursor::new(input);
    let mut stack = ParseStack::with_capacity(initial_capacity);

    // Dispatch the first token before entering the main loop.
    let first_byte = cursor.peek().ok_or("Empty input")?;
    match first_byte {
        b if b.is_ascii_digit() => {
            let (len, _) = cursor.parse_length_prefix()?;
            let bytes = cursor.read_bytes(len).ok_or("Truncated string content")?;
            let value = BencodeValue::Str(bytes.to_vec());
            return Ok((value, cursor.remaining()));
        }
        b'i' => {
            let num = cursor.parse_integer()?;
            let value = BencodeValue::Int(num);
            return Ok((value, cursor.remaining()));
        }
        b'l' => {
            cursor.advance(1);
            stack.push(ParseFrame::List {
                items: Vec::new(),
            });
        }
        b'd' => {
            cursor.advance(1);
            stack.push(ParseFrame::Dict {
                entries: std::collections::BTreeMap::new(),
                pending_key: None,
            });
        }
        _ => return Err("Unknown type prefix"),
    };

    // Main iterative parse loop.
    loop {
        if cursor.at_end() {
            return Err("Unexpected end of input");
        }

        let byte = cursor.peek().unwrap();

        match byte {
            // --- String token ---
            b if b.is_ascii_digit() => {
                let (len, _) = cursor.parse_length_prefix()?;
                let bytes = cursor.read_bytes(len).ok_or("Truncated string content")?;
                let value = BencodeValue::Str(bytes.to_vec());
                push_value(&mut stack, value)?;
            }

            // --- Integer token ---
            b'i' => {
                let num = cursor.parse_integer()?;
                let value = BencodeValue::Int(num);
                push_value(&mut stack, value)?;
            }

            // --- Open list ---
            b'l' => {
                cursor.advance(1);
                stack.push(ParseFrame::List { items: Vec::new() });
            }

            // --- Open dict ---
            b'd' => {
                cursor.advance(1);
                stack.push(ParseFrame::Dict {
                    entries: std::collections::BTreeMap::new(),
                    pending_key: None,
                });
            }

            // --- Close container ---
            b'e' => {
                cursor.advance(1);
                let frame = stack.pop().ok_or("Unmatched terminator")?;
                let completed = complete_frame(frame)?;

                if stack.is_empty() {
                    return Ok((completed, cursor.remaining()));
                }

                push_value(&mut stack, completed)?;
            }

            _ => return Err("Unknown token in input"),
        }
    }
}

/// Push a completed value onto the top frame of the stack.
#[inline]
fn push_value(stack: &mut ParseStack, value: BencodeValue) -> Result<(), &'static str> {
    let top = stack.last_mut().ok_or("No container to append to")?;

    match top {
        ParseFrame::List { items } => {
            items.push(value);
            Ok(())
        }
        ParseFrame::Dict { entries, pending_key } => {
            if let Some(key) = pending_key.take() {
                entries.insert(key, value);
                Ok(())
            } else {
                match &value {
                    BencodeValue::Str(bytes) => {
                        *pending_key = Some(bytes.clone());
                        Ok(())
                    }
                    _ => Err("Dictionary key must be a string"),
                }
            }
        }
    }
}

/// Finalize a popped frame into a `BencodeValue`.
#[inline]
fn complete_frame(frame: ParseFrame) -> Result<BencodeValue, &'static str> {
    match frame {
        ParseFrame::List { items } => Ok(BencodeValue::List(items)),
        ParseFrame::Dict { entries, pending_key } => {
            if pending_key.is_some() {
                return Err("Dictionary has unpaired key at end");
            }
            Ok(BencodeValue::Dict(entries))
        }
    }
}
