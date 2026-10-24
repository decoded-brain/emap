# emap local fixtures

Scope: `tests` within this repository. Inherit the repository and workspace
policies; this file adds only local behavior constraints.

- Keep fixtures independent of real accounts and production credentials. Match the actual public behavior and contract of this checkout.
- Preserve deterministic event ordering and independent expected outcomes. Do not weaken assertions, inflate timeouts or change production semantics merely for a green result.
- Follow the active task's verification phase; this directory does not impose an automatic full-suite or coverage campaign.

Local entry points: `benchmark.rs`, `iter_mut_contract.py`.
