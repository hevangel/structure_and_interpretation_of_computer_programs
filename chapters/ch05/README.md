# Chapter 5: Computing with Register Machines

Where do environments, recursion, garbage collection, and function calls actually come from? Chapter 5 strips away host-language recursion by compiling and running programs on explicit registers, stacks, and vector memory.

The chapter builds from register-transfer diagrams and a simulated assembler up to **vector memory & Cheney stop-and-copy GC**, a **7-register explicit-control evaluator** (proving tail recursion runs in O(1) stack space), and a **compile-time register-preserving compiler with lexical addressing**.

## Rust and Python perspectives

- **In Rust:** Rust enums represent instructions, tagged machine pointers (`Pair`, `BrokenHeart`, `Int`), and register IDs cleanly; explicit state machines use `loop` and `Vec` stacks with deterministic counters for pushes and maximum stack depth.
- **In Python:** Python's dataclasses, lists-as-stacks, and dictionaries-as-register-banks let you inspect every register transition, stack frame push/pop, two-space heap flip, and compiled opcode sequence cleanly with zero boilerplate.
- **Comparing both:** Implementing the *same* register machine and compiler in both Rust and Python shows that tail-call optimization, Cheney's two-pointer GC scan, and `preserving` register allocation depend strictly on explicit control discipline—not on whether the host simulator is compiled or interpreted.

## Sections in this chapter

- [5.1 Designing Register Machines](section-01/INTRO.md)
- [5.2 A Register-Machine Simulator](section-02/INTRO.md)
- [5.3 Storage Allocation and Garbage Collection](section-03/INTRO.md)
- [5.4 The Explicit-Control Evaluator](section-04/INTRO.md)
- [5.5 Compilation](section-05/INTRO.md)

## How to study

1. Read each section and subsection note before inspecting the source files.
2. Predict the output, invariants, and time/space complexity on paper.
3. Compare the Rust (`main.rs`) and Python (`main.py`) implementations in the viewer (`index.html` Side-by-Side view) and run both test suites.
4. Change one representation detail or invariant and observe which tests break.

The commentary here is independently written educational material, not a reproduction of the textbook. · [Edition notes](../../docs/editions.md) · [Coverage ledger](../../COVERAGE.md)
