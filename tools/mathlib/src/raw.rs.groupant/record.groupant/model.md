# mathlib-raw-record

## descriptant

### mathlib-raw-record

Start record resets accumulator, header status sets deleted, version appends a tuple; recognized leaf text updates fields or latest version. End record moves accumulator to output; EOF returns.

## approximant

### Forward relation

Start record resets accumulator, header status sets deleted, version appends a tuple; recognized leaf text updates fields or latest version. End record moves accumulator to output; EOF returns.

Model observations include returns, retained state and ordered partial effects; on external failures the successful-prefix relation replaces a total-function claim.

## catalog

## behavior_complement

A header-only identifier cannot create nonempty rec.id; deleted records still appear in parse_page output (run filters them later). Unknown leaf text does not become title.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## reverse_reconstruction

Each Rec corresponds to many XML encodings and ignored elements. Repeated title fields retain last recognized text; earlier values and namespace prefixes are not recoverable.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## version_attachment

date/size/source_type update only the last appended version; before any version they have no effect. Encounter order fixes first/last version observations downstream.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## record_boundary

End record pushes current accumulator even with empty id; Start record resets it. Well-formed conventional OAI nesting is the intended domain; malformed nesting lacks a full schema guarantee.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## text_transport

read_text then unescape with raw fallback, clean collapse for title/authors/comments and trim-only for id/license/datestamp. Whitespace equivalence classes differ by field; parses_record checks entities and version values.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## partial_result_on_error

parse_page appends each completed record directly to the caller vector. If a later XML/read_text/attribute error occurs, prior entries remain; the Result error does not roll back caller state. Tests cover successful records only, so the failure-prefix relation is source-derived.

Subject and source identity: manifest.json. Domain: scoped forward/invariant; relation is source-derived and partial. Evidence: model.md#tests distinguishes actual checks from unexecuted effect paths; finite fixtures do not prove all inputs.

## tests

### Bounded evidence

Selected actual source checks:

- mathlib::raw::tests::licenses_sizes_dates
- mathlib::raw::tests::parses_record

These checks are file-level evidence; fragment branches not reached by these names remain source-derived.

External wave33 mathlib receipt retains native-commands.json, numeric statuses, raw debug/release logs, sorted exact outcomes, mutation logs and original/restored hashes. Seventeen named checks pass twice per profile; six production behavior mutants reject in both profiles, with restored positives. Source syntax is regenerated twice after formatting and compared byte-for-byte. Structural validation tests bindings, not Rust semantics.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## invariant-and-cost

### Invariant and cost

#### Invariant

Recognized text fields are normalized independently; header identifier is read but never sets rec.id. check_end_names=false weakens XML validation. Error can leave already appended records in caller vector.

#### Cost

O(XML bytes plus emitted record payload); whole XML is in memory, vectors grow with record/version count.

#### Domain and gaps

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained before changes. Existing production bytes are preserved; additions are bounded inline tests. Nine source groupants expose ingestion and routing boundaries; typed children bind exact source bytes and reuse their parent syntax projection. Native tests run twice per profile; behavior mutants run as compiled source changes and exact bytes are restored before positive checks.

Important retained observation: network-looking helpers are not all pure (DNS inside github_host and books build); selection deliberately excludes those effects. Mixed-format manifest order is not deterministic, and error paths do not imply rollback.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

### Undated · behavior_complement

Subject: mathlib-raw-record, source SHA-256 7d08c4e6f06ecd7eabc2c57092405b6d010841ad384398c17f96e1438c098790; finite transition observation.

### Undated · reverse_reconstruction

Subject: mathlib-raw-record, source SHA-256 7d08c4e6f06ecd7eabc2c57092405b6d010841ad384398c17f96e1438c098790; finite transition observation.

### Undated · version_attachment

Subject: mathlib-raw-record, source SHA-256 7d08c4e6f06ecd7eabc2c57092405b6d010841ad384398c17f96e1438c098790; finite transition observation.

### Undated · record_boundary

Subject: mathlib-raw-record, source SHA-256 7d08c4e6f06ecd7eabc2c57092405b6d010841ad384398c17f96e1438c098790; finite transition observation.

### Undated · text_transport

Subject: mathlib-raw-record, source SHA-256 7d08c4e6f06ecd7eabc2c57092405b6d010841ad384398c17f96e1438c098790; finite transition observation.
