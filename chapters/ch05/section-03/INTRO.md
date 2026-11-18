# 5.3 Storage Allocation and Garbage Collection · [Chapter 5](../README.md)

## The big idea

Scheme pairs and environments look like infinite fresh allocations—where do `cons` cells live in a flat linear machine memory? Inside two parallel index vectors **`the_cars`** and **`the_cdrs`**, addressed by typed pointers (`Pair(idx)`, `Int(n)`, `Nil`, `BrokenHeart(idx)`).

When `free` reaches the end of memory, **garbage collection** reclaims unreachable objects. Compare **Mark–Sweep** (mark reachable graph nodes in place, then sweep unmarked slots) with SICP's flagship **Cheney Stop-and-Copy GC**: two breadth-first pointers (`free` and `scan`) copy only reachable cells cleanly into compacted `new_cars`/`new_cdrs`, leaving `BrokenHeart` forwarding pointers so shared DAGs and cycles copy *exactly once* before `gc_flip`!

## What you should focus on

Focus on ****flat `the_cars`/`the_cdrs` typed vector memory, reachability from roots, Mark–Sweep vs Cheney Stop-and-Copy, and `BrokenHeart` forwarding for cycles/DAGs****. Every machine model and compiler pass makes hidden runtime cost directly measurable.

## Connecting the concept to Rust and Python

- **In Rust:** Models explicit vector heaps (`Vec<Option<Node>>` in `mark-sweep-gc` and dual `Vec<Word>` `the_cars`/`the_cdrs` banks in `stop-and-copy-gc`) so pointer chasing and forwarding invariants are completely explicit.
- **In Python:** Although CPython uses reference counting + a generational cycle collector automatically, implementing `mark-sweep-gc` and `stop-and-copy-gc` manually on Python integer index arrays exposes the exact invariants hidden under CPython's memory manager.
- **Key contrast:** Both implementations highlight why `BrokenHeart(new_addr)` prevents copying shared diamond nodes twice or looping forever on cyclic pairs (`set_cdr(a, a)`).

## Subsections

- [5.3.1 Memory as Vectors](subsection-01/INTRO.md)
- [5.3.2 Garbage Collection](subsection-02/INTRO.md)

## Runnable examples in this section

- [5.3 Reachability-based mark/sweep garbage collection](../../../examples/ch05/section-03/mark-sweep-gc/README.md)
- [5.3 Vector-memory pairs and Cheney stop-and-copy garbage collector](../../../examples/ch05/section-03/stop-and-copy-gc/README.md)

## Check your understanding

1. Why is Cheney's two-pointer `scan`/`free` loop strictly **iterative** (zero auxiliary recursion stack needed during GC)?
2. What happens if two live pointers reference the same cell before `gc_flip` and you forget to write `BrokenHeart(new_addr)`?

**Experiment:** Construct a cyclic 2-node pair plus unreachable garbage in `stop-and-copy-gc` and watch consecutive GC flips compact live cells down cleanly.

[Chapter 5 overview](../README.md) · [Edition notes](../../../docs/editions.md)
