# Chapter 1: Building Abstractions with Procedures

How do you manage complexity when software grows? Chapter 1 shows that computation is built from three ingredients—**primitive elements**, **means of combination**, and **means of abstraction**—turning step-by-step arithmetic into reusable procedures and higher-order combinators.

Starting from expressions and naming, the chapter develops a mental model of execution (substitution), distinguishes recursive *syntax* from iterative *processes*, classifies resource growth, and culminates with functions that accept and return other functions.

## Rust and Python perspectives

- **In Rust:** Rust separates **expressions** from **bindings**, checks types statically at compile time, and requires explicit loop/state control because tail-call optimization is not guaranteed. Passing or returning behavior uses generics (`impl Fn`), traits (`Fn`, `FnMut`, `FnOnce`), or trait objects (`Rc<dyn Fn(...) -> ...>`).
- **In Python:** Python treats every function (`def`, `lambda`) as a first-class heap object with dynamic typing and lexical closures (`nonlocal` for shared state). Like Rust, CPython does **not** perform tail-call elimination (deep recursion hits `RecursionError` around 1,000 frames), so iterative processes use `while`/`for` loops or trampolines.
- **Comparing both:** Reading both side by side exposes what is fundamental (evaluation order, invariants, orders of growth, closure capture) versus what is host-language policy (static borrowing vs garbage-collected function objects).

## Sections in this chapter

- [1.1 The Elements of Programming](section-01/INTRO.md)
- [1.2 Procedures and the Processes They Generate](section-02/INTRO.md)
- [1.3 Formulating Abstractions with Higher-Order Procedures](section-03/INTRO.md)

## How to study

1. Read each section and subsection note before inspecting the source files.
2. Predict the output, invariants, and time/space complexity on paper.
3. Compare the Rust (`main.rs`) and Python (`main.py`) implementations in the viewer (`index.html` Side-by-Side view) and run both test suites.
4. Change one representation detail or invariant and observe which tests break.

The commentary here is independently written educational material, not a reproduction of the textbook. · [Edition notes](../../docs/editions.md) · [Coverage ledger](../../COVERAGE.md)
