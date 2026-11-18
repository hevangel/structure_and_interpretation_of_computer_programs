# 3.2.3 Frames and Local State

## What this subsection teaches

Why does `w1 = make_account(100)` keep private state safe across calls? Because creating `make_account` extends the global frame with `balance = 100` in frame `E1`, then builds closure `withdraw` whose environment pointer is **`E1`**.

Each call to `w1(25)` creates a temporary call frame `E2` whose parent is **`E1`**, mutates `balance` *inside `E1`* (`100 -> 75`), and discards `E2` when returning.

## Connecting the concept to Rust and Python

- **In Rust:** Arena frames (`Vec<Frame>`) paired with mutable `set!(env_id, name, val)` walk parent pointers to mutate the exact ancestor frame (`E1`) holding `balance`, rather than shadowing it in `E2`.
- **In Python:** Python closures distinguish *rebinding locally* (`x = ...`) from *mutating the ancestor frame cell* (`nonlocal balance; balance -= amount`) precisely for this reason!
- **Why compare both:** Look at [`cse-environment-machine`](../../../../examples/ch03/section-02/cse-environment-machine/README.md): both Rust and Python trace the shared parent frame `E1` surviving while per-call parameter frames come and go.

## Exercise

Draw the complete frame diagram for `w1 = make_account(100)`, `w2 = make_account(50)`, and calling `w1(30)`.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
