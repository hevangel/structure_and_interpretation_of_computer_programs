# 2.1.3 What Is Data?

## What this subsection teaches

What actually makes `(cons a b)`, `car`, and `cdr` a pair? **Not** two adjacent words of RAM—any representation satisfying `car(cons(a, b)) == a` and `cdr(cons(a, b)) == b` *is* a valid pair!

Even pure functions can represent pairs (`pair(x, y) = m -> m(x, y)`, `first(p) = p((x, y) -> x)`) and natural numbers (`Church numerals`: `zero = f -> x -> x`, `two = f -> x -> f(f(x))`).

## Connecting the concept to Rust and Python

- **In Rust:** Encoding procedural pairs and Church numerals in Rust uses higher-ranked/generic closures (`Rc<dyn Fn(...)>` or generic traits) because functions capturing arbitrary types need uniform signatures.
- **In Python:** In Python `pair = lambda x, y: (lambda m: m(x, y))` and `zero = lambda f: (lambda x: x)` are direct two-line closures with zero type annotation needed.
- **Why compare both:** See [`functional-pairs`](../../../../examples/ch02/section-01/functional-pairs/README.md) and [`church-numerals`](../../../../examples/ch02/section-01/church-numerals/README.md): Python reads almost identically to lambda calculus, while Rust reveals every capture type.

## Exercise

Express `add(m, n)` and `mul(m, n)` directly on Church numerals without converting back to integers until printing.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
