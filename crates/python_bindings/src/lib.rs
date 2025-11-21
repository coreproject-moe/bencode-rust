use std::collections::BTreeMap;

use bencode::enums::bencode::BencodeValue;
use pyo3::{
    IntoPyObjectExt,
    exceptions::PyValueError,
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
    if let Ok(int_val) = obj.extract::<i64>() {
        return Ok(BencodeValue::Int(int_val));
    }

    // Strings -> UTF-8 bytes
    if let Ok(s) = obj.extract::<String>() {
        return Ok(BencodeValue::Str(s));
    }

    // Bytes
    if let Ok(bytes) = obj.extract::<Vec<u8>>() {
        let s = bytes.iter().map(|&b| b as char).collect();
        return Ok(BencodeValue::Str(s));
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
        BencodeValue::Int(i) => Ok(i.into_bound_py_any(py).unwrap()),
        BencodeValue::Str(s) => Ok(s.into_bound_py_any(py).unwrap()),
        BencodeValue::List(list) => {
            let py_list = PyList::empty(py);
            for item in list {
                let py_item = bencode_tokens_to_py(py, item)?;
                py_list.append(py_item)?;
            }
            Ok(py_list.into_any())
        }
        BencodeValue::Dict(dict) => {
            let py_dict = PyDict::new(py);
            for (key, value) in dict {
                let py_value = bencode_tokens_to_py(py, value)?;
                py_dict.set_item(key, py_value)?;
            }
            Ok(py_dict.into_any())
        }
    }
}

#[pymodule(name = "bencode_rs")]
mod python_bindings {
    use ::bencode::{decoders::_bencode::decode_bencode, encoders::_bencode::encode_bencode};
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;

    use super::{bencode_tokens_to_py, py_to_bencode_tokens};

    #[pyfunction]
    fn bencode(obj: Bound<PyAny>) -> PyResult<Vec<u8>> {
        let objects = py_to_bencode_tokens(obj)?;
        let encoded = encode_bencode(objects).unwrap();
        let encoded_bytes = encoded.into_bytes();
        Ok(encoded_bytes)
    }

    #[pyfunction]
    fn bdecode<'py>(py: Python<'py>, string: &[u8]) -> PyResult<Bound<'py, PyAny>> {
        let object = String::from_utf8(string.to_vec()).map_err(|e| PyValueError::new_err(e))?;
        let (decoded_objects, _rest) =
            decode_bencode(&object).map_err(|e| PyValueError::new_err(e))?;
        let python_objects = bencode_tokens_to_py(py, decoded_objects)?;
        Ok(python_objects)
    }
}
