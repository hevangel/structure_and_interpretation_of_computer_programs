# Balanced ordered-set BST operations and infix differentiation

**Book concepts:** Section 2.3 (Subsections 2.3.2 and 2.3.3). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Algebraic simplification and precedence-aware infix parsing extend symbolic differentiation with the power rule, while linear-time `tree_to_list`, `list_to_balanced_tree`, and sorted merges implement O(n) set union and intersection over balanced binary search trees.

## Implementation walkthrough

`parse_infix` applies recursive descent with `+ < * < **` precedence to build `Expr` trees including `Pow(base, exp)` (Exercises 2.56–2.58). Separately, `partial_tree` partitions a sorted sequence in-order in a single linear pass (`O(n)`) to construct a balanced BST of height at most `floor(log2(n)) + 1`, allowing `union_set` and `intersection_set` (Exercise 2.65) to run in O(n) total time.

## Complexity and correctness

Every element of the sorted list is visited once during `tree_to_list`, `union_sorted`/`intersection_sorted`, and `partial_tree`, avoiding the O(n log n) cost of repeated tree insertions and guaranteeing logarithmic tree height up to any size.

## Run it

```sh
cargo run --bin ch02-ordered-sets-infix-deriv
cargo test --bin ch02-ordered-sets-infix-deriv
python3 examples/ch02/section-03/ordered-sets-infix-deriv/main.py
python3 -m unittest examples/ch02/section-03/ordered-sets-infix-deriv/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add subtraction (`-`) and symbolic exponents `u ** v` (with `ln`) to the infix parser and differentiation rules.

[Section introduction](../../../../chapters/ch02/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
