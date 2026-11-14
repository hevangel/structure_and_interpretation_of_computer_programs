# A lazy stream of prime numbers

**SICP:** Section 3.5 | **Concept shared:** shared

## Learning goal

An infinite-sequence description can be evaluated only as far as a consumer needs. Previously found primes provide enough information to test the next candidate.

## Algorithm and Rust design

Implement Iterator with a current candidate and a growing cache of known primes. Test divisibility only through sqrt(candidate) and append the next prime when discovered.

## Correctness and tradeoffs

A prime request uses incremental trial division; memory grows with the number of primes already produced. Iteration is demand-driven, not an eager infinite array.

## Run it

```sh
cargo run --bin ch03-lazy-prime-stream
cargo test --bin ch03-lazy-prime-stream
```

## Extension problem

Compare this cached-prime stream to a genuine incremental sieve with a priority queue of composites.

[Section notes](../../../../chapters/ch03/section-05/INTRO.md) · This Rust program and explanation are independently written.
