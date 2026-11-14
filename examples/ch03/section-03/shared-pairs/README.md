# Aliasing with mutable pairs and weak references

**SICP:** Section 3.3 | **Concept shared:** shared

## Learning goal

Show how two handles can point to the same mutable node, while a weak back-link avoids making a reference-counting cycle.

## Algorithm and Rust design

Rc manages shared ownership, RefCell makes mutation explicit, and Weak represents a non-owning relation. Editing through one handle is visible through another.

## Correctness and tradeoffs

Rc<RefCell<T>> is single-threaded. Cyclic strong references leak under reference counting, so back-links that should not own objects use Weak.

## Run it

```sh
cargo run --bin ch03-shared-pairs
cargo test --bin ch03-shared-pairs
```

## Extension problem

Replace the weak back-link with a strong one and explain which reference counts would prevent reclamation.

[Section notes](../../../../chapters/ch03/section-03/INTRO.md) · This Rust program and explanation are independently written.
