# 3.1.3 Costs of Assignment

## What this subsection teaches

The moment two functions share a mutable binding or two handles point to the **same** account, **aliasing** and **call order** change the answer (`f(x) + f(x) != 2 * f(x)` if `f` mutates state!).

Without assignment, two `make_account(100)` objects look interchangeable (`same == same`); *with* assignment, **identity** (`pointer equality`) matters: withdrawing from one handle updates every alias of that joint account.

## Connecting the concept to Rust and Python

- **In Rust:** Rust prevents accidental shared aliasing at compile time: either only one `&mut` owner exists, or sharing is explicitly opt-in via `Rc<RefCell<Account>>` (`joint-account`).
- **In Python:** Python aliases mutable objects freely—`a = b = Account(100)` makes `a is b` (`True`), and `a.withdraw(30)` surprises anyone who forgot `a` and `b` share identity.
- **Why compare both:** See [`joint-account`](../../../../examples/ch03/section-01/joint-account/README.md): Rust makes shared mutability loud in the type signature (`Rc<RefCell<_>>`), while Python requires discipline to avoid unintended aliasing.

## Exercise

Construct a program where `eval_arg(1) + eval_arg(2)` gives a different sum than `eval_arg(2) + eval_arg(1)` because of shared state.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
