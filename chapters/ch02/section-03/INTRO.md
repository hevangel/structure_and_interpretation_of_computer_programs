# 2.3 Symbolic Data

## The big idea

Model mathematical syntax as algebraic data and transform it by structural recursion.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **quotation, symbolic differentiation, sets, binary-search trees, huffman coding**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Enums and pattern matching provide an explicit, exhaustively checked form of symbolic dispatch.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

What distinguishes manipulating an expression from evaluating it?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
