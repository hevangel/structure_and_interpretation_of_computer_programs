from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Branch:
    length: int
    hanging: "int | Mobile"


@dataclass(frozen=True)
class Mobile:
    left: Branch
    right: Branch


def structure_weight(structure: int | Mobile) -> int:
    if isinstance(structure, int):
        return structure
    return total_weight(structure)


def total_weight(mobile: Mobile) -> int:
    return structure_weight(mobile.left.hanging) + structure_weight(mobile.right.hanging)


def branch_balanced(branch: Branch) -> bool:
    if isinstance(branch.hanging, int):
        return True
    return balanced(branch.hanging)


def balanced(mobile: Mobile) -> bool:
    # A balanced parent may still contain an unbalanced child mobile.
    return (
        mobile.left.length * structure_weight(mobile.left.hanging)
        == mobile.right.length * structure_weight(mobile.right.hanging)
        and branch_balanced(mobile.left)
        and branch_balanced(mobile.right)
    )


class BalancedMobilesTests(unittest.TestCase):
    def test_checks_recursive_balance(self) -> None:
        inner = Mobile(Branch(1, 2), Branch(1, 2))
        outer = Mobile(Branch(1, inner), Branch(2, 2))
        self.assertEqual(total_weight(outer), 6)
        self.assertTrue(balanced(outer))
        wrong = Mobile(Branch(1, 2), Branch(1, 4))
        self.assertFalse(balanced(wrong))


if __name__ == "__main__":
    inner = Mobile(Branch(1, 2), Branch(1, 2))
    mobile = Mobile(Branch(1, inner), Branch(2, 2))
    print(f"weight {total_weight(mobile)}, balanced {balanced(mobile)}")
