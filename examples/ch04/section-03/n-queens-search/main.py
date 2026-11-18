import unittest


def solve(n: int) -> list[list[int]]:
    solutions: list[list[int]] = []

    def search(columns: list[int]) -> None:
        row = len(columns)
        if row == n:
            solutions.append(list(columns))
            return
        for col in range(n):
            # Reject shared columns or either diagonal.
            if all(c != col and abs(c - col) != row - r for r, c in enumerate(columns)):
                columns.append(col)
                search(columns)
                columns.pop()

    search([])
    return solutions


class NQueensSearchTests(unittest.TestCase):
    def test_solution_counts(self) -> None:
        self.assertEqual(len(solve(1)), 1)
        self.assertEqual(len(solve(4)), 2)
        self.assertEqual(len(solve(8)), 92)


if __name__ == "__main__":
    print(f"8-queen solutions: {len(solve(8))}")
