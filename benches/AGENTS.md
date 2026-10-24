# Map benchmarks

Scope: `benches` within this repository. Inherit the repository and workspace
policies; this file adds only local behavior constraints.

- Keep compared workloads, capacity, occupancy, toolchain and features explicit.
- Do not promote benchmark-only unchecked behavior into the safe API or claim speed from incomparable measurements.

Local entry points: `compare_with_intmap.rs`, `insert.rs`, `remove.rs`.
