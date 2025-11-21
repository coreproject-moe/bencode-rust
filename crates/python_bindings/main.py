from bencode_rs import bencode as rust_bencode, bdecode as rust_bdecode
from bencode import bdecode

rust_bdecode(
    [
        1,
        "abc"
    ]
)
