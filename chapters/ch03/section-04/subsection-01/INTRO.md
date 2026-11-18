# 3.4.1 Time in Concurrent Systems

## What this subsection teaches

When two processes share a variable (`balance = 100`), a compound statement like `balance = balance + 10` is not atomic—it compiles into *(read balance → add 10 → write balance)*. If another thread interleaves a withdrawal between that read and write, one update vanishes (**lost update**).

Therefore, correctness means proving invariants hold across **all valid interleavings**, not merely the lucky schedule tested once.

## Connecting the concept to Rust and Python

- **In Rust:** Rust forbids unsynchronized shared mutation across threads (`!Send`/`!Sync` on `Rc`/`RefCell`), catching accidental data races at compile time rather than waiting for rare test flakes.
- **In Python:** Python allows multiple `threading.Thread` workers to share objects implicitly; although the GIL serializes individual VM instructions, multi-step Python reads/writes still race and lose updates cleanly across yields/switches.
- **Why compare both:** Both implementations in [`serialized-transfers`](../../../../examples/ch03/section-04/serialized-transfers/README.md) start by enumerating every distinct balance achievable under unserialized interleavings so the hazard is mathematically concrete.

## Exercise

List all possible final values of `balance = 100` if Thread 1 sets `balance = balance + 10` and Thread 2 sets `balance = balance * 2` without locking.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
