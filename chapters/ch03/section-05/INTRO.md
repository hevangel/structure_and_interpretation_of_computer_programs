# 3.5 Streams · [Chapter 3](../README.md)

## The big idea

**Streams** model changing quantities *without* assignment: treat an entire time history or infinite sequence (all primes, formal power series `e^x`, `sin x`, `cos x`, even an ODE feedback loop) as a single immutable sequence whose tail is **delayed and memoized**.

Because each tail computes at most once on demand, `fibs = 0, 1, add_streams(fibs, tail(fibs))` and `sieve(integers_from(2))` compute infinite sequences cleanly without exponential re-evaluation.

## What you should focus on

Focus on ****delayed-tail memoization (`delay`/`force`), infinite streams & sieve, formal power series (`sin`/`cos`/`exp`), tableau sequence acceleration, and delayed-feedback ODE solvers****. Always ask which parts of a system depend on *when* operations run versus *what* values they compute.

## Connecting the concept to Rust and Python

- **In Rust:** Built-in `Iterator` is *single-pass unmemoized* pull-laziness (`zip`/`map`/`take`), which works great for linear pipelines (`lazy-streams`)—while true self-referential SICP streams (`fibs`, `sin/cos` power series, `solve`) require explicit memoized thunks (`Rc<RefCell<StreamNode<T>>>`, as in `memoized-streams-series`).
- **In Python:** Python generators (`yield`) are also *single-pass unmemoized* iterators—so recursive feedback streams need either an explicit `MemoStream` thunk caching `(head, tail_fn)` or `itertools.tee` cleanly managed on finite prefixes.
- **Key contrast:** Contrast single-pass Rust `Iterator` / Python generators against full memoized `Stream` cells in `memoized-streams-series` and `stream-acceleration-pairs` to see why memoizing forced tails is crucial for self-referential streams.

## Subsections

- [3.5.1 Streams as Delayed Lists](subsection-01/INTRO.md)
- [3.5.2 Infinite Streams](subsection-02/INTRO.md)
- [3.5.3 The Stream Paradigm](subsection-03/INTRO.md)
- [3.5.4 Streams and Delayed Evaluation](subsection-04/INTRO.md)
- [3.5.5 Functional Modularity and Objects](subsection-05/INTRO.md)

## Runnable examples in this section

- [3.5 Lazy streams with Rust iterators](../../../examples/ch03/section-05/lazy-streams/README.md)
- [3.5 Numerical integration as a lazy stream](../../../examples/ch03/section-05/stream-integration/README.md)
- [3.5 A lazy stream of prime numbers](../../../examples/ch03/section-05/lazy-prime-stream/README.md)
- [3.5 Signal processing with adjacent stream samples](../../../examples/ch03/section-05/zero-crossings/README.md)
- [3.5 Memoized lazy streams, sieve, and formal power series](../../../examples/ch03/section-05/memoized-streams-series/README.md)
- [3.5 Accelerated streams, infinite pairs, and delayed-feedback ODE solver](../../../examples/ch03/section-05/stream-acceleration-pairs/README.md)

## Check your understanding

1. What happens to runtime if `add_streams(fibs, fibs.tail())` does **not** memoize forced tails?
2. How does Euler/tableau stream acceleration converge on the alternating Leibniz series for `pi/4` in ~8 terms instead of thousands?

**Experiment:** Count thunk evaluations in `memoized-streams-series` with memoization vs without, and check `solve` on `dy/dt = y` in `stream-acceleration-pairs`.

[Chapter 3 overview](../README.md) · [Edition notes](../../../docs/editions.md)
