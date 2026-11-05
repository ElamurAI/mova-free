# onto-eval

## descriptant

### onto-eval

read_frozen: bytes→SHA256→optional with_extension(sha256) gate→UTF8→TSV>=4. score branches verdict gold first, frame values before Unknown fallback. run converts query errors to Unknown. tally zips Q/results, preserves first group encounter, appends total; shuffled permutes taxonomic targets then rebuilds CSR.

## approximant

### Forward relation

read_frozen: bytes→SHA256→optional with_extension(sha256) gate→UTF8→TSV>=4. score branches verdict gold first, frame values before Unknown fallback. run converts query errors to Unknown. tally zips Q/results, preserves first group encounter, appends total; shuffled permutes taxonomic targets then rebuilds CSR.

#### Scoped equation

S(a,g)=Correct if g∈{yes,no,undetermined} and code(a)=g; otherwise Unknown iff verdict Unknown else Wrong. For g=unknown, Correct iff Unknown else Wrong. For frame g: Correct iff ∃v∈values:v≈ASCII g, else Unknown iff Unknown else Wrong.

Domain: finite in-memory synthetic inputs and fresh tiny temporary fixtures, valid graph/CSR/rank coordinates when graph APIs are used, sufficient resources and representable integer intermediates. Malformed-input behavior is only claimed where explicitly stated; no universal panic-freedom. External teacher, live dump, corpus and database effects were not executed.

Executable approximant gap: no useful percentage metric established for these categorical/effectful observations. No surrogate or production substitution. Future caps 1/2/5/10% require a metric/domain and evidence; no numerical error bound follows here.

## invariant

# Invariant

Frame match is Correct even for Unknown verdict. Hash sidecar absence allowed; mismatch precedes decoding. The final appended total row sums to min(Q.len,R.len); input group named total creates another total row, and group_acc selects the first matching label, which can be that subgroup. Empty yields total[0,0,0]. Taxonomic target multiset and non-taxonomic edges survive shuffle; topology need not.

## Cost

O(B) hash/parse; score O(value bytes); tally O(min(Q,R)*groups); query costs additive. Shuffle O(E log E+V), storage O(E+V). Counters and offsets require representability.

Domain: finite in-memory synthetic inputs and fresh tiny temporary fixtures, valid graph/CSR/rank coordinates when graph APIs are used, sufficient resources and representable integer intermediates. Malformed-input behavior is only claimed where explicitly stated; no universal panic-freedom. External teacher, live dump, corpus and database effects were not executed.

## composition

### Composition

read_frozen: bytes→SHA256→optional with_extension(sha256) gate→UTF8→TSV>=4. score branches verdict gold first, frame values before Unknown fallback. run converts query errors to Unknown. tally zips Q/results, preserves first group encounter, appends total; shuffled permutes taxonomic targets then rebuilds CSR.

Fragment `answer-scoring` covers bytes [1878,2692); its equation contributes the indicated local relation. Other source branches/effects remain in the parent model; fragment does not inherit a whole-file proof.

Actual joins: wdx::Tsv emits rank-bearing unquoted schemas read by wdgraph and wdlex; wdgraph rebuilds local indices so wdwalk/rank_kinds must remap Q IDs. wdalign source masks serialize through src_names into wdwalk::n_sources_str (all16 categories checked). Wd uses first-step P31 even for present graph nodes, whereas wdwalk::run only adds P31 for absent graph Qs: these are distinct adapters. wdwalk candidates feed coresel confirmation; wdvmm parsing does not prove truth. wsd consumes model::Graph, kb::up/ancestors, query::lookup and core::of_concept; eval scores query::Answer rather than proof correctness. build::Builder consumes Zip::read; central/local offsets and method gates are not full ZIP validation.

## models

## behavior_complement

Exclude parsing after hash mismatch; Correct unknown gold with non-Unknown verdict; tally counts outside zipped prefix. Do not exclude Correct frame on Unknown. leak_check compares verdict only, not values/proofs.

## reverse_reconstruction

Correct/frame g admits every Answer containing an ASCII-case equivalent g with arbitrary verdict/other values. Tally admits arbitrary IDs/queries/gold and unmatched suffix. Shuffled result admits different original target arrangements preserving multiset.

## aggregation_projection

Encounter-order groups; majority tie chooses lexicographically maximal gold.

## hash_failure_boundary

Replacement extension .sha256, first whitespace token; hash mismatch before UTF8 before TSV width.

## graph_target_transport

CSR sort retains metadata and taxonomic target multiplicities, not ancestor reachability.

## report_scope

report uses fixed frozen path text and zip prefix; presentation does not establish freeze was checked.

## tests

### Executed checks

Native command: `cargo test --offline -p onto --bin onto -- --exact --test-threads=1 eval::wave29_ontowiki::frozen_hash_before_utf8 eval::wave29_ontowiki::score_and_zip_tally` (add `--release` before `--` for optimized profile). These exact names were included in the inspected selected command, twice per profile. All 21 sorted named outcomes matched across four runs. This source's compiled mutant returned exit101 with a failing assertion; exact source restoration followed by its passing test.

Scoring, tally, empty majority and tiny hash/UTF8 failure ordering executed. Query execution, graph shuffle/leak/unknown controls/report effects unexecuted; no frozen benchmark set read.

Syntax generated twice identically after restoring source. Evidence receipts: debug-1/debug-2/release-1/release-2 logs, outcomes and exit files; negative-eval, restored-eval, controls.json and native-commands.json in external wave29 ontowiki run.

No strict Clippy claim: inherited dependency/target warnings observed; no production logic changed, focused rustfmt applied to added test modules only.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Full source reviewed before changes; prospective predictions retained in the external wave29 receipt. Added tests preserve production behavior. Scoring, tally, empty majority and tiny hash/UTF8 failure ordering executed. Query execution, graph shuffle/leak/unknown controls/report effects unexecuted; no frozen benchmark set read. Registration is partial modeling, not semantic proof.
