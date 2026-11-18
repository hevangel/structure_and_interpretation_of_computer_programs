# 2.3 Symbolic Data · [Chapter 2](../README.md)

## The big idea

Programs can manipulate **symbols and formulas** (`x + 3*y`) directly as structured data trees rather than immediately collapsing them to numbers. That unlocks algebraic simplification, **symbolic differentiation** (`d/dx(u*v) = u*dv + v*du`), balanced **ordered sets** (`O(n)` tree-to-list merges & `partial-tree`), and variable-length prefix-free **Huffman codes**.

## What you should focus on

Focus on ****quoted syntax vs values, algebraic expression ASTs, balanced BST invariants, and prefix-free Huffman trees****. Across every example, test which callers remain completely untouched when you swap the internal representation.

## Connecting the concept to Rust and Python

- **In Rust:** Algebraic expressions and Huffman nodes are ideal fits for `enum Expr` / `enum Tree` with exhaustive `match` rules that guarantee every operator (`Add`, `Mul`, `Pow`) or tree case (`Leaf`, `Branch`) is handled.
- **In Python:** Uses tagged `@dataclass` unions (with structural `match`/`case` or `isinstance` checks), strings for symbolic identifiers, and tuples/lists for lightweight ASTs.
- **Key contrast:** Rust `match` catches unhandled AST constructors at compile time, while Python's readable `@dataclass` trees make writing recursive tree rewrites and prefix-code encoders fast and clean.

## Subsections

- [2.3.1 Quotation and Strings](subsection-01/INTRO.md)
- [2.3.2 Symbolic Differentiation](subsection-02/INTRO.md)
- [2.3.3 Representing Sets](subsection-03/INTRO.md)
- [2.3.4 Huffman Encoding Trees](subsection-04/INTRO.md)

## Runnable examples in this section

- [2.3 Symbolic differentiation](../../../examples/ch02/section-03/symbolic-differentiation/README.md)
- [2.3 Ordered sets as binary search trees](../../../examples/ch02/section-03/binary-search-tree/README.md)
- [2.3 Huffman prefix-code construction](../../../examples/ch02/section-03/huffman-coding/README.md)
- [2.3 Set union and intersection by merging](../../../examples/ch02/section-03/ordered-set-operations/README.md)
- [2.3 Tree traversal as ordered-sequence conversion](../../../examples/ch02/section-03/tree-to-ordered-list/README.md)
- [2.3 Balanced ordered-set BST operations and infix differentiation](../../../examples/ch02/section-03/ordered-sets-infix-deriv/README.md)

## Check your understanding

1. Why can no code word be a prefix of another in a valid Huffman tree, and how does decoding stay unambiguous without delimiters?
2. How does converting a BST to an ordered list, merging in `O(n)`, and rebuilding a balanced tree avoid `O(n log n)` skew?

**Experiment:** Add `sin`/`cos` rules to `ordered-sets-infix-deriv` or encode a skewed character distribution in `huffman-coding`.

[Chapter 2 overview](../README.md) · [Edition notes](../../../docs/editions.md)
