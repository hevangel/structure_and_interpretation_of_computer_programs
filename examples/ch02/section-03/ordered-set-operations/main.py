import unittest


def sorted_unique(items: list[int]) -> bool:
    return all(a < b for a, b in zip(items, items[1:]))


def combine(a: list[int], b: list[int]) -> tuple[list[int], list[int]]:
    assert sorted_unique(a) and sorted_unique(b)
    i = j = 0
    union: list[int] = []
    intersection: list[int] = []
    while i < len(a) and j < len(b):
        if a[i] < b[j]:
            union.append(a[i])
            i += 1
        elif a[i] > b[j]:
            union.append(b[j])
            j += 1
        else:
            union.append(a[i])
            intersection.append(a[i])
            i += 1
            j += 1
    union.extend(a[i:])
    union.extend(b[j:])
    return union, intersection


class OrderedSetOperationsTests(unittest.TestCase):
    def test_set_merge_laws(self) -> None:
        self.assertEqual(combine([1, 3, 5], [2, 3, 6]), ([1, 2, 3, 5, 6], [3]))
        self.assertEqual(combine([], [2]), ([2], []))
        self.assertEqual(combine([1, 2], [1, 2]), ([1, 2], [1, 2]))


if __name__ == "__main__":
    print(combine([1, 3, 5], [2, 3, 6]))
