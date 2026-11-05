# kg-cols-integer-codecs

## descriptant

### kg-cols-integer-codecs

Private LEB/delta/zigzag/byteplane transformations.

## approximant

### Forward relation

LEB emits 7-bit groups; delta wraps x-prev; inverse wraps prev+d. Zigzag encodes signed wrapping difference. Planes transpose four little-endian bytes across all values.

#### Executable approximant gap

Exact categorical structure, identifiers, parser decisions and effects have no useful percentage metric established here. No executable surrogate, automatic substitution or 1/2/5/10% guarantee is claimed.

## invariant

### Invariant and domain

Decode of encoder-produced values and exact count recovers u64/u32 bitpatterns including MAX and repeated zero.

#### Cost and resources

O(n), <=10 LEB bytes/u64,4n planes; allocation count arithmetic not checked.

## cost

### Cost and resources

O(n), <=10 LEB bytes/u64,4n planes; allocation count arithmetic not checked.

## composition

### Composition

Private LEB/delta/zigzag/byteplane transformations.

LEB emits 7-bit groups; delta wraps x-prev; inverse wraps prev+d. Zigzag encodes signed wrapping difference. Planes transpose four little-endian bytes across all values.

The parent invokes scoped transformations through the actual functions named in their source ranges. Children partition useful observations, not a complete semantic proof; boundaries can omit helpers, tests and effects.

## models

## behavior_complement

Truncated continuation/too-long LEB and wrong plane length error. Trailing LEB bytes and overlarge terminal payload are not fully rejected.

## reverse_reconstruction

Plane transpose at fixed n is bijective; decoded LEB vector has multiple byte preimages from trailing/noncanonical encodings.

## modulo

Wrapping arithmetic handles descending src too, despite usual sorted production use.

## signed_boundary

u64 endpoints reinterpret as i64 differences; zigzag and inverse preserve two's complement bitpattern.

## empty_count

n0 decodes no LEB values even with extra bytes; plane n0 requires empty bytes.

## error_cursor

unleb increments position after each consumed byte; no rollback on error.


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
