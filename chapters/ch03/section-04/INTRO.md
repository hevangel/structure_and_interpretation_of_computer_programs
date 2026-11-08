# 3.4 Concurrency: Time Is of the Essence

## The big idea

Make many threads update shared state and test that synchronization preserves a global invariant.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **interleavings, race conditions, serialization, mutexes, deadlock**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Arc<Mutex<T>> is a safe starting point; lock-order design remains a semantic responsibility.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

Which operations must behave atomically to preserve consistency?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
