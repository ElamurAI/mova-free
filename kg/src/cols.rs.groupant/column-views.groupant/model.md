# kg-cols-column-views

## descriptant

### kg-cols-column-views

Aligned tuple push, unstable sort, mask selection and outgoing range.

## approximant

### Forward relation

Push six coordinates; sort one index permutation by(src,rel,dst); mask keeps every row satisfying conjunction of bits; out uses sorted-src lower/upper bounds.

#### Executable approximant gap

Exact categorical structure, identifiers, parser decisions and effects have no useful percentage metric established here. No executable surrogate, automatic substitution or 1/2/5/10% guarantee is claimed.

## invariant

### Invariant and domain

Aligned n<2^32 keeps tuple associations; filter preserves input order.

#### Cost and resources

sort O(n log n)/O(n), filter O(n), out O(log n).

## cost

### Cost and resources

sort O(n log n)/O(n), filter O(n), out O(log n).

## composition

### Composition

Aligned tuple push, unstable sort, mask selection and outgoing range.

Push six coordinates; sort one index permutation by(src,rel,dst); mask keeps every row satisfying conjunction of bits; out uses sorted-src lower/upper bounds.

The parent invokes scoped transformations through the actual functions named in their source ranges. Children partition useful observations, not a complete semantic proof; boundaries can omit helpers, tests and effects.

## models

## behavior_complement

No missing-bit row survives filter. Sorting does not merge duplicates, and out does not repair unsorted inputs.

## reverse_reconstruction

Filter preimage includes arbitrary rejected rows; sort preimage includes permutations and equal-key tie histories.

## zero_mask

mask0 retains all, including flags0.

## ties

Equal(src,rel,dst) ties use unstable sort; gs/gd/flags do not break ties.

## alignment

Public unequal column lengths can panic; source length alone defines len.

## index_width

sort casts n to u32; domain excludes n>=2^32 to preserve rows.


## tests

### Evidence

Parent source checks (a fragment inherits only the behavior they actually exercise):

- cols::tests::roundtrip_and_filter
- cols::wave29::codec_boundaries_and_errors

Selected kg tests: 15 named outcomes, twice debug/twice release, equal sorted outcomes. Standalone actual global/build.rs: 3 named outcomes per run, twice debug/twice optimized; build_child is normally no-op, parent fixture invokes it against isolated synthetic seeds. Eight compiled source mutations (one per source) each compiled successfully and exited101 on the focused assertion; exact source restoration followed by positive rerun.

This fragment's untested branches remain source-derived. No live graph, DB, corpus, teacher, GPU or held-out evaluation was used. Command specs and numeric exits are retained in the external wave29 graphlang receipt. Strict kg Clippy reports existing source lints; see receipt.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Source read before tests and models; retained prospective notes accompany the parent. Current evidence uses bounded invented inputs only. This is partial modeling, not proof or corpus accuracy.
