# Geometric frame-mapped picture language and painters

**Book concepts:** Section 2.2 (Subsection 2.2.4). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Henderson's picture language separates *what* a painter draws from *where* it is placed using affine frame mappings (`frame_coord_map`), achieving closure under combination (`beside`, `below`, `corner_split`, `square_of_four`, and `square_limit`).

## Implementation walkthrough

A `Frame` consists of an `origin` vector and two spanning edge vectors `edge1` and `edge2`. `frame_coord_map` maps unit-square coordinates `(x, y)` into `origin + x * edge1 + y * edge2`. `transform_painter` transforms any painter by re-expressing its sub-frame inside the parent frame's coordinate map, enabling `flip_vert`, `flip_horiz`, `rotate90`, `beside`, and `below` without touching pixel arrays.

## Complexity and correctness

Each `corner_split(p, n)` subdivides recursively at half scale while preserving affine invariance; every coordinate generated inside `[0, 1]^2` remains inside `[0, 1]^2` under convex sub-frames, and `rotate90` composed four times equals the identity painter.

## Run it

```sh
cargo run --bin ch02-frame-painters
cargo test --bin ch02-frame-painters
python3 examples/ch02/section-02/frame-painters/main.py
python3 -m unittest examples/ch02/section-02/frame-painters/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Implement a `squash_inwards` or `rotate180` painter transformation purely in terms of `transform_painter`.

[Section introduction](../../../../chapters/ch02/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
