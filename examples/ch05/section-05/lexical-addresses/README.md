# Compile variable names to lexical addresses

**Section:** 5.5 · **Source concept:** shared

## Goal

Lexical addresses identify a binding by its environment depth and slot number. The runtime no longer needs to search by string name for every access.

## Implementation walkthrough

Resolve a variable by walking frame layouts from inner to outer. Then read the value in the corresponding runtime frame using the encoded pair.

## Complexity and correctness

Compilation lookup is O(depth × frame width) in this simple implementation; a compiled runtime address lookup is O(1).

## Run and validate

```sh
cargo run --bin ch05-lexical-addresses
cargo test --bin ch05-lexical-addresses
```

## Extend the example

Create a compiler from name-based variable ASTs to lexical-address nodes and compare execution costs.

[Section introduction](../../../../chapters/ch05/section-05/INTRO.md). Source code and explanatory text are original Rust teaching material.
