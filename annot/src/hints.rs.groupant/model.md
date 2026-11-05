# annot-hints

## descriptant

### annot-hints

Optional expert-rule hints on Rust drafts. Rules are loaded strictly, severity filtered, evaluated over an Ann-to-Sentence projection, then rendered in sorted unique order. Hints are advisory, not annotations or acceptance gates.

## approximant

### Forward approximant

load rejects if expert::load reports any parse error, otherwise from_rules retains r when warn or severity==error. for_ann projects every token with space_after=true and all layer flags=true, iterates rule violation bindings, renders one-based token coordinates and sorts/deduplicates lines.

Cost: A annotation tokens, V violation bindings and H rendered bytes: projection O(A), delegated expert rule evaluation cost E(rules,A), render O(H), sorting O(V log V) string comparisons and O(A+H) retained memory. No practical bound on expert joins is established.

Executable approximation gap: categorical parsing, effects and validation lack a useful measured percent metric here. No executable surrogate or 1/2/5/10% profile is attached; an exact test is not an approximant.

## invariant

### Invariants and limits

Output is lexicographically sorted with no duplicate strings; no mutation of Ann. Rule binding indices must be in 1..=a.len() or indexing panics. warn=true retains every severity, not only error/warn. Empty rules yields empty output.

- Rule-file error and warn=true paths are source modeled, not directly executed here.
- No arbitrary expert binding safety or linguistic accuracy claim.

## composition

### Source composition

Nested scopes use UTF-8 byte offsets [start,end) against the exact source SHA-256. They share the parent lexical projection; no nested fragment becomes a second registry source. Source outside these scopes, including helper types/tests, remains parent-modeled.

- selection [762,1280): Strict load and Boolean severity filtering define the retained rule sequence.
- projection [1384,2302): Ann fields project to a Sentence, violations render with one-based indices and sort/dedup into advisory strings.

load rejects if expert::load reports any parse error, otherwise from_rules retains r when warn or severity==error. for_ann projects every token with space_after=true and all layer flags=true, iterates rule violation bindings, renders one-based token coordinates and sorts/deduplicates lines.

Cross-file join: ann.rs supplies Ann/Item, parse/check/write; main supplies Outcome and finish; batch and llm prompts feed codex_batch; hints and repair influence draft content before gates. These joins do not establish remote correctness or crash durability.

## models

## behavior_complement

No hint originates from a retained rule with zero violations. warn=false excludes rules whose severity differs from exact error. Load errors cannot silently return a reduced usable rule set.

## reverse_reconstruction

A hint constrains at least one retained rule id/what and one violation binding with those forms, but different rules or repeated bindings can render the same deduplicated string. An empty result is compatible with no rules, no violations or empty matching domains; it does not reconstruct a correct annotation.

## severity_selection

Selection is Boolean warn OR exact severity equality; severity casing and custom labels are significant when warn=false.

## binding_coordinates

expert::check bindings index one-based original annotation positions; no reindexing occurs during projection, range safety is delegated.

## hint_set

Sort then dedup erases multiplicity and rule input ordering from the result; lexical rendering determines ties.

## advisory_join

chunk_hints omits empty vectors; patch_prompt displays nonempty hints but acceptance still passes through ann::check. Hints neither rewrite drafts nor guarantee teacher correctness.


## tests

### Bounded source evidence

Selected actual-code tests (twice debug, twice release):

- hints::tests::hints_fire_only_on_violations

Run command: cargo test -p annot --bin annot --test codex_batches [--release] -- --test-threads=1 --skip ann:: --skip bfs_transfer_checks. The fake executable fixture uses temporary files, no live teacher/API. Paired named outcomes must match. Six compiled production mutations target batch last-wins, llm closed-set, repair shape gate, hints severity, main seed alias and queue resume. Each compilation exits0, test exits101, exact source restored and focused positive rerun exits0. The fixture itself has no separate mutation. Existing repair test may early-return without dictionary; independent added repair test always runs.

Unexecuted limits:
- Rule-file error and warn=true paths are source modeled, not directly executed here.
- No arbitrary expert binding safety or linguistic accuracy claim.

Validation limits: strict scoped Clippy exited101 on six pre-existing lints (three collapsible_if, one needless_range_loop, one type_complexity, one redundant_locals). Re-running with those four known lint categories allowed exited0. changed with an external augmented registry exited0 and found no missing changed-source models. Local groupants inspection reports only expected ancestor child/source-binding updates owned by integration; no owned leaf/fragment diagnostics.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source read before tests/models. Prospective notes and numeric receipts retained externally under the annotation wave27 run. Four new inline tests supplement existing bounded tests. Preserve distinctions: compatibility cost is not subscription cost; mock process tests do not establish live teacher behavior; registry presence is partial modeling.
