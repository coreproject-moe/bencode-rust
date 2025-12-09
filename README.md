<div align='center'>
    <img src="./docs/src/img/logo.svg" alt="logo" />
</div>
<p align="center">
    A modern bencode parser written from scratch for the next generation of torrent streaming
</p>

---

`bencode-rust` is a new parser for bencode written from scratch ( using rust! )

The main goals are:

-   Readable: The main goal of this project is to be readable.
-   Fast: The other goal of this project is to be fast ( like really fast ).
-   Feature Parity: Support the entire [bencode encoding scheme](https://en.wikipedia.org/wiki/Bencode#Encoding_Algorithm)
-   Optimize: The parser should be optimized, every byte optimizes will theoritically optimize other lower libs that depend on it
-   Extensible: The library should support as many language as possible

# License

The project is licensed under Apache 2.0 license
