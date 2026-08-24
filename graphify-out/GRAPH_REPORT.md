# Graph Report - emap  (2026-08-24)

## Corpus Check
- Corpus is ~9,877 words - fits in a single context window. You may not need a graph.

## Summary
- 332 nodes · 577 edges · 28 communities (21 shown, 7 thin omitted)
- Extraction: 97% EXTRACTED · 2% INFERRED · 1% AMBIGUOUS · INFERRED: 11 edges (avg confidence: 0.9)
- Token cost: at least 149,608 input · 19,304 output; subsequent host-agent usage was unavailable
- Integrity warning: 45 dangling-endpoint edges and 22 undirected same-endpoint collapses; this graph is usable but lossy

## Community Hubs (Navigation)
- Community 0
- Community 1
- Community 2
- Community 3
- Community 4
- Community 5
- Community 6
- Community 7
- Community 8
- Community 9
- Community 10
- Community 11
- Community 12
- Community 13
- Community 14
- Community 15
- Community 16
- Community 18
- Community 19
- Community 20
- Community 21
- Community 22
- Community 23
- Community 24
- Community 25
- Community 26
- Community 27

## God Nodes (most connected - your core abstractions)
1. `Map<V>` - 17 edges
2. `NodeId` - 17 edges
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

## Communities (28 total, 7 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.10
Nodes (24): F, checks_key(), clear_and_len(), clears_it_up(), default_clear(), first_used_remove(), Foo, gets_missing_key() (+16 more)

### Community 1 - "Community 1"
Cohesion: 0.12
Nodes (25): Cell, Drop, Rc, calculates_size_of_memory(), drop_after_boundary_panic_without_initialization(), drop_after_partial_with_capacity_some_panics(), drops_correctly(), drops_multiple_values() (+17 more)

### Community 2 - "Community 2"
Cohesion: 0.13
Nodes (15): node_complex_type(), node_creation(), node_debug(), node_empty(), node_id_basic(), node_id_const(), node_id_equality(), node_id_undef() (+7 more)

### Community 3 - "Community 3"
Cohesion: 0.17
Nodes (15): IntoIterator, Layout, Foo, &'a Map<V>, IntoIter, IntoValues, Iter, IterMut (+7 more)

### Community 4 - "Community 4"
Cohesion: 0.12
Nodes (19): D, Deserialize, Error, M, Ok, S, Serialize, Map<V> (+11 more)

### Community 5 - "Community 5"
Cohesion: 0.16
Nodes (19): Benchmark Workflow, Cargo Workflow, Copyrights Workflow, REUSE Workflow, Tarpaulin Workflow, Up Workflow, MIT License, REUSE MIT License Copy (+11 more)

### Community 6 - "Community 6"
Cohesion: 0.18
Nodes (13): insert_and_jump_over_next(), into_values_basic(), into_values_consumes(), into_values_empty(), into_values_full(), into_values_mixed(), into_values_with_gaps(), Map<V> (+5 more)

### Community 7 - "Community 7"
Cohesion: 0.22
Nodes (12): insert_and_jump_over_next_key(), keys_after_remove(), keys_all_keys_present(), keys_basic(), keys_duplicate_insert(), keys_empty_map(), keys_full_map(), keys_iterator_does_not_drop_stored_value() (+4 more)

### Community 8 - "Community 8"
Cohesion: 0.32
Nodes (12): compare_ctors_empty(), compare_ctors_prefill(), compare_insert(), compare_keys(), compare_values(), criterion_config(), Criterion, setup_emap_prefilled_safe() (+4 more)

### Community 9 - "Community 9"
Cohesion: 0.24
Nodes (11): bench_insert_str(), bench_insert_string(), bench_insert_u64(), bench_single_insert_latency(), criterion_config(), Criterion, Duration, benchmark() (+3 more)

### Community 10 - "Community 10"
Cohesion: 0.23
Nodes (5): insert_and_into_iterate(), insert_and_jump_over_next(), iterate_and_mutate(), Map<V>, V

### Community 11 - "Community 11"
Cohesion: 0.42
Nodes (9): bench_remove_safe(), bench_remove_unchecked(), criterion_config(), remove_benchmarks(), Criterion, setup_prefilled_map(), BenchmarkGroup, T (+1 more)

### Community 12 - "Community 12"
Cohesion: 0.33
Nodes (6): IntoIter<'a, V>, Iter<'a, V>, IterMut<'a, V>, Item, Iterator, Option

### Community 14 - "Community 14"
Cohesion: 0.38
Nodes (5): IntoValues<'a, V>, Item, Iterator, Option, Values<'a, V>

### Community 15 - "Community 15"
Cohesion: 0.33
Nodes (5): Debug, Display, Map<V>, Formatter, Result

### Community 16 - "Community 16"
Cohesion: 0.40
Nodes (4): Index, IndexMut, Map<V>, V

### Community 18 - "Community 18"
Cohesion: 0.40
Nodes (4): Keys<'_, V>, Item, Iterator, Option

### Community 19 - "Community 19"
Cohesion: 0.50
Nodes (3): config:base, extends, $schema

### Community 20 - "Community 20"
Cohesion: 0.50
Nodes (3): Map<V>, Clone, Self

### Community 21 - "Community 21"
Cohesion: 0.67
Nodes (3): Markdown Lint Workflow, PDD Workflow, Typos Workflow

## Ambiguous Edges - Review These
- `Benchmark Workflow` → `IntMap Benchmark Results`  [AMBIGUOUS]
  .github/workflows/benchmark.yml · relation: conceptually_related_to
- `Cargo Workflow` → `Rultor Merge Quality Gate`  [AMBIGUOUS]
  .github/workflows/cargo.yml · relation: conceptually_related_to
- `Tarpaulin Workflow` → `Rultor Merge Quality Gate`  [AMBIGUOUS]
  .github/workflows/tarpaulin.yml · relation: conceptually_related_to
- `Up Workflow` → `Semantic-Version Release Pipeline`  [AMBIGUOUS]
  .github/workflows/up.yml · relation: conceptually_related_to

## Knowledge Gaps
- **18 isolated node(s):** `emap`, `rebuild_benchmark.sh script`, `$schema`, `config:base`, `Foo` (+13 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **7 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `Benchmark Workflow` and `IntMap Benchmark Results`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **What is the exact relationship between `Cargo Workflow` and `Rultor Merge Quality Gate`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **What is the exact relationship between `Tarpaulin Workflow` and `Rultor Merge Quality Gate`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **What is the exact relationship between `Up Workflow` and `Semantic-Version Release Pipeline`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **Why does `Keys` connect `Community 3` to `Community 7`?**
  _High betweenness centrality (0.130) - this node is a cross-community bridge._
- **Why does `NodeId` connect `Community 3` to `Community 2`?**
  _High betweenness centrality (0.075) - this node is a cross-community bridge._
- **Why does `Map` connect `Community 3` to `Community 11`?**
  _High betweenness centrality (0.072) - this node is a cross-community bridge._
