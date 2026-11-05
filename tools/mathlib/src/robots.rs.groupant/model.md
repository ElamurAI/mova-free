# mathlib-robots

## descriptant

### mathlib-robots

Robots parses user-agent groups, selects all matching specific agents else wildcard groups, compiles literal/glob path patterns, and evaluates longest byte-length match with allow tie. Gate caches host rules, rejects GitHub names/addresses, fetches robots on first host and paces approved requests.

## approximant

# Forward relation

Robots parses user-agent groups, selects all matching specific agents else wildcard groups, compiles literal/glob path patterns, and evaluates longest byte-length match with allow tie. Gate caches host rules, rejects GitHub names/addresses, fetches robots on first host and paces approved requests.

Model observations include returns, retained state and ordered partial effects; on external failures the successful-prefix relation replaces a total-function claim.

## Composition

Robots parses user-agent groups, selects all matching specific agents else wildcard groups, compiles literal/glob path patterns, and evaluates longest byte-length match with allow tie. Gate caches host rules, rejects GitHub names/addresses, fetches robots on first host and paces approved requests.

Scoped children isolate the declared contiguous source intervals. Their operations compose only where the source calls them; costs of overlapping parent/child descriptions are not added twice. Remaining effect paths retain explicit gaps.

## catalog

## behavior_complement

Within valid finite-delay states, denied path cannot pass Gate.check; known GitHub host rejects before robots request. A 4xx robots response becomes allow-all, including 403/429, while 5xx/transport errors cache deny-all. No DNS or HTTP routes are executed here.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## reverse_reconstruction

An allowed boolean has many compatible rule sets and paths; false constrains the maximal matching raw pattern length to a disallow with no tied allow. Gate result cannot reconstruct fetch history without cache/clock state.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## agent_selection

Specific group is selected when lowercased agent contains any nonempty nonstar group token; all such groups combine, not longest agent token only. Otherwise all star groups combine. Header grouping ends on any non-user-agent parsed directive.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## match_tie

Best=(max pattern byte length, allow preferred). Equal length allow defeats deny regardless rule order; no match allows. Terminal $ anchors end, * becomes .*, all remaining regex metacharacters escaped.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## delay_domain

crawl-delay folds parsed f64 with max, then Gate delay=max(base,Duration::from_secs_f64(d)). Finite nonnegative representable d is required for no panic; parse alone does not enforce it. Tests use finite seconds and no sleeps.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## cache_origin_alias

Gate keys robots/last/github by lowercased host string only. Two schemes with the same host share robots policy and last timestamp; a previously cached HTTP response can govern HTTPS requests in that Gate. This is actual key projection, not full origin isolation, and remains source-derived.

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

Specific agent selection excludes wildcard groups when any specific match exists. Rule length counts raw pattern bytes including wildcard/end marker. Cache keys are lowercased host strings, not complete origins; clock pacing uses request-start timestamps at Gate and completion timestamps in Polite.

#### Cost

Parsing O(text+compiled regex bytes). allowed scans all rules, regex cost depends on pattern and path lengths. Gate storage O(hosts+rules); DNS, HTTP and sleeps unbounded externally. Nonfinite/negative crawl-delay can panic in Duration conversion.

#### Domain and gaps

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained before changes. Existing production bytes are preserved; additions are bounded inline tests. Nine source groupants expose ingestion and routing boundaries; typed children bind exact source bytes and reuse their parent syntax projection. Native tests run twice per profile; behavior mutants run as compiled source changes and exact bytes are restored before positive checks.

Important retained observation: network-looking helpers are not all pure (DNS inside github_host and books build); selection deliberately excludes those effects. Mixed-format manifest order is not deterministic, and error paths do not imply rollback.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

### Undated · behavior_complement

Subject: mathlib-robots, source SHA-256 84539710d111a1045385202e26b845b3894fbd8883804cbcbdf39135d6d22897; finite transition observation.

### Undated · reverse_reconstruction

Subject: mathlib-robots, source SHA-256 84539710d111a1045385202e26b845b3894fbd8883804cbcbdf39135d6d22897; finite transition observation.

### Undated · agent_selection

Subject: mathlib-robots, source SHA-256 84539710d111a1045385202e26b845b3894fbd8883804cbcbdf39135d6d22897; finite transition observation.

### Undated · match_tie

Subject: mathlib-robots, source SHA-256 84539710d111a1045385202e26b845b3894fbd8883804cbcbdf39135d6d22897; finite transition observation.

### Undated · delay_domain

Subject: mathlib-robots, source SHA-256 84539710d111a1045385202e26b845b3894fbd8883804cbcbdf39135d6d22897; finite transition observation.
