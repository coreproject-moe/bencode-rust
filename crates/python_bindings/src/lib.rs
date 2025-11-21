use std::collections::BTreeMap;

use bencode::enums::bencode::BencodeValue;
use pyo3::{
    prelude::*,
    types::{PyDict, PyList, PyTuple},
};

fn py_to_bencode_tokens(obj: Bound<PyAny>) -> PyResult<BencodeValue> {
    // Integers
    if let Ok(int_val) = obj.extract::<i64>() {
        return Ok(BencodeValue::Int(int_val));
    }

    // Strings -> UTF-8 bytes
    if let Ok(s) = obj.extract::<String>() {
        return Ok(BencodeValue::Str(s));
    }

    // Bytes
    if let Ok(bytes) = obj.extract::<Vec<u8>>() {
        return Ok(BencodeValue::Str(String::from_utf8(bytes).map_err(
            |_| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Dictionary key bytes are not valid UTF-8",
                )
            },
        )?));
    }

    // List (PyList)
    if let Ok(list) = obj.cast::<PyList>() {
        let mut items = Vec::new();
        for item in list.iter() {
            items.push(py_to_bencode_tokens(item)?);
        }
        return Ok(BencodeValue::List(items));
    }

    // Tuple -> treat exactly like a list
    if let Ok(tuple) = obj.cast::<PyTuple>() {
        let mut items = Vec::new();
        for item in tuple.iter() {
            items.push(py_to_bencode_tokens(item)?);
        }
        return Ok(BencodeValue::List(items));
    }

    // Dict
    if let Ok(dict) = obj.cast::<PyDict>() {
        let mut map = BTreeMap::new();

        for (key, value) in dict.iter() {
            // Keys must be byte-like
            let key_bytes = if let Ok(s) = key.extract::<String>() {
                s
            } else if let Ok(b) = key.extract::<Vec<u8>>() {
                String::from_utf8(b).map_err(|_| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "Dictionary key bytes are not valid UTF-8",
                    )
                })?
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Dictionary keys must be str or bytes",
                ));
            };

            let val = py_to_bencode_tokens(value)?;
            map.insert(key_bytes, val);
        }

        return Ok(BencodeValue::Dict(map));
    }

    // Unsupported type
    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
        "Unsupported type: {}",
        obj.get_type().name()?
    )))
}

/// A Python module implemented in Rust.
#[pymodule(name = "bencode_rs")]
mod python_bindings {
    use ::bencode::encoders::_bencode::encode_bencode;
    use pyo3::prelude::*;

    use super::py_to_bencode_tokens;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn bencode(obj: Bound<PyAny>) -> PyResult<Vec<u8>> {
        let objects = py_to_bencode_tokens(obj)?;
        let encoded = encode_bencode(objects).unwrap();
        let encoded_bytes = encoded.into_bytes();
        Ok(encoded_bytes)
    }
}
