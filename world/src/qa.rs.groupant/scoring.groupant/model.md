# world-qa-scoring

## forward

TSV requires six fields and numeric document identity. answer scans ordered || readings and selects the first Ok, otherwise joins all errors. verdict first separates unavailable state; each comma group independently requires an ordered sequence of substring alternatives. The earliest end among alternative matches dominates later ends for every remaining suffix. Matching is case-insensitive substring matching, not token identity.

## invariant

Successful readings stop fallback; not-in-state answers cannot score Hit in qa. Each group restarts at byte zero, so groups may reuse occurrences. Empty expected alternatives match empty strings by existing policy.

## cost

For answer length L and total alternatives A, substring scans and case normalization cost O(AL) plus query execution. TSV O(input bytes). Output and parsed records allocate proportional to observed text.

## composition

Public joins: World::apply on constructed Cmd/Line values -> answer -> verdict; read_questions -> answers_md on supplied frozen expectations. No neural parser, teacher, corpus, CLI persistence or courses executed.

## behavior-complement

Domain/universe: terminating calls on consistent supplied worlds and well-formed observations in this scope. Inner absent-behavior relation excludes transitions violating the invariant above. This is a source-derived partial exclusion; unknown paths and malformed world states remain outside it. It is not a complement of merely sampled tests.

## reverse-reconstruction

A Hit retains every answer containing at least one admissible ordered match per group. A Miss retains all failed groups and unavailable words, not a unique absent fact. Case and anchors not demanded by the expectation are discarded.

## projection-boundary

Finite independent oracle enumerates all 32 five-letter binary strings against five ordered patterns (160 pairs), recursively enumerating possible positions rather than reproducing greedy search. Report joins preserve ordinary abstention separately from control honesty.

All three typed entries bind the manifest revision and scope. Operators are invariant exclusion, output preimage and observation constraint respectively; status is partial source modeling with finite native evidence. Reverse relations preserve all described ambiguity; no complete solver or universal inverse is provided.

## checks

cargo test -p world --lib qa::wave12_models -- --test-threads=1. See the file ../model.md#evidence for actual repeated profiles and compiled controls. Partial syntax projection leaves macro bodies opaque; no semantic AST proof.

## epoch-memory

Prospective constant-cost guards and tie comparison predicted rejection without changing valid selection or signatures. Four old-code witnesses failed: empty holder success, malformed interval broadening, empty sort panic and invented earlier narration for a tie. Narrow repairs retain ordinary substring scoring and existing historical explanation behavior. Future epistemic explanations and metadata consistency remain explicit limitations. No optimization or general percentage guarantee is inferred.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective constant-cost guards and tie comparison predicted rejection without changing valid selection or signatures. Four old-code witnesses failed: empty holder success, malformed interval broadening, empty sort panic and invented earlier narration for a tie. Narrow repairs retain ordinary substring scoring and existing historical explanation behavior. Future epistemic explanations and metadata consistency remain explicit limitations. No optimization or general percentage guarantee is inferred.
