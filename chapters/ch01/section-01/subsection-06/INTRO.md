# 1.1.6 Conditionals and Predicates

## What this subsection teaches

A **predicate** returns a truth value (`true`/`false`), and a **conditional** selects *one* branch to evaluate based on that predicate.

Crucially, conditionals must **not** evaluate both branches eagerly—otherwise a guarded recursive base case (`if n == 0 { 1 } else { n * fact(n - 1) }`) or a division guard (`x != 0 and y / x > 2`) would never stop or would crash.

## Connecting the concept to Rust and Python

- **In Rust:** `if cond { a } else { b }`, `match`, `&&`, and `||` are built-in short-circuiting constructs; wrapping a branch inside a normal `fn` call loses short-circuiting because Rust evaluates arguments eagerly.
- **In Python:** `if/elif/else`, conditional expressions (`a if cond else b`), `and`, and `or` short-circuit identically; a regular `def my_if(c, a, b):` would force both `a` and `b` before branching.
- **Why compare both:** In both Rust and Python, lazy branch selection is a special property of syntax—which is why Section 4.2 introduces lazy thunks to make user-defined functions behave like conditionals.

## Exercise

Implement `abs_val(x)` and a zero-guarded division predicate, and test negative edge cases like `i64::MIN` in Rust vs arbitrary-precision `-x` in Python.

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
