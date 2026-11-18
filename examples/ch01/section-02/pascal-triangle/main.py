import unittest


def row(n: int) -> list[int]:
    current = [1]
    for _ in range(n):
        nxt = [1] * (len(current) + 1)
        for i in range(1, len(current)):
            # Entry i of row k+1 sums adjacent entries from row k.
            nxt[i] = current[i - 1] + current[i]
        current = nxt
    return current


class PascalTriangleTests(unittest.TestCase):
    def test_known_rows_and_symmetry(self) -> None:
        self.assertEqual(row(0), [1])
        self.assertEqual(row(4), [1, 4, 6, 4, 1])
        for n in range(16):
            values = row(n)
            self.assertEqual(sum(values), 1 << n)
            self.assertEqual(values, list(reversed(values)))


if __name__ == "__main__":
    for level in range(6):
        print(row(level))
