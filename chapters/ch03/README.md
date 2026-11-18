# Chapter 3: Modularity, Objects, and State

Once an object can change its internal state over time, substitution no longer explains program behavior: two calls with identical text can return different answers.

Chapter 3 explores local state (`set!`), the **environment model** of linked frames, mutable graph structures (queues, tables, digital-circuit wires, constraint propagators), **concurrency & deadlock avoidance**, and **lazy memoized streams** as a timeless functional alternative to mutable objects.

## Rust and Python perspectives

- **In Rust:** Rust makes mutation and aliasing explicit via `&mut self`, `Rc<RefCell<T>>` (single-threaded runtime borrow checks), `Weak<T>` cycle breaking, and `Arc<Mutex<T>>` for thread-safe serialization.
- **In Python:** Python objects and variables use mutable references by default (`nonlocal` bindings, `list`/`dict` aliasing, garbage-collected cycles). Shared concurrency uses `threading.Lock` (or lock ordering by account ID) and streams map naturally to cached thunks or lazy generators.
- **Comparing both:** Rust forces you to prove or explicitly tag every shared mutable alias and thread boundary; Python lets aliasing happen implicitly, making it easy to write mutable graphs—and just as easy to hit subtle aliasing bugs unless invariants are stated explicitly.

## Sections in this chapter

- [3.1 Assignment and Local State](section-01/INTRO.md)
- [3.2 The Environment Model of Evaluation](section-02/INTRO.md)
- [3.3 Modeling with Mutable Data](section-03/INTRO.md)
- [3.4 Concurrency: Time Is of the Essence](section-04/INTRO.md)
- [3.5 Streams](section-05/INTRO.md)

## How to study

1. Read each section and subsection note before inspecting the source files.
2. Predict the output, invariants, and time/space complexity on paper.
3. Compare the Rust (`main.rs`) and Python (`main.py`) implementations in the viewer (`index.html` Side-by-Side view) and run both test suites.
4. Change one representation detail or invariant and observe which tests break.

The commentary here is independently written educational material, not a reproduction of the textbook. · [Edition notes](../../docs/editions.md) · [Coverage ledger](../../COVERAGE.md)
