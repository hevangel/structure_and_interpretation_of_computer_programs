# 5.5 Compilation

## The big idea

Compile a small expression tree into virtual-machine instructions and execute them.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **instruction selection, target machine, linkage, register preservation**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Separate front end, code generation, and virtual machine to make each stage testable.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

How can two instruction sequences implement the same expression semantics?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
