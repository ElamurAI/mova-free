# world-fbench-loading

## forward

Items skip the first line, skip exactly empty lines, parse five tab-separated metadata cells and keep the remaining text verbatim. Gold skips short rows, parses ID/index, inserts ID before frame validity, stably sorts frames by numeric index.

## invariant

Invalid numeric metadata returns Err. Item order and duplicate IDs survive. Duplicate gold indices preserve input order; an unreadable frame can leave an empty gold entry. Header names and split vocabulary are not validated.

## cost

O(B + sum g_i log g_i) time for bytes B and frames g_i per ID; O(B+G) allocated data.

## composition

load_items reads lines.tsv then parse_items; load_gold maps missing file to empty map and otherwise parse_gold. IDs join items to gold; no file version/digest or uniqueness check protects this join.

## epoch-memory

Wave11. Synthetic old-code witnesses failed before narrow repairs: frame display lost attrs/cond; marker spelling priority selected an earlier answer; unchecked arithmetic panicked; dotted page answer 57 was mistaken for an item. Current tests preserve these counterexamples. No optimization, percentage-error surrogate or general correctness claim.

## checks

cargo test -p world --lib fbench::wave11_models -- --test-threads=1. See file-root ../model.md#evidence for repeated debug/release identities and compiled mutants. Syntax is a source-bound partial projection; macro bodies are opaque.

## behavior-complement

Subject/revision: manifest source hash and this scope. Universe: bounded supplied inputs and terminating observations under fixed dependencies and successful allocation. Inner absent-behavior model excludes any transition violating: Invalid numeric metadata returns Err. Item order and duplicate IDs survive. Duplicate gold indices preserve input order; an unreadable frame can leave an empty gold entry. Header names and split vocabulary are not validated. External effects and unmodeled malformed states are unknown, not excluded. Finite tests support only named fixtures.

## reverse-reconstruction

Subject/revision: manifest source hash. Domain: the scoped forward input/output relation, fixed dependencies and successful allocation. Operator is the full compatible preimage: Successful output admits arbitrary first-line headers, ignored gold short rows and unreadable frame text. Item duplicate identities are not recoverable after later map overwrite; error strings do not uniquely identify malformed input. This is a partial source model, not an implemented complete inverse search; no unique causal explanation is inferred.

## coupling

Subject/revision: manifest source hash. Constraint relation on shared identities/intermediate states: load_items reads lines.tsv then parse_items; load_gold maps missing file to empty map and otherwise parse_gold. IDs join items to gold; no file version/digest or uniqueness check protects this join. Source inspection and actual finite joins are distinguished.

## observation-fiber

Subject/revision: manifest source hash. Information-loss relation holds inputs in the same fiber when their declared observation agrees: Empty gold map differs from an empty frame vector at a present ID: selection uses contains_key, so the latter is eligible. This is a documented legacy boundary, not valid annotation certification. Finite correspondence only; unrestricted soundness/completeness is not established.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave11. Synthetic old-code witnesses failed before narrow repairs: frame display lost attrs/cond; marker spelling priority selected an earlier answer; unchecked arithmetic panicked; dotted page answer 57 was mistaken for an item. Current tests preserve these counterexamples. No optimization, percentage-error surrogate or general correctness claim.
