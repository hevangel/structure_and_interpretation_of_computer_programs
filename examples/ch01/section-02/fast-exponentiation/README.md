# Exponentiation by repeated squaring

**Book concepts:** Section 1.2. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Replace a linear number of multiplications with logarithmically many by using the parity of the exponent.

## Implementation walkthrough

Keep an accumulator, the current base, and the remaining exponent. At each step the product of the accumulator and base to the remaining exponent is invariant.

## Complexity and correctness

O(log exponent) time, O(1) extra space; arithmetic overflow is deliberately outside the small-input teaching contract.

## Run it

```sh
cargo run --bin ch01-fast-exponentiation
cargo test --bin ch01-fast-exponentiation
python3 examples/ch01/section-02/fast-exponentiation/main.py
python3 -m unittest examples/ch01/section-02/fast-exponentiation/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Extend the function to modular exponentiation without constructing huge intermediate powers.

[Section introduction](../../../../chapters/ch01/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
