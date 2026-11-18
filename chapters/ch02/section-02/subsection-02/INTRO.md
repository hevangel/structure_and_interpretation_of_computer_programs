# 2.2.2 Hierarchical Structures

## What this subsection teaches

When sequence elements themselves can be sequences—like mobile branches (`Left`/`Right` sub-mobiles), expression ASTs, or nested lists—you get a **tree**.

Recursive algorithms mirror the tree shape directly: handle the leaf base case, recurse on each child subtree, and combine child results (e.g. checking torque `left_len * weight(left) == right_len * weight(right)` on every node).

## Connecting the concept to Rust and Python

- **In Rust:** Algebraic `enum Tree { Leaf(i64), Branch(Vec<Tree>) }` makes nested structures strongly typed—the compiler verifies you handle both leaves and internal nodes.
- **In Python:** Trees can be modeled flexibly either as nested Python lists (`[1, [2, 3]]` checked via `isinstance(x, list)`) or structured `@dataclass` union nodes (`Leaf | Node`).
- **Why compare both:** Compare [`recursive-trees`](../../../../examples/ch02/section-02/recursive-trees/README.md) and [`balanced-mobiles`](../../../../examples/ch02/section-02/balanced-mobiles/README.md): Rust catches invalid tree shapes statically, while Python lets you inspect nested lists interactively.

## Exercise

Write `fringe(tree)` collecting all leaf values from left to right and check balanced mobile torques recursively.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
