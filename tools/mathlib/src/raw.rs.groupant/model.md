# mathlib-raw

## descriptant

### mathlib-raw

parse_page folds XML events into Rec fields and versions; run reads sorted page files per supplied dir, discards deleted/id-empty records, merges by id taking strictly newer lexical datestamp, then writes sorted-id TSV with normalized category/license/date/size fields.

## approximant

# Forward relation

parse_page folds XML events into Rec fields and versions; run reads sorted page files per supplied dir, discards deleted/id-empty records, merges by id taking strictly newer lexical datestamp, then writes sorted-id TSV with normalized category/license/date/size fields.

Model observations include returns, retained state and ordered partial effects; on external failures the successful-prefix relation replaces a total-function claim.

## Composition

parse_page folds XML events into Rec fields and versions; run reads sorted page files per supplied dir, discards deleted/id-empty records, merges by id taking strictly newer lexical datestamp, then writes sorted-id TSV with normalized category/license/date/size fields.

Scoped children isolate the declared contiguous source intervals. Their operations compose only where the source calls them; costs of overlapping parent/child descriptions are not added twice. Remaining effect paths retain explicit gaps.

## catalog

## behavior_complement

Unknown license host cannot become Open through URL suffix resemblance. Deleted or empty-id records never enter best. A later deletion does not remove an existing id; no tombstone semantics is claimed.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## reverse_reconstruction

TSV row antecedents include XML namespace/whitespace variants and older duplicate records hidden by latest merge. One output id does not recover version chronology beyond recorded encounter order, discarded fields or harvest page layout.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## license_normalization

URL trim, trailing slash removal, ASCII lowercase, optional scheme and repeated www. stripping precede exact domain/path match. Empty->None/Arxiv; unknown->Other. Actual licenses_sizes_dates constrains false-host exclusion.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## merge_ties

Replace old iff r.datestamp>old.datestamp as strings. Tags append only unseen dir basenames. Equal dates retain earlier traversal, and dates are not parsed/validated for merge.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## numeric_projection

size_kb applies suffix multiplier then ceil and Rust float-to-u64 cast: parse failure/NaN/negative becomes zero; excessive positive saturates. iso_date accepts any parsed day/year with recognized month, not Gregorian validation.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## category_and_version_projection

Category token order determines primary before each token's legacy normalization. First/last version dates and last size follow encounter order; size_kb_all sums every version. Permuting versions can change first/last output while preserving sum, so no version-number chronology is inferred.

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

Equal datestamps retain earlier record while adding distinct harvest tags. Deleted records are discarded rather than deleting earlier stored versions. Version order is encounter order, not version-number sort. Whitespace-cleared textual fields have no embedded tabs.

#### Cost

XML decode O(B) per page; storage O(unique record payload plus page bytes). Hash merge expected O(records), sorted ids O(U log U). Size sums require nonoverflowing u64; output O(serialized bytes).

#### Domain and gaps

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained before changes. Existing production bytes are preserved; additions are bounded inline tests. Nine source groupants expose ingestion and routing boundaries; typed children bind exact source bytes and reuse their parent syntax projection. Native tests run twice per profile; behavior mutants run as compiled source changes and exact bytes are restored before positive checks.

Important retained observation: network-looking helpers are not all pure (DNS inside github_host and books build); selection deliberately excludes those effects. Mixed-format manifest order is not deterministic, and error paths do not imply rollback.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

### Undated · behavior_complement

Subject: mathlib-raw, source SHA-256 7d08c4e6f06ecd7eabc2c57092405b6d010841ad384398c17f96e1438c098790; finite transition observation.

### Undated · reverse_reconstruction

Subject: mathlib-raw, source SHA-256 7d08c4e6f06ecd7eabc2c57092405b6d010841ad384398c17f96e1438c098790; finite transition observation.

### Undated · license_normalization

Subject: mathlib-raw, source SHA-256 7d08c4e6f06ecd7eabc2c57092405b6d010841ad384398c17f96e1438c098790; finite transition observation.

### Undated · merge_ties

Subject: mathlib-raw, source SHA-256 7d08c4e6f06ecd7eabc2c57092405b6d010841ad384398c17f96e1438c098790; finite transition observation.

### Undated · numeric_projection

Subject: mathlib-raw, source SHA-256 7d08c4e6f06ecd7eabc2c57092405b6d010841ad384398c17f96e1438c098790; finite transition observation.
