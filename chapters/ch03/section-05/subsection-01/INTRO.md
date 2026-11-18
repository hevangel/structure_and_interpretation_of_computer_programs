# 3.5.1 Streams as Delayed Lists

## What this subsection teaches

Eager `map`/`filter` pipelines materialize huge intermediate lists in memory just to grab the second prime in `[10_000..1_000_000]`.

A **stream** represents a pair `(head, delayed_tail)` where `head` is evaluated now and the rest waits inside a promise (**thunk**) until `tail(stream)` forces the next step—delivering modular pipeline style with incremental `O(1)` demand.

## Connecting the concept to Rust and Python

- **In Rust:** Simple single-pass streams correspond to Rust `Iterator` chains (`(10_000..).filter(is_prime).nth(1)`), whereas reusable SICP stream pairs wrap delayed tails in explicit thunks.
- **In Python:** Simple single-pass streams correspond to Python generator expressions (`itertools.islice((n for n in count(10_000) if is_prime(n)), 2)`), whereas shareable SICP streams wrap `(head, lazy_tail_thunk)` pairs cleanly.
- **Why compare both:** See [`lazy-streams`](../../../../examples/ch03/section-05/lazy-streams/README.md) vs [`memoized-streams-series`](../../../../examples/ch03/section-05/memoized-streams-series/README.md) to distinguish single-pass pull iterators from shareable SICP streams.

## Exercise

Trace which elements get tested when taking only the second prime above `10,000` lazily vs eagerly building the full list first.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
