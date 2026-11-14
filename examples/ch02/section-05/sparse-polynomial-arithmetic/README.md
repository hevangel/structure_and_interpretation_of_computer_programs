# Sparse polynomial addition and multiplication

**Section:** 2.5 · **Edition scope:** shared

## Big idea

Polynomials are another form of generic data: operations should work over terms rather than be hard-coded for one degree.

## Rust design and invariants

Use a BTreeMap from exponent to coefficient. Addition merges terms; multiplication distributes every pair of terms and combines equal exponents. Zero coefficients are removed from the canonical representation.

## Complexity

For a and b nonzero terms, multiplication takes O(ab log(ab)) time with a balanced map and O(ab) worst-case term storage.

## Run

```sh
cargo run --bin ch02-sparse-polynomial-arithmetic
cargo test --bin ch02-sparse-polynomial-arithmetic
```

## Exercise

Make the coefficient type generic over a ring trait, and explore how zero detection changes.

[Undergraduate section guide](../../../../chapters/ch02/section-05/INTRO.md). This code and explanation are original pedagogical work, not a reproduction of the textbook's code.
