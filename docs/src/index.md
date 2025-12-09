---
title: 'Introduction'
icon: lucide/rocket
---

<div align="center">
    <img src="./img/logo.svg" loading="lazy" alt="Logo"  />
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

| Language        | Support Type |                                                    Location                                                    |
| --------------- | :----------: | :------------------------------------------------------------------------------------------------------------: |
| Rust            |    :star:    |                                                      Core                                                      |
| Python[^1]      |    :star:    | [`crates/python_binding/`](https://github.com/coreproject-moe/bencode-rust/tree/master/crates/python_bindings) |
| WebAssembly[^2] |    :star:    |   [`crates/wasm_binding/`](https://github.com/coreproject-moe/bencode-rust/tree/master/crates/wasm_bindings)   |
| Node.js[^3]     |   :rocket:   |                                                       -                                                        |
| Deno[^4]        |   :rocket:   |                                                       -                                                        |
| Bun[^5]         |   :rocket:   |                                                       -                                                        |

<small>

-   :star: : Support via native binding
-   :rocket: : Support via WebAssembly

</small>

---

[^1]: Python support is due to CoreProject tracker and backend being written in Python
[^2]: Webassembly support is due to CoreProject's main target of streaming content in a browser, also it indirectly allows us to target [node.js](https://nodejs.org/)/[deno](https://deno.com/)/[bun](https://bun.com/) users
[^3]: Node.js natively [supports](https://nodejs.org/en/learn/getting-started/nodejs-with-webassembly) wasm
[^4]: Deno natively [supports](https://docs.deno.com/runtime/reference/wasm/) wasm
[^5]: Bun natively [supports loading wasm](https://bun.com/docs/runtime/file-types)
