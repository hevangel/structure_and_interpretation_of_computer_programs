# 1.2.6 Testing for Primality

## What this subsection teaches

Two contrasting ways to test primality:
- **Deterministic (`O(sqrt(n))`):** Any composite `n` has a factor `<= sqrt(n)`, so scanning divisors up to `sqrt(n)` proves primality with 100% certainty.
- **Probabilistic (`O(k log n)`):** Fermat's Little Theorem (`a^(n-1) ≡ 1 mod n`) runs dramatically faster via `expmod`, *except* rare **Carmichael numbers** (like `561` or `1729`) fool it for every coprime base. **Miller–Rabin** closes this loophole by spotting nontrivial square roots of `1 mod n` during squaring.

## Connecting the concept to Rust and Python

- **In Rust:** Rust uses `u128` widening inside `expmod` (`(acc as u128 * base as u128) % m as u128`) so squaring 64-bit residues never overflows, and deterministic LCG PRNG seeds keep tests reproducible.
- **In Python:** Python handles modular products without overflow (`(a * b) % m` or `pow(a, exp, m)`), making the Fermat vs Miller–Rabin witness logic almost purely mathematical.
- **Why compare both:** Both implementations in [`prime-testing`](../../../../examples/ch01/section-02/prime-testing/README.md) and [`fermat-miller-rabin`](../../../../examples/ch01/section-02/fermat-miller-rabin/README.md) let you directly test Carmichael numbers against both algorithms.

## Exercise

Verify why `561` passes every Fermat base `a < 561` with `gcd(a, 561) == 1`, yet triggers a nontrivial square root in Miller–Rabin.

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
