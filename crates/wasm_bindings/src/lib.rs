use std::collections::BTreeMap;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use bencode::enums::bencode::BencodeValue;

fn js_to_bencode(value: &JsValue, decode_utf: Option<bool>) -> Result<BencodeValue, JsValue> {
    let decode = decode_utf.unwrap_or(false);
    // null and undefined not supported
    if value.is_null() || value.is_undefined() {
        return Err(JsValue::from_str("null/undefined unsupported in bencode"));
    }

    // Number
    if let Some(n) = value.as_f64() {
        return Ok(BencodeValue::Int(n as isize));
    }

    // String
    if let Some(s) = value.as_string() {
        return Ok(BencodeValue::Str(s.into_bytes()));
    }

    // Uint8Array
    if js_sys::Uint8Array::instanceof(value) {
        let arr = js_sys::Uint8Array::new(value);
        return Ok(BencodeValue::Str(arr.to_vec()));
    }

    // Array
    if js_sys::Array::is_array(value) {
        let js_array = js_sys::Array::from(value);
        let mut items = Vec::new();
        for v in js_array.iter() {
            items.push(js_to_bencode(&v, Some(decode))?);
        }
        return Ok(BencodeValue::List(items));
    }

    // Object → Dict
    if value.is_object() {
        let obj = js_sys::Object::from(value.clone());
        let entries = js_sys::Object::entries(&obj);

        let mut map = BTreeMap::<Vec<u8>, BencodeValue>::new();

        for entry in entries.iter() {
            let pair = js_sys::Array::from(&entry);
            let key_js = pair.get(0);
            let val_js = pair.get(1);

            // Convert key to bytes
            let key_bytes: Vec<u8> = if decode {
                key_js
                    .as_string()
                    .ok_or_else(|| {
                        JsValue::from_str("dict keys must be strings in `decode_utf=true`")
                    })?
                    .into_bytes()
            } else if let Some(s) = key_js.as_string() {
                s.into_bytes()
            } else if js_sys::Uint8Array::instanceof(&key_js) {
                js_sys::Uint8Array::new(&key_js).to_vec()
            } else {
                return Err(JsValue::from_str("dict key must be string or Uint8Array"));
            };

            map.insert(key_bytes, js_to_bencode(&val_js, Some(decode))?);
        }

        return Ok(BencodeValue::Dict(map));
    }

    Err(JsValue::from_str("Unsupported JS type"))
}

fn bencode_to_js(value: BencodeValue, decode_utf: Option<bool>) -> JsValue {
    let decode = decode_utf.unwrap_or(false);
    match value {
        BencodeValue::Int(i) => JsValue::from_f64(i as f64),

        BencodeValue::Str(bytes) => {
            if decode {
                if let Ok(s) = String::from_utf8(bytes.clone()) {
                    return JsValue::from_str(&s);
                }
            }
            let arr = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
            arr.copy_from(&bytes);
            arr.into()
        }

        BencodeValue::List(list) => {
            let arr = js_sys::Array::new();
            for v in list {
                arr.push(&bencode_to_js(v, Some(decode)));
            }
            arr.into()
        }

        BencodeValue::Dict(dict) => {
            let obj = js_sys::Object::new();
            for (key, val) in dict {
                let key_js = if decode {
                    match String::from_utf8(key.clone()) {
                        Ok(s) => JsValue::from_str(&s),
                        Err(e) => {
                            let raw =
                                js_sys::Uint8Array::new_with_length(e.into_bytes().len() as u32);
                            raw.into()
                        }
                    }
                } else {
                    let raw = js_sys::Uint8Array::new_with_length(key.len() as u32);
                    raw.copy_from(&key);
                    raw.into()
                };

                js_sys::Reflect::set(&obj, &key_js, &bencode_to_js(val, Some(decode))).unwrap();
            }
            obj.into()
        }
    }
}

#[wasm_bindgen]
pub fn bencode(value: JsValue, decode_utf: Option<bool>) -> Result<Vec<u8>, JsValue> {
    let tokens = js_to_bencode(&value, decode_utf)?;
    let bytes = bencode::dispatcher::bencode::encode_bencode(tokens)
        .map_err(|_| JsValue::from_str("encode error"))?;
    Ok(bytes)
}

#[wasm_bindgen]
pub fn bdecode(bytes: &[u8], decode_utf: Option<bool>) -> Result<JsValue, JsValue> {
    let (tokens, _) =
        bencode::dispatcher::bdecode::decode_bencode(bytes).map_err(|e| JsValue::from_str(e))?;
    Ok(bencode_to_js(tokens, decode_utf))
}
