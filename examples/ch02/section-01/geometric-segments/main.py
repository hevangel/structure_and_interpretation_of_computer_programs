from dataclasses import dataclass
import math
import unittest


@dataclass(frozen=True)
class Point:
    x: float
    y: float


@dataclass(frozen=True)
class Segment:
    start: Point
    end: Point

    def midpoint(self) -> Point:
        return Point((self.start.x + self.end.x) / 2.0, (self.start.y + self.end.y) / 2.0)

    def length(self) -> float:
        return math.hypot(self.end.x - self.start.x, self.end.y - self.start.y)


class Rectangle:
    def __init__(self, lower_left: Point, upper_right: Point) -> None:
        assert lower_left.x <= upper_right.x and lower_left.y <= upper_right.y
        self.lower_left = lower_left
        self.upper_right = upper_right

    def area(self) -> float:
        return (self.upper_right.x - self.lower_left.x) * (
            self.upper_right.y - self.lower_left.y
        )

    def perimeter(self) -> float:
        return 2.0 * (
            (self.upper_right.x - self.lower_left.x)
            + (self.upper_right.y - self.lower_left.y)
        )


class GeometricSegmentsTests(unittest.TestCase):
    def test_representation_independent_geometry(self) -> None:
        s = Segment(Point(0.0, 0.0), Point(3.0, 4.0))
        self.assertEqual(s.length(), 5.0)
        p = s.midpoint()
        self.assertEqual((p.x, p.y), (1.5, 2.0))
        r = Rectangle(Point(0.0, 1.0), Point(3.0, 5.0))
        self.assertEqual(r.area(), 12.0)
        self.assertEqual(r.perimeter(), 14.0)


if __name__ == "__main__":
    s = Segment(Point(0.0, 0.0), Point(3.0, 4.0))
    r = Rectangle(Point(0.0, 1.0), Point(3.0, 5.0))
    print(
        f"length {s.length()}, midpoint {s.midpoint()}, rect area {r.area()}, perimeter {r.perimeter()}"
    )
