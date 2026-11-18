import unittest


Picture = list[str]


def beside(left: Picture, right: Picture) -> Picture:
    assert len(left) == len(right), "equal height"
    return [f"{a}{b}" for a, b in zip(left, right)]


def above(top: Picture, bottom: Picture) -> Picture:
    t_len = len(top[0]) if top else None
    b_len = len(bottom[0]) if bottom else None
    assert t_len == b_len
    return [*top, *bottom]


class PictureCompositionTests(unittest.TestCase):
    def test_compositions_are_pictures(self) -> None:
        p = ["ab", "cd"]
        self.assertEqual(beside(p, p), ["abab", "cdcd"])
        self.assertEqual(len(above(p, p)), 4)


if __name__ == "__main__":
    corner = ["/\\", "\\/"]
    for row in above(beside(corner, corner), beside(corner, corner)):
        print(row)
