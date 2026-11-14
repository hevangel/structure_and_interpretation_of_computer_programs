# Horner's rule as a fold

**Section:** 2.2 · **Edition scope:** shared

## Big idea

Polynomial evaluation is a conventional sequence accumulation problem. Horner's rule exposes a computation pattern independent of polynomial degree.

## Rust design and invariants

Store coefficients from constant term upward. Process them in reverse order and repeatedly multiply the accumulator by x before adding the next coefficient.

## Complexity

O(n) arithmetic operations and O(1) auxiliary memory. This avoids separately forming high powers of x.

## Run

```sh
cargo run --bin ch02-horner-evaluation
cargo test --bin ch02-horner-evaluation
```

## Exercise

Evaluate the same polynomial naively and compare the multiplication counts.

[Undergraduate section guide](../../../../chapters/ch02/section-02/INTRO.md). This code and explanation are original pedagogical work, not a reproduction of the textbook's code.
