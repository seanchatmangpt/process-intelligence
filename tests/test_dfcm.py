import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("dfcm", ROOT / "tools" / "dfcm.py")
module = importlib.util.module_from_spec(SPEC)
sys.modules["dfcm"] = module
assert SPEC.loader
SPEC.loader.exec_module(module)


class DfcmTests(unittest.TestCase):
    def manifest(self, text: str) -> Path:
        tmp = tempfile.NamedTemporaryFile("w", suffix=".toml", delete=False)
        tmp.write(text)
        tmp.close()
        return Path(tmp.name)

    def test_canonical_space_is_large_and_admitted(self):
        data = module.load_manifest(ROOT / "dfcm" / "process-intelligence.toml")
        admitted, refused = module.enumerate_candidates(data)
        self.assertGreaterEqual(len(admitted), 128)
        self.assertEqual(len(admitted) + len(refused), 7 * 6 * 5 * 4 * 4 * 2)

    def test_every_do_candidate_is_refused(self):
        data = module.load_manifest(ROOT / "dfcm" / "process-intelligence.toml")
        admitted, refused = module.enumerate_candidates(data)
        self.assertTrue(all(row["candidate"]["effect"] == "construct_only" for row in admitted))
        self.assertTrue(any(row["refusal"]["code"] == module.REASON_DO for row in refused))

    def test_board_claim_requires_receipt_and_replay(self):
        data = module.load_manifest(ROOT / "dfcm" / "process-intelligence.toml")
        admitted, _ = module.enumerate_candidates(data)
        board = [row["candidate"] for row in admitted if row["candidate"]["projection"] == "board_claim"]
        self.assertTrue(board)
        self.assertTrue(all(c["evidence"] == "receipts" and c["verification"] == "replay" for c in board))

    def test_deterministic_selection_and_receipt(self):
        manifest = ROOT / "dfcm" / "process-intelligence.toml"
        first = module.compile_plan(manifest, ROOT)
        second = module.compile_plan(manifest, ROOT)
        self.assertEqual(first["selected"], second["selected"])
        self.assertEqual(first["receipt_sha256"], second["receipt_sha256"])

    def test_invalid_do_mode_is_refused(self):
        path = self.manifest('''
[meta]
name="x"
version="1"
mode="DO"
[policy]
actuation_axis="effect"
construct_value="construct_only"
[[axes]]
name="effect"
values=["construct_only","external_state"]
''')
        with self.assertRaises(ValueError):
            module.load_manifest(path)

    def test_rule_prunes_invalid_operationalization(self):
        data = module.load_manifest(ROOT / "dfcm" / "process-intelligence.toml")
        admitted, _ = module.enumerate_candidates(data)
        invalid = [
            r for r in admitted
            if r["candidate"]["phase"] == "operationalization"
            and r["candidate"]["verification"] in {"static", "semantic"}
        ]
        self.assertEqual(invalid, [])

    def test_selected_rows_are_ranked_stably(self):
        receipt = module.compile_plan(ROOT / "dfcm" / "process-intelligence.toml", ROOT)
        scores = [row["score"] for row in receipt["selected"]]
        self.assertEqual(scores, sorted(scores, reverse=True))
        self.assertEqual(receipt["status"], "ALIVE")
        self.assertFalse(receipt["falsifiers"]["do_path_admitted"])


if __name__ == "__main__":
    unittest.main()
