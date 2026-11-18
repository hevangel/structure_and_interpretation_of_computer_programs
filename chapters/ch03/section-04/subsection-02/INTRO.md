# 3.4.2 Controlling Concurrency

## What this subsection teaches

A **serializer** wraps groups of procedures with a mutual-exclusion lock (`Mutex`) so conflicting critical sections never interleave.

Two subtleties remain even with locks:
1. Locking *reads* separately from *writes* still breaks cross-account transfers—you must serialize the whole transaction.
2. Locking `(A, then B)` while another thread locks `(B, then A)` causes **deadlock**. Assigning every account a unique numeric ID and always acquiring the smaller-ID lock first prevents cyclic waits completely.

## Connecting the concept to Rust and Python

- **In Rust:** Uses `Arc<Mutex<Account>>` paired with sorted lock acquisition (`if a.id < b.id { (a, b) } else { (b, a) }`) so deadlock is impossible even under opposing concurrent `swap(A, B)` and `swap(B, A)` threads.
- **In Python:** Uses `threading.Lock()` paired with the exact same ascending-ID lock acquisition discipline inside `with first.lock, second.lock:`.
- **Why compare both:** Rust prevents unsynchronized access **statically**; ordered lock acquisition prevents deadlock **algorithmically** in *both* languages (`serialized-transfers`).

## Exercise

Explain why two threads running `exchange(A, B)` and `exchange(B, A)` can deadlock without lock ordering, and why global ID ordering breaks the wait cycle.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
