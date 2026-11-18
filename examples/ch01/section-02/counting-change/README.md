# Counting change, Pascal's triangle, and O(lg n) Fibonacci

**Book concepts:** Section 1.2 (Subsections 1.2.2 and 1.2.4). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Tree-recursive decomposition directly mirrors the recurrence relation for making change and Pascal's triangle, while dynamic programming eliminates overlapping subproblems and squaring a linear transformation computes Fibonacci numbers in logarithmic time.

## Implementation walkthrough

`count_change_tree` branches on either omitting the first coin or subtracting one of that coin, while `count_change_dp` accumulates combinations in a one-dimensional table. `pascal` computes binomial coefficients recursively. `fib_log` applies Exercise 1.19's transformation `T_{p,q}` whose square is `T_{p^2 + q^2, 2pq + q^2}`.

## Complexity and correctness

The tree-recursive change counter avoids duplicate permutations by fixing coin order, at exponential worst-case time; dynamic programming takes O(target * coins) time and O(target) space. `fib_log` performs O(lg n) arithmetic steps.

## Run it

```sh
cargo run --bin ch01-counting-change
cargo test --bin ch01-counting-change
python3 examples/ch01/section-02/counting-change/main.py
python3 -m unittest examples/ch01/section-02/counting-change/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Memoize `count_change_tree` with a `(amount, remaining_coins)` key and compare its recursive call count against `count_change_dp`.

[Section introduction](../../../../chapters/ch01/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
