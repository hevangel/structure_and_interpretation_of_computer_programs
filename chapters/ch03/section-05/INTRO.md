# 3.5 Streams

## The big idea

Generate an effectively unbounded sequence and consume only a finite prefix.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **lazy sequences, delayed evaluation, stream maps, infinite processes**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Rust Iterator is pull-based laziness; it provides a practical analogue to delayed streams but has different memoization semantics.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

How much of an infinite computation must be performed to answer a finite query?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
