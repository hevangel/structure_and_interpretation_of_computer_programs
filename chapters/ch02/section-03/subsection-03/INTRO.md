# 2.3.3 Representing Sets

## What this subsection teaches

Mathematically, a set `{1, 2, 3}` supports `contains`, `union`, and `intersection`—but performance depends drastically on representation:
- **Unordered list:** `O(n)` lookup, `O(n^2)` union
- **Sorted list:** `O(n)` two-pointer merge for union/intersection
- **Balanced binary search tree (BST):** `O(log n)` lookup, and converting `tree -> sorted_list -> partial_tree` performs **union/intersection of two BSTs in `O(n)` time** while keeping height logarithmic!

## Connecting the concept to Rust and Python

- **In Rust:** BST nodes (`enum Tree { Empty, Node { val, left, right } }`) store ordered subtrees cleanly, and recursive slice halving `partial_tree(sorted_slice)` builds balanced trees in place without extra allocations.
- **In Python:** Python uses recursive `@dataclass` BST nodes paired with generator/in-order flattening (`tree_to_list`) and `list_to_tree` index slicing.
- **Why compare both:** Explore [`binary-search-tree`](../../../../examples/ch02/section-03/binary-search-tree/README.md), [`ordered-set-operations`](../../../../examples/ch02/section-03/ordered-set-operations/README.md), [`tree-to-ordered-list`](../../../../examples/ch02/section-03/tree-to-ordered-list/README.md), and [`ordered-sets-infix-deriv`](../../../../examples/ch02/section-03/ordered-sets-infix-deriv/README.md).

## Exercise

Verify why building a balanced BST from `[0..1023]` guarantees tree height `<= 10`, whereas naive sequential insertion gives height `1023`.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
