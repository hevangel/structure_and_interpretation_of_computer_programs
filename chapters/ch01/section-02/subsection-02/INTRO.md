# 1.2.2 Tree Recursion

## What this subsection teaches

When a function splits into two or more recursive subcalls—such as `fib(n - 1) + fib(n - 2)` or splitting coin change into *with-first-coin* + *without-first-coin*—execution traces a **call tree**.

Tree recursion often mirrors a mathematical definition transparently, but if subtrees overlap (like `fib(3)` inside both `fib(5)` and `fib(4)`), runtime explodes exponentially (`O(phi^n)`) even though only `O(n)` distinct subproblems exist!

## Connecting the concept to Rust and Python

- **In Rust:** Naive recursive `match` trees make exponential blowup easy to spot; pairing them with a bottom-up `Vec` DP table or iterative state tuple reduces work dramatically while keeping exact results.
- **In Python:** Similarly, recursive `def` trees are effortless to sketch, and adding `@functools.lru_cache` or a bottom-up list collapses redundant subtree recomputation from exponential to linear/polynomial.
- **Why compare both:** Both languages show the same contrast in [`fibonacci-processes`](../../../../examples/ch01/section-02/fibonacci-processes/README.md) and [`counting-change`](../../../../examples/ch01/section-02/counting-change/README.md): clarity of specification vs avoiding duplicate work.

## Exercise

Draw the recursion tree for `fib(5)` and circle every repeated evaluation of `fib(2)` and `fib(1)`.

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
