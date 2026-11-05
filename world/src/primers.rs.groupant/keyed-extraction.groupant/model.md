# world-primers-keyed-extraction

## forward

Scan page headers and numbered answers; consume the first answer token before later item delimiters. Paged lookup searches approximate page windows and first matching problem number. OpenStax uses chapter/item keys and last numeric answer token with filters, then sorted files.

## invariant

BTreeMap duplicate keys overwrite earlier values. A bare dotted integer immediately after an item is its answer, including 57. after 27.; absent-answer versus next-label ambiguity remains. Page/chapter parsing is heuristic, with malformed input tolerated.

## cost

Answer token scan O(B) apart from allocations; section association worst O(A S P) for A answers, S sections and P chunks/section. File I/O and normalization costs are separate.

## composition

Fixture checks page91/item26=40, page91/item27=57 and page92/item26=5. Filesystem and OpenStax extraction are source inspected only. Page +/-1 arithmetic is unchecked for arbitrary extreme metadata.

## epoch-memory

Wave11. Synthetic old-code witnesses failed before narrow repairs: frame display lost attrs/cond; marker spelling priority selected an earlier answer; unchecked arithmetic panicked; dotted page answer 57 was mistaken for an item. Current tests preserve these counterexamples. No optimization, percentage-error surrogate or general correctness claim.

## checks

cargo test -p world --lib primers::wave11_models -- --test-threads=1. See file-root ../model.md#evidence for repeated debug/release identities and compiled mutants. Syntax is a source-bound partial projection; macro bodies are opaque.

## behavior-complement

Subject/revision: manifest source hash and this scope. Universe: bounded supplied inputs and terminating observations under fixed dependencies and successful allocation. Inner absent-behavior model excludes any transition violating: BTreeMap duplicate keys overwrite earlier values. A bare dotted integer immediately after an item is its answer, including 57. after 27.; absent-answer versus next-label ambiguity remains. Page/chapter parsing is heuristic, with malformed input tolerated. External effects and unmodeled malformed states are unknown, not excluded. Finite tests support only named fixtures.

## reverse-reconstruction

Subject/revision: manifest source hash. Domain: the scoped forward input/output relation, fixed dependencies and successful allocation. Operator is the full compatible preimage: For a keyed answer retain all headers, repeated overwritten keys and admissible answer spellings producing that map. Selected section coordinates require scan order as latent state; map alone loses discarded candidates. This is a partial source model, not an implemented complete inverse search; no unique causal explanation is inferred.

## coupling

Subject/revision: manifest source hash. Constraint relation on shared identities/intermediate states: Fixture checks page91/item26=40, page91/item27=57 and page92/item26=5. Filesystem and OpenStax extraction are source inspected only. Page +/-1 arithmetic is unchecked for arbitrary extreme metadata. Source inspection and actual finite joins are distinguished.

## observation-fiber

Subject/revision: manifest source hash. Information-loss relation holds inputs in the same fiber when their declared observation agrees: Last duplicate key wins while first matching section wins; these two policies are not interchangeable. Numeric-only dotted answer versus omitted answer is not disambiguated by the grammar. Finite correspondence only; unrestricted soundness/completeness is not established.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave11. Synthetic old-code witnesses failed before narrow repairs: frame display lost attrs/cond; marker spelling priority selected an earlier answer; unchecked arithmetic panicked; dotted page answer 57 was mistaken for an item. Current tests preserve these counterexamples. No optimization, percentage-error surrogate or general correctness claim.
