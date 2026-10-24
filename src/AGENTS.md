# Map storage implementation

Scope: `src` within this repository. Inherit the repository and workspace
policies; this file adds only local behavior constraints.

- Preserve capacity/key bounds, occupied/free links, length and iterator consistency as one state transition.
- Insertion/replacement/removal must maintain correct drop behavior; cloning and serialization preserve logical entries without leaking internal free-list state.

Local entry points: `lib.rs`, `clone.rs`, `ctors.rs`, `debug.rs`.
