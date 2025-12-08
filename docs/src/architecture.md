---
icon: lucide/house
---

## Encoder

Our current encoder is based on [tagged-union](https://en.wikipedia.org/wiki/Tagged_union) type dispatch via pattern matching

```mermaid
flowchart TD
    Start --> A["encode_bencode(value)"]
    A --> B{"BencodeValue type?"}

    B -- "Int(n)" --> C1["Call encode_integer(n)"]
    C1 --> D1["Return Vec<u8> of integer"]

    B -- "Str(s)" --> C2["Call encode_string(s)"]
    C2 --> D2["Return Vec<u8> of string"]

    B -- "List(l)" --> C3["Call encode_list(l) (recursively encodes elements)"]
    C3 --> D3["Return Vec<u8> of concatenated list elements with 'l'...'e'"]

    B -- "Dict(d)" --> C4["Call encode_dict(d) (recursively encodes keys & values)"]
    C4 --> D4["Return Vec<u8> of dictionary with 'd'...'e'"]


```

## Decoder

Our current architecture is based on [recursive-descent parser](https://en.wikipedia.org/wiki/Recursive_descent_parser)

```mermaid
flowchart TD
    Start --> A
    A["decode_bencode(data)"] --> B{"Is data empty?"}
    B -- "Yes" --> C1["Return Err('Empty input')"]
    B -- "No" --> D["Check first byte"]

    D --> E1{"First byte type?"}
    E1 -- "Digit" --> F1["Call decode_string(data)"]
    F1 --> G1["Wrap as BencodeValue::Str"]
    G1 --> H1["Return (Str, rest)"]

    E1 -- "i" --> F2["Call decode_integer(data)"]
    F2 --> G2["Wrap as BencodeValue::Int"]
    G2 --> H2["Return (Int, rest)"]

    E1 -- "l" --> F3["Call decode_list(data)"]
    F3 --> G3["Wrap as BencodeValue::List"]
    G3 --> H3["Return (List, rest)"]

    E1 -- "d" --> F4["Call decode_dictionary(data)"]
    F4 --> G4["Wrap as BencodeValue::Dict"]
    G4 --> H4["Return (Dict, rest)"]

    %% E1 -- "Other" --> C2["Return Err('Unknown type prefix')"]

```

!!! info "Future"

    The next generation of the parser might be based on `Zero copy, cursor based Parser`, i will look into it after i create other parts of the project
