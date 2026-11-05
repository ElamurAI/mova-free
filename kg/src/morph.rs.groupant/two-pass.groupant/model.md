# kg-morph-two-pass

## descriptant

### kg-morph-two-pass

Vocabulary/lemma selection across two dictionary passes.

## approximant

### Forward relation

Pass1 normalizes, selects vocab analyses and remembers lemma keys; Pass2 selects all synthesis entries with remembered keys.

#### Executable approximant gap

Exact categorical structure, identifiers, parser decisions and effects have no useful percentage metric established here. No executable surrogate, automatic substitution or 1/2/5/10% guarantee is claimed.

## invariant

### Invariant and domain

Selected forms and synthesis share Gram and lemma_key functions, assuming unchanged file; nonrequested selected-lemma forms synthesized.

#### Cost and resources

O(2B+A+S) decompression/retained strings with hash factors.

## cost

### Cost and resources

O(2B+A+S) decompression/retained strings with hash factors.

## composition

### Composition

Vocabulary/lemma selection across two dictionary passes.

Pass1 normalizes, selects vocab analyses and remembers lemma keys; Pass2 selects all synthesis entries with remembered keys.

The parent invokes scoped transformations through the actual functions named in their source ranges. Children partition useful observations, not a complete semantic proof; boundaries can omit helpers, tests and effects.

## models

## behavior_complement

Unselected lemma synth entries excluded; empty vocabulary selects none but still reads input twice.

## reverse_reconstruction

Same retained Morph admits arbitrary unselected dictionary lines and discarded comments, plus duplicate source histories.

## government_inheritance

Only prep/verb tags retain current lemma comment mask; other POS gets0.

## xp_join

Both passes apply with_xp before lemma_key, preserving shared paradigm identity.

## source_order

Vector append preserves dictionary traversal order and duplicates, influencing generation ties.

## file_boundary

File open/decompression errors propagate; two opens admit file-change races without snapshot guarantee.


## tests

### Evidence

Parent source checks (a fragment inherits only the behavior they actually exercise):

- morph::tests::lines_and_government
- morph::wave29::two_pass_synthetic_dictionary

Selected kg tests: 15 named outcomes, twice debug/twice release, equal sorted outcomes. Standalone actual global/build.rs: 3 named outcomes per run, twice debug/twice optimized; build_child is normally no-op, parent fixture invokes it against isolated synthetic seeds. Eight compiled source mutations (one per source) each compiled successfully and exited101 on the focused assertion; exact source restoration followed by positive rerun.

This fragment's untested branches remain source-derived. No live graph, DB, corpus, teacher, GPU or held-out evaluation was used. Command specs and numeric exits are retained in the external wave29 graphlang receipt. Strict kg Clippy reports existing source lints; see receipt.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Source read before tests and models; retained prospective notes accompany the parent. Current evidence uses bounded invented inputs only. This is partial modeling, not proof or corpus accuracy.
