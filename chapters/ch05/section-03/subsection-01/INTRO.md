# 5.3.1 Memory as Vectors

## What this subsection teaches

A low-level machine has no heap `Box` or Python objects—only flat arrays of words! Representing LISP/Scheme pairs inside two parallel arrays (**`the_cars`** and **`the_cdrs`**) encodes pointers as tagged integers:
- `Pair(index)` points to slot `index` in both `the_cars[index]` and `the_cdrs[index]`
- `Int(n)`, `Sym(id)`, `Nil` store immediate scalars inline.

Crucially, `set_cdr!(p, q)` simply mutates `the_cdrs[p] = q`—just one vector write!

## Connecting the concept to Rust and Python

- **In Rust:** `struct Memory { the_cars: Vec<Word>, the_cdrs: Vec<Word>, new_cars: Vec<Word>, new_cdrs: Vec<Word>, free: usize }` models SICP's flat vector heap directly (`stop-and-copy-gc`).
- **In Python:** Mirrored cleanly with plain Python lists `the_cars: list[Word]`, `the_cdrs: list[Word]` storing immutable `Word` dataclasses (`Pair(idx)`, `Int(n)`, `Nil`, `BrokenHeart(idx)`).
- **Why compare both:** See [`stop-and-copy-gc`](../../../../examples/ch05/section-03/stop-and-copy-gc/README.md): modeling pairs as typed index arrays reveals the concrete machine layout behind dynamic cons cells in both languages.

## Exercise

Draw the `the_cars` / `the_cdrs` table entries produced by `(let ((x (list 1 2))) (cons x x))` (a shared DAG!).

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
