# Fixed-capacity integer map

Read `/Users/stefanbalvanovich/decodedtrading/AGENTS.md` explicitly, including
when this component or a linked worktree is opened separately. These local rules
add domain constraints; the global/workspace workflow, delegation and Graphify
policy remain authoritative. Read applicable deeper `AGENTS.md` files before
changing their scope; do not load unrelated sibling guides.

- Rust map with usize keys, fixed capacity and explicit occupied/free-slot bookkeeping.
- Preserve key bounds, iterator behavior, drop/clone/serialization semantics and free-list integrity. Do not trade memory safety for a benchmark result.
- Use Cargo.toml and existing source organization; optional unchecked benchmarking behavior is not permission to weaken the ordinary API.

Local entry points: `Cargo.toml`, `REUSE.toml`.
