import unittest


def crossings(samples: list[float]) -> list[tuple[int, str]]:
    events: list[tuple[int, str]] = []
    for index in range(1, len(samples)):
        before = samples[index - 1]
        after = samples[index]
        # Report an event at the index of the newly arrived sample.
        if before <= 0.0 < after:
            events.append((index, "up"))
        elif before >= 0.0 > after:
            events.append((index, "down"))
    return events


class ZeroCrossingsTests(unittest.TestCase):
    def test_reports_crossings_in_order(self) -> None:
        self.assertEqual(
            crossings([-1.0, 1.0, 3.0, -2.0, 0.0, 5.0]),
            [(1, "up"), (3, "down"), (5, "up")],
        )
        self.assertEqual(crossings([]), [])
        self.assertEqual(crossings([3.0]), [])


if __name__ == "__main__":
    print(crossings([-1.0, 1.0, 3.0, -2.0]))
