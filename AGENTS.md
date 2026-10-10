# Emap instructions

This is an upstream-derived Rust fixed-capacity integer-key map library,
not a trading service. Preserve its actual checkout, upstream conventions
and branch topology; do not introduce the trading Dev/release workflow.

Read Cargo.toml, README.md and the relevant .github/workflows check for the
affected behavior. Cargo.toml declares Rust 2024, optional serde and
bench_unchecked. Do not assume a workspace-wide MSRV.

## Work and verification

Analysis is read-only unless implementation is requested. Before edits inspect
root, branch, upstream and status; preserve unrelated work. Safely synchronize
only when compatible with the task. Use isolation for substantial changes.
Commit validated owned changes. Do not rewrite history, discard others' work,
push, publish crates or run release workflows without authorization.

For non-trivial behavior changes, use focused deterministic regressions.
Check affected iteration, capacity, insertion/removal and ownership behavior.
Check serde or benchmark features when affected; benchmark numerical speed claims
on comparable workloads. A previous branch name does not prove a security fix.

Keep test bodies/helpers in dedicated test files. Run applicable Cargo tests
and Clippy for the affected feature set. Docs-only changes need diff/link checks.

Rust formatting is the final code-changing step after implementation,
tests and review: cargo fmt --all, then cargo fmt --all -- --check.
Keep formatting task-scoped; do not add intermediate formatting hooks.

When delegating inside the aggregate, use its current saved profiles and
economical-delegation policy. Do not hardcode an obsolete scout effort here.
