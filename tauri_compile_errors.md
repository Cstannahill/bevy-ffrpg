# Rust Compilation Fixes for Tauri Projects

This document summarizes common build errors and how to resolve them.

## Missing `num_tokens`
`num_tokens` is not exported from `tiktoken-rs`. Replace calls to `num_tokens(text)` with:

```rust
let bpe = get_bpe_from_model("gpt2").unwrap();
let num_tokens = bpe.encode_with_special_tokens(text).len();
```

## Unresolved `mime` crate
Either add the `mime` crate with `cargo add mime` and use constants such as `mime::APPLICATION`, or match on the mime type string directly like `"application/pdf"`.

## Private `payload` module in `qdrant-client`
Access payload data through public structs such as `PointStruct` and `Value`:

```rust
use qdrant_client::qdrant::{PointStruct, Value};
```

Build a payload as a `HashMap<String, Value>` and pass it to `PointStruct`.

## Using `and_then` on `HashMap`
`HashMap` does not implement `and_then`. Instead, use `get`:

```rust
if let Some(value) = map.get(key) {
    // handle value
}
```

## Unused `Uuid`
Remove unused imports or use them, e.g.:

```rust
let id = Uuid::new_v4().to_string();
```
