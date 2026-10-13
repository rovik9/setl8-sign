# setl8-sign design

## Purpose

The application never holds a signing key. It sends canonical transaction bytes and context to
the signer. The signer verifies them against policy and either signs or refuses. The signer is
the only component that ever touches a sector evaluator's signing key.

## Threat model

The calling backend (setl8-back) may be fully compromised. The signer must still limit the
damage to what policy already allows: an attacker who controls every request can obtain only
signatures that policy would have granted anyway, at no more than the permitted rate.

## Policy v1

- One key per sector, never exported.
- An allowlist of program IDs and instructions per key. For a sector key the instructions are
  post_journal, report_fail, confirm_pass and hold_account, for that sector only.
- A strictly increasing seq per account, persisted durably before the signature is returned.
- Ceilings on batches per minute and on confirm_pass per hour.
- A fresh signed heartbeat from the verifier (setl8-verif) is required. A stale heartbeat means
  refuse everything (dead-man switch).
- A halt flag refuses everything.
- Any parse failure refuses.
- Every decision logs only hashes and reason codes, never key material or full message bytes.

All amount, seq and counter math is integer-only. Policy code never reads the wall clock; time
is passed in as an argument.

## Rotation

Each key has two slots, with an overlap window during which both are valid. The on-chain config
change needs both admin keys and is done by the owner offline, never by this repo.

## Key backends

Either a hardware security module that the owner buys and operates, or an encrypted keyfile as
an interim. Neither is implemented yet; the keystore crate defines only the trait.

## Persistence

The smallest durable store that can hold monotonic counters, with fsync before a signature is
returned. The chosen option is recorded under DECISIONS in the scaffold PR.

## Non-goals

Business logic, price checks, trade validation, and any outbound network access.
