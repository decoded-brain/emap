# Graph Report - emap  (2026-10-10)

## Corpus Check
- 28 files · ~11,676 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 5 file(s) not represented in the graph (top: (none) 3, .toml 2)

## Summary
- 368 nodes · 553 edges · 31 communities (13 shown, 18 thin omitted)
- Extraction: 96% EXTRACTED · 3% INFERRED · 1% AMBIGUOUS · INFERRED: 16 edges (avg confidence: 0.88)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `eb2b5317`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- map.rs
- Map<V>
- node.rs
- NodeId
- serialization.rs
- emap README
- ctors_tests.rs
- keys.rs
- compare_with_intmap.rs
- benchmark.rs
- iterators_tests.rs
- insert.rs
- Item
- index.rs
- .next
- .fmt
- Map<V>
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
- Emap instructions
- README.md

## God Nodes (most connected - your core abstractions)
1. `NodeId` - 18 edges
2. `Map<V>` - 17 edges
3. `Map` - 11 edges
4. `Node` - 11 edges
5. `Node<V>` - 11 edges
6. `Map<V>` - 10 edges
7. `emap README` - 9 edges
8. `IntoIter` - 7 edges
9. `Keys` - 7 edges
10. `bench_remove_safe()` - 6 edges

## Surprising Connections (you probably didn't know these)
- `MIT License` --semantically_similar_to--> `REUSE MIT License Copy`  [INFERRED] [semantically similar]
  LICENSE.txt → LICENSES/MIT.txt
- `Benchmark Workflow` --conceptually_related_to--> `IntMap Benchmark Results`  [AMBIGUOUS]
  .github/workflows/benchmark.yml → README.md
- `Cargo Workflow` --conceptually_related_to--> `Rultor Merge Quality Gate`  [AMBIGUOUS]
  .github/workflows/cargo.yml → .rultor.yml
- `Tarpaulin Workflow` --conceptually_related_to--> `Rultor Merge Quality Gate`  [AMBIGUOUS]
  .github/workflows/tarpaulin.yml → .rultor.yml
- `Up Workflow` --conceptually_related_to--> `Semantic-Version Release Pipeline`  [AMBIGUOUS]
  .github/workflows/up.yml → .rultor.yml

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Performance and Release Loop** — github_workflows_benchmark_benchmark_workflow, github_workflows_cargo_cargo_workflow, github_workflows_tarpaulin_tarpaulin_workflow, github_workflows_up_up_workflow, rultor_semantic_version_release_pipeline, readme_intmap_benchmark_results, readme_performance_regression_contribution_gate [INFERRED 0.75]
- **Continuous Quality Suite** — github_workflows_actionlint_actionlint_workflow, github_workflows_bashate_bashate_workflow, github_workflows_cargo_cargo_workflow, github_workflows_markdown_lint_markdown_lint_workflow, github_workflows_pdd_pdd_workflow, github_workflows_shellcheck_shellcheck_workflow, github_workflows_tarpaulin_tarpaulin_workflow, github_workflows_typos_typos_workflow, github_workflows_xcop_xcop_workflow, github_workflows_yamllint_yamllint_workflow, rultor_merge_quality_gate [INFERRED 0.85]
- **License Compliance Suite** — github_workflows_copyrights_copyrights_workflow, github_workflows_reuse_reuse_workflow, license_mit_license, licenses_mit_mit_license_copy [INFERRED 0.95]

## Communities (31 total, 18 thin omitted)

### Community 0 - "map.rs"
Cohesion: 0.09
Nodes (22): checks_key(), clear_and_len(), clears_it_up(), default_clear(), first_used_remove(), Foo, gets_missing_key(), insert_and_check_length() (+14 more)

### Community 2 - "node.rs"
Cohesion: 0.14
Nodes (12): node_complex_type(), node_creation(), node_debug(), node_empty(), node_id_basic(), node_id_const(), node_id_equality(), node_id_undef() (+4 more)

### Community 3 - "NodeId"
Cohesion: 0.14
Nodes (13): &'a Map<V>, Map<V>, IntoIter, IntoValues, Iter, IterMut, Keys, Map (+5 more)

### Community 4 - "serialization.rs"
Cohesion: 0.10
Nodes (5): Map<V>, serde_big_map(), serialize_and_deserialize(), Vi, Vi<V>

### Community 5 - "emap README"
Cohesion: 0.16
Nodes (18): Benchmark Workflow, Cargo Workflow, Copyrights Workflow, REUSE Workflow, Tarpaulin Workflow, Up Workflow, MIT License, REUSE MIT License Copy (+10 more)

### Community 6 - "ctors_tests.rs"
Cohesion: 0.06
Nodes (5): CountedValue, drop_after_partial_with_capacity_some_panics(), Foo, panicking_reinitialization_preserves_old_values_and_drops_new_clones(), PanicOnClone

### Community 7 - "keys.rs"
Cohesion: 0.12
Nodes (11): insert_and_jump_over_next_key(), keys_after_remove(), keys_all_keys_present(), keys_basic(), keys_duplicate_insert(), keys_empty_map(), keys_full_map(), keys_iterator_does_not_drop_stored_value() (+3 more)

### Community 8 - "compare_with_intmap.rs"
Cohesion: 0.23
Nodes (11): compare_ctors_empty(), compare_ctors_prefill(), compare_insert(), compare_keys(), compare_values(), criterion_config(), PASSES, setup_emap_prefilled_safe() (+3 more)

### Community 9 - "benchmark.rs"
Cohesion: 0.27
Nodes (6): benchmark(), benchmark_and_print(), CAP, compare!, measure!, main()

### Community 11 - "insert.rs"
Cohesion: 0.17
Nodes (12): bench_insert_str(), bench_insert_string(), bench_insert_u64(), bench_single_insert_latency(), criterion_config(), SIZES, bench_remove_safe(), bench_remove_unchecked() (+4 more)

### Community 12 - "Item"
Cohesion: 0.29
Nodes (3): IntoIter<'a, V>, Iter<'a, V>, IterMut<'a, V>

### Community 19 - "renovate.json"
Cohesion: 0.50
Nodes (3): config:base, extends, $schema

### Community 21 - "Markdown Lint Workflow"
Cohesion: 0.67
Nodes (3): Markdown Lint Workflow, PDD Workflow, Typos Workflow

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
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 150 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **18 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Keys` connect `NodeId` to `keys.rs`?**
  _High betweenness centrality (0.066) - this node is a cross-community bridge._
- **What connects `emap`, `config:base`, `Map<V>` to the rest of the system?**
  _23 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `map.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09308510638297872 - nodes in this community are weakly interconnected._
- **Why does `NodeId` connect `NodeId` to `node.rs`?**
  _High betweenness centrality (0.065) - this node is a cross-community bridge._
- **Should `node.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13538461538461538 - nodes in this community are weakly interconnected._

### Low-confidence Hints
_AMBIGUOUS edges — the extractor was unsure. Verify before acting on these._

- **What is the exact relationship between `IntMap Benchmark Results` and `Benchmark Workflow`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **What is the exact relationship between `Rultor Merge Quality Gate` and `Cargo Workflow`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._