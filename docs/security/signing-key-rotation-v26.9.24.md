# Signing-key rotation (v26.9.24)

Recorded 2026-09-24 (fleet key scan after the single-repo migration). Base `4c6d448eedee` of `process-intelligence`.
Every private key listed here was committed to this repository and is therefore compromised: every receipt or
attestation signed with it carries no signing authority (standing REFUSED, broken_term R_missing_authority).
The keys leave the tree (history is not rewritten; no force-push), and each key directory's `.gitignore` now
covers both halves. Every checkout keeps its own pair: ggen generates one on first use, and a tracked public
half without its private half would make that first `ggen sync` refuse [FM-KEY-010/011]. The canonical
checkout's new public key is published below for anyone verifying its future receipts.

| key dir | removed private key sha256 | removed public key sha256 | new public key (canonical checkout) |
|---|---|---|---|
| `.ggen/keys` | `72d941ae17a5538c564787a4f3ec230aaf6726803100e548a80067994a6c5f06` | `a7db7a220174a56e85477f53958ca2e3ed7c88711b835de8988dd91894b345df` | `3b6fca2c06a418c065617b235ee3bd99388defed8445af7671933825ab5204ce` |
| `ggen/.ggen/keys` | `7a4a720b9ecc59142d87bcecad892e8d5d6c8ff0d563293aa0d426824866f375` | `92d83e16fee107ec420a0c1c108c064399a979979747814a449b03f8149146a7` | `c8a53d72ed6da9ff9cacbd55a8683550960be53dce36cf681750507c84371bc5` |
| `research/pi-program/ggen/.ggen/keys` | `08e63a1e0df3db498a5775f3042597e045d3d364eafd4a55327ac5586276b01f` | `2a7543cb701bf51fef7f3eddf926d181bf2418ddecb447fe5817993da6d1fb3b` | `bc406ba84e1c7b956b602c9d1ab1e609b7097a8e2c805bf39765b5b1acd7df2d` |
| `research/prompt-manufactory/ggen/.ggen/keys` | `3e9f7018b7c3698d08fd57491e67135bf00e288409c9b7bd27dffe38c1afdea5` | `96ff50089d5df930e4b057c4f0d60df040f274795eb7ed1df6ccc3e5e55bcc81` | `179c441c3f06296df4b2114a513689163424db5ce0e83f4859bfd76db90f85b6` |
