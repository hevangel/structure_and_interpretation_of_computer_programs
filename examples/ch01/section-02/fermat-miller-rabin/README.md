# Fermat primality, Carmichael numbers, and Miller–Rabin

**Book concepts:** Section 1.2 (Subsection 1.2.6). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Fermat's little theorem is a necessary condition for primality, not a sufficient one: Carmichael numbers pass `a^n = a (mod n)` for every coprime base `a < n`. The Miller–Rabin test catches these composites by inspecting each squaring step for non-trivial square roots of 1 modulo `n`.

## Implementation walkthrough

`expmod` uses repeated squaring with widened intermediate products (`u128` in Rust, arbitrary-precision `int` in Python) to prevent overflow. `all_coprime_fermat_pass` exhaustively verifies Carmichael witnesses (`561`, `1105`, `1729`, `2465`, `2821`, `6601`). `_expmod_checked` / `expmod_checked` signals `0` whenever a non-trivial square root of `1 (mod n)` appears during `a^(n-1)`.

## Complexity and correctness

Each modular exponentiation performs O(lg n) modular multiplications. Unlike Fermat's test, at least 3/4 of bases in `1..n` witness compositeness for any odd composite `n`, and deterministic small prime bases are exact across all integers up to 2^64.

## Run it

```sh
cargo run --bin ch01-fermat-miller-rabin
cargo test --bin ch01-fermat-miller-rabin
python3 examples/ch01/section-02/fermat-miller-rabin/main.py
python3 -m unittest examples/ch01/section-02/fermat-miller-rabin/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Find the smallest witness base `a >= 2` for which each Carmichael number fails `miller_rabin_witness`, and verify that base 2 already catches `561`.

[Section introduction](../../../../chapters/ch01/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
