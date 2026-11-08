# 5.3 Storage Allocation and Garbage Collection

## The big idea

Trace reachability from roots and reclaim unreachable nodes in a toy heap.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **heap layout, reachable objects, mark/sweep, copying collectors**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

A managed-language collector is educational here; it is not a replacement for Rust ownership.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

Why does reachability differ from whether a value will ever be used again?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
