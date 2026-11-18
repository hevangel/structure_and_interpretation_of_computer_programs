from __future__ import annotations

from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Tree:
    key: int
    left: Tree | None = None
    right: Tree | None = None


def insert(tree: Tree | None, key: int) -> Tree:
    if tree is None:
        return Tree(key)
    if key < tree.key:
        return Tree(tree.key, insert(tree.left, key), tree.right)
    if key > tree.key:
        return Tree(tree.key, tree.left, insert(tree.right, key))
    return tree


def inorder(tree: Tree | None, out: list[int]) -> None:
    if tree is not None:
        inorder(tree.left, out)
        out.append(tree.key)
        inorder(tree.right, out)


class TreeToOrderedListTests(unittest.TestCase):
    def test_sorted_without_duplicates(self) -> None:
        tree: Tree | None = None
        for key in (5, 2, 8, 1, 3, 2):
            tree = insert(tree, key)
        values: list[int] = []
        inorder(tree, values)
        self.assertEqual(values, [1, 2, 3, 5, 8])


if __name__ == "__main__":
    tree: Tree | None = None
    for key in (5, 2, 8, 1, 3):
        tree = insert(tree, key)
    ordered: list[int] = []
    inorder(tree, ordered)
    print(ordered)
