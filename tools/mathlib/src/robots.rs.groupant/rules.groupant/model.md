# mathlib-robots-rules

## descriptant

### mathlib-robots-rules

Parse textual groups into allowed patterns and optional max crawl-delay; allowed chooses matching rule with greatest raw byte length and then allow on tie, default true.

## approximant

### Forward relation

Parse textual groups into allowed patterns and optional max crawl-delay; allowed chooses matching rule with greatest raw byte length and then allow on tie, default true.

Model observations include returns, retained state and ordered partial effects; on external failures the successful-prefix relation replaces a total-function claim.

## catalog

## behavior_complement

Disallow /a.b$ cannot match /axb because dot is escaped; it cannot match /a.b?x because end anchor requires termination. A tied Allow cannot be overridden by same-length Disallow.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## reverse_reconstruction

For false at path p, compatible rule sets have a matching maximal-length deny and no matching allow at that length. Arbitrary unmatched rules or shorter matches can be added without changing false.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## specific_override

If any group agent is a substring of requested agent, wildcard groups are excluded entirely; wildcard denial does not combine with specific allowances.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## wildcard_compilation

Translate p by removing terminal $, splitting *, escaping each segment, joining .*, prefixing ^, then restoring $. Length ranking still uses original bytes, not regex length or matched literal count.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## delay_fold

Only chosen groups contribute crawl-delay; invalid numeric strings are ignored. Last or max semantics for NaN depend on f64::max and initial state; finite nonnegative domain is the testable pacing model.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## raw_length_specificity

Specificity uses v.len() in UTF-8 bytes, counting '*' and terminal '$'. Thus ranking differs from literal-character count or matched-prefix length. For equal resulting boolean, raw patterns with different bytes remain distinct reconstruction possibilities; tie tests only constrain equal raw lengths.

Subject and source identity: manifest.json. Domain: scoped forward/invariant; relation is source-derived and partial. Evidence: model.md#tests distinguishes actual checks from unexecuted effect paths; finite fixtures do not prove all inputs.

## tests

### Bounded evidence

Selected actual source checks:

- mathlib::robots::wave33::tie_specificity_and_literal_pattern
- mathlib::robots::wave33::delay_and_url_without_dns

These checks are file-level evidence; fragment branches not reached by these names remain source-derived.

External wave33 mathlib receipt retains native-commands.json, numeric statuses, raw debug/release logs, sorted exact outcomes, mutation logs and original/restored hashes. Seventeen named checks pass twice per profile; six production behavior mutants reject in both profiles, with restored positives. Source syntax is regenerated twice after formatting and compared byte-for-byte. Structural validation tests bindings, not Rust semantics.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## invariant-and-cost

### Invariant and cost

#### Invariant

Pattern translation escapes regex literals, uses ^ prefix, optional $ suffix, and .* between star-separated parts. Empty allow/disallow values contribute no rules.

#### Cost

O(text+regex compilation) to construct; matching scans every stored regex against path, storage O(pattern bytes).

#### Domain and gaps

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained before changes. Existing production bytes are preserved; additions are bounded inline tests. Nine source groupants expose ingestion and routing boundaries; typed children bind exact source bytes and reuse their parent syntax projection. Native tests run twice per profile; behavior mutants run as compiled source changes and exact bytes are restored before positive checks.

Important retained observation: network-looking helpers are not all pure (DNS inside github_host and books build); selection deliberately excludes those effects. Mixed-format manifest order is not deterministic, and error paths do not imply rollback.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

### Undated · behavior_complement

Subject: mathlib-robots-rules, source SHA-256 84539710d111a1045385202e26b845b3894fbd8883804cbcbdf39135d6d22897; finite transition observation.

### Undated · reverse_reconstruction

Subject: mathlib-robots-rules, source SHA-256 84539710d111a1045385202e26b845b3894fbd8883804cbcbdf39135d6d22897; finite transition observation.

### Undated · specific_override

Subject: mathlib-robots-rules, source SHA-256 84539710d111a1045385202e26b845b3894fbd8883804cbcbdf39135d6d22897; finite transition observation.

### Undated · wildcard_compilation

Subject: mathlib-robots-rules, source SHA-256 84539710d111a1045385202e26b845b3894fbd8883804cbcbdf39135d6d22897; finite transition observation.

### Undated · delay_fold

Subject: mathlib-robots-rules, source SHA-256 84539710d111a1045385202e26b845b3894fbd8883804cbcbdf39135d6d22897; finite transition observation.
