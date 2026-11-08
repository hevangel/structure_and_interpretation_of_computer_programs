# 2.3.4 Huffman Encoding Trees

## What this subsection teaches

A weighted binary tree assigns short prefix-free codes to common symbols. Decoding succeeds because no code is a prefix of another.

## Connecting the concept to Rust

Use enum syntax trees and exhaustive match arms to manipulate symbols and trees. The translation is intended to explain computational ideas, not imitate the exact source language syntax. Think about the input, output, representation invariant, and what changes when evaluation order or mutable state enters the picture.

## Exercise

Encode and decode a repeated-character message.

**Study method:** trace a small input on paper, run the closest example from this repository, then explain the result and resource usage. Which aspects are mathematical and which depend on the implementation?

This original study note follows the 1996/2022 [SICP comparison table of contents](https://sicp.sourceacademy.org/). The 1984 original may organize the subject differently.

[Parent section](../INTRO.md) · [Edition notes](../../../../docs/editions.md)
