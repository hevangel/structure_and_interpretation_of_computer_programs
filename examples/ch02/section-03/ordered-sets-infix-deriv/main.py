from __future__ import annotations

from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Num:
    value: int

    def __str__(self) -> str:
        return str(self.value)


@dataclass(frozen=True)
class Var:
    name: str

    def __str__(self) -> str:
        return self.name


@dataclass(frozen=True)
class Add:
    left: object
    right: object

    def __str__(self) -> str:
        return f"({self.left} + {self.right})"


@dataclass(frozen=True)
class Mul:
    left: object
    right: object

    def __str__(self) -> str:
        return f"({self.left} * {self.right})"


@dataclass(frozen=True)
class Pow:
    base: object
    exp: int

    def __str__(self) -> str:
        return f"({self.base} ** {self.exp})"


def make_sum(a: object, b: object) -> object:
    if a == Num(0):
        return b
    if b == Num(0):
        return a
    if isinstance(a, Num) and isinstance(b, Num):
        return Num(a.value + b.value)
    return Add(a, b)


def make_prod(a: object, b: object) -> object:
    if a == Num(0) or b == Num(0):
        return Num(0)
    if a == Num(1):
        return b
    if b == Num(1):
        return a
    if isinstance(a, Num) and isinstance(b, Num):
        return Num(a.value * b.value)
    return Mul(a, b)


def make_pow(base: object, exp: int) -> object:
    if exp == 0:
        return Num(1)
    if exp == 1:
        return base
    if isinstance(base, Num) and exp >= 0:
        return Num(base.value**exp)
    return Pow(base, exp)


def deriv(expr: object, var: str) -> object:
    if isinstance(expr, Num):
        return Num(0)
    if isinstance(expr, Var):
        return Num(1 if expr.name == var else 0)
    if isinstance(expr, Add):
        return make_sum(deriv(expr.left, var), deriv(expr.right, var))
    if isinstance(expr, Mul):
        return make_sum(
            make_prod(expr.left, deriv(expr.right, var)),
            make_prod(deriv(expr.left, var), expr.right),
        )
    if isinstance(expr, Pow):
        return make_prod(
            make_prod(Num(expr.exp), make_pow(expr.base, expr.exp - 1)),
            deriv(expr.base, var),
        )
    raise TypeError(expr)


def lex_infix(src: str) -> list[tuple[str, object]]:
    tokens: list[tuple[str, object]] = []
    i = 0
    while i < len(src):
        ch = src[i]
        if ch.isspace():
            i += 1
        elif ch.isdigit():
            j = i
            while j < len(src) and src[j].isdigit():
                j += 1
            tokens.append(("NUM", int(src[i:j])))
            i = j
        elif ch.isalpha():
            j = i
            while j < len(src) and (src[j].isalnum() or src[j] == "_"):
                j += 1
            tokens.append(("IDENT", src[i:j]))
            i = j
        elif src.startswith("**", i):
            tokens.append(("POW", "**"))
            i += 2
        elif ch in "+*()":
            tokens.append((ch, ch))
            i += 1
        else:
            raise ValueError(f"unexpected character: {ch}")
    return tokens


def parse_infix(src: str) -> object:
    tokens = lex_infix(src)
    pos = 0

    def parse_sum() -> object:
        nonlocal pos
        lhs = parse_prod()
        while pos < len(tokens) and tokens[pos][0] == "+":
            pos += 1
            lhs = make_sum(lhs, parse_prod())
        return lhs

    def parse_prod() -> object:
        nonlocal pos
        lhs = parse_pow()
        while pos < len(tokens) and tokens[pos][0] == "*":
            pos += 1
            lhs = make_prod(lhs, parse_pow())
        return lhs

    def parse_pow() -> object:
        nonlocal pos
        base = parse_atom()
        if pos < len(tokens) and tokens[pos][0] == "POW":
            pos += 1
            kind, val = tokens[pos]
            if kind != "NUM":
                raise ValueError("expected integer exponent")
            pos += 1
            return make_pow(base, int(val))  # type: ignore[arg-type]
        return base

    def parse_atom() -> object:
        nonlocal pos
        kind, val = tokens[pos]
        pos += 1
        if kind == "NUM":
            return Num(int(val))  # type: ignore[arg-type]
        if kind == "IDENT":
            return Var(str(val))
        if kind == "(":
            inner = parse_sum()
            assert tokens[pos][0] == ")"
            pos += 1
            return inner
        raise ValueError(f"unexpected token: {kind}")

    res = parse_sum()
    assert pos == len(tokens)
    return res


@dataclass(frozen=True)
class TreeNode:
    entry: int
    left: TreeNode | None
    right: TreeNode | None


def tree_to_list(tree: TreeNode | None) -> list[int]:
    out: list[int] = []

    def walk(node: TreeNode | None) -> None:
        if node is not None:
            walk(node.left)
            out.append(node.entry)
            walk(node.right)

    walk(tree)
    return out


def tree_height(tree: TreeNode | None) -> int:
    if tree is None:
        return 0
    return 1 + max(tree_height(tree.left), tree_height(tree.right))


def partial_tree(elts: list[int], start: int, n: int) -> tuple[TreeNode | None, int]:
    if n == 0:
        return None, start
    left_size = (n - 1) // 2
    left_tree, mid = partial_tree(elts, start, left_size)
    entry = elts[mid]
    right_size = n - (left_size + 1)
    right_tree, next_idx = partial_tree(elts, mid + 1, right_size)
    return TreeNode(entry, left_tree, right_tree), next_idx


def list_to_balanced_tree(elts: list[int]) -> TreeNode | None:
    tree, consumed = partial_tree(elts, 0, len(elts))
    assert consumed == len(elts)
    return tree


def union_sorted(a: list[int], b: list[int]) -> list[int]:
    i = j = 0
    out: list[int] = []
    while i < len(a) and j < len(b):
        if a[i] < b[j]:
            out.append(a[i])
            i += 1
        elif a[i] > b[j]:
            out.append(b[j])
            j += 1
        else:
            out.append(a[i])
            i += 1
            j += 1
    out.extend(a[i:])
    out.extend(b[j:])
    return out


def intersection_sorted(a: list[int], b: list[int]) -> list[int]:
    i = j = 0
    out: list[int] = []
    while i < len(a) and j < len(b):
        if a[i] < b[j]:
            i += 1
        elif a[i] > b[j]:
            j += 1
        else:
            out.append(a[i])
            i += 1
            j += 1
    return out


def union_set(a: TreeNode | None, b: TreeNode | None) -> TreeNode | None:
    return list_to_balanced_tree(union_sorted(tree_to_list(a), tree_to_list(b)))


def intersection_set(a: TreeNode | None, b: TreeNode | None) -> TreeNode | None:
    return list_to_balanced_tree(intersection_sorted(tree_to_list(a), tree_to_list(b)))


class OrderedSetsInfixDerivTest(unittest.TestCase):
    def test_infix_power_differentiation(self) -> None:
        d = deriv(parse_infix("x ** 4 + 3 * x ** 2 + 7"), "x")
        self.assertEqual(str(d), "((4 * (x ** 3)) + (3 * (2 * x)))")
        self.assertEqual(deriv(parse_infix("(x + 1) ** 2"), "x"), parse_infix("2 * (x + 1)"))

    def test_balanced_tree_logarithmic_height(self) -> None:
        for n in (1, 7, 63, 255, 1023):
            items = list(range(1, n + 1))
            tree = list_to_balanced_tree(items)
            self.assertEqual(tree_to_list(tree), items)
            self.assertLessEqual(tree_height(tree), n.bit_length())

    def test_linear_bst_set_ops(self) -> None:
        a = list_to_balanced_tree([1, 2, 4, 8, 16])
        b = list_to_balanced_tree([2, 3, 4, 5, 16])
        self.assertEqual(tree_to_list(union_set(a, b)), [1, 2, 3, 4, 5, 8, 16])
        self.assertEqual(tree_to_list(intersection_set(a, b)), [2, 4, 16])


if __name__ == "__main__":
    e = parse_infix("x ** 3 + 2 * x + 5")
    print(f"d/dx({e}) = {deriv(e, 'x')}")
    t1 = list_to_balanced_tree([1, 3, 5, 7, 9])
    t2 = list_to_balanced_tree([2, 3, 6, 7, 10])
    print("union =", tree_to_list(union_set(t1, t2)), "intersection =", tree_to_list(intersection_set(t1, t2)))
