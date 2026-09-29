from tools import castle_capability_intake as intake


def test_registry_binds_exact_ash_ex4pm_subject_without_execution_authority():
    assert intake.PROJECTION_SOURCE.endswith(
        "50fdfa20c84205a80c6eb94e916cffbedc4b816e"
    )
    assert intake.OWNER_CAPABILITY == "PROCESS_ANALYTICS"
    assert intake.AUTHORITY_CEILING == "CONSTRUCT"

    donor = intake.donor("seanchatmangpt/ash_ex4pm")
    assert donor is not None
    assert donor["sha"] == "64a21504d3aab0165275cb3e1c9e2fc124d83413"
    assert donor["runtime_placement"] == "ASH_ADAPTER"
    assert not intake.process_execution_authority("seanchatmangpt/ash_ex4pm")
    assert intake.donor("seanchatmangpt/unknown") is None
