# Emap dependency guidance

This is an upstream-derived Rust fixed-capacity map repository, not a trading service. Preserve its actual branch topology and upstream conventions; the aggregate trading dev/main release workflow does not create a dev branch here. Inspect the current branch/upstream before synchronization. Do not force-push or rewrite history.

Read `Cargo.toml`, `README.md` and the relevant `.github/workflows/` check before changing behavior. Use focused deterministic Rust regression tests for map iteration, capacity and ownership changes, then relevant formatting/Clippy checks. Optional serde and benchmark features require their own affected checks. Do not call a revision security-patched based on an old branch name; establish the exact revision and evidence.

Do not run package publishing or release workflows as validation. Documentation-only maintenance needs diff/link checks.
