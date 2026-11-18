from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Solution:
    baker: int
    cooper: int
    fletcher: int
    miller: int
    smith: int


def search() -> list[Solution]:
    solutions: list[Solution] = []

    def explore(floors: list[int]) -> None:
        person = len(floors)
        if person == 5:
            b, c, f, m, s = floors
            if m > c and abs(s - f) != 1 and abs(f - c) != 1:
                solutions.append(Solution(b, c, f, m, s))
            return
        for floor in range(1, 6):
            if floor in floors:
                continue
            # Baker != 5, Cooper != 1, Fletcher not in {1, 5}.
            if (
                (person == 0 and floor == 5)
                or (person == 1 and floor == 1)
                or (person == 2 and floor in (1, 5))
            ):
                continue
            floors.append(floor)
            explore(floors)
            floors.pop()

    explore([])
    return solutions


class MultipleDwellingTests(unittest.TestCase):
    def test_one_solution_satisfies_every_clue(self) -> None:
        self.assertEqual(search(), [Solution(baker=3, cooper=2, fletcher=4, miller=5, smith=1)])


if __name__ == "__main__":
    print(f"solutions: {search()}")
