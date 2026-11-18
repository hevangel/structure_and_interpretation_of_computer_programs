# Rational numbers as an abstract data type

**Book concepts:** Section 2.1. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Treat a rational number as an abstract mathematical value, not a pair clients can manipulate arbitrarily.

## Implementation walkthrough

The constructor normalizes signs and reduces by the gcd. Private fields guarantee that all instances remain in canonical form.

## Complexity and correctness

Construction is dominated by gcd, O(log min(|numerator|,denominator)). Arithmetic here assumes small values to avoid i64 overflow.

## Run it

```sh
cargo run --bin ch02-rational-numbers
cargo test --bin ch02-rational-numbers
python3 examples/ch02/section-01/rational-numbers/main.py
python3 -m unittest examples/ch02/section-01/rational-numbers/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Implement checked add and mul that return a Result instead of overflowing.

[Section introduction](../../../../chapters/ch02/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
