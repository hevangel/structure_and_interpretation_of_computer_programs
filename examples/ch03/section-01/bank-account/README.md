# Stateful bank account with invariants

**Book concepts:** Section 3.1. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

An account's history matters: withdrawals and deposits alter what future calls return.

## Implementation walkthrough

The balance is private. Deposit and withdraw are checked operations; rejected transactions must not mutate state.

## Complexity and correctness

Each operation is O(1), but reasoning about many operations requires a state-transition invariant: balance is never negative.

## Run it

```sh
cargo run --bin ch03-bank-account
cargo test --bin ch03-bank-account
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Model an account with an immutable transaction log, then derive the balance by folding over the log.

[Section introduction](../../../../chapters/ch03/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
