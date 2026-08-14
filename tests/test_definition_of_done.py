import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("verify_done", ROOT / "tools" / "verify_done.py")
module = importlib.util.module_from_spec(SPEC)
sys.modules["verify_done"] = module
assert SPEC.loader
SPEC.loader.exec_module(module)


class DefinitionOfDoneTests(unittest.TestCase):
    def make_repo(self, manifest: str, files: list[str]):
        td = tempfile.TemporaryDirectory()
        root = Path(td.name)
        (root / "dod").mkdir()
        (root / "dod/process-intelligence.toml").write_text(manifest)
        for rel in files:
            p = root / rel
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text("evidence\n")
        return td, root

    def test_canonical_manifest_has_five_scopes(self):
        manifest = module.load_manifest(ROOT / "dod/process-intelligence.toml")
        scopes = [d["scope"] for d in manifest["definitions"]]
        self.assertEqual(scopes, ["artifact", "evidence", "gate", "repository", "release_candidate"])

    def test_every_definition_has_requirements_and_falsifiers(self):
        manifest = module.load_manifest(ROOT / "dod/process-intelligence.toml")
        for definition in manifest["definitions"]:
            self.assertTrue(definition["requires"])
            self.assertTrue(definition["falsifiers"])

    def test_missing_required_file_refuses_done(self):
        text = (ROOT / "dod/process-intelligence.toml").read_text()
        td, repo = self.make_repo(text, [])
        try:
            receipt = module.inspect(repo, repo / "dod/process-intelligence.toml")
            self.assertEqual(receipt["status"], "REFUSED")
            self.assertTrue(receipt["falsifiers"]["missing_required_file"])
        finally:
            td.cleanup()

    def test_duplicate_scope_refuses_done(self):
        text = (ROOT / "dod/process-intelligence.toml").read_text()
        text += '\n[[definitions]]\nscope="artifact"\nrequires=["x"]\nfalsifiers=["y"]\n'
        files = [i["path"] for i in module.load_manifest(ROOT / "dod/process-intelligence.toml")["required_files"]]
        td, repo = self.make_repo(text, files)
        try:
            receipt = module.inspect(repo, repo / "dod/process-intelligence.toml")
            self.assertEqual(receipt["status"], "REFUSED")
            self.assertTrue(receipt["falsifiers"]["duplicate_scope"])
        finally:
            td.cleanup()

    def test_alive_command_matches_verifier_contract(self):
        manifest = module.load_manifest(ROOT / "dod/process-intelligence.toml")
        command = manifest["commands"]["alive"]
        self.assertIn("--receipt", command)
        self.assertNotIn("--output", command)
        self.assertNotIn("--check", command)

    def test_receipt_digest_is_deterministic(self):
        a = module.inspect(ROOT, ROOT / "dod/process-intelligence.toml")
        b = module.inspect(ROOT, ROOT / "dod/process-intelligence.toml")
        self.assertEqual(a["receipt_sha256"], b["receipt_sha256"])


if __name__ == "__main__":
    unittest.main()
