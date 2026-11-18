# 1.1.3 Evaluating Combinations

## What this subsection teaches

To evaluate a compound combination like `square(2 + 3) + square(4)`, the evaluator follows a general tree rule: evaluate the operator and every sub-expression first, then apply the procedure to the resulting argument values.

Thinking of evaluation as **percolating values upward** from the leaves of an expression tree makes complex nested calls easy to trace on paper—and highlights when evaluation order matters if any sub-expression has side effects.

## Connecting the concept to Rust and Python

- **In Rust:** Rust evaluates function arguments from left to right before calling the function; operators like `+` and `*` follow strict precedence and never hide arbitrary user overload costs without explicit trait implementations.
- **In Python:** Python likewise evaluates the callable first, then positional arguments strictly left-to-right before invoking the body (`square(5)` then `square(4)`, then `+`).
- **Why compare both:** In both languages, compound combinations obey the same recursive evaluation rule regardless of nesting depth; only short-circuiting forms (`&&`/`||` in Rust, `and`/`or`/`if-else` in Python) skip sub-expressions.

## Exercise

Draw the evaluation tree for `square(2 + 3) + square(4)` showing which values flow upward at each node.

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
