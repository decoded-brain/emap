# Graph Report - emap  (2026-10-24)

## Corpus Check
- 46 files · ~9,654 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 5 file(s) not represented in the graph (top: (none) 3, .toml 2)

## Summary
- 583 nodes · 786 edges · 44 communities (26 shown, 18 thin omitted)
- Extraction: 97% EXTRACTED · 2% INFERRED · 1% AMBIGUOUS · INFERRED: 16 edges (avg confidence: 0.88)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `2ac20e54`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- map.rs
- Map<V>
- node.rs
- Map
- serialization.rs
- emap README
- ctors_tests.rs
- keys.rs
- benches/AGENTS.md
- cargo.yml
- iterators_tests.rs
- compare_with_intmap.rs
- Item
- index.rs
- .next
- src/AGENTS.md
- tests/AGENTS.md
- steps
- .next
- renovate.json
- Map<V>
- Markdown Lint Workflow
- actionlint Workflow
- bashate Workflow
- rebuild_benchmark.sh
- iter_mut_contract.py
- XCOP Workflow
- emap
- debug.rs
- AGENTS.md
- README.md
- markdown-lint
- actionlint
- copyrights
- pdd
- reuse
- shellcheck
- typos
- xcop
- yamllint
- benchmark
- up
- check
- .rultor.yml

## God Nodes (most connected - your core abstractions)
1. `Map` - 23 edges
2. `NodeId` - 21 edges
3. `Map<V>` - 17 edges
4. `Node` - 12 edges
5. `Node<V>` - 11 edges
6. `Map<V>` - 10 edges
7. `emap README` - 9 edges
8. `IntoIter` - 8 edges
9. `IntoValues` - 7 edges
10. `Keys` - 7 edges

## Surprising Connections (you probably didn't know these)
- `Benchmark Workflow` --conceptually_related_to--> `IntMap Benchmark Results`  [AMBIGUOUS]
  .github/workflows/benchmark.yml → README.md
- `Performance Regression Contribution Gate` --semantically_similar_to--> `Rultor Merge Quality Gate`  [INFERRED] [semantically similar]
  README.md → .rultor.yml
- `emap README` --references--> `Cargo Workflow`  [EXTRACTED]
  README.md → .github/workflows/cargo.yml
- `Copyrights Workflow` --conceptually_related_to--> `MIT License`  [INFERRED]
  .github/workflows/copyrights.yml → LICENSE.txt
- `REUSE Workflow` --conceptually_related_to--> `REUSE MIT License Copy`  [INFERRED]
  .github/workflows/reuse.yml → LICENSES/MIT.txt

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Performance and Release Loop** — github_workflows_benchmark_benchmark_workflow, github_workflows_cargo_cargo_workflow, github_workflows_tarpaulin_tarpaulin_workflow, github_workflows_up_up_workflow, rultor_semantic_version_release_pipeline, readme_intmap_benchmark_results, readme_performance_regression_contribution_gate [INFERRED 0.75]
- **Continuous Quality Suite** — github_workflows_actionlint_actionlint_workflow, github_workflows_bashate_bashate_workflow, github_workflows_cargo_cargo_workflow, github_workflows_markdown_lint_markdown_lint_workflow, github_workflows_pdd_pdd_workflow, github_workflows_shellcheck_shellcheck_workflow, github_workflows_tarpaulin_tarpaulin_workflow, github_workflows_typos_typos_workflow, github_workflows_xcop_xcop_workflow, github_workflows_yamllint_yamllint_workflow, rultor_merge_quality_gate [INFERRED 0.85]
- **License Compliance Suite** — github_workflows_copyrights_copyrights_workflow, github_workflows_reuse_reuse_workflow, license_mit_license, licenses_mit_mit_license_copy [INFERRED 0.95]

## Communities (44 total, 18 thin omitted)

### Community 0 - "map.rs"
Cohesion: 0.10
Nodes (23): checks_key(), clear_and_len(), clears_it_up(), default_clear(), first_used_remove(), Foo, gets_missing_key(), insert_and_check_length() (+15 more)

### Community 2 - "node.rs"
Cohesion: 0.14
Nodes (12): node_complex_type(), node_creation(), node_debug(), node_empty(), node_id_basic(), node_id_const(), node_id_equality(), node_id_undef() (+4 more)

### Community 3 - "Map"
Cohesion: 0.10
Nodes (13): &'a Map<V>, Map<V>, IntoIter, IntoValues, Iter, IterMut, Keys, Map (+5 more)

### Community 4 - "serialization.rs"
Cohesion: 0.11
Nodes (5): Map<V>, serde_big_map(), serialize_and_deserialize(), Vi, Vi<V>

### Community 5 - "emap README"
Cohesion: 0.16
Nodes (18): Benchmark Workflow, Cargo Workflow, Copyrights Workflow, REUSE Workflow, Tarpaulin Workflow, Up Workflow, MIT License, REUSE MIT License Copy (+10 more)

### Community 6 - "ctors_tests.rs"
Cohesion: 0.06
Nodes (5): CountedValue, drop_after_partial_with_capacity_some_panics(), Foo, panicking_reinitialization_preserves_old_values_and_drops_new_clones(), PanicOnClone

### Community 7 - "keys.rs"
Cohesion: 0.20
Nodes (11): insert_and_jump_over_next_key(), keys_after_remove(), keys_all_keys_present(), keys_basic(), keys_duplicate_insert(), keys_empty_map(), keys_full_map(), keys_iterator_does_not_drop_stored_value() (+3 more)

### Community 9 - "cargo.yml"
Cohesion: 0.11
Nodes (17): runs-on, steps, timeout-minutes, concurrency, cancel-in-progress, group, jobs, build (+9 more)

### Community 11 - "compare_with_intmap.rs"
Cohesion: 0.07
Nodes (32): compare_ctors_empty(), compare_ctors_prefill(), compare_insert(), compare_keys(), compare_values(), criterion_config(), IntMapI32, PASSES (+24 more)

### Community 12 - "Item"
Cohesion: 0.29
Nodes (3): IntoIter<'a, V>, Iter<'a, V>, IterMut<'a, V>

### Community 17 - "steps"
Cohesion: 0.12
Nodes (15): runs-on, steps, timeout-minutes, jobs, bashate, name, on, pull_request (+7 more)

### Community 19 - "renovate.json"
Cohesion: 0.50
Nodes (3): config:base, extends, $schema

### Community 21 - "Markdown Lint Workflow"
Cohesion: 0.67
Nodes (3): Markdown Lint Workflow, PDD Workflow, Typos Workflow

### Community 31 - "markdown-lint"
Cohesion: 0.13
Nodes (14): jobs, markdown-lint, runs-on, steps, timeout-minutes, name, on, pull_request (+6 more)

### Community 32 - "actionlint"
Cohesion: 0.15
Nodes (12): runs-on, steps, timeout-minutes, jobs, actionlint, name, on, pull_request (+4 more)

### Community 33 - "copyrights"
Cohesion: 0.15
Nodes (12): runs-on, steps, timeout-minutes, jobs, copyrights, name, on, pull_request (+4 more)

### Community 34 - "pdd"
Cohesion: 0.15
Nodes (12): jobs, pdd, name, on, pull_request, push, runs-on, steps (+4 more)

### Community 35 - "reuse"
Cohesion: 0.15
Nodes (12): jobs, reuse, name, on, pull_request, push, branches, branches (+4 more)

### Community 36 - "shellcheck"
Cohesion: 0.15
Nodes (12): jobs, shellcheck, name, on, pull_request, push, branches, branches (+4 more)

### Community 37 - "typos"
Cohesion: 0.15
Nodes (12): jobs, typos, name, on, pull_request, push, branches, branches (+4 more)

### Community 38 - "xcop"
Cohesion: 0.15
Nodes (12): jobs, xcop, name, on, pull_request, push, branches, branches (+4 more)

### Community 39 - "yamllint"
Cohesion: 0.15
Nodes (12): jobs, yamllint, name, on, pull_request, push, branches, branches (+4 more)

### Community 40 - "benchmark"
Cohesion: 0.17
Nodes (11): runs-on, steps, timeout-minutes, jobs, benchmark, name, on, push (+3 more)

### Community 41 - "up"
Cohesion: 0.17
Nodes (11): jobs, up, name, on, push, branches, tags, uses (+3 more)

### Community 42 - "check"
Cohesion: 0.18
Nodes (10): runs-on, steps, timeout-minutes, jobs, check, name, on, pull_request (+2 more)

### Community 43 - ".rultor.yml"
Cohesion: 0.18
Nodes (10): assets, credentials, docker, image, install, merge, script, release (+2 more)

## Ambiguous Edges - Review These
- `IntMap Benchmark Results` → `Benchmark Workflow`  [AMBIGUOUS]
  .github/workflows/benchmark.yml · relation: conceptually_related_to
- `Rultor Merge Quality Gate` → `Cargo Workflow`  [AMBIGUOUS]
  .github/workflows/cargo.yml · relation: conceptually_related_to
- `Rultor Merge Quality Gate` → `Tarpaulin Workflow`  [AMBIGUOUS]
  .github/workflows/tarpaulin.yml · relation: conceptually_related_to
- `Semantic-Version Release Pipeline` → `Up Workflow`  [AMBIGUOUS]
  .github/workflows/up.yml · relation: conceptually_related_to

## Knowledge Gaps
- **23 isolated node(s):** `emap`, `config:base`, `Map<V>`, `Map<V>`, `Iter<'a, V>` (+18 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 252 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **18 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Map` connect `Map` to `map.rs`, `serialization.rs`, `keys.rs`, `iterators_tests.rs`, `compare_with_intmap.rs`, `index.rs`, `debug.rs`?**
  _High betweenness centrality (0.155) - this node is a cross-community bridge._
- **What connects `emap`, `config:base`, `Map<V>` to the rest of the system?**
  _23 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `map.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09898242368177614 - nodes in this community are weakly interconnected._
- **Why does `NodeId` connect `Map` to `map.rs`, `node.rs`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **Should `node.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13538461538461538 - nodes in this community are weakly interconnected._

### Low-confidence Hints
_AMBIGUOUS edges — the extractor was unsure. Verify before acting on these._

- **What is the exact relationship between `IntMap Benchmark Results` and `Benchmark Workflow`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **What is the exact relationship between `Rultor Merge Quality Gate` and `Cargo Workflow`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._