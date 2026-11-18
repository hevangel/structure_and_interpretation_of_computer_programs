from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Constant:
    value: str


@dataclass(frozen=True)
class Variable:
    name: str


Term = Constant | Variable


def matches(pattern: Term, fact: str, env: dict[str, str]) -> bool:
    if isinstance(pattern, Constant):
        return pattern.value == fact
    if pattern.name in env:
        return env[pattern.name] == fact
    env[pattern.name] = fact
    return True


def query(
    facts: list[tuple[str, str]], parent: Term, child: Term
) -> list[dict[str, str]]:
    results: list[dict[str, str]] = []
    for a, b in facts:
        bindings: dict[str, str] = {}
        if matches(parent, a, bindings) and matches(child, b, bindings):
            results.append(bindings)
    return results


class LogicRelationsTests(unittest.TestCase):
    def test_bindings_and_consistency(self) -> None:
        facts = [("a", "b"), ("a", "c"), ("x", "x")]
        r = query(facts, Constant("a"), Variable("who"))
        self.assertEqual(len(r), 2)
        same = query(facts, Variable("v"), Variable("v"))
        self.assertEqual(len(same), 1)
        self.assertEqual(same[0]["v"], "x")


if __name__ == "__main__":
    facts = [("ada", "bob"), ("ada", "cora"), ("bob", "dana")]
    result = query(facts, Constant("ada"), Variable("child"))
    print(f"ada has {len(result)} children")
