# onto-wdgraph

## descriptant

### onto-wdgraph

from_edges sort/dedup triples, sort Q endpoints, build upward CSR. ancestors BFS shortest proper depths. Tarjan labels SCCs; break_cycles removes all internal edges of nontrivial SCC/self-loop and rebuilds. Kahn residual count; closure repeated parallel BFS counts/histograms and optional TSV. run_closure auto chooses All iff pairs<=limit else Wiki and writes before final gate refusal.

## approximant

### Forward relation

from_edges sort/dedup triples, sort Q endpoints, build upward CSR. ancestors BFS shortest proper depths. Tarjan labels SCCs; break_cycles removes all internal edges of nontrivial SCC/self-loop and rebuilds. Kahn residual count; closure repeated parallel BFS counts/histograms and optional TSV. run_closure auto chooses All iff pairs<=limit else Wiki and writes before final gate refusal.

#### Scoped equation

Remove R={(s,o,p)∈E:comp(s)=comp(o)∧(size(comp)>1∨s=o)}; keep E\R, sort audit records and rebuild CSR. Kahn returns None iff all n vertices eliminated via indegree-zero queue.

Domain: finite in-memory synthetic inputs and fresh tiny temporary fixtures, valid graph/CSR/rank coordinates when graph APIs are used, sufficient resources and representable integer intermediates. Malformed-input behavior is only claimed where explicitly stated; no universal panic-freedom. External teacher, live dump, corpus and database effects were not executed.

Executable approximant gap: no useful percentage metric established for these categorical/effectful observations. No surrogate or production substitution. Future caps 1/2/5/10% require a metric/domain and evidence; no numerical error bound follows here.

## invariant

# Invariant

Identity across rebuild uses Q, never old index. Different edge kinds remain parallel. Orphan nodes disappear. max_depth0 returns empty. threads0 closure returns zeros without processing; meaningful domain requires workers>0. Histogram depth saturates63; u16 depth arithmetic not guarded.

## Cost

Sort O(E log E), binary endpoint lookup O(E log V), SCC/Kahn O(V+E). Closure worst O(V(V+E)), worker arrays O(TV), output O(reachable pairs). u32 offsets/u16 depths and allocations finite.

Domain: finite in-memory synthetic inputs and fresh tiny temporary fixtures, valid graph/CSR/rank coordinates when graph APIs are used, sufficient resources and representable integer intermediates. Malformed-input behavior is only claimed where explicitly stated; no universal panic-freedom. External teacher, live dump, corpus and database effects were not executed.

## composition

### Composition

from_edges sort/dedup triples, sort Q endpoints, build upward CSR. ancestors BFS shortest proper depths. Tarjan labels SCCs; break_cycles removes all internal edges of nontrivial SCC/self-loop and rebuilds. Kahn residual count; closure repeated parallel BFS counts/histograms and optional TSV. run_closure auto chooses All iff pairs<=limit else Wiki and writes before final gate refusal.

Fragment `cycle-removal` covers bytes [6353,8449); its equation contributes the indicated local relation. Other source branches/effects remain in the parent model; fragment does not inherit a whole-file proof.

Actual joins: wdx::Tsv emits rank-bearing unquoted schemas read by wdgraph and wdlex; wdgraph rebuilds local indices so wdwalk/rank_kinds must remap Q IDs. wdalign source masks serialize through src_names into wdwalk::n_sources_str (all16 categories checked). Wd uses first-step P31 even for present graph nodes, whereas wdwalk::run only adds P31 for absent graph Qs: these are distinct adapters. wdwalk candidates feed coresel confirmation; wdvmm parsing does not prove truth. wsd consumes model::Graph, kb::up/ancestors, query::lookup and core::of_concept; eval scores query::Answer rather than proof correctness. build::Builder consumes Zip::read; central/local offsets and method gates are not full ZIP validation.

## models

## behavior_complement

Exclude retained SCC-internal edges; do not exclude node deletion/index changes. Kahn residual includes blocked downstream nodes, not just cycle members. Duplicate exact triples excluded but differing p retained.

## reverse_reconstruction

edges(from_edges(E)) determines unique triples, not multiplicity/order/isolated vertices. Kept edges plus cycle records reconstruct original unique edges; kept alone admits arbitrary removed components. Reachability admits transitive edge variants.

## csr_transport

Rebuilt graph requires remapping through Q IDs.

## cycle_partition

All internal edges removed, not minimal feedback cut.

## parallel_observation

Counts/histogram deterministic safe arithmetic; max-count Q tie and output shard partition may depend scheduling.

## effect_order

Even Materialize::None writes edges; gate failure may follow files already written.

## tests

### Executed checks

Native command: `cargo test --offline -p onto --bin onto -- --exact --test-threads=1 wdgraph::t::cycles_gate` (add `--release` before `--` for optimized profile). These exact names were included in the inspected selected command, twice per profile. All 21 sorted named outcomes matched across four runs. This source's compiled mutant returned exit101 with a failing assertion; exact source restoration followed by its passing test.

cycles_gate covers removal/renumber, BFS, two-thread count-only closure and injected cycle. Deep200000 chain skipped, no dumps/materialization.

Syntax generated twice identically after restoring source. Evidence receipts: debug-1/debug-2/release-1/release-2 logs, outcomes and exit files; negative-wdgraph, restored-wdgraph, controls.json and native-commands.json in external wave29 ontowiki run.

No strict Clippy claim: inherited dependency/target warnings observed; no production logic changed, focused rustfmt applied to added test modules only.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Full source reviewed before changes; prospective predictions retained in the external wave29 receipt. Added tests preserve production behavior. cycles_gate covers removal/renumber, BFS, two-thread count-only closure and injected cycle. Deep200000 chain skipped, no dumps/materialization. Registration is partial modeling, not semantic proof.
