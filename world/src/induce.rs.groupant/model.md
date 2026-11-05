# world-induce

## descriptant

### world-induce

acode(f,v)=((f<<16)|v)+1; pack1..3 sorted codes at21bit offsets. Fix=number of head-correct/label-wrong words carrying subset and current/from,target/gold; Break=head-and-label-correct words carrying(from,subset). Sort (-Fix+Break,atom_count,Rule). apply changes labels using per-rule feature snapshot; run needs precision>=.85 and Fix>=min+Break. explore_all gd>0,gh>=0 reports p without significance filter; selfplay adoption needs p<.05.

## approximant

### Forward relation

acode(f,v)=((f<<16)|v)+1; pack1..3 sorted codes at21bit offsets. Fix=number of head-correct/label-wrong words carrying subset and current/from,target/gold; Break=head-and-label-correct words carrying(from,subset). Sort (-Fix+Break,atom_count,Rule). apply changes labels using per-rule feature snapshot; run needs precision>=.85 and Fix>=min+Break. explore_all gd>0,gh>=0 reports p without significance filter; selfplay adoption needs p<.05.

Domain: Synthetic aligned Sent/gold/Feats passed directly to real scoring; no annotator, corpus, journal, shipped-cache, training/adopt or GPU paths.

No executable percent surrogate: these categorical/effect observations lack a useful approximation metric. Future supported caps1/2/5/10% need a defined domain/metric and real reference evidence; no error or speed bound is claimed.

## invariant

Explicit total order removes FxMap iteration dependence. Empty LAS0, bootstrap1;1000 fixed-seed resamples, all positive deltas p0. parse_rules preserves valid atom order/duplicates. Add exact-dedups while holds_in checks broader coverage, not identical operation effect.

Cost component: [cost](model.md#cost).

## cost

Subset work W*(A+C(A,2)+C(A,3)), candidate sort O(G log G), local per-worker counts/maps. Feature extraction scans sentence dependents roughlyO(L^2) plus delegated graph costs. u32 counts/indices and i64 sums require representability. No timing claim.

## composition

family transfers Change/scope then adds admission/guard rules. big matrix pseudo-labels exclude matrix feature predicates for new rules. GPU adapter only flattens head-and-label-correct words; equivalence requires valid widths/count bounds.

## models

## behavior_complement

Wrong-head words excluded from both counts. apply cannot alter heads/forms. Invalid atom fragments silently dropped; empty conjunction accepted. explore_changes can return p>=.05. Atom/parser feature domain not enforced at public parse_rules; malformed values can panic show/encoding.

## reverse_reconstruction

Valid packed subset uniquely decodes its1..3 atoms, but leaves omitted atoms unknown. Count vectors admit word/sentence permutations and arbitrary wrong-head additions. Changed label to admits prior from matches and prior to; inverse needs ordered rules/feature context. Change removal preimage preserves arbitrary removed multiplicity.

## world_induce_invariants

Explicit total order removes FxMap iteration dependence. Empty LAS0, bootstrap1;1000 fixed-seed resamples, all positive deltas p0. parse_rules preserves valid atom order/duplicates. Add exact-dedups while holds_in checks broader coverage, not identical operation effect.

## world_induce_resource_cost

Subset work W*(A+C(A,2)+C(A,3)), candidate sort O(G log G), local per-worker counts/maps. Feature extraction scans sentence dependents roughlyO(L^2) plus delegated graph costs. u32 counts/indices and i64 sums require representability. No timing claim.

## world_induce_representation_join

family transfers Change/scope then adds admission/guard rules. big matrix pseudo-labels exclude matrix feature predicates for new rules. GPU adapter only flattens head-and-label-correct words; equivalence requires valid widths/count bounds.

## world_induce_domain_partition

Synthetic aligned Sent/gold/Feats passed directly to real scoring; no annotator, corpus, journal, shipped-cache, training/adopt or GPU paths.

## tests

### Finite actual-code evidence

Names: ["induce::wave29_worldlearn::counting_requires_correct_heads_and_orders_candidates", "induce::wave29_worldlearn::packed_subsets_and_empty_bootstrap", "induce::wave29_worldlearn::permissive_parse_and_set_valued_changes"]

Selected CPU command: CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=ISOLATED_TARGET cargo test --offline --locked -p world --lib --example retention_probe [--release] wave29_worldlearn -- --test-threads=1

15 selected cases passed twice debug and twice release with identical sorted names/outcomes, exits0. control-induce: compiled source mutation failed with exit101; source restored byte-for-byte and15positive tests passed. No live teacher, corpus, state activation, GPU, or full-school run. Temporary synthetic files use unique process-id paths and are removed after passing tests.

Pure codes/counting/parsing/changes/threshold tests; feature extraction, persistent effects and live learning remain partial, no surrogate.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source/branch/cost hypotheses were saved externally before source edits. This partial source-derived model retains observed boundaries rather than claiming complete verification. Pure codes/counting/parsing/changes/threshold tests; feature extraction, persistent effects and live learning remain partial, no surrogate.

### 2026-10-31 · tests

Heading `### Finite actual-code evidence — 2026-10-31` stays in tests as `### Finite actual-code evidence`.

Source SHA256: 0297cbe15b922cb5aeebd8632fbb9bcb3d0d8da3e6a6dd287b0978638c1fe2ee. Syntax generated independently twice and compared; lexical only, no semantic proof or CUDA compilation. Strict whole-crate Clippy was not repeated merely to reproduce existing dependency/source lints; Cargo warnings retained in receipts.
