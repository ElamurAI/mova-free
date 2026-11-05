# global-build-fences

## descriptant

### global-build-fences

Closed fenced-block extraction only.

## approximant

### Forward relation

Scan trimmed lines with optional current(kind,lines). Any fence closes current; outside, only nonempty kind opens. Append nonempty trimmed body lines; discard current at EOF.

#### Executable approximant gap

Exact categorical structure, identifiers, parser decisions and effects have no useful percentage metric established here. No executable surrogate, automatic substitution or 1/2/5/10% guarantee is claimed.

## invariant

### Invariant and domain

Output order equals closing-fence order; no unclosed block escapes.

#### Cost and resources

O(B) scan and retained closed-body bytes.

## cost

### Cost and resources

O(B) scan and retained closed-body bytes.

## composition

### Composition

Closed fenced-block extraction only.

Scan trimmed lines with optional current(kind,lines). Any fence closes current; outside, only nonempty kind opens. Append nonempty trimmed body lines; discard current at EOF.

The parent invokes scoped transformations through the actual functions named in their source ranges. Children partition useful observations, not a complete semantic proof; boundaries can omit helpers, tests and effects.

## models

## behavior_complement

Empty outside fence and ordinary outside lines produce no block. A fence with another kind closes rather than opens replacement.

## reverse_reconstruction

A block list admits arbitrary outside prose, blank lines, indentation and discarded trailing opener.

## empty_document

Empty input yields empty list; bare opener/closer alone yields no block.

## closing_policy

```other closes ```concept without preserving other as a new block.

## trim_projection

Leading/trailing line whitespace and blank body lines are erased.

## caller_boundary

main interprets returned kinds; extractor itself admits unknown kinds and does not parse colon grammar.


## tests

### Evidence

Parent source checks (a fragment inherits only the behavior they actually exercise):

- wave29::block_and_code_boundaries
- wave29::isolated_seed_compile_and_duplicate_gate
- wave29::build_child

Selected kg tests: 15 named outcomes, twice debug/twice release, equal sorted outcomes. Standalone actual global/build.rs: 3 named outcomes per run, twice debug/twice optimized; build_child is normally no-op, parent fixture invokes it against isolated synthetic seeds. Eight compiled source mutations (one per source) each compiled successfully and exited101 on the focused assertion; exact source restoration followed by positive rerun.

This fragment's untested branches remain source-derived. No live graph, DB, corpus, teacher, GPU or held-out evaluation was used. Command specs and numeric exits are retained in the external wave29 graphlang receipt. Strict kg Clippy reports existing source lints; see receipt.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Source read before tests and models; retained prospective notes accompany the parent. Current evidence uses bounded invented inputs only. This is partial modeling, not proof or corpus accuracy.
