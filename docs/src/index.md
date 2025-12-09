---
title: 'Introduction'
icon: lucide/rocket
---

<div align="center">

![logo](./img/logo.svg){loading=lazy}

</div>

<p align="center">
    A modern bencode parser written from scratch for the next generation of torrent streaming
</p>

---

Documentation : https://bencode-rust.readthedocs.io/

Source Code : https://github.com/baseplate-admin/bencode-rust

---

Bencode rust is a new parser for bencode written from scratch ( using :simple-rust: )

The main goals are:

-   Readable: The main goal of this project is to be readable.
-   Fast: The other goal of this project is to be fast ( like really fast ).
-   Feature Parity: Support the entire [bencode encoding scheme](https://en.wikipedia.org/wiki/Bencode#Encoding_Algorithm)
-   Optimize: The parser should be optimized, every byte optimizes will theoritically optimize other lower libs that depend on it
-   Extensible: The library should support as many language as possible

# Language Support

| Language        | Support Type |         Location         |
| --------------- | :----------: | :----------------------: |
| Rust            |    :star:    |           Core           |
| Python[^1]      |    :star:    | `crates/python_binding/` |
| WebAssembly[^2] |    :star:    |  `crates/wasm_binding/`  |

<small>:star: : Priority Support</small>

---

[^1]: Python support is due to CoreProject tracker and backend being written in Python
[^2]: Webassembly support is due to CoreProject's main target of streaming content in a browser, also it indirectly allows us to target [node.js](https://nodejs.org/)/[deno](https://deno.com/)/[bun](https://bun.com/) users
