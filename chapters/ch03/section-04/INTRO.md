# 3.4 Concurrency: Time Is of the Essence · [Chapter 3](../README.md)

## The big idea

When multiple independent threads read and write shared state concurrently, operations **interleave** unpredictably: even simple `x = x + 1` splits into *read → add → write*, letting two concurrent increments lose an update.

**Serializers (mutexes)** enforce mutual exclusion around critical sections so invariants like total bank balance (`a + b == constant`) survive arbitrary scheduling—until two threads try to lock `(A then B)` while the other locks `(B then A)`, causing **deadlock**, solved by global resource ordering (`min(id1, id2)` first).

## What you should focus on

Focus on ****race interleavings, critical-section atomicity, serializer/mutex abstractions, and deadlock prevention via total lock ordering****. Always ask which parts of a system depend on *when* operations run versus *what* values they compute.

## Connecting the concept to Rust and Python

- **In Rust:** Rust's `Send`/`Sync` traits prevent data races **at compile time**: cross-thread shared mutation requires `Arc<Mutex<T>>` (or `RwLock<T>`). However, *deadlocks* are still logic bugs—lock ordering (`id_a < id_b`) remains the programmer's responsibility.
- **In Python:** Python threads share objects freely (`threading.Lock`) under the GIL (which serializes individual bytecode ops, **not** multi-step read-modify-write transactions like transfers!), so explicit `with lock:` blocks and ordered locking remain mandatory.
- **Key contrast:** Compare `serialized-transfers`: Rust stops unsynchronized cross-thread sharing at compile time; Python relies on programmer-inserted `threading.Lock` guards—and in *both* languages, acquiring locks in ascending account ID order eliminates swap deadlocks.

## Subsections

- [3.4.1 Time in Concurrent Systems](subsection-01/INTRO.md)
- [3.4.2 Controlling Concurrency](subsection-02/INTRO.md)

## Runnable examples in this section

- [3.4 Concurrency and serialized transfers](../../../examples/ch03/section-04/serialized-transfers/README.md)

## Check your understanding

1. What distinct balance outcomes are possible when `balance = balance + 10`, `balance = balance - 20`, and `balance = balance * 2` interleave without serialization?
2. Why does Python's GIL **not** make a two-step bank transfer thread-safe without a `Lock`?

**Experiment:** Enumerate all unserialized interleavings in `serialized-transfers` and run concurrent opposing swaps `(A->B)` + `(B->A)` to verify ordered locking never deadlocks.

[Chapter 3 overview](../README.md) · [Edition notes](../../../docs/editions.md)
