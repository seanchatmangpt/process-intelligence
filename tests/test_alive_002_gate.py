import importlib.util
import tempfile
import unittest
import sys
from pathlib import Path

MODULE_PATH = Path(__file__).parents[1] / "tools" / "verify_alive_002.py"
spec = importlib.util.spec_from_file_location("verify_alive_002", MODULE_PATH)
gate = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = gate
assert spec.loader is not None
spec.loader.exec_module(gate)


class Alive002GateTests(unittest.TestCase):
    def write(self, root: Path, relative: str, text: str) -> None:
        path = root / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")

    def make_passing_fixture(self, root: Path) -> None:
        filler = "word " * 205
        for i in range(5):
            self.write(root, f"doctrine/d{i}.md", f"## Law\n\n**Source:** sources/papers/p0.md\n\n{filler}")
        for i in range(10):
            self.write(root, f"standards/s{i}.md", "**Authority:** Public standard\n\n## Coverage Mapping\n\nMapped section.\n")
        for i in range(7):
            self.write(root, f"sources/papers/p{i}.md", f"Finding (van der Aalst, 20{10+i}).\n")
        for i in range(2):
            self.write(root, f"gaps/GAP_{i}.md", "**Status:** OPEN\n\n## Resolution Path\n\nBounded repair.\n")

    def test_passing_fixture_is_alive(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            self.make_passing_fixture(root)
            receipt = gate.build_receipt(root)
            self.assertEqual(receipt["status"], "ALIVE")
            self.assertTrue(all(receipt["criteria"].values()))

    def test_zero_open_gaps_is_monotonic_and_alive(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            self.make_passing_fixture(root)
            for path in (root / "gaps").glob("*.md"):
                path.write_text("**Status:** CLOSED\n\n## Resolution Path\n", encoding="utf-8")
            receipt = gate.build_receipt(root)
            self.assertEqual(receipt["status"], "ALIVE")
            self.assertEqual(receipt["counts"]["open_gaps_with_resolution_path"], 0)
            self.assertTrue(receipt["criteria"]["all_open_gaps_have_resolution_path"])

    def test_semantic_headings_and_authority_are_admitted(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            self.make_passing_fixture(root)
            doctrine = root / "doctrine/d0.md"
            doctrine.write_text("# Canonical Definition\n\n**Paper:** Example (2020)\n\n" + "word " * 205, encoding="utf-8")
            standard = root / "standards/s0.md"
            standard.write_text("**Authority:** ISO example\n\n## Runtime Implementation\n\nMapped.\n", encoding="utf-8")
            receipt = gate.build_receipt(root)
            self.assertEqual(receipt["status"], "ALIVE")

    def test_status_parser_admits_supported_repository_spellings(self):
        self.assertEqual(gate.status_of("**Status:** OPEN"), "OPEN")
        self.assertEqual(gate.status_of("**Status**: PARTIAL"), "PARTIAL")
        self.assertEqual(gate.status_of("status: BLOCKED"), "BLOCKED")

    def test_unmitigated_open_gap_fails_closed(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            self.make_passing_fixture(root)
            self.write(root, "gaps/GAP_EXTRA.md", "**Status:** BLOCKED\n\n## Evidence\n\nMissing repair path.\n")
            receipt = gate.build_receipt(root)
            self.assertEqual(receipt["status"], "PARTIAL_ALIVE")
            self.assertIn("gaps/GAP_EXTRA.md", receipt["unmitigated_open_gaps"])


if __name__ == "__main__":
    unittest.main()
