from __future__ import annotations

from dataclasses import dataclass
from typing import Callable
import unittest


@dataclass(frozen=True)
class Vect:
    x: float
    y: float

    def add(self, other: Vect) -> Vect:
        return Vect(self.x + other.x, self.y + other.y)

    def sub(self, other: Vect) -> Vect:
        return Vect(self.x - other.x, self.y - other.y)

    def scale(self, s: float) -> Vect:
        return Vect(self.x * s, self.y * s)


@dataclass(frozen=True)
class Segment:
    start: Vect
    end: Vect


@dataclass(frozen=True)
class Frame:
    origin: Vect
    edge1: Vect
    edge2: Vect


def frame_coord_map(frame: Frame) -> Callable[[Vect], Vect]:
    return lambda v: frame.origin.add(frame.edge1.scale(v.x)).add(frame.edge2.scale(v.y))


Canvas = list[Segment]
Painter = Callable[[Frame, Canvas], None]


def segments_to_painter(segments: list[Segment]) -> Painter:
    def paint(frame: Frame, canvas: Canvas) -> None:
        m = frame_coord_map(frame)
        for seg in segments:
            canvas.append(Segment(m(seg.start), m(seg.end)))

    return paint


def transform_painter(painter: Painter, origin: Vect, corner1: Vect, corner2: Vect) -> Painter:
    def paint(frame: Frame, canvas: Canvas) -> None:
        m = frame_coord_map(frame)
        new_origin = m(origin)
        painter(
            Frame(new_origin, m(corner1).sub(new_origin), m(corner2).sub(new_origin)),
            canvas,
        )

    return paint


def identity_painter(painter: Painter) -> Painter:
    return painter


def flip_vert(painter: Painter) -> Painter:
    return transform_painter(painter, Vect(0.0, 1.0), Vect(1.0, 1.0), Vect(0.0, 0.0))


def flip_horiz(painter: Painter) -> Painter:
    return transform_painter(painter, Vect(1.0, 0.0), Vect(0.0, 0.0), Vect(1.0, 1.0))


def rotate90(painter: Painter) -> Painter:
    return transform_painter(painter, Vect(1.0, 0.0), Vect(1.0, 1.0), Vect(0.0, 0.0))


def beside(left: Painter, right: Painter) -> Painter:
    split = Vect(0.5, 0.0)
    p_left = transform_painter(left, Vect(0.0, 0.0), split, Vect(0.0, 1.0))
    p_right = transform_painter(right, split, Vect(1.0, 0.0), Vect(0.5, 1.0))

    def paint(frame: Frame, canvas: Canvas) -> None:
        p_left(frame, canvas)
        p_right(frame, canvas)

    return paint


def below(bottom: Painter, top: Painter) -> Painter:
    split = Vect(0.0, 0.5)
    p_bot = transform_painter(bottom, Vect(0.0, 0.0), Vect(1.0, 0.0), split)
    p_top = transform_painter(top, split, Vect(1.0, 0.5), Vect(0.0, 1.0))

    def paint(frame: Frame, canvas: Canvas) -> None:
        p_bot(frame, canvas)
        p_top(frame, canvas)

    return paint


def right_split(painter: Painter, n: int) -> Painter:
    if n == 0:
        return painter
    smaller = right_split(painter, n - 1)
    return beside(painter, below(smaller, smaller))


def up_split(painter: Painter, n: int) -> Painter:
    if n == 0:
        return painter
    smaller = up_split(painter, n - 1)
    return below(painter, beside(smaller, smaller))


def corner_split(painter: Painter, n: int) -> Painter:
    if n == 0:
        return painter
    up = up_split(painter, n - 1)
    right = right_split(painter, n - 1)
    top_left = beside(up, up)
    bottom_right = below(right, right)
    corner = corner_split(painter, n - 1)
    return beside(below(painter, top_left), below(bottom_right, corner))


def square_of_four(
    tl: Callable[[Painter], Painter],
    tr: Callable[[Painter], Painter],
    bl: Callable[[Painter], Painter],
    br: Callable[[Painter], Painter],
) -> Callable[[Painter], Painter]:
    return lambda p: below(beside(bl(p), br(p)), beside(tl(p), tr(p)))


def square_limit(painter: Painter, n: int) -> Painter:
    return square_of_four(flip_horiz, identity_painter, rotate90, flip_vert)(
        corner_split(painter, n)
    )


class FramePaintersTest(unittest.TestCase):
    def test_rotate90_four_times_is_identity(self) -> None:
        unit = Frame(Vect(0.0, 0.0), Vect(1.0, 0.0), Vect(0.0, 1.0))
        base = segments_to_painter([Segment(Vect(0.2, 0.3), Vect(0.7, 0.8))])
        r4 = rotate90(rotate90(rotate90(rotate90(base))))
        c1: Canvas = []
        c2: Canvas = []
        base(unit, c1)
        r4(unit, c2)
        self.assertEqual(len(c1), len(c2))
        self.assertAlmostEqual(c1[0].start.x, c2[0].start.x, places=12)
        self.assertAlmostEqual(c1[0].start.y, c2[0].start.y, places=12)
        self.assertAlmostEqual(c1[0].end.x, c2[0].end.x, places=12)
        self.assertAlmostEqual(c1[0].end.y, c2[0].end.y, places=12)

    def test_square_limit_bounds(self) -> None:
        unit = Frame(Vect(0.0, 0.0), Vect(1.0, 0.0), Vect(0.0, 1.0))
        stroke = segments_to_painter([Segment(Vect(0.1, 0.2), Vect(0.9, 0.8))])
        canvas: Canvas = []
        square_limit(stroke, 2)(unit, canvas)
        self.assertGreater(len(canvas), 0)
        for seg in canvas:
            for pt in (seg.start, seg.end):
                self.assertGreaterEqual(pt.x, -1e-12)
                self.assertLessEqual(pt.x, 1.0 + 1e-12)
                self.assertGreaterEqual(pt.y, -1e-12)
                self.assertLessEqual(pt.y, 1.0 + 1e-12)


if __name__ == "__main__":
    unit = Frame(Vect(0.0, 0.0), Vect(1.0, 0.0), Vect(0.0, 1.0))
    wedge = segments_to_painter([Segment(Vect(0.1, 0.1), Vect(0.8, 0.4))])
    out: Canvas = []
    square_limit(wedge, 2)(unit, out)
    print(f"square_limit(2) rendered {len(out)} vector segments")
