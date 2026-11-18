import unittest


def ancestors(child: str, facts: list[tuple[str, str]]) -> set[str]:
    visited: set[str] = set()
    frontier = [child]
    while frontier:
        person = frontier.pop()
        for parent, offspring in facts:
            if offspring == person and parent not in visited:
                visited.add(parent)
                # Newly learned parents may themselves have ancestors.
                frontier.append(parent)
    visited.discard(child)  # Cycles never make someone their own ancestor.
    return visited


class AncestorRelationTests(unittest.TestCase):
    def test_transitive_facts(self) -> None:
        facts = [("Ada", "Bob"), ("Bob", "Carol"), ("Ada", "Dan")]
        self.assertEqual(ancestors("Carol", facts), {"Ada", "Bob"})
        self.assertEqual(ancestors("Dan", facts), {"Ada"})
        self.assertEqual(ancestors("Unknown", facts), set())


if __name__ == "__main__":
    facts = [("Ada", "Bob"), ("Bob", "Carol"), ("Ada", "Dan")]
    print(f"Carol's ancestors: {sorted(ancestors('Carol', facts))}")
