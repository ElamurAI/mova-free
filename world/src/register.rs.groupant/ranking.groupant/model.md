# corpus-register-ranking

## descriptant

# corpus-register-ranking

ranked_rows and run score formatting/output.

## Forward

Evaluate pooled score, filter false comparisons including NaN, rank and render rounded values. run orchestrates counts and output.

## Invariant

Retained keys belong to the union maps and satisfy both source comparisons. NaN z or NaN threshold excludes a row. Ordering is descending total_cmp(z), then lexical key when scores compare equal; count ties need not be score ties. Original u64 counts are retained in rows and compared exactly to min. Score/rate conversion to f64 can lose precision above 2^53. Finite corpus-swap antisymmetry is checked on bounded fixtures.

## Cost

O(V log V), O(V) keys and rows.

## Composition

This fragment has no children. ranked_rows receives maps/totals, computes pooled priors and scores, filters and sorts. run formats two-decimal z and one-decimal per-million rates. Reverse reconstruction retains exact compatible scores before formatting; write completion is a separate conditional trace.

## Epoch memory

## Gaps

Syntax is source-bound and partial: macro expansion, names/types, cfg evaluation and external modules absent. Native fixtures exercise pure helpers and body slicing. Filesystem errors, invalid UTF-8 reads, formatted nonfinite rates and overflow remain unexecuted; large-count behavior has one targeted witness, not exhaustive coverage. No parser/annotator, sentence-splitting, live corpus, teacher or GPU execution is involved.

## Exact-count correction

## catalog

# corpus-register-ranking typed models

## nonbehavior

Type: behavior_complement.

U=(maps,options,rows). Excludes admitted NaN, missing-union key or ascending finite-score pair. Oracle threshold/degeneracy tests.

## antecedents

Type: reverse_reconstruction.

Exact tuple word,z,ya,yb admits totals and other counts satisfying sum and score equations; other words not uniquely identified. Two-decimal output loses more information. Oracle proposed finite fibers.

## finite-domain

Type: domain_partition.

If 0<ya+alpha<na+1000 and same for b, log arguments positive finite and variance positive. Zero complementary mass can create infinities/NaN. Oracle 64 interior pairs and singleton.

## sign

Type: monotonicity_relation.

For equal totals and finite domain, shared prior and monotone log odds imply sign(z)=sign(ya-yb); swap negates. Oracle native pair loops.

## rates

Type: format_projection.

Rates ya*1e6/na and yb*1e6/nb print one decimal; empty reference may print nonfinite rate for finite admitted z. Header attempted books differs from successful reads. Oracle source-only formatting.

## History

### 2026-10-30 · Epoch memory

2026-10-30. Native cases cover 64 independently expanded score pairs, swaps, thresholds, ties and degeneracies. The large-singleton fixture demonstrates rounding can leave positive complementary denominators and admit z=0. The numerical algorithm is preserved; review corrected event-specific invariant prose. No executable approximation/error-cap claim.

### 2026-10-30 · Exact-count correction

2026-10-30: independent large-count native witnesses fail on the former f64-to-u64 count round trip. ranked_rows now retains raw ca/cb, checks ca >= min and emits ca/cb exactly. The f64 z formula and NaN policy are unchanged. At ca=2^53+1 the old gate wrongly excluded equality; at ca=2^53+3 it could admit min=ca+1. The corrected native fixtures cover both directions.

### 2026-10-30 · document

Subject: source hash in manifest, epoch 2026-10-30. Scope: ranked_rows and run score formatting/output.. Domain: terminating Rust calls without integer overflow, narrowed below. Models are partial source-derived relations, not proofs. Complement is an inner bound in the stated input/output universe. Reconstruction retains all compatible antecedents, not a unique inverse. Oracle status is explicit below; ../model.md#tests records executed coverage.
