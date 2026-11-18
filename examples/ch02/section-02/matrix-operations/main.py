import unittest


def multiply(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    assert a and b and b[0]
    k = len(b)
    n = len(b[0])
    assert all(len(row) == k for row in a)
    assert all(len(row) == n for row in b)
    return [
        [sum(a[i][t] * b[t][j] for t in range(k)) for j in range(n)]
        for i in range(len(a))
    ]


class MatrixOperationsTests(unittest.TestCase):
    def test_dimension_and_values(self) -> None:
        self.assertEqual(
            multiply([[1, 2], [3, 4]], [[5, 6], [7, 8]]),
            [[19, 22], [43, 50]],
        )


if __name__ == "__main__":
    print(multiply([[1, 2]], [[3], [4]]))
