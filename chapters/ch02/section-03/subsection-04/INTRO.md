# 2.3.4 Huffman Encoding Trees

## What this subsection teaches

Fixed-width codes (like 3 bits per symbol for 8 letters) waste space when `E` appears 40% of the time and `Z` appears 1%. **Huffman coding** merges the two lowest-weight subtrees greedily into a binary tree: frequent letters stay near the root (short bitstrings) while rare letters sink deeper.

Because every symbol sits strictly at a **leaf**, no codeword is ever a prefix of another—allowing instant bit-by-bit streaming decoding with zero separator bits!

## Connecting the concept to Rust and Python

- **In Rust:** An ordered priority queue (`BinaryHeap` or sorted `Vec`) repeatedly pops the two smallest `Weight` nodes to construct `enum HuffmanTree { Leaf, Branch }`; bitstreams use `Vec<bool>` or `u8` slices.
- **In Python:** Python uses `heapq` (or sorting) over `(weight, node)` tuples with `@dataclass` `Leaf`/`Branch` nodes and clean string/list bit encoding.
- **Why compare both:** Both implementations in [`huffman-coding`](../../../../examples/ch02/section-03/huffman-coding/README.md) preserve the exact same greedy merge invariant and prefix-free round-trip guarantee.

## Exercise

Build a Huffman tree for symbol weights `{A:8, B:3, C:1, D:1}` and decode `0101110` back to symbols.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
