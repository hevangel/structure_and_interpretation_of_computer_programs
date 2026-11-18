from __future__ import annotations

from collections import Counter
from dataclasses import dataclass
import unittest


@dataclass
class Node:
    weight: int
    symbol: str | None = None
    left: Node | None = None
    right: Node | None = None


def make_tree(text: str) -> Node:
    counts = Counter(text)
    assert counts, "text must not be empty"
    forest = [Node(weight, ch) for ch, weight in sorted(counts.items())]
    while len(forest) > 1:
        forest.sort(key=lambda node: node.weight)
        a = forest.pop(0)
        b = forest.pop(0)
        forest.append(Node(a.weight + b.weight, None, a, b))
    return forest[0]


def find_code(node: Node, ch: str, path: list[bool]) -> bool:
    if node.symbol == ch:
        return True
    if node.left is not None:
        path.append(False)
        if find_code(node.left, ch, path):
            return True
        path.pop()
    if node.right is not None:
        path.append(True)
        if find_code(node.right, ch, path):
            return True
        path.pop()
    return False


def encode(root: Node, text: str) -> list[bool]:
    bits: list[bool] = []
    for ch in text:
        code: list[bool] = []
        assert find_code(root, ch, code)
        if not code:
            code.append(False)
        bits.extend(code)
    return bits


def decode(root: Node, bits: list[bool]) -> str:
    if root.symbol is not None:
        assert all(not b for b in bits)
        return root.symbol * len(bits)
    out: list[str] = []
    current = root
    for bit in bits:
        nxt = current.right if bit else current.left
        assert nxt is not None
        current = nxt
        if current.symbol is not None:
            out.append(current.symbol)
            current = root
    assert current is root, "incomplete encoded symbol"
    return "".join(out)


class HuffmanCodingTests(unittest.TestCase):
    def test_round_trip(self) -> None:
        for text in ("banana bandana", "aaaaa", "abcabc", "python"):
            tree = make_tree(text)
            self.assertEqual(decode(tree, encode(tree, text)), text)


if __name__ == "__main__":
    message = "banana bandana"
    tree = make_tree(message)
    code = encode(tree, message)
    print(f"{len(code)} bits decode to {decode(tree, code)!r}")
