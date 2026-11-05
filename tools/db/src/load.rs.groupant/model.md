# db-src-load

## descriptant

### CoNLL staging and identity assignment

load_file canonicalizes, reads/hash/parses, rejects empty sentences, queries state, then skips identical path/hash or rejects changed path. Run reuse/new IDs precede six TSV writers. For each sentence, prior-state candidates require sent_id and text equality; Auto/Existing accept exactly one, reject ties, Existing rejects zero, New creates. New document keys track newdoc/file/idprefix policy; repeated newdoc IDs gain ~n. Gate failures are stored as labels. Six flushed tables execute in SQL then counts reconcile.

#### Retained limitations

No SQL evaluation was substituted with a mock process. Missing DuckDB is a dependency gap, not passing database evidence. Partial syntax has no macro expansion/name resolution/types; Rust helpers and fixtures do not prove all models.

## approximant

### CoNLL staging and identity assignment forward relation

load_file canonicalizes, reads/hash/parses, rejects empty sentences, queries state, then skips identical path/hash or rejects changed path. Run reuse/new IDs precede six TSV writers. For each sentence, prior-state candidates require sent_id and text equality; Auto/Existing accept exactly one, reject ties, Existing rejects zero, New creates. New document keys track newdoc/file/idprefix policy; repeated newdoc IDs gain ~n. Gate failures are stored as labels. Six flushed tables execute in SQL then counts reconcile.

#### Domain

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

#### Approximation limits

Categorical parsing, exact identity, transactional effects and resource errors have no useful percentage approximation metric here. No executable surrogate is claimed. Future profiles, if justified, may use only 1%, 2%, 5%, 10% with a defined metric/reference/domain.

## composition

### Local scope composition

Parent relation: load_file canonicalizes, reads/hash/parses, rejects empty sentences, queries state, then skips identical path/hash or rejects changed path. Run reuse/new IDs precede six TSV writers. For each sentence, prior-state candidates require sent_id and text equality; Auto/Existing accept exactly one, reject ties, Existing rejects zero, New creates. New document keys track newdoc/file/idprefix policy; repeated newdoc IDs gain ~n. Gate failures are stored as labels. Six flushed tables execute in SQL then counts reconcile.

Child fragments isolate selected mechanisms and do not cover the entire file. Invocation order and shared state remain the parent's relation. Costs of a child are included in parent cost, not added a second time; independent branch scopes are tagged. A child reverse relation must preserve intermediate state compatibility, not imply unique whole-file reconstruction.

Children: [{"id":"db-src-load-sentence-document-join","path":"sentence-document-join.groupant"}]

## models

# CoNLL staging and identity assignment typed model catalog

Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Observation is one call/trace in the declared scope. Operators below are partial source-derived relations; complement is a proposed inner exclusion relation, reverse an outer compatible-antecedent relation retaining ambiguity. No universal proof follows from finite checks. Each oracle is the relevant named helper check below when it reaches the behavior; remaining SQL/effect cases are source review or proposed real-CLI checks.

## behavior_complement

Same canonical path with changed SHA1 cannot reach write execution. Ambiguous exact sentence candidates cannot be arbitrarily tie-broken by Auto/Existing. A non-ok annotation gate does not exclude the load: it is counted and staged.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## reverse_reconstruction

Given report counts and resulting raw rows, antecedents include all prior maxima and run/doc maps consistent with new IDs, all matching candidates producing chosen sids, and source text with matching stored comments/blank metadata. Counts alone cannot recover match identities, original bytes or unused state.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## sentence_match_partition

Let m=count prior entries with matching id and text. Existing succeeds only m=1; Auto creates at m=0 and errors m>1; New creates regardless. Newly staged equal sentences do not change m. SQL uniqueness/races remain assumptions.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## document_key_transition

NewDoc advances current key on newdoc comments before sentence matching; repeated keys within this file gain ~2,~3. Existing matched sentences retain their DB doc even when a new current key was opened.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## comment_information_projection

comment_kv splits literal space-equals-space or trailing space-equals; absent value and present empty string remain different. Raw annotation comments preserve duplicate/order information that metadata JSON maps can overwrite.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## commit_reconciliation_boundary

TSV validation and ambiguity errors precede exec; post.tsv parsing/count mismatch follows exec and may report error after committed effects. The two phases must not be modeled as one rollback guarantee.

Domain/assumptions: Finite valid-UTF8 inputs, representable counters/IDs, available memory; filesystem fixtures confined to owned temporary root. SQL semantics are conditional and unexecuted. Subject is the source and SHA256 bound in manifest; fragment applies only to declared byte range.

Evidence: see model.md#tests; unexecuted database branches remain conditional.

## tests

### Selected checks

Named target-scoped checks for parent source: ["load::wave31::option_and_comment_representation","load::tests::comments_and_dates","load::teacher_provenance_tests::codex_runs_are_not_labelled_as_legacy_claude"]

Native commands and numeric status/log/outcomes live in external native-commands.json and receipts. Integration fixture checks use actual en::conllu parsing only. Legacy prag_layer_and_negative_controls, qa_table_loader_and_negative_controls and lossless_round_trip_and_negative_controls were inspected but not executed: real DuckDB unavailable. A fragment's listed parent tests may only cover part of its relation; SQL/query/transaction/IO-fault claims remain source-derived.

#### Negative controls

Six source-behavior mutations: quote doubling, TSV NULL rejection, QA positivity, prag bit truth, MWT counter, sentence boundary predicate. Each compiled, failed an assertion, restored exact original bytes and passed the selected positive. No assertion-only mutations and no surviving behavioral mutants.

## invariant-and-cost

### CoNLL staging and identity assignment invariants and costs

#### Invariant

rep.created+rep.matched=rep.sents on success. Every staged annotation has one aid and ten raw columns per row. Matching uses the pre-load snapshot: newly staged sentences do not become candidates. Unchecked i64 IDs, ord/para and usize conversions require representable bounds; exhaustion behavior is not a checked error contract.

#### Cost

O(file bytes + prior-state rows + staged rows + sum candidate-vector scans) expected Rust work; memory retains complete Doc/state and staged buffers. Repeated duplicate document keys can cause quadratic suffix probing. Database query/transaction and annotation gate costs remain additive and unmeasured.

Symbolic bounds are source-derived, not measured wall-clock speed or universal complexity proof. Allocation failures, integer exhaustion and concurrency are outside finite evidence.

## History

### 2026-10-31 · descriptant

#### Prospective epoch — 2026-10-31

Full source was reviewed before edits; external prospective.md records API states, failure order and predicted checks. Production prefix remains byte-identical; test additions are isolated cfg(test)/test items.

#### Observed epoch — 2026-10-31

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.

### 2026-10-31 · tests

2026-10-31 wave31: 16 selected target/name cases passed twice debug and twice release with identical sorted outcomes; six compiled source-behavior mutants failed assertions, exact bytes restored and positive checks passed. Evidence scope is only the named helpers/fixtures. External receipt directory: $MOVA_DATA/runs/octopus-wave31-store-2026-10-31. No DuckDB integration, production loader/corpus, training or GPU run.
