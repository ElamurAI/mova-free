# world-wordnet

## model

# world-wordnet

WordNet JSON snapshot projection, lexical candidates and bounded ancestry.

## forward

load sorts JSON paths and builds synsets plus lemma/POS entries, then projects senses and up links. Public wordnet lazily caches the loaded maps forever; a missing, unreadable or entry-less OEWN directory panics with a message naming the directory, ./install.sh and MOVA_OEWN=optional, and only with MOVA_OEWN=optional caches an empty WordNet after one stderr warning (loaded_or_missing). Queries derive lowercase candidate forms and use POS-specific senses; ancestry follows synset IDs breadth-first and emits at most 40 distinct first-member labels.

## invariant

Loaded sense keys and up first labels are lowercased. Synset and sense order remain deterministic under identical directory bytes and dependencies. Mutable external files cannot mutate an already returned WordNet; process-global OnceLock never refreshes.

## cost

For B JSON bytes, V synsets, E hypernym edges, S sense references and maximum per-synset hypernym/member lengths H/M, a conservative load bound is O(B+(V+E+S)log V+S*(H log V+M)), ignoring string widths. Ancestry is O((V+E)log V+40E) for reached graph, memory O(V+E); the output cap does not cap graph work when many labels duplicate. No timing claim.

## composition

knowledge::verb_synonyms directly reads first two verb senses and caches by input key; opchoice::kind caches kind_of by raw word/POS. Both depend on immutable global WordNet lifetime. Native tests exercise actual load->first_sense_in and synset_up_in on supplied data; global OnceLock activation and downstream caches are source-inspected only.

## epoch-memory

First lexical epoch. Actual OEWN schema and knowledge/opchoice callers inspected read-only; native tests use only generated miniature JSON and explicit maps. Old traversal returned all 55 direct parents despite an outer out.len<40 guard. The repair stops at the fortieth distinct label without reordering. Pure snapshot helpers preserve public APIs. No live OEWN activation, cache reload, semantic quality or timing claim.

## checks

Native filter wordnet::wave10_models; see model.md#evidence for fragments or model.md#evidence at the root. Finite supplied fixtures and independent oracles; no live parser, corpus, teacher or global dictionary activation. Partial syntax has opaque macro bodies and no semantic AST. No executable percentage surrogate or speed claim.

## behavior-complement

Universe: fixed successful snapshot and query outputs. Inner absent relation includes >40 ancestry labels, duplicate output labels, returning the start synset solely through a cycle, and nonmatching POS in first_sense. Empty results are not proof a concept has no kind: loading failure, missing entries, POS, normalization and missing links all contribute.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## reverse-reconstruction

For a query result retain every snapshot/query yielding that result under ordered normalization and projection. Truncated senses, synonyms, definitions, labels and failures create many-to-one fibers. Lowercase first-member ancestry cannot recover synset graph or the source directory; even identical labels can come from different meanings.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## cache-epoch

First wordnet() initializes W exactly once from oewn_dir(); a load error or an empty result is fatal unless MOVA_OEWN=optional, which caches empty maps after a warning; later filesystem or MOVA_DATA changes do not trigger retries. Caller caches therefore have implicit process snapshot identity, not filesystem freshness. Native tests prove owned loaded-snapshot independence, not process-static reset behavior.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## first-kind-divergence

kind_of tries candidate forms until a matching POS first sense has a first kind; it may continue after a matching sense with no kind. first_sense/synonyms/kinds_up stop at the first POS sense even if synonyms or ancestry are empty. Do not assume these queries select one identical sense in all empty-kind cases.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## evidence

### Lexical evidence

Old-code witness: five named failures before fixes (raw sparse heads, duplicate-token intersection, missing shared verb, invalid-head panic and cap overshoot). WordNet private snapshot helper extraction preceded the witness without changing traversal. Production mutation common.len()>=2 to >=1 fails judge_finite_set_oracle and two literal tests. Production cap 40 to 39 fails ancestry_exact_output_cap. Both mutations compiled in one native run; failures are named assertions, not compile failures. Original bytes were restored and compared, then positive tests repeated. Final change thereafter only formatted the appended test modules; final bound tests repeat again.

Syntax generated twice per source and compared byte-for-byte. External registry scaffold preserves the base registry under a synthetic root and adds the two owned leaves; groupants repeats are identical and valid, changed reports exactly both sources with no missing registration. Root integration of authoritative registry/parents is separate. Narrow rustfmt covers only appended tests; git diff --check passes. No whole-world Clippy claim.

Actual joins exercised: read_cupt->Sent dense coordinates; judge->count observations plus zero-prediction score/pr; WordNet::load->first_sense_in; owned snapshot replacement vs existing snapshot; synset graph->label sequence. Global expression/idiom matching, global WordNet OnceLock, downstream caches and full school inference were inspected but not executed. No corpus, dataset download, teacher or GPU was used. No timing or percentage-error guarantee is asserted.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first lexical epoch. Actual OEWN schema and knowledge/opchoice callers inspected read-only; native tests use only generated miniature JSON and explicit maps. Old traversal returned all 55 direct parents despite an outer out.len()<40 guard. The repair stops at the fortieth distinct label without reordering. Pure snapshot helpers preserve public APIs. No live OEWN activation, cache reload, semantic quality or timing claim.

### 2026-10-30 · evidence

Heading `### Lexical evidence, 2026-10-30` stays in evidence as `### Lexical evidence`.

Source SHA-256: 2030e62abe1908c664ef2d6113623ac6291faea0e03072f54f612398a32ec666. Partial syntax SHA-256: 3f2b5c0a458ff94bfd0487b2ed172568a82bbe745a73fb6f7f2452a88fb03a8f.

Native filter: cargo test -p world --lib wave10_models -- --test-threads=1; repeat with --release. Eleven tests pass twice in each profile with identical normalized identities/results (SHA-256 83635ae7bd7b29bf6f2e86e7080b21891c03143b57417b374587989e06310ceb). Six MWE tests include 512 finite overlap/duplication cases, greedy-order/count fixtures, sparse/ambiguous/invalid ID files and invalid-head rejection. Five WordNet tests include 1536 independent transitive-closure cases over every three-node directed graph, label-collision/level fixtures, a 55-parent cap fixture, ordered normalization/POS fibers and actual load/lookup of synthetic temporary JSON.

Raw logs are external under $MOVA_DATA/runs/octopus-wave10-lexical-2026-10-30/. Digests:

- debug-bound-1.log: f55d682252ddf462967810cf9e5a3ade8fb05592ea1783ba448f1887b1132022

- debug-bound-2.log: da6ba581a4f22ddc5c85fef5ba0bbb29b24dd87b47dbd8d72367124427f937d2

- release-bound-1.log: 75167b14ddc6325a420d526251bbc34c6d74ef6cba1cd70501c6e1fd9f2c3120

- release-bound-2.log: b80c415943dbd34cc0eb61a745a7646213e131bdec14ffaad722cf6f78d7f069

- old-witness.log: 6df1d2d35ea996a3f0b37318dacebe487362400d627dbc9fa96fc8d6f74531cb

- mutants.log: ad9afebe67a7cd53cfcf90c857fb6103cc6e14917e49d290e5f20c3e55ad7ffe

- groupants-final-1.json: 9cd75ad8ca9f32b8acc26920cd9a57f169a3130296a61e79fc25171f39ffd6e0

- groupants-final-2.json: 9cd75ad8ca9f32b8acc26920cd9a57f169a3130296a61e79fc25171f39ffd6e0

- changed.json: bc99a5f762d597c352cbfbcd7cf3d7c7fdf8ff83795b1c8e476f9a55878a6aa0

### 2026-11-04 · document

#### epoch-memory 2026-11-04

The silent empty fallback degraded a fresh install without any message (Copilot review 2026-11-04). loaded_or_missing makes absence an error by default; install.sh fetches OEWN 2025 JSON with a pinned SHA-256. The test missing_oewn_is_an_error_unless_optional is red when the optional branch is forced (negative control run 2026-11-04). The panic path itself is source-inspected, not executed in tests.
