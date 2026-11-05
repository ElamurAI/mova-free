# kg-uk-parse-gates

## descriptant

### kg-uk-parse-gates

Parse gates, proper-name filtering and candidate cardinality.

## approximant

### Forward relation

Reject punctuation in source order; empty,length,ASCII-digit,conjunction,unknown gates follow; filter proper/common, reject dash+verb, enumerate candidates then unique accept.

#### Executable approximant gap

Exact categorical structure, identifiers, parser decisions and effects have no useful percentage metric established here. No executable surrogate, automatic substitution or 1/2/5/10% guarantee is claimed.

## invariant

### Invariant and domain

Only one distinct surviving Fact returned; word index means filtered word coordinate.

#### Cost and resources

O(B+n*a+candidate construction/equality); n<=8 after gate.

## cost

### Cost and resources

O(B+n*a+candidate construction/equality); n<=8 after gate.

## composition

### Composition

Parse gates, proper-name filtering and candidate cardinality.

Reject punctuation in source order; empty,length,ASCII-digit,conjunction,unknown gates follow; filter proper/common, reject dash+verb, enumerate candidates then unique accept.

The parent invokes scoped transformations through the actual functions named in their source ranges. Children partition useful observations, not a complete semantic proof; boundaries can omit helpers, tests and effects.

## models

## behavior_complement

TooLong precedes ASCII-digit rejection; other Unicode numerals are not rejected by that digit gate; comma before ? yields comma error. Case filtering does not remove sole proper-only or common-only analyses.

## reverse_reconstruction

Reject reason identifies a set of inputs with earlier gates passed; accepted Fact loses rejected alternatives and punctuation.

## proper_context

With both classes, lowercase→common; mid-sentence uppercase→proper; sentence-initial uppercase needs names membership.

## unknown_order

First word without forms rejects before finite/dash construction.

## candidate_dedup

Identical Facts from duplicate analyses count once; two different Facts return Ambiguous.

## nominal_dispatch

Nominal_sentence runs only when no finite predicate candidates exist, not when all verbal constructions fail.


## tests

### Evidence

Parent source checks (a fragment inherits only the behavior they actually exercise):

- uk::tests::tokens_strip_stress_and_keep_apostrophes
- uk::tests::render_check_catches_damage
- uk::wave29::failure_precedence_and_real_parse

Selected kg tests: 15 named outcomes, twice debug/twice release, equal sorted outcomes. Standalone actual global/build.rs: 3 named outcomes per run, twice debug/twice optimized; build_child is normally no-op, parent fixture invokes it against isolated synthetic seeds. Eight compiled source mutations (one per source) each compiled successfully and exited101 on the focused assertion; exact source restoration followed by positive rerun.

This fragment's untested branches remain source-derived. No live graph, DB, corpus, teacher, GPU or held-out evaluation was used. Command specs and numeric exits are retained in the external wave29 graphlang receipt. Strict kg Clippy reports existing source lints; see receipt.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Source read before tests and models; retained prospective notes accompany the parent. Current evidence uses bounded invented inputs only. This is partial modeling, not proof or corpus accuracy.
