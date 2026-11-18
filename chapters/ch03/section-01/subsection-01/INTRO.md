# 3.1.1 Local State Variables

## What this subsection teaches

A closure that holds a private mutable binding—like `make_withdraw(balance)`—remembers its own history across calls (`w1(25) -> 75`, `w1(25) -> 50`).

Calling `make_withdraw(100)` a *second* time allocates a completely fresh state cell (`w2`), so `w1` and `w2` behave like two separate independent objects even though both execute the exact same code.

## Connecting the concept to Rust and Python

- **In Rust:** `struct BankAccount { balance: i64 }` with `&mut self` methods makes state transitions explicit; a `FnMut` closure captures a private local `let mut balance = 100;` cleanly.
- **In Python:** A factory function defines `def withdraw(amount): nonlocal balance; ...` so each call to `make_withdraw(100)` allocates an isolated closure cell for `balance` (or uses a class instance).
- **Why compare both:** Compare [`bank-account`](../../../../examples/ch03/section-01/bank-account/README.md): Rust `struct` methods make mutation signatures (`&mut self`) visible to callers, whereas `FnMut` and Python `nonlocal` hide mutable state inside closure cells.

## Exercise

Create two independent counters (`c1`, `c2`), interleave increments/withdrawals, and prove their histories never cross.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
