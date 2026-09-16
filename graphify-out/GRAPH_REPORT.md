# Graph Report - emap  (2026-09-16)

## Corpus Check
- 20 files · ~9,877 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 4 file(s) not represented in the graph (top: (none) 3, .toml 1)

## Summary
- 338 nodes · 583 edges · 28 communities (20 shown, 8 thin omitted)
- Extraction: 97% EXTRACTED · 2% INFERRED · 1% AMBIGUOUS · INFERRED: 11 edges (avg confidence: 0.9)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `10bfa03f`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- map.rs
- ctors.rs
- NodeId
- Map
- .deserialize
- emap README
- values.rs
- keys.rs
- compare_with_intmap.rs
- insert.rs
- iterators.rs
- remove.rs
- .next
- index.rs
- .next
- Map<V>
- Map<V>
- .next
- renovate.json
- Map<V>
- Markdown Lint Workflow
- actionlint Workflow
- bashate Workflow
- rebuild_benchmark.sh
- Map<V>
- XCOP Workflow
- emap

## God Nodes (most connected - your core abstractions)
1. `NodeId` - 18 edges
2. `Map<V>` - 17 edges
3. `Map` - 12 edges
4. `Node` - 12 edges
5. `Node<V>` - 11 edges
6. `Map<V>` - 10 edges
7. `emap README` - 9 edges
8. `PanicOnClone` - 8 edges
9. `IntoIter` - 8 edges
10. `Keys` - 8 edges

## Surprising Connections (you probably didn't know these)
- `Performance Regression Contribution Gate` --semantically_similar_to--> `Rultor Merge Quality Gate`  [INFERRED] [semantically similar]
  README.md → .rultor.yml
- `MIT License` --semantically_similar_to--> `REUSE MIT License Copy`  [INFERRED] [semantically similar]
  LICENSE.txt → LICENSES/MIT.txt
- `Benchmark Workflow` --conceptually_related_to--> `IntMap Benchmark Results`  [AMBIGUOUS]
  .github/workflows/benchmark.yml → README.md
- `Cargo Workflow` --conceptually_related_to--> `Rultor Merge Quality Gate`  [AMBIGUOUS]
  .github/workflows/cargo.yml → .rultor.yml
- `Tarpaulin Workflow` --conceptually_related_to--> `Rultor Merge Quality Gate`  [AMBIGUOUS]
  .github/workflows/tarpaulin.yml → .rultor.yml

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Performance and Release Loop** — github_workflows_benchmark_benchmark_workflow, github_workflows_cargo_cargo_workflow, github_workflows_tarpaulin_tarpaulin_workflow, github_workflows_up_up_workflow, rultor_semantic_version_release_pipeline, readme_intmap_benchmark_results, readme_performance_regression_contribution_gate [INFERRED 0.75]
- **Continuous Quality Suite** — github_workflows_actionlint_actionlint_workflow, github_workflows_bashate_bashate_workflow, github_workflows_cargo_cargo_workflow, github_workflows_markdown_lint_markdown_lint_workflow, github_workflows_pdd_pdd_workflow, github_workflows_shellcheck_shellcheck_workflow, github_workflows_tarpaulin_tarpaulin_workflow, github_workflows_typos_typos_workflow, github_workflows_xcop_xcop_workflow, github_workflows_yamllint_yamllint_workflow, rultor_merge_quality_gate [INFERRED 0.85]
- **License Compliance Suite** — github_workflows_copyrights_copyrights_workflow, github_workflows_reuse_reuse_workflow, license_mit_license, licenses_mit_mit_license_copy [INFERRED 0.95]

## Communities (28 total, 8 thin omitted)

### Community 0 - "map.rs"
Cohesion: 0.10
Nodes (24): F, checks_key(), clear_and_len(), clears_it_up(), default_clear(), first_used_remove(), Foo, gets_missing_key() (+16 more)

### Community 1 - "ctors.rs"
Cohesion: 0.12
Nodes (25): Cell, Drop, Rc, calculates_size_of_memory(), drop_after_boundary_panic_without_initialization(), drop_after_partial_with_capacity_some_panics(), drops_correctly(), drops_multiple_values() (+17 more)

### Community 2 - "NodeId"
Cohesion: 0.11
Nodes (17): node_complex_type(), node_creation(), node_debug(), node_empty(), node_id_basic(), node_id_const(), node_id_equality(), node_id_undef() (+9 more)

### Community 3 - "Map"
Cohesion: 0.19
Nodes (14): IntoIterator, Layout, Foo, &'a Map<V>, IntoIter, IntoValues, Iter, IterMut (+6 more)

### Community 4 - ".deserialize"
Cohesion: 0.12
Nodes (19): D, Deserialize, Error, M, Ok, S, Serialize, Map<V> (+11 more)

### Community 5 - "emap README"
Cohesion: 0.16
Nodes (19): Benchmark Workflow, Cargo Workflow, Copyrights Workflow, REUSE Workflow, Tarpaulin Workflow, Up Workflow, MIT License, REUSE MIT License Copy (+11 more)

### Community 6 - "values.rs"
Cohesion: 0.18
Nodes (13): insert_and_jump_over_next(), into_values_basic(), into_values_consumes(), into_values_empty(), into_values_full(), into_values_mixed(), into_values_with_gaps(), Map<V> (+5 more)

### Community 7 - "keys.rs"
Cohesion: 0.22
Nodes (12): insert_and_jump_over_next_key(), keys_after_remove(), keys_all_keys_present(), keys_basic(), keys_duplicate_insert(), keys_empty_map(), keys_full_map(), keys_iterator_does_not_drop_stored_value() (+4 more)

### Community 8 - "compare_with_intmap.rs"
Cohesion: 0.26
Nodes (14): compare_ctors_empty(), compare_ctors_prefill(), compare_insert(), compare_keys(), compare_values(), criterion_config(), PASSES, Criterion (+6 more)

### Community 9 - "insert.rs"
Cohesion: 0.20
Nodes (13): bench_insert_str(), bench_insert_string(), bench_insert_u64(), bench_single_insert_latency(), criterion_config(), Criterion, SIZES, Duration (+5 more)

### Community 10 - "iterators.rs"
Cohesion: 0.23
Nodes (5): insert_and_into_iterate(), insert_and_jump_over_next(), iterate_and_mutate(), Map<V>, V

### Community 11 - "remove.rs"
Cohesion: 0.36
Nodes (10): bench_remove_safe(), bench_remove_unchecked(), CAPACITY, criterion_config(), remove_benchmarks(), Criterion, setup_prefilled_map(), BenchmarkGroup (+2 more)

### Community 12 - ".next"
Cohesion: 0.33
Nodes (6): IntoIter<'a, V>, Iter<'a, V>, IterMut<'a, V>, Item, Iterator, Option

### Community 14 - ".next"
Cohesion: 0.38
Nodes (5): IntoValues<'a, V>, Item, Iterator, Option, Values<'a, V>

### Community 15 - "Map<V>"
Cohesion: 0.33
Nodes (5): Debug, Display, Map<V>, Formatter, Result

### Community 16 - "Map<V>"
Cohesion: 0.40
Nodes (4): Index, IndexMut, Map<V>, V

### Community 18 - ".next"
Cohesion: 0.40
Nodes (4): Keys<'_, V>, Item, Iterator, Option

### Community 19 - "renovate.json"
Cohesion: 0.50
Nodes (3): config:base, extends, $schema

### Community 20 - "Map<V>"
Cohesion: 0.50
Nodes (3): Map<V>, Clone, Self

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
- **24 isolated node(s):** `emap`, `SIZES`, `PASSES`, `SIZES`, `CAPACITY` (+19 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 85 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **8 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `IntMap Benchmark Results` and `Benchmark Workflow`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **What is the exact relationship between `Rultor Merge Quality Gate` and `Cargo Workflow`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **What is the exact relationship between `Rultor Merge Quality Gate` and `Tarpaulin Workflow`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **What is the exact relationship between `Semantic-Version Release Pipeline` and `Up Workflow`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **Why does `Keys` connect `Map` to `NodeId`, `keys.rs`?**
  _High betweenness centrality (0.129) - this node is a cross-community bridge._
- **Why does `NodeId` connect `NodeId` to `Map`?**
  _High betweenness centrality (0.078) - this node is a cross-community bridge._
- **Why does `Map` connect `Map` to `NodeId`, `remove.rs`?**
  _High betweenness centrality (0.075) - this node is a cross-community bridge._