# sources-cli

## descriptant

### sources-cli

Dispatch selects plan/verify/merge/bib. load filters before uniqueness/name checks. plan writes provenance, migrates matching-size files, queues remaining by host, deletes old curl configs then writes new. check orders metadata/size, MD5, SHA1, magic. merge resolves slug precedence, subsets then partial ownership.

## approximant

### Forward model

Dispatch selects plan/verify/merge/bib. load filters before uniqueness/name checks. plan writes provenance, migrates matching-size files, queues remaining by host, deletes old curl configs then writes new. check orders metadata/size, MD5, SHA1, magic. merge resolves slug precedence, subsets then partial ownership.

Domain and assumptions: Bounded JSONL with confined safe slugs/names and representable u64 sums. Arbitrary slug confinement is not validated by production. Unknown size accepts any existing length; queue omission is not content verification. Effects nontransactional.

This source-derived relation abstracts allocation, formatting and external failures; no executable percentage surrogate is asserted. Categorical decisions/effects have no supported error metric yet. Future budget caps are 1%, 2%, 5%, 10% only.

## composition

### Local composition

Dispatch selects plan/verify/merge/bib. load filters before uniqueness/name checks. plan writes provenance, migrates matching-size files, queues remaining by host, deletes old curl configs then writes new. check orders metadata/size, MD5, SHA1, magic. merge resolves slug precedence, subsets then partial ownership.

Children are scoped observations of the same source and share syntax.json. Their state/output constraints must agree at actual call boundaries; they are not independent interchangeable stages. The library public namespace joins bibliography row JSON into sources Resource parsing, covered by the bounded plan/check join fixture. Value is an independent tagged orchestration path, not a bibliography caller.

## catalog

# Typed model catalog

Subject: exact manifest source SHA-256 and optional byte scope. Domain/assumptions: forward and invariant companion. Observation is one completed helper transition or the stated IO prefix; external interference is excluded for finite fixtures. All models are partial source-derived specifications; the complement is an inner set of excluded behaviors, the reverse a compatible-antecedent relation. Evidence: model.md#tests; unsampled branches are not proven.

## behavior_complement

Rejected load excludes plan writes. plan never invokes curl. check size failure cannot reach successful digest/magic stages.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## reverse_reconstruction

Queue pairs admit many manifests with different metadata/already-present/skipped files. Merge output admits arbitrary dropped duplicate/subset rows; no unique inverse.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## admission_partition

Filtered kind/status skips before slug accounting; accepted manual rows reserve slug but produce no Planned. JSON/read errors abort while duplicate errors accumulate.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## naming_precedence

save_as beats script fallback and last URL segment. Duplicate automatic names mutate sequentially, not symmetric renaming of every member.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## verification_order

metadata -> size -> stream hashes -> active MD5 -> SHA1 -> extension magic. Empty/hyphenated MD5 skipped; unknown extension imposes empty magic.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## queue_state

Expected-size files skip queue; migration uses size not digest. Old curl configs removed only after resource writes; interruptions may leave partial state.

Operator and approximation: the stated constraint on the source transition, with all unmentioned state held fixed; not a total solver or uniqueness claim. Falsifiable oracle: compare actual helper outputs/effect order on a bounded input satisfying the domain. Current executed subset and unsupported observations are listed in model.md#tests.

## tests

### Checks and limits

Exact target-qualified native names for this source family:

- tests::names_and_hosts
- path_tests::disambiguated_names
- script_tests::script_names
- wave33_tests::bibliography_row_load_plan_check_join
- wave33_tests::check_size_then_digest_then_magic
- wave33_tests::merge_subset_and_first_partial_owner

Run each selected exact name twice debug and twice release, one test thread, offline/locked dependencies. External wave33 sources receipts contain numeric statuses/raw logs and sorted outcome comparisons. Scoped children inherit only relevant evidence: parse/IDs directly exercised; admission/check/merge actual helpers exercised; subprocess actual helper exercised. bench_transport has source-derived evidence only, never an executed benchmark. Library namespace is compile/representation evidence.

Compiled source controls change bibliography license threshold, sources size guard and value status guard; targeted assertions must reject, then exact backup restoration and positive rerun. Finite controls are not comprehensive fault coverage. See model.md#evidence at source root for receipts summary.

## evidence

### Wave33 bounded evidence

Four source groupants, seven scoped child groupants and 66 typed models. Four independently paired syntax projections compare byte-for-byte; children share projections with explicit byte ranges. No new Rust source files.

Seventeen exact target-qualified tests pass twice debug/twice release (68 exits zero), with identical sorted named outcomes. sources library ten; sources binary six; value binary one. Namespace is compile/representation evidence plus actual parse/row projection. Synthetic bibliography-to-Resource-to-plan/check and topic admission fixtures exercise actual helpers. Value executes bounded sh only; bench/main never run.

Three compiled behavior mutants reject at runtime: bibliography license threshold and sources size guard via direct assertions; value disabled exit-status guard via unwrap_err panic on unexpected Ok. Each original/restored SHA-256 pair and byte comparison matches, followed by positive rerun. No assertion-only or surviving mutants. This is finite sensitivity, not universal fault coverage.

Production repair has two connected sites. Entry::anthology_id rejects foreign authorities and accepts canonical case-insensitive HTTP(S), stripping query/fragment before last path segment. Topic run now uses that canonical helper for JSONL admission after rendering abstracts. Old actual-helper witnesses fail (exit 101) for lookalike identity and two-row foreign topic admission; repaired fixtures pass, retaining foreign matched abstract text but only official JSONL. Relative URLs, userinfo and ports unsupported. Other production bytes unchanged; appended modules cfg(test).

Final exact tests, compiled controls and syntax receipts were refreshed after the topic repair. Earlier receipts remain in previous-receipts externally. Base-registry structural validation passes; integration owns registration and ancestor bindings. Initial disconnected augmented-registry attempt remains an honest diagnostic, not a passing check.

Raw logs/statuses, backups/hashes, outcomes, prospective documents, native metadata and production-delta.json reside in external wave33 sources receipts. No external network/data/model/DB, benchmark workflow, publication, universal proof, speed or percentage-surrogate claim.

## invariant-and-cost

### Invariants and costs

Bounded JSONL with confined safe slugs/names and representable u64 sums. Arbitrary slug confinement is not validated by production. Unknown size accepts any existing length; queue omission is not content verification. Effects nontransactional.

M rows,F files,U URL bytes,S disk bytes: naming O(F²+U), directory walk O(entries), hash O(S) and 1 MiB buffer; merge O(M²F) expected hashing. Long path_name remove(0) can be quadratic.

Bounds are symbolic, conditional on stated domain and successful allocation. No unmeasured speedup or universal finite-test guarantee.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective full-source read preceded edits. Model separates source-derived effects from bounded actual-helper evidence. Bibliography host identity old code admitted foreign authorities; a failing fixture motivated the narrow HTTP(S) exact-authority repair. Other production bytes were retained; added test modules are cfg(test). Topic run now uses the same canonical ID guard for JSONL admission; an old-code failing fixture confirms foreign matched entries remain only in abstract listings. Namespace compilation is representation evidence, not independent runtime coverage. Value workflow and bench transport remain unexecuted. No live data, network, model, DB or benchmark invocation was used.

### 2026-10-31 · evidence

Heading `### Wave33 bounded evidence — 2026-10-31` stays in evidence as `### Wave33 bounded evidence`.
