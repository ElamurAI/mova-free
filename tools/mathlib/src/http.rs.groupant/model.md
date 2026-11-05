# mathlib-http

## descriptant

### mathlib-http

Polite gates each GET by elapsed time, returns nested transport/status results, reads text lossily or downloads through .part with content-length/stream cap, rolling hashes, sync and rename. Helpers hex/encode/civil/free_bytes project bytes, time and df output.

## approximant

# Forward relation

Polite gates each GET by elapsed time, returns nested transport/status results, reads text lossily or downloads through .part with content-length/stream cap, rolling hashes, sync and rename. Helpers hex/encode/civil/free_bytes project bytes, time and df output.

Model observations include returns, retained state and ordered partial effects; on external failures the successful-prefix relation replaces a total-function claim.

## Composition

Polite gates each GET by elapsed time, returns nested transport/status results, reads text lossily or downloads through .part with content-length/stream cap, rolling hashes, sync and rename. Helpers hex/encode/civil/free_bytes project bytes, time and df output.

Scoped children isolate the declared contiguous source intervals. Their operations compose only where the source calls them; costs of overlapping parent/child descriptions are not added twice. Remaining effect paths retain explicit gaps.

## catalog

## behavior_complement

Known Content-Length>limit cannot create the .part file. Stream overflow cannot return Fetched. Non-2xx cannot be silently retried inside Polite. Network and filesystem failure paths are source-derived, not exercised.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## reverse_reconstruction

hex has unique byte inverse on even lowercase hex strings; encode is injective on UTF-8 bytes with canonical percent escapes. Lossy UTF-8 text has many compatible raw byte antecedents; Fetched metadata cannot reconstruct response body uniquely.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## percent_codec

Unreserved ASCII bytes survive; every other byte becomes uppercase %HH, so existing '%' is encoded again. Non-ASCII encoding follows UTF-8 bytes, not Unicode scalar values. Actual byte_codecs_and_civil_boundaries covers this.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## download_commit

Headers read before file creation; body chunks counted before cap, then hashed and written. sync precedes last timestamp and rename. Write/sync/create failures may leave temp artifacts or stale pacing timestamp.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## clock_projection

civil uses Euclidean 400-year eras; valid bounded days map Gregorian dates including negative epoch days. utc_now clamps pre-epoch SystemTime error to zero. Delay timing is not tested; civil fixtures are exact.

Domain: stated source predicates, bounded inputs and available resources; arithmetic assumptions and external effect gaps as in the invariant. Operator is the stated relation/projection; complement is a source-derived inner exclusion, reverse retains compatible antecedents. Other views are source-derived partial constraints, not inverse laws.

Evidence: model.md#tests identifies executed helpers and unexecuted paths. No universal guarantee follows from the finite fixtures.

## df_output_projection

free_bytes invokes df -B1 --output=avail, ignores child status and parses only the second stdout line as trimmed u64. A successful status with missing second line errors, while a failed status with numeric second line may return Ok. This process helper remains unexecuted here.

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

Status bodies are capped at 4096 bytes; text bodies have no cap. download succeeds only after bytes<=limit, sync and rename. Error cleanup and pacing reset are branch-specific, not uniformly guaranteed.

#### Cost

hex O(n) output 2n; encode O(n) output n..3n UTF-8 bytes; download streaming O(B) and 64 KiB buffer plus hashes. text retains full body. sleep/HTTP/filesystem/df costs external.

#### Domain and gaps

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective model retained before changes. Existing production bytes are preserved; additions are bounded inline tests. Nine source groupants expose ingestion and routing boundaries; typed children bind exact source bytes and reuse their parent syntax projection. Native tests run twice per profile; behavior mutants run as compiled source changes and exact bytes are restored before positive checks.

Important retained observation: network-looking helpers are not all pure (DNS inside github_host and books build); selection deliberately excludes those effects. Mixed-format manifest order is not deterministic, and error paths do not imply rollback.

No network/DNS/download, real catalog/corpus/model/teacher or database execution. Effect paths are source-derived; finite helper tests are not universal verification. No executable percentage surrogate or speed claim; future caps require metric/domain/reference and remain 1/2/5/10%.

### Undated · behavior_complement

Subject: mathlib-http, source SHA-256 3df63f2ba1d7bcdb4f0bf02fa7c7a1f0feb5817d22f7409456945ccf6a57aa32; finite transition observation.

### Undated · reverse_reconstruction

Subject: mathlib-http, source SHA-256 3df63f2ba1d7bcdb4f0bf02fa7c7a1f0feb5817d22f7409456945ccf6a57aa32; finite transition observation.

### Undated · percent_codec

Subject: mathlib-http, source SHA-256 3df63f2ba1d7bcdb4f0bf02fa7c7a1f0feb5817d22f7409456945ccf6a57aa32; finite transition observation.

### Undated · download_commit

Subject: mathlib-http, source SHA-256 3df63f2ba1d7bcdb4f0bf02fa7c7a1f0feb5817d22f7409456945ccf6a57aa32; finite transition observation.

### Undated · clock_projection

Subject: mathlib-http, source SHA-256 3df63f2ba1d7bcdb4f0bf02fa7c7a1f0feb5817d22f7409456945ccf6a57aa32; finite transition observation.
