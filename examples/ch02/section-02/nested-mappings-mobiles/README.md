# Binary mobiles, fold direction, and nested mappings

**Book concepts:** Section 2.2 (Subsections 2.2.2 and 2.2.3). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Hierarchical tree data structures (`Mobile`/`Branch`), sequence accumulation direction (`fold_right` vs `fold_left`), and nested mappings via `flatmap` (`prime_sum_pairs` and recursive `permutations`).

## Implementation walkthrough

`Mobile` combines weighted or sub-mobile branches recursively and requires torque equality plus recursive sub-mobile balance. `fold_right` and `fold_left` agree when the combining operator is both associative and commutative (or obeys `(op x (op y z)) = (op (op x y) z)` with matching identity). `flatmap` flattens nested sequence generation for prime-sum pairs and permutations.

## Complexity and correctness

`total_weight` and `is_balanced` visit every branch once (O(nodes)). `prime_sum_pairs(n)` inspects O(n^2) pairs, and `permutations` produces all `n!` orderings when elements are distinct.

## Run it

```sh
cargo run --bin ch02-nested-mappings-mobiles
cargo test --bin ch02-nested-mappings-mobiles
python3 examples/ch02/section-02/nested-mappings-mobiles/main.py
python3 -m unittest examples/ch02/section-02/nested-mappings-mobiles/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Implement the Exercise 2.42 N-queens solver using `flatmap` over columns and `adjoin_position` / `is_safe` filters.

[Section introduction](../../../../chapters/ch02/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
