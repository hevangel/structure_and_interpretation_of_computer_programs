# 3.1 Assignment and Local State · [Chapter 3](../README.md)

## The big idea

Introducing assignment (`set!`, field mutation) breaks the substitution model forever: `withdraw(25)` followed by `withdraw(25)` returns `75`, then `50`—two identical calls produce different answers because **time and internal state** entered the picture.

With state comes the subtle distinction between **value equality** (two objects happen to hold `100` right now) and **object identity** (both references point to the *exact same* stateful bank account, so mutating one changes the other).

## What you should focus on

Focus on ****encapsulated local state, referential transparency loss, value equality vs object identity (`eq?`/`is`), and stateful vs pure Monte Carlo sampling****. Always ask which parts of a system depend on *when* operations run versus *what* values they compute.

## Connecting the concept to Rust and Python

- **In Rust:** Mutation is always explicit: `&mut self` grants exclusive temporary write access (checked at compile time), while shared handles to one joint account require interior mutability (`Rc<RefCell<Account>>` single-threaded or `Arc<Mutex<Account>>` across threads).
- **In Python:** Variables inside closures capture mutable state via `nonlocal`, objects share identity via references (`a is b` vs `a == b`), and multiple callers alias the same instance by default.
- **Key contrast:** Rust makes **shared mutation** visible in the type (`Rc<RefCell<_>>`) while Python lets two variables alias the same mutable instance silently—contrast both in `bank-account`, `joint-account`, and `monte-carlo-pi`.

## Subsections

- [3.1.1 Local State Variables](subsection-01/INTRO.md)
- [3.1.2 Benefits of Assignment](subsection-02/INTRO.md)
- [3.1.3 Costs of Assignment](subsection-03/INTRO.md)

## Runnable examples in this section

- [3.1 Stateful bank account with invariants](../../../examples/ch03/section-01/bank-account/README.md)
- [3.1 Joint account with shared local state](../../../examples/ch03/section-01/joint-account/README.md)
- [3.1 Monte Carlo simulation with explicit pseudo-random state](../../../examples/ch03/section-01/monte-carlo-pi/README.md)

## Check your understanding

1. Why does substitution fail once a closure mutates a private `balance` variable?
2. How does a stateful `make_rand` generator differ from passing an explicit `seed` cleanly through every Monte Carlo step?

**Experiment:** Construct two independent counters and one aliased pair of handles sharing the same underlying state; test which pairs stay synchronized after a mutation.

[Chapter 3 overview](../README.md) · [Edition notes](../../../docs/editions.md)
