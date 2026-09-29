from pathlib import Path
import unittest

GRAPH = Path(__file__).resolve().parents[1] / "ecosystem" / "castle-capability-intake.ttl"


class CastleCapabilityIntakeTest(unittest.TestCase):
    def test_ash_ex4pm_is_projection_not_process_authority(self):
        graph = GRAPH.read_text()

        self.assertIn("50fdfa20c84205a80c6eb94e916cffbedc4b816e", graph)
        self.assertIn("seanchatmangpt/ash_ex4pm", graph)
        self.assertIn("64a21504d3aab0165275cb3e1c9e2fc124d83413", graph)
        self.assertIn('eco:ownerCapability "PROCESS_ANALYTICS"', graph)
        self.assertIn('eco:runtimePlacement "ASH_ADAPTER"', graph)
        self.assertIn('eco:projectionStanding "CANDIDATE"', graph)
        self.assertIn('eco:authorityCeiling "CONSTRUCT"', graph)
        self.assertNotIn('eco:projectionStanding "ALIVE"', graph)
        self.assertNotIn('eco:authorityCeiling "DO"', graph)


if __name__ == "__main__":
    unittest.main()
