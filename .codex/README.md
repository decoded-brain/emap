# Codex Local environment

Codex reads [environment.toml](environments/environment.toml) when preparing this
repository worktree. Setup checks the installed Rust toolchain and fetches the
existing Cargo.lock with `cargo fetch --locked`. Install required tools yourself;
setup does not install or upgrade them. Dependency fetching may need network access
and existing Git authentication.

Run setup manually from the repository root:

```sh
rustc --version
cargo --version
cargo fetch --locked
```

Actions can be run manually from the repository root:

- **Check:** `cargo check --all-features --locked --offline`
- **Test:** `cargo test --lib --all-features --locked --offline`
- **Format:** `cargo fmt --all -- --check`

The selected tests are deterministic local checks. Cargo offline mode controls dependency fetching, not
network sockets. These actions do not launch services or live/testnet suites;
run other focused checks according to repository guidance.
