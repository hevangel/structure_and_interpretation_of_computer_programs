# Compile expressions to stack-machine bytecode

**Book concepts:** Section 5.5. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Translate a small abstract syntax tree into lower-level instructions, then execute the instructions.

## Implementation walkthrough

Postorder compilation emits code for operands before an operator. The virtual machine uses an explicit operand stack and checks for underflow.

## Complexity and correctness

Compilation and execution take O(nodes) time for this tree with no loops. Maximum stack usage is bounded by expression depth for these examples.

## Run it

```sh
cargo run --bin ch05-bytecode-compiler
cargo test --bin ch05-bytecode-compiler
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add constant folding at compile time and compare the length of generated programs.

[Section introduction](../../../../chapters/ch05/section-05/INTRO.md) · [Coverage](../../../../COVERAGE.md)
