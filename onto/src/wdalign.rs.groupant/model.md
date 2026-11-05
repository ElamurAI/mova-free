# onto-wdalign

## descriptant

### onto-wdalign

Oewn indexes keys/ILI/noun lemmas, descendant count through depth64, generic_limit=(noun_count*share) as u32. gather ORs source masks. Wd unions Q graph start and first P31 steps, folk depth20, min depths, removes self. decide seeks proper OEWN depth10 supports below generic limit; then count>=min_src and per-synset lexicographic rank direct,gloss>=2,depth,wiki desc,Q asc.

## approximant

### Forward relation

Oewn indexes keys/ILI/noun lemmas, descendant count through depth64, generic_limit=(noun_count*share) as u32. gather ORs source masks. Wd unions Q graph start and first P31 steps, folk depth20, min depths, removes self. decide seeks proper OEWN depth10 supports below generic limit; then count>=min_src and per-synset lexicographic rank direct,gloss>=2,depth,wiki desc,Q asc.

#### Scoped equation

A(s,q)={(sa,qa,ds,dq):sa proper OEWN ancestor ds<=10,desc(sa)<=limit,qa proper WD ancestor dq<=20,(sa,qa) candidate}. Pick first minimum ds+dq. If A empty Unsupported; else n(mask)<min_src Single; else per-s rank K=(no_direct,no_gloss,ds+dq,Reverse(wiki),q); first Accepted, rest Also.

Domain: finite in-memory synthetic inputs and fresh tiny temporary fixtures, valid graph/CSR/rank coordinates when graph APIs are used, sufficient resources and representable integer intermediates. Malformed-input behavior is only claimed where explicitly stated; no universal panic-freedom. External teacher, live dump, corpus and database effects were not executed.

Executable approximant gap: no useful percentage metric established for these categorical/effectful observations. No surrogate or production substitution. Future caps 1/2/5/10% require a metric/domain and evidence; no numerical error bound follows here.

## invariant

# Invariant

n_sources=label+lex+(ili∨wn). Candidate anchor need not itself be accepted. Support equal-depth ties retain first traversal candidate. Direct sources bypass label eligibility. Unsupported=no support; Single=support but too few categories.

## Cost

O(C log C) candidate sorts plus cached OEWN BFS, folk walk per distinct Q and support cross-products. Shard scans retain graph/candidate/text state. u16 depth sums bounded by intended search depths; float share cast follows Rust saturation.

Domain: finite in-memory synthetic inputs and fresh tiny temporary fixtures, valid graph/CSR/rank coordinates when graph APIs are used, sufficient resources and representable integer intermediates. Malformed-input behavior is only claimed where explicitly stated; no universal panic-freedom. External teacher, live dump, corpus and database effects were not executed.

## composition

### Composition

Oewn indexes keys/ILI/noun lemmas, descendant count through depth64, generic_limit=(noun_count*share) as u32. gather ORs source masks. Wd unions Q graph start and first P31 steps, folk depth20, min depths, removes self. decide seeks proper OEWN depth10 supports below generic limit; then count>=min_src and per-synset lexicographic rank direct,gloss>=2,depth,wiki desc,Q asc.

Fragment `support-decision` covers bytes [10610,13805); its equation contributes the indicated local relation. Other source branches/effects remain in the parent model; fragment does not inherit a whole-file proof.

Actual joins: wdx::Tsv emits rank-bearing unquoted schemas read by wdgraph and wdlex; wdgraph rebuilds local indices so wdwalk/rank_kinds must remap Q IDs. wdalign source masks serialize through src_names into wdwalk::n_sources_str (all16 categories checked). Wd uses first-step P31 even for present graph nodes, whereas wdwalk::run only adds P31 for absent graph Qs: these are distinct adapters. wdwalk candidates feed coresel confirmation; wdvmm parsing does not prove truth. wsd consumes model::Graph, kb::up/ancestors, query::lookup and core::of_concept; eval scores query::Answer rather than proof correctness. build::Builder consumes Zip::read; central/local offsets and method gates are not full ZIP validation.

## models

## behavior_complement

Exclude Accepted/Also without support or sufficient source count. Exclude ILI+WN counting twice. Unsupported candidate may serve as support. HashSet-rendered report lists lack universal deterministic order.

## reverse_reconstruction

Accepted(s,q) admits candidate pools with a valid ancestor pair and no better eligible Q for s. OR-masks lose duplicate/provenance history; ancestor depth does not recover path or CSR indices.

## source_mask_transport

src_names lex/ili/wn/label order, n_sources equals wdwalk::n_sources_str for all16 masks.

## ranking_threshold

Gloss100 and2 equal boolean rank; direct source beats gloss disagreement.

## coordinate_join

Q→idx→folk traversal→nodes[ancestor] transports identity.

## effect_boundary

run writes align-sample/report; write_rows feeds wdwalk/coresel and never confirms truth.

## tests

### Executed checks

Native command: `cargo test --offline -p onto --bin onto -- --exact --test-threads=1 wdalign::t::rng_and_shuffle wdalign::wave29_ontowiki::source_masks_and_p31_join` (add `--release` before `--` for optimized profile). These exact names were included in the inspected selected command, twice per profile. All 21 sorted named outcomes matched across four runs. This source's compiled mutant returned exit101 with a failing assertion; exact source restoration followed by its passing test.

All16 source masks join to wdwalk strings, P31 hierarchy join and seeded shuffle executed. Full gather/decide/run/gold/database/report paths unexecuted.

Syntax generated twice identically after restoring source. Evidence receipts: debug-1/debug-2/release-1/release-2 logs, outcomes and exit files; negative-wdalign, restored-wdalign, controls.json and native-commands.json in external wave29 ontowiki run.

No strict Clippy claim: inherited dependency/target warnings observed; no production logic changed, focused rustfmt applied to added test modules only.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Full source reviewed before changes; prospective predictions retained in the external wave29 receipt. Added tests preserve production behavior. All16 source masks join to wdwalk strings, P31 hierarchy join and seeded shuffle executed. Full gather/decide/run/gold/database/report paths unexecuted. Registration is partial modeling, not semantic proof.
