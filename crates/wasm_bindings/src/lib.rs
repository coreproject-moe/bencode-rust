use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

use bencode::enums::bencode::BencodeValue;

/// Maximum safe integer for JS f64: 2^53 - 1
const F64_MAX_SAFE: i128 = (1_i128 << 53) - 1;

/// Convert a JS value into a `BencodeValue`.
fn js_to_bencode(value: &JsValue) -> Result<BencodeValue, JsValue> {
    if value.is_null() || value.is_undefined() {
        return Err(JsValue::from_str("null/undefined unsupported in bencode"));
    }

    // Number -> Int (f64 safe integer range)
    if let Some(n) = value.as_f64() {
        return Ok(BencodeValue::Int(n as i128));
    }

    // BigInt -> Int (lossless for arbitrary-size integers)
    if value.is_bigint() {
        let big = js_sys::BigInt::from(value.clone());
        let obj = js_sys::Object::from(big);
        let s: String = js_sys::Object::to_string(&obj).into();
        let n: i128 = s
            .parse()
            .map_err(|_| JsValue::from_str("BigInt out of i128 range"))?;
        return Ok(BencodeValue::Int(n));
    }

    // String -> Str
    if let Some(s) = value.as_string() {
        return Ok(BencodeValue::Str(s.into_bytes()));
    }

    // Uint8Array -> Str
    if js_sys::Uint8Array::instanceof(value) {
        let arr = js_sys::Uint8Array::new(value);
        return Ok(BencodeValue::Str(arr.to_vec()));
    }

    // Array -> List
    if js_sys::Array::is_array(value) {
        let js_array = js_sys::Array::from(value);
        let len = js_array.length() as usize;
        let mut items = Vec::with_capacity(len);
        for v in js_array.iter() {
            items.push(js_to_bencode(&v)?);
        }
        return Ok(BencodeValue::List(items));
    }

    // Object -> Dict
    if value.is_object() {
        let obj = js_sys::Object::from(value.clone());
        let entries = js_sys::Object::entries(&obj);

        let mut map = BTreeMap::new();
        for entry in entries.iter() {
            let pair = js_sys::Array::from(&entry);
            let key_js = pair.get(0);
            let val_js = pair.get(1);

            let key_bytes: Vec<u8> = key_js
                .as_string()
                .ok_or_else(|| JsValue::from_str("dict keys must be strings"))?
                .into_bytes();

            map.insert(key_bytes, js_to_bencode(&val_js)?);
        }

        return Ok(BencodeValue::Dict(map));
    }

    Err(JsValue::from_str("Unsupported JS type"))
}

/// Convert a `BencodeValue` back into a JS value.
fn bencode_to_js(value: BencodeValue, decode_utf: bool) -> JsValue {
    match value {
        BencodeValue::Int(i) => {
            // Use f64 for values within safe integer range, BigInt (via i128) for larger values
            if (-(F64_MAX_SAFE)..=F64_MAX_SAFE).contains(&i) {
                JsValue::from_f64(i as f64)
            } else {
                // wasm-bindgen converts i128 to JS BigInt automatically
                JsValue::from(i)
            }
        }

        BencodeValue::Str(bytes) => {
            if decode_utf {
                match String::from_utf8(bytes) {
                    Ok(s) => return JsValue::from_str(&s),
                    Err(e) => {
                        let raw = e.into_bytes();
                        let arr = js_sys::Uint8Array::new_with_length(raw.len() as u32);
                        arr.copy_from(&raw);
                        return arr.into();
                    }
                }
            }
            let arr = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
            arr.copy_from(&bytes);
            arr.into()
        }

        BencodeValue::List(list) => {
            let js_arr = js_sys::Array::new_with_length(list.len() as u32);
            for (i, v) in list.into_iter().enumerate() {
                js_arr.set(u32::try_from(i).unwrap(), bencode_to_js(v, decode_utf));
            }
            js_arr.into()
        }

        BencodeValue::Dict(dict) => {
            let obj = js_sys::Object::new();
            for (key, val) in dict {
                let key_str = String::from_utf8_lossy(&key);
                let key_js = JsValue::from_str(&key_str);
                js_sys::Reflect::set(&obj, &key_js, &bencode_to_js(val, decode_utf)).ok();
            }
            obj.into()
        }
    }
}

/// Encode a JavaScript value to bencode bytes.
///
/// Args:
///   value - The JavaScript value to encode
///   decode_utf - Reserved for API symmetry with bdecode; ignored during encoding
///
/// Accepts: number, BigInt, string, Uint8Array, Array, Object
/// Returns: Uint8Array of bencode-encoded bytes
#[wasm_bindgen(js_name = "bencode")]
pub fn wasm_bencode(value: JsValue, _decode_utf: bool) -> Result<js_sys::Uint8Array, JsValue> {
    let tokens = js_to_bencode(&value)?;
    let bytes = bencode::encode_bencode(tokens).map_err(JsValue::from_str)?;
    let result = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
    result.copy_from(&bytes);
    Ok(result)
}

/// Decode bencode bytes to a JavaScript value.
///
/// Args:
///   bytes - Uint8Array or ArrayBuffer of bencode-encoded data
///   decode_utf - If true, return strings instead of Uint8Arrays for string data
///
/// Returns: JavaScript value (number/BigInt, string/Uint8Array, Array, Object)
#[wasm_bindgen(js_name = "bdecode")]
pub fn wasm_bdecode(bytes: &[u8], decode_utf: bool) -> Result<JsValue, JsValue> {
    let (tokens, _) = bencode::decode_bencode(bytes).map_err(JsValue::from_str)?;
    Ok(bencode_to_js(tokens, decode_utf))
}
