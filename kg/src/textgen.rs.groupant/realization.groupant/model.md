# kg-textgen-realization

## descriptant

### kg-textgen-realization

Form pool choice, recursive modifier chunks and final surface text.

## approximant

### Forward relation

Strip NEG/IMPLICIT synthesis flags, choose lexicographic frequency winner first-on-tie, restrict н-pronouns after prep; build adjectives/head/nonadj children; order chunks and add dash/case/punctuation.

#### Executable approximant gap

Exact categorical structure, identifiers, parser decisions and effects have no useful percentage metric established here. No executable surrogate, automatic substitution or 1/2/5/10% guarantee is claimed.

## invariant

### Invariant and domain

Used modifier flag prevents duplicate attachment; depth<=4,1..12 heads admitted. Orphan modifiers can remain unused.

#### Cost and resources

Variant scans plus member scans per chunk, bounded recursion then DP; retained generated strings.

## cost

### Cost and resources

Variant scans plus member scans per chunk, bounded recursion then DP; retained generated strings.

## composition

### Composition

Form pool choice, recursive modifier chunks and final surface text.

Strip NEG/IMPLICIT synthesis flags, choose lexicographic frequency winner first-on-tie, restrict н-pronouns after prep; build adjectives/head/nonadj children; order chunks and add dash/case/punctuation.

The parent invokes scoped transformations through the actual functions named in their source ranges. Children partition useful observations, not a complete semantic proof; boundaries can omit helpers, tests and effects.

## models

## behavior_complement

No synth/depth5/bad head count→None; no н pool→ordinary variants remain allowed.

## reverse_reconstruction

Text admits multiple morphological variants/count histories/Facts; implicit heads and original word coordinates do not appear in output.

## conditional_priority

A conditional1/global0 beats conditional0/global8; zero-zero tie chooses dictionary first.

## pronoun_pool

After prep+PRON+person3+any н, every non-н option excluded irrespective of frequency.

## modifier_order

PRON adjectives first, ordinary adjectives next, head then recursive nonadjectives by member order.

## punctuation

Gram0 noun subject/predicate pair gets dash unless pronoun; IMPR ! else period; initial Unicode char uppercased.


## tests

### Evidence

Parent source checks (a fragment inherits only the behavior they actually exercise):

- textgen::tests::subject_verb_object
- textgen::wave29::variant_priority_and_head_bounds

Selected kg tests: 15 named outcomes, twice debug/twice release, equal sorted outcomes. Standalone actual global/build.rs: 3 named outcomes per run, twice debug/twice optimized; build_child is normally no-op, parent fixture invokes it against isolated synthetic seeds. Eight compiled source mutations (one per source) each compiled successfully and exited101 on the focused assertion; exact source restoration followed by positive rerun.

This fragment's untested branches remain source-derived. No live graph, DB, corpus, teacher, GPU or held-out evaluation was used. Command specs and numeric exits are retained in the external wave29 graphlang receipt. Strict kg Clippy reports existing source lints; see receipt.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Source read before tests and models; retained prospective notes accompany the parent. Current evidence uses bounded invented inputs only. This is partial modeling, not proof or corpus accuracy.
