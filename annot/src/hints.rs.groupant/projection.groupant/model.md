# annot-hints-projection

## descriptant

### annot-hints-projection

Ann fields project to a Sentence, violations render with one-based indices and sort/dedup into advisory strings.

## approximant

### Scoped forward relation

Ann fields project to a Sentence, violations render with one-based indices and sort/dedup into advisory strings.

Parent composition and symbolic resource costs remain in ../model.md#approximant. No executable percentage surrogate is supplied for this categorical/effect scope.

## invariant

### Scoped fragment: projection

Local scope: Ann fields project to a Sentence, violations render with one-based indices and sort/dedup into advisory strings.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

### Invariants and limits

Output is lexicographically sorted with no duplicate strings; no mutation of Ann. Rule binding indices must be in 1..=a.len() or indexing panics. warn=true retains every severity, not only error/warn. Empty rules yields empty output.

- Rule-file error and warn=true paths are source modeled, not directly executed here.
- No arbitrary expert binding safety or linguistic accuracy claim.

## models

# Scoped fragment: projection

Local scope: Ann fields project to a Sentence, violations render with one-based indices and sort/dedup into advisory strings.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

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

### Scoped fragment: projection

Local scope: Ann fields project to a Sentence, violations render with one-based indices and sort/dedup into advisory strings.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

### Bounded source evidence

Selected actual-code tests (twice debug, twice release):

- hints::tests::hints_fire_only_on_violations

Run command: cargo test -p annot --bin annot --test codex_batches [--release] -- --test-threads=1 --skip ann:: --skip bfs_transfer_checks. The fake executable fixture uses temporary files, no live teacher/API. Paired named outcomes must match. Six compiled production mutations target batch last-wins, llm closed-set, repair shape gate, hints severity, main seed alias and queue resume. Each compilation exits0, test exits101, exact source restored and focused positive rerun exits0. The fixture itself has no separate mutation. Existing repair test may early-return without dictionary; independent added repair test always runs.

Unexecuted limits:
- Rule-file error and warn=true paths are source modeled, not directly executed here.
- No arbitrary expert binding safety or linguistic accuracy claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: scoped source-derived model. Byte scope [1384,2302); AST is shared partial projection, not a separate semantic proof.
