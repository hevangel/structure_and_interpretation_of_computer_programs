# Analyzing evaluator (separating syntax analysis from execution)

**Book concepts:** Section 4.1 (Subsection 4.1.7). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Separating syntactic analysis (`analyze`) from runtime execution compiles an AST once into a tree of execution closures (`ExecFn`), eliminating repeated AST decoding inside recursive loops and expanding derived `let` forms at analysis time (Exercise 4.22).

## Implementation walkthrough

`analyze` traverses every expression node once, recursively analyzing subexpressions and returning a closure taking only `(env, counters)`. Execution closures (`If`, `Lambda`, `Let`, `Call`) run directly over pre-analyzed sub-closures while explicit `analyze_steps` and `exec_steps` counters prove zero AST re-traversal occurs while computing `fact(10)`.

## Complexity and correctness

Syntactic dispatch cost drops from `O(steps * AST_work)` to `O(AST_nodes)` upfront plus `O(1)` closure invocation per execution step.

## Run it

```sh
cargo run --bin ch04-analyzing-evaluator
cargo test --bin ch04-analyzing-evaluator
python3 examples/ch04/section-01/analyzing-evaluator/main.py
python3 -m unittest examples/ch04/section-01/analyzing-evaluator/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Benchmark a naive metacircular evaluator against `analyze` on a tight 1,000-iteration tail-recursive sum loop.

[Section introduction](../../../../chapters/ch04/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
