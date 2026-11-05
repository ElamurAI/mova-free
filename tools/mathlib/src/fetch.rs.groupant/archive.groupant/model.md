# mathlib-fetch-archive

## descriptant

### mathlib-fetch-archive

compact_tar_gz streams regular tar entries, keeps recognized text extensions with size<=5<<20, rewrites headers, syncs temporary gzip and renames only when kept>0.

## approximant

### Forward relation

compact_tar_gz streams regular tar entries, keeps recognized text extensions with size<=5<<20, rewrites headers, syncs temporary gzip and renames only when kept>0.

Model observations include returns, retained state and ordered partial effects; on external failures the successful-prefix relation replaces a total-function claim.

## catalog

## behavior_complement

Regular .png entries and text entries above 5 MiB never appear in successful compact output; kept=0 removes temporary file and cannot create dst by rename.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## reverse_reconstruction

Given compact archive, antecedents include arbitrary excluded binary/nonregular entries interleaved with retained text and differing discarded metadata. Retained order and contents constrain regular kept entries; original gzip bytes are not reconstructible.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## extension_filter

Extension is final path suffix lowercased and membership is exact in fourteen strings. Uppercase TEX passes; .tex.png fails. compacts_sources tests text/figure selection.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## header_projection

Kept entries have mode 0644, regular type, original size and mtime or zero. Ownership and other original header data are projected away.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## empty_commit

Completion closes gzip and syncs before checking kept. Zero kept removes tmp and returns None; caller retains original. Error cleanup is not uniformly guaranteed.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## inclusive_entry_cap

The text-size comparison admits exactly 5*2^20 bytes and rejects the next size. The cap is per retained header entry and does not limit total archive bytes, regular dropped entries or decompression work. Boundary execution remains a gap; fixture exercises only tiny files.

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

Nonregular entries are ignored, not counted dropped. kept+dropped equals regular entries encountered on successful scan. No archive path is extracted to disk.

#### Cost

O(decompressed input plus compression work); bounded streaming buffers, no aggregate decompressed-size bound.

#### Domain and gaps

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained before changes. Existing production bytes are preserved; additions are bounded inline tests. Nine source groupants expose ingestion and routing boundaries; typed children bind exact source bytes and reuse their parent syntax projection. Native tests run twice per profile; behavior mutants run as compiled source changes and exact bytes are restored before positive checks.

Important retained observation: network-looking helpers are not all pure (DNS inside github_host and books build); selection deliberately excludes those effects. Mixed-format manifest order is not deterministic, and error paths do not imply rollback.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

### Undated · behavior_complement

Subject: mathlib-fetch-archive, source SHA-256 9a892fd61299338f640f9d8d08471ef7f09cac19235ad80b90d894a12489996d; finite transition observation.

### Undated · reverse_reconstruction

Subject: mathlib-fetch-archive, source SHA-256 9a892fd61299338f640f9d8d08471ef7f09cac19235ad80b90d894a12489996d; finite transition observation.

### Undated · extension_filter

Subject: mathlib-fetch-archive, source SHA-256 9a892fd61299338f640f9d8d08471ef7f09cac19235ad80b90d894a12489996d; finite transition observation.

### Undated · header_projection

Subject: mathlib-fetch-archive, source SHA-256 9a892fd61299338f640f9d8d08471ef7f09cac19235ad80b90d894a12489996d; finite transition observation.

### Undated · empty_commit

Subject: mathlib-fetch-archive, source SHA-256 9a892fd61299338f640f9d8d08471ef7f09cac19235ad80b90d894a12489996d; finite transition observation.
