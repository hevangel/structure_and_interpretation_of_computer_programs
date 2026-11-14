# Joint account with shared local state

**SICP:** Section 3.1 | **Concept shared:** shared

## Learning goal

Share the *same* balance through two authenticated interfaces without copying the state.

## Algorithm and Rust design

A shared Rc<RefCell<Account>> owns the balance; handles carry separate passwords. Withdrawal requires authentication before mutation and checks for insufficient funds.

## Correctness and tradeoffs

Each handle is an alias of the same account. RefCell enforces borrow rules at runtime and is suitable only for single-threaded educational use.

## Run it

```sh
cargo run --bin ch03-joint-account
cargo test --bin ch03-joint-account
```

## Extension problem

Build a thread-safe version with Arc<Mutex<_>> and an explicit password-management policy.

[Section notes](../../../../chapters/ch03/section-01/INTRO.md) · This Rust program and explanation are independently written.
