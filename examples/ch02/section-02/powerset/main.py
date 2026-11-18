from typing import TypeVar
import unittest


T = TypeVar("T")


def subsets(items: list[T]) -> list[list[T]]:
    result: list[list[T]] = [[]]
    for item in items:
        # Each existing subset gets a clone extended with the current item.
        existing = len(result)
        for index in range(existing):
            result.append([*result[index], item])
    return result


class PowersetTests(unittest.TestCase):
    def test_includes_empty_and_complete_subsets(self) -> None:
        all_sets = subsets([1, 2, 3])
        self.assertEqual(len(all_sets), 8)
        self.assertIn([], all_sets)
        self.assertIn([1, 2, 3], all_sets)
        self.assertEqual(subsets([]), [[]])


if __name__ == "__main__":
    print(subsets([1, 2, 3]))
