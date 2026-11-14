# Prime-sum pairs using nested enumeration

**Section:** 2.2 · **Edition scope:** shared

## Big idea

Generate unordered pairs of integers, test a predicate on each pair, then retain only those whose sum is prime.

## Rust design and invariants

The outer loop chooses the larger number; the inner loop chooses smaller numbers, ensuring each pair appears once. The prime predicate is separate from enumeration.

## Complexity

O(n² sqrt n) time with simple trial division and O(n²) worst-case output storage.

## Run

```sh
cargo run --bin ch02-prime-sum-pairs
cargo test --bin ch02-prime-sum-pairs
```

## Exercise

Replace trial division with a prime sieve, then reason about the total cost.

[Undergraduate section guide](../../../../chapters/ch02/section-02/INTRO.md). This code and explanation are original pedagogical work, not a reproduction of the textbook's code.
