# 3.5.2 Infinite Streams

## What this subsection teaches

An infinite sequence—integers from `1`, Fibonacci numbers, prime numbers by Eratosthenes' **sieve**, or formal **power series** coefficients `c0 + c1 x + c2 x^2 + ...`—fits in finite memory because only a finite prefix is ever forced.

Implicit definitions like `integers = cons(1, add_streams(ones, integers))` or `exp_series = cons(1, integrate_series(exp_series))` work as directly as finite recurrences!

## Connecting the concept to Rust and Python

- **In Rust:** Self-referential streams require heap-shared memoized cells (`Rc<RefCell<StreamNode<T>>>`) because a Rust `Iterator` consumes `self` in one direction (`memoized-streams-series`, `lazy-prime-stream`).
- **In Python:** Self-referential generators require an explicit `MemoStream` thunk node (wrapping `head` + cached `tail()` callable) since a bare Python generator raises or exhausts if iterated twice from the same point.
- **Why compare both:** Compare [`lazy-prime-stream`](../../../../examples/ch03/section-05/lazy-prime-stream/README.md) and [`memoized-streams-series`](../../../../examples/ch03/section-05/memoized-streams-series/README.md): power-series multiplication and inversion look nearly identical in Rust and Python once `MemoStream` caches tails.

## Exercise

Verify the power-series identity `sin(x)^2 + cos(x)^2 == 1` up to degree 12 using `integrate_series` without calling `math.sin`.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
