from typing import Union
import unittest


Tree = Union[int, list["Tree"]]


def tree_sum(tree: Tree) -> int:
    if isinstance(tree, int):
        return tree
    return sum(tree_sum(child) for child in tree)


def count_leaves(tree: Tree) -> int:
    if isinstance(tree, int):
        return 1
    return sum(count_leaves(child) for child in tree)


class RecursiveTreesTests(unittest.TestCase):
    def test_folds_nesting(self) -> None:
        t: Tree = [2, [3]]
        self.assertEqual(tree_sum(t), 5)
        self.assertEqual(count_leaves(t), 2)


if __name__ == "__main__":
    t: Tree = [2, [3, 5]]
    print(f"{count_leaves(t)} leaves, sum {tree_sum(t)}")
