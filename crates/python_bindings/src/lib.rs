use std::collections::BTreeMap;

use bencode::enums::bencode::BencodeValue;
use bencode::encode_bencode;
use bencode::decode_bencode;
use pyo3::{
    prelude::*,
    types::{PyDict, PyList, PyTuple},
};

/// Convert a Python object into a `BencodeValue`.
fn py_to_bencode_tokens(obj: &Bound<PyAny>) -> PyResult<BencodeValue> {
    // List
    if let Ok(list) = obj.cast::<PyList>() {
        let items = list
            .iter()
            .map(|item| py_to_bencode_tokens(&item))
            .collect::<PyResult<Vec<_>>>()?;
        return Ok(BencodeValue::List(items));
    }

    // Tuple -> treat as list
    if let Ok(tuple) = obj.cast::<PyTuple>() {
        let items = tuple
            .iter()
            .map(|item| py_to_bencode_tokens(&item))
            .collect::<PyResult<Vec<_>>>()?;
        return Ok(BencodeValue::List(items));
    }

    // Integer
    if let Ok(int_val) = obj.extract::<i128>() {
        return Ok(BencodeValue::Int(int_val));
    }

    // String -> UTF-8 bytes
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
            let key_bytes: Vec<u8> = if let Ok(s) = key.extract::<&str>() {
                s.as_bytes().to_vec()
            } else if let Ok(py_bytes) = key.extract::<&[u8]>() {
                py_bytes.to_vec()
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "Dictionary keys must be str or bytes",
                ));
            };

            let val = py_to_bencode_tokens(&value)?;
            map.insert(key_bytes, val);
        }

        return Ok(BencodeValue::Dict(map));
    }

    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
        "Unsupported type: {:?}",
        obj.get_type(),
    )))
}

/// Convert a `BencodeValue` back into a Python object.
fn bencode_to_py<'py>(
    py: Python<'py>,
    tokens: BencodeValue,
    decode_utf: bool,
) -> PyResult<Bound<'py, PyAny>> {
    match tokens {
        BencodeValue::Int(i) => {
            Ok(i.into_pyobject(py)?.into_any())
        }
        BencodeValue::Str(bytes) => {
            if decode_utf {
                match String::from_utf8(bytes) {
                    Ok(s) => Ok(pyo3::types::PyString::new(py, &s).into_any()),
                    Err(e) => {
                        let b = e.into_bytes();
                        Ok(pyo3::types::PyBytes::new(py, &b).into_any())
                    }
                }
            } else {
                Ok(pyo3::types::PyBytes::new(py, &bytes).into_any())
            }
        }
        BencodeValue::List(list) => {
            let py_list = PyList::empty(py);
            for item in list {
                let py_item = bencode_to_py(py, item, decode_utf)?;
                py_list.append(py_item)?;
            }
            Ok(py_list.into_any())
        }
        BencodeValue::Dict(dict) => {
            let py_dict = PyDict::new(py);
            for (key, value) in dict {
                let key_str = String::from_utf8_lossy(&key);
                let key_obj = pyo3::types::PyString::new(py, &key_str);
                let val_obj = bencode_to_py(py, value, decode_utf)?;
                py_dict.set_item(key_obj, val_obj)?;
            }
            Ok(py_dict.into_any())
        }
    }
}

/// bencode_rs Python module.
///
/// Provides `bencode()` and `bdecode()` for encoding/decoding bencode data.
/// Uses a cursor-based iterative parser with heap-allocated stack frames
/// for zero-recursion, arbitrarily deep nesting.
///
/// Built with pyo3 GIL-less API (Bound<'py, PyAny>) for Python 3.12+ stable ABI.
#[pymodule(name = "bencode_rs")]
mod python_bindings {
    use super::*;

    /// Encode a Python object into bencode bytes.
    #[pyfunction]
    fn bencode(obj: Bound<PyAny>) -> PyResult<Vec<u8>> {
        let tokens = py_to_bencode_tokens(&obj)?;
        encode_bencode(tokens)
            .map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)
    }

    /// Decode bencode bytes into a Python object.
    #[pyfunction]
    #[pyo3(signature = (data, decode_utf = true))]
    fn bdecode<'py>(py: Python<'py>, data: &[u8], decode_utf: bool) -> PyResult<Bound<'py, PyAny>> {
        let (tokens, _) = decode_bencode(data)
            .map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;
        bencode_to_py(py, tokens, decode_utf)
    }
}
