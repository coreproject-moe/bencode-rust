from bencode_rs import bencode as rust_bencode, bdecode as rust_bdecode
from bencode import bdecode, bencode

# l = [1, "abc", b"\xff", [2, 3], {"x": 5}]
# print(rust_bencode(l))
# print(rust_bdecode(rust_bencode([1, "abc", b"\xff", [2, 3], {"x": 5}])))
# print(bencode(l))

print(rust_bdecode(rust_bencode(b"\xff")))
print(bdecode(bencode("বাংলা")))
