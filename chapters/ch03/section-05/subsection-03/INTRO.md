# 3.5.3 The Stream Paradigm

## What this subsection teaches

Streams open two powerful patterns impossible with finite lists:
1. **Sequence acceleration:** feeding a slowly converging series `pi/4 = 1 - 1/3 + 1/5 - ...` through **Euler's transform** (`s_{n+1} - (s_{n+1} - s_n)^2 / (s_{n-1} - 2*s_n + s_{n+1})`) and recursively iterating that transform (**a tableau**) yields 10+ digits in ~8 terms!
2. **Infinite diagonal pairs:** interleaving `pairs(S, T)` explores the infinite upper triangle cleanly without getting stuck along row 0 forever (producing Hardy–Ramanujan numbers like `1729` via weighted merge).

## Connecting the concept to Rust and Python

- **In Rust:** Uses cloned `MemoStream<f64>` handles to build `make_tableau` and diagonal `interleave`/`merge_weighted` streams cleanly without consuming earlier rows (`stream-acceleration-pairs`).
- **In Python:** Python `MemoStream` handles slice cleanly across rows of the acceleration tableau and lazily yield weighted `(i, j)` pairs via `heapq`-style or recursive merge.
- **Why compare both:** See [`stream-acceleration-pairs`](../../../../examples/ch03/section-05/stream-acceleration-pairs/README.md): watching `pi_stream` converge in 8 tableau steps shows pure functional laziness in both languages.

## Exercise

Explain why `append(row0, pairs(s.tail(), t.tail()))` would never reach `(1, 0)` on infinite streams, whereas `interleave` visits every `(i, j)` pair.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
