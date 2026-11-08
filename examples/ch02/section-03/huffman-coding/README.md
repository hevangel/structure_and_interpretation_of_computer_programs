# Huffman prefix-code construction

**Book concepts:** Section 2.3. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Build a prefix-free binary encoding by repeatedly combining the two least frequent trees.

## Implementation walkthrough

A leaf stores one symbol; a parent stores the combined weight. Traverse the final tree to construct bit codes, then decode a path at a time.

## Complexity and correctness

Naively sorting the forest per merge takes O(k² log k) for k distinct symbols; a priority queue can improve construction.

## Run it

```sh
cargo run --bin ch02-huffman-coding
cargo test --bin ch02-huffman-coding
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Replace repeated sorting with a binary heap, then test the round-trip on random short strings.

[Section introduction](../../../../chapters/ch02/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
