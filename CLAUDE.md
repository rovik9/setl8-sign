# setl8-sign

Policy signer for Setl8 (Solana prop-trading platform). The ONLY component that touches a
sector evaluator's signing key. The backend (setl8-back) sends canonical transaction bytes and
context; this service decides whether to sign. Assume the calling backend can be fully
compromised. Smart contracts are never developed here. Sibling repo setl8-verif is the
independent verifier.

## HARD RULES

1. No private keys, seed phrases, wallet files or secrets anywhere: code, tests, fixtures, CI, logs. Tests may use throwaway keys generated at test runtime only.
2. All amount, seq and counter math is integer-only.
3. Policy code never reads the wall clock. Time is passed in as an argument.
4. Never edit interfaces/ or vendor/. They are exported by the owner from the contract repos on his own machine and are read-only for sessions.
5. Never push to main (except the one-time README above). Work on a branch and open a PR.
6. Every commit is authored and committed as: Rovik <therovikdev@gmail.com>. No Co-Authored-By trailer, no AI attribution, no session-link trailer, in commits or PR bodies.
7. If the spec is silent or contradictory, take the smallest safe option and list it under DECISIONS in the PR. Never invent protocol details (request formats, instruction layouts); mark them TODO(interfaces).
8. Smallest possible dependency set. Every new dependency needs a one-line justification in the PR. #![forbid(unsafe_code)] in every crate. Secrets in memory use zeroize. Never log key material or full message bytes, only hashes.

## Repo map

- `crates/policy`: pure decision logic (no I/O, no clock). SignRequest, Decision, Policy.
- `crates/keystore`: `KeyBackend` trait with two key slots. Test-only in-memory backend.
- `crates/service`: the daemon (config loading, request loop). Local listen only.
- `interfaces/`, `vendor/`: read-only, owner-exported. See their READMEs.
- `docs/DESIGN.md`: purpose, threat model, policy v1.
- `scripts/`: `check_identity.sh`, `check_protected_paths.sh` (used by CI).
- `.github/workflows/ci.yml`: fmt, clippy, test, cargo-deny, gitleaks, identity, protected paths.
- `deny.toml`: cargo-deny config. `.claude/settings.json`: attribution disabled.
