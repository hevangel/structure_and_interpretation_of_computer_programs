# 2.1.1 Rational Arithmetic

## What this subsection teaches

Representing a rational number `n / d` means enforcing two representation invariants: `d != 0` and canonical lowest terms (`gcd(|n|, |d|) == 1` with sign stored consistently on the numerator, so `2 / -4` normalizes to `-1 / 2`).

Once constructors (`make_rat`) and selectors (`numer`, `denom`) obey these laws, fraction arithmetic (`+`, `-`, `*`, `/`) never touches raw tuple fields directly.

## Connecting the concept to Rust and Python

- **In Rust:** A `struct Rat { numer: i64, denom: i64 }` with private fields guarantees outside code can only build normalized rationals through `Rat::new(n, d) -> Result<Rat, ...>`.
- **In Python:** A `@dataclass(frozen=True)` with normalization inside a constructor helper or `__post_init__` (using `math.gcd`) stores reduced numerator/denominator cleanly with `ValueError` on `denom == 0`.
- **Why compare both:** Python gives arbitrary-precision numerators/denominators automatically, whereas Rust's fixed-width `i64` makes reducing by `gcd` at every step crucial to avoid premature overflow.

## Exercise

Normalize `2 / -4`, `-6 / -9`, and zero `0 / 5`, then add them using [`rational-numbers`](../../../../examples/ch02/section-01/rational-numbers/README.md).

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
