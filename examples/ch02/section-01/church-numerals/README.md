# Church numerals as higher-order functions

**Book concepts:** Section 2.1. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Encode a natural number as a function that applies another function that many times.

## Implementation walkthrough

A boxed function receives an operation and initial value. Successor composes one additional application rather than storing an integer field.

## Complexity and correctness

Applying the numeral n takes O(n) function calls; this representation teaches that data can be represented entirely by behavior.

## Run it

```sh
cargo run --bin ch02-church-numerals
cargo test --bin ch02-church-numerals
python3 examples/ch02/section-01/church-numerals/main.py
python3 -m unittest examples/ch02/section-01/church-numerals/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Implement Church addition and multiplication and compare function application counts.

[Section introduction](../../../../chapters/ch02/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
