# Full register-machine assembler, stack monitor, and recursive controllers

**Book concepts:** Section 5.1.4 and Section 5.2 (Subsections 5.2.1–5.2.4). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

A general register-machine assembler and simulator supporting arbitrary named registers, resolved controller labels, `save`/`restore` stack monitoring (`total_pushes`, `current_depth`, `max_depth`), pluggable `op` primitives, instruction execution counts, and trace logging. Both recursive factorial and double-recursive Fibonacci controllers assemble from text instructions and match analytical stack formulas.

## Implementation walkthrough

During `assemble`, label definitions record instruction indices and instructions reference registers, constants, labels, or primitive operations symbolically. Execution runs an explicit fetch-execute loop updating `pc`, tracking stack depth peaks on `save`/`restore`, and validating a step budget.

## Complexity and correctness

For recursive `fact(n)`, saving `continue` and `n` yields exact `total_pushes = max_depth = 2(n - 1)`. Double-recursive `fib(n)` executes exact tree-recursion pushes proportional to `Fib(n + 1)`.

## Run it

```sh
cargo run --bin ch05-register-simulator
cargo test --bin ch05-register-simulator
python3 examples/ch05/section-02/register-simulator/main.py
python3 -m unittest examples/ch05/section-02/register-simulator/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add breakpoint support by label name and extend the recursive `fib` controller to print `(total_pushes, max_depth)` as a function of `Fib(n)`.

[Section introduction](../../../../chapters/ch05/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
