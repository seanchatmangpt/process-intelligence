"""CASTLE projected capability intake for process-intelligence.

This registry is descriptive and side-effect free. It binds the exact
ash_ex4pm donor subject to PROCESS_ANALYTICS without granting process execution
or consequence authority.
"""

PROJECTION_SOURCE = (
    "seanchatmangpt/ggen-ecosystem@"
    "50fdfa20c84205a80c6eb94e916cffbedc4b816e"
)
OWNER_CAPABILITY = "PROCESS_ANALYTICS"
AUTHORITY_CEILING = "CONSTRUCT"

DONORS = {
    "seanchatmangpt/ash_ex4pm": {
        "sha": "64a21504d3aab0165275cb3e1c9e2fc124d83413",
        "capability": "ASH_PROCESS_PROJECTION",
        "disposition": "WRAP",
        "runtime_placement": "ASH_ADAPTER",
    }
}


def donor(repository: str):
    """Return the projected donor record or None."""
    return DONORS.get(repository)


def process_execution_authority(repository: str) -> bool:
    """Projected analytics adapters never gain process-execution authority."""
    _ = repository
    return False
