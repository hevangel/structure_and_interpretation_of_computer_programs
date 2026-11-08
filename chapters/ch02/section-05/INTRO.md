# 2.5 Systems with Generic Operations

## The big idea

Mix numeric types by defining common operations and conversions explicitly.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **generic arithmetic, type towers, coercion, data-directed programming**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Rust's static traits differ from Scheme's runtime operation table; both are ways to manage extensibility.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

How should a system choose between a lossless coercion and a lossy one?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
