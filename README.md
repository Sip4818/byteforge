# Byteforge

[![Rust](https://img.shields.io/badge/rust-2024_edition-orange.svg)](https://www.rust-lang.org/)
[![CI](https://github.com/Sip4818/byteforge/actions/workflows/ci.yml/badge.svg)](https://github.com/Sip4818/byteforge/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Byteforge** is a fast, lightweight Byte-Level Byte-Pair Encoding (BPE) tokenizer implemented from scratch in idiomatic Rust (2024 edition).

It forms the foundational tokenization subsystem for high-performance LLM preprocessing and inference pipelines.

---

## Key Features

- **Byte-Level Base Vocabulary:** Initializes with all 256 individual byte values ($0\text{–}255$), guaranteeing zero out-of-vocabulary (OOV) tokens for arbitrary Unicode, emojis, numbers, or binary data.
- **Pure Idiomatic Rust:** Zero unsafe code, built with Rust 2024 edition standards.
- **Round-Trip Fidelity:** Guarantees bit-exact reconstruction of original text via lossless byte decoding: $\text{decode}(\text{encode}(text)) == text$.
- **Automated Benchmarking:** Integrated [Criterion.rs](https://github.com/bheisler/criterion.rs) benchmark suite measuring throughput across training, encoding, and decoding phases.
- **Strict Quality CI:** Fully verified through automated GitHub Actions running `cargo fmt`, `cargo clippy` (`-D warnings`), unit tests, and benchmarks.

---

## Architecture Overview

Byteforge is organized into modular pipelines under [`src/`](src/):

| Module | Description |
| :--- | :--- |
| [`src/preprocessing.rs`](src/preprocessing.rs) | Pre-tokenization and chunking of raw text into character-aligned byte ID buffers. |
| [`src/training.rs`](src/training.rs) | Pair-frequency counting across chunks and iterative vocabulary expansion via learned BPE merges. |
| [`src/encoding.rs`](src/encoding.rs) | Sequential merge rule application transforming byte arrays into learned token IDs. |
| [`src/decoding.rs`](src/decoding.rs) | Lookup and reconstruction of raw byte tokens from token IDs back to valid UTF-8 strings. |
| [`src/tokenizer.rs`](src/tokenizer.rs) | High-level `BpeTokenizer` interface exposing `train`, `encode`, and `decode`. |

---

## Quickstart

Add `byteforge` to your `Cargo.toml`:

```toml
[dependencies]
byteforge = { path = "../byteforge" }
```

### Usage Example

```rust
use byteforge::tokenizer::BpeTokenizer;

fn main() {
    let corpus = "The quick brown fox jumps over the lazy dog. Rust provides memory safety and speed.";
    let max_vocab_size = 350;

    // 1. Train the tokenizer on a corpus
    let tokenizer = BpeTokenizer::train(corpus, max_vocab_size);

    // 2. Encode text into token IDs
    let input_text = "Rust provides safety!";
    let tokens: Vec<u32> = tokenizer.encode(input_text);
    println!("Tokens: {:?}", tokens);

    // 3. Decode token IDs back into string
    let decoded_text: String = tokenizer.decode(&tokens);
    assert_eq!(input_text, decoded_text);
    println!("Decoded: {}", decoded_text);
}
```

---

## Development & Verification

Run the test suite:
```bash
cargo test --all-features
```

Run linter and formatting checks:
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

Run Criterion performance benchmarks:
```bash
cargo bench
```

---

## Serving Engine Roadmap

Byteforge is being extended to serve as the tokenizer frontend for a high-concurrency LLM serving engine featuring PagedAttention and continuous batching. See the execution plan in [**`docs/ROADMAP_DECEMBER_2026.md`**](docs/ROADMAP_DECEMBER_2026.md).

---

## License

Dual-licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
