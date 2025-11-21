use std::collections::BTreeMap;

use bencode::enums::bencode::BencodeValue;
use pyo3::{
    IntoPyObjectExt,
    prelude::*,
    types::{PyDict, PyList, PyTuple},
};

fn py_to_bencode_tokens(obj: Bound<PyAny>) -> PyResult<BencodeValue> {
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

    // Integers
    if let Ok(int_val) = obj.extract::<isize>() {
        return Ok(BencodeValue::Int(int_val));
    }

    // Strings -> UTF-8 bytes
    if let Ok(s) = obj.extract::<String>() {
        return Ok(BencodeValue::Str(s.into_bytes()));
    }

    // Bytes
    if let Ok(bytes) = obj.extract::<Vec<u8>>() {
        return Ok(BencodeValue::Str(bytes));
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

fn bencode_tokens_to_py(py: Python, tokens: BencodeValue) -> PyResult<Bound<PyAny>> {
    match tokens {
        BencodeValue::Int(i) => i.into_bound_py_any(py),
        BencodeValue::Str(bytes) => {
            // Attempt UTF-8 decoding
            match String::from_utf8(bytes) {
                Ok(utf8_string) => utf8_string.into_bound_py_any(py),
                Err(e) => {
                    // Fall back to bytes for invalid UTF-8
                    let raw_bytes = e.into_bytes();
                    raw_bytes.into_bound_py_any(py)
                }
            }
        }
        BencodeValue::List(list) => {
            let py_list = PyList::empty(py);
            for item in list {
                py_list.append(bencode_tokens_to_py(py, item)?)?;
            }
            Ok(py_list.into_any())
        }
        BencodeValue::Dict(dict) => {
            let py_dict = PyDict::new(py);
            for (key, value) in dict {
                py_dict.set_item(key, bencode_tokens_to_py(py, value)?)?;
            }
            Ok(py_dict.into_any())
        }
    }
}

#[pymodule(name = "bencode_rs")]
mod python_bindings {
    use ::bencode::{decoders::bencode::decode_bencode, encoders::bencode::encode_bencode};
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;

    use super::{bencode_tokens_to_py, py_to_bencode_tokens};

    #[pyfunction]
    fn bencode(obj: Bound<PyAny>) -> PyResult<Vec<u8>> {
        let objects = py_to_bencode_tokens(obj)?;
        let encoded = encode_bencode(objects).unwrap();
        Ok(encoded)
    }

    #[pyfunction]
    fn bdecode<'py>(py: Python<'py>, string: &[u8]) -> PyResult<Bound<'py, PyAny>> {
        let (decoded_objects, _rest) =
            decode_bencode(string).map_err(|e| PyValueError::new_err(e))?;
        let python_objects = bencode_tokens_to_py(py, decoded_objects)?;
        Ok(python_objects)
    }
}
