---
icon: simple/python
---

Python support is first class

### Installation

```console
pip install bencode-rust
```

### Usage

```python
from bencode_rs import bencode, bdecode

bencode("test") # b'4:test'
bdecode(b'4:test') # test
```

### Goals

The main goal of python wrapper is to:

-   Fastest: Be the fastest bencode parser in the wild
-   Type Hinting: Have 100% type coverage
