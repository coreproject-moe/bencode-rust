from bencode_rs import bencode as rust_bencode, bdecode as rust_bdecode
from bencode import bdecode, bencode

l = [1, "abc", b"\xff", [2, 3], {"x": 5}]


# print(rust_bdecode(b"i-0e"))
# print(rust_bencode(b"\xff"))
def test_list_mixed_types():
    l = [1, "abc", b"\xff", [2, 3], {"x": 5}]
    r = bencode(l)
    print(r)
    print(rust_bdecode(r))


test_list_mixed_types()
