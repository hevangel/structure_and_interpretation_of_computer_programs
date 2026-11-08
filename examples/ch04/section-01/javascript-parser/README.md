# Expression parsing with operator precedence

**Book concepts:** Section 4.1. **Edition applicability:** 2022-javascript.

## What this demonstrates

Represent a text program as tokens and then an abstract syntax tree before evaluating it.

## Implementation walkthrough

The tiny Pratt parser distinguishes prefix atoms from infix operators. Binding powers make multiplication bind more strongly than addition, and parentheses override defaults.

## Complexity and correctness

Tokenizing and parsing are O(n) for this restricted grammar. The parser rejects unsupported characters but uses assertions rather than production diagnostics.

## Run it

```sh
cargo run --bin ch04-javascript-parser
cargo test --bin ch04-javascript-parser
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add subtraction and unary minus; explain why token precedence alone is not enough.

[Section introduction](../../../../chapters/ch04/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
