# mathlib-gcs

## descriptant

### mathlib-gcs

list creates output and header, sends paginated JSON requests with encoded prefix/token, emits items in page order, sums parsed string sizes, and continues exactly while nextPageToken is a nonempty string.

## approximant

### Forward relation

list creates output and header, sends paginated JSON requests with encoded prefix/token, emits items in page order, sums parsed string sizes, and continues exactly while nextPageToken is a nonempty string.

Model observations include returns, retained state and ordered partial effects; on external failures the successful-prefix relation replaces a total-function claim.

#### Composition

list creates output and header, sends paginated JSON requests with encoded prefix/token, emits items in page order, sums parsed string sizes, and continues exactly while nextPageToken is a nonempty string.

Scoped children isolate the declared contiguous source intervals. Their operations compose only where the source calls them; costs of overlapping parent/child descriptions are not added twice. Remaining effect paths retain explicit gaps.

## catalog

## behavior_complement

HTTP error cannot yield item rows for that response; parsing occurs only after successful text. Output/header can already exist when first request fails. Listing does not download object content.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## reverse_reconstruction

For observed TSV, compatible JSON pages may have arbitrary unselected fields and page boundaries; size 0 can arise from '0', absent, nonstring or malformed size. Token history is lost.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## token_transport

Initial request has encoded prefix and no pageToken; subsequent token is encoded bytewise exactly once by http::encode, including any literal percent. Bucket remains raw interpolation.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## size_projection

size= parse_u64(item.size.as_str) else 0. JSON numeric size is ignored; negative string and out-of-range string become zero. bytes is sum under nonoverflowing u64 domain.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## partial_output

File::create and header precede first HTTP. pages increments before JSON parsing; final flush/report require terminal empty/missing token and all preceding writes. No atomic replace or rollback.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## item_order_and_fields

Within each page, JSON array order determines emitted row order; across pages token traversal order determines concatenation. Name/updated/md5 nonstrings project to empty strings. No sorting, duplicate elimination or tab/newline escaping is performed, so exact TSV column shape additionally requires delimiter-free fields.

Subject and source identity: manifest.json. Domain: scoped forward/invariant; relation is source-derived and partial. Evidence: model.md#tests distinguishes actual checks from unexecuted effect paths; finite fixtures do not prove all inputs.

## tests

### Bounded evidence

No native execution of this source's runtime route. Compile/source representation evidence only.

External wave33 mathlib receipt retains native-commands.json, numeric statuses, raw debug/release logs, sorted exact outcomes, mutation logs and original/restored hashes. Seventeen named checks pass twice per profile; six production behavior mutants reject in both profiles, with restored positives. Source syntax is regenerated twice after formatting and compared byte-for-byte. Structural validation tests bindings, not Rust semantics.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## invariant-and-cost

### Invariant and cost

#### Invariant

Absent/nonarray items emit no rows. Missing or nonstring sizes become zero. Output is a prefix on request/parse/write failure. Counters require no overflow for this arithmetic model; bucket is interpolated without encoding.

#### Cost

O(JSON bytes+rows) per page plus full page JSON memory; no cycle detection or finite page limit, so token cycles permit unbounded requests. Network latency dominates and is unmeasured.

#### Domain and gaps

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained before changes. Existing production bytes are preserved; additions are bounded inline tests. Nine source groupants expose ingestion and routing boundaries; typed children bind exact source bytes and reuse their parent syntax projection. Native tests run twice per profile; behavior mutants run as compiled source changes and exact bytes are restored before positive checks.

Important retained observation: network-looking helpers are not all pure (DNS inside github_host and books build); selection deliberately excludes those effects. Mixed-format manifest order is not deterministic, and error paths do not imply rollback.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

### Undated · behavior_complement

Subject: mathlib-gcs, source SHA-256 1c497d6224bcd2c1561bb1f524542f7e8d27ed6dc398b82e81a61bbd320821e8; finite transition observation.

### Undated · reverse_reconstruction

Subject: mathlib-gcs, source SHA-256 1c497d6224bcd2c1561bb1f524542f7e8d27ed6dc398b82e81a61bbd320821e8; finite transition observation.

### Undated · token_transport

Subject: mathlib-gcs, source SHA-256 1c497d6224bcd2c1561bb1f524542f7e8d27ed6dc398b82e81a61bbd320821e8; finite transition observation.

### Undated · size_projection

Subject: mathlib-gcs, source SHA-256 1c497d6224bcd2c1561bb1f524542f7e8d27ed6dc398b82e81a61bbd320821e8; finite transition observation.

### Undated · partial_output

Subject: mathlib-gcs, source SHA-256 1c497d6224bcd2c1561bb1f524542f7e8d27ed6dc398b82e81a61bbd320821e8; finite transition observation.
