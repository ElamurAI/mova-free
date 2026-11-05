# mathlib-fetch-manifest

## descriptant

### mathlib-fetch-manifest

Partition Done by items[item_idx].slug in a BTreeMap; serialize sorted slug groups to JSONL, then serialize Done in original order to TSV.

## approximant

### Forward relation

Partition Done by items[item_idx].slug in a BTreeMap; serialize sorted slug groups to JSONL, then serialize Done in original order to TSV.

Model observations include returns, retained state and ordered partial effects; on external failures the successful-prefix relation replaces a total-function claim.

## catalog

## behavior_complement

Multiple distinct licenses cannot yield the single-license group's license URL; it is empty. Empty Done writes zero JSONL rows and only TSV header, assuming successful writes.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## reverse_reconstruction

A JSONL group determines a fiber of Done orderings across different slugs and original Item arrays; repeated item_idx and unused Items are not recoverable. TSV preserves Done row order but omits orig metadata.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## license_union

For group license set L, |L|=1 selects first license and status; otherwise label='per file: '+sort(L), status=open. Domain contains valid Done indices.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## format_nondeterminism

format joins distinct final extensions from HashSet; membership is determined, order is unspecified. Exact output comparison must not impose an order absent from the source.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## paired_outputs

JSONL flush precedes TSV create. A TSV failure can coexist with completed JSONL; outputs are not atomically coupled. Source-derived effect model; no actual DB consumer test.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## item_index_coupling

Every Done index selects both its grouping slug and all input metadata from the same Item. Changing a valid index can move a file between groups without changing bytes/hash; out-of-range index panics before serialization. The model requires valid indices and does not infer filesystem existence from manifest rows.

Subject and source identity: manifest.json. Domain: scoped forward/invariant; relation is source-derived and partial. Evidence: model.md#tests distinguishes actual checks from unexecuted effect paths; finite fixtures do not prove all inputs.

## tests

### Bounded evidence

Selected actual source checks:

- mathlib::fetch::tests::compacts_sources
- mathlib::fetch::tests::ext_and_licenses
- mathlib::fetch::wave33::list_signature_hash_and_existing

These checks are file-level evidence; fragment branches not reached by these names remain source-derived.

External wave33 mathlib receipt retains native-commands.json, numeric statuses, raw debug/release logs, sorted exact outcomes, mutation logs and original/restored hashes. Seventeen named checks pass twice per profile; six production behavior mutants reject in both profiles, with restored positives. Source syntax is regenerated twice after formatting and compared byte-for-byte. Structural validation tests bindings, not Rust semantics.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## invariant-and-cost

### Invariant and cost

#### Invariant

Per-group files keep Done order. A single license gives first item's license URL/status; multiple licenses sort code strings and clear group license URL.

#### Cost

O(D log S) grouping plus per-group license sorting O(L log L); whole serialized files in bounded writer buffers plus vectors.

#### Domain and gaps

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained before changes. Existing production bytes are preserved; additions are bounded inline tests. Nine source groupants expose ingestion and routing boundaries; typed children bind exact source bytes and reuse their parent syntax projection. Native tests run twice per profile; behavior mutants run as compiled source changes and exact bytes are restored before positive checks.

Important retained observation: network-looking helpers are not all pure (DNS inside github_host and books build); selection deliberately excludes those effects. Mixed-format manifest order is not deterministic, and error paths do not imply rollback.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

### Undated · behavior_complement

Subject: mathlib-fetch-manifest, source SHA-256 9a892fd61299338f640f9d8d08471ef7f09cac19235ad80b90d894a12489996d; finite transition observation.

### Undated · reverse_reconstruction

Subject: mathlib-fetch-manifest, source SHA-256 9a892fd61299338f640f9d8d08471ef7f09cac19235ad80b90d894a12489996d; finite transition observation.

### Undated · license_union

Subject: mathlib-fetch-manifest, source SHA-256 9a892fd61299338f640f9d8d08471ef7f09cac19235ad80b90d894a12489996d; finite transition observation.

### Undated · format_nondeterminism

Subject: mathlib-fetch-manifest, source SHA-256 9a892fd61299338f640f9d8d08471ef7f09cac19235ad80b90d894a12489996d; finite transition observation.

### Undated · paired_outputs

Subject: mathlib-fetch-manifest, source SHA-256 9a892fd61299338f640f9d8d08471ef7f09cac19235ad80b90d894a12489996d; finite transition observation.
