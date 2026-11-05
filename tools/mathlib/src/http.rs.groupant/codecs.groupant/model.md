# mathlib-http-codecs

## descriptant

### mathlib-http-codecs

hex emits two lowercase digits per byte; encode emits unreserved ASCII or uppercase %HH per byte; civil maps bounded signed day offsets by Euclidean Gregorian eras.

## approximant

### Forward relation

hex emits two lowercase digits per byte; encode emits unreserved ASCII or uppercase %HH per byte; civil maps bounded signed day offsets by Euclidean Gregorian eras.

Model observations include returns, retained state and ordered partial effects; on external failures the successful-prefix relation replaces a total-function claim.

## catalog

## behavior_complement

Canonical encode output never contains literal space, slash or non-ASCII; hex contains only lowercase ASCII hexadecimal characters.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## reverse_reconstruction

hex decode and canonical percent decode uniquely recover bytes on their image. Civil inverse determines day number only for valid Gregorian triples in the bounded nonoverflowing domain; utc_now formatting loses subsecond precision.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## byte_length

UTF-8 é has two encoded bytes and output %C3%A9 length six; counting code points would give a wrong cost/output-length model.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## escape_idempotence

encode is not idempotent: encoding %2F gives %252F. It is a value encoder, not a normalizer of already-escaped URLs.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## era_boundary

civil(-1)=1969-12-31 and civil(11017)=2000-03-01; leap handling follows century/400-year terms. Fixtures constrain boundaries, not all i64 arithmetic.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## calendar_projection_scope

civil's output is a date triple; utc_now computes days=secs/86400 and rem=secs%86400, then appends whole-second clock fields. Different subsecond instants share output; civil itself has no clock or filesystem effects. Extreme i64 offsets can overflow before era decomposition and are outside the stated domain.

Subject and source identity: manifest.json. Domain: scoped forward/invariant; relation is source-derived and partial. Evidence: model.md#tests distinguishes actual checks from unexecuted effect paths; finite fixtures do not prove all inputs.

## tests

### Bounded evidence

Selected actual source checks:

- mathlib::http::tests::encodes_and_dates
- mathlib::http::wave33::byte_codecs_and_civil_boundaries

These checks are file-level evidence; fragment branches not reached by these names remain source-derived.

External wave33 mathlib receipt retains native-commands.json, numeric statuses, raw debug/release logs, sorted exact outcomes, mutation logs and original/restored hashes. Seventeen named checks pass twice per profile; six production behavior mutants reject in both profiles, with restored positives. Source syntax is regenerated twice after formatting and compared byte-for-byte. Structural validation tests bindings, not Rust semantics.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## invariant-and-cost

### Invariant and cost

#### Invariant

Hex output length=2n. Encoded output length=n+2k for k non-unreserved bytes. Empty input maps empty for both codecs.

#### Cost

O(input bytes) codecs; civil fixed arithmetic work, subject to nonoverflowing signed offsets.

#### Domain and gaps

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained before changes. Existing production bytes are preserved; additions are bounded inline tests. Nine source groupants expose ingestion and routing boundaries; typed children bind exact source bytes and reuse their parent syntax projection. Native tests run twice per profile; behavior mutants run as compiled source changes and exact bytes are restored before positive checks.

Important retained observation: network-looking helpers are not all pure (DNS inside github_host and books build); selection deliberately excludes those effects. Mixed-format manifest order is not deterministic, and error paths do not imply rollback.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

### Undated · behavior_complement

Subject: mathlib-http-codecs, source SHA-256 3df63f2ba1d7bcdb4f0bf02fa7c7a1f0feb5817d22f7409456945ccf6a57aa32; finite transition observation.

### Undated · reverse_reconstruction

Subject: mathlib-http-codecs, source SHA-256 3df63f2ba1d7bcdb4f0bf02fa7c7a1f0feb5817d22f7409456945ccf6a57aa32; finite transition observation.

### Undated · byte_length

Subject: mathlib-http-codecs, source SHA-256 3df63f2ba1d7bcdb4f0bf02fa7c7a1f0feb5817d22f7409456945ccf6a57aa32; finite transition observation.

### Undated · escape_idempotence

Subject: mathlib-http-codecs, source SHA-256 3df63f2ba1d7bcdb4f0bf02fa7c7a1f0feb5817d22f7409456945ccf6a57aa32; finite transition observation.

### Undated · era_boundary

Subject: mathlib-http-codecs, source SHA-256 3df63f2ba1d7bcdb4f0bf02fa7c7a1f0feb5817d22f7409456945ccf6a57aa32; finite transition observation.
