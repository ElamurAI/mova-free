# corpus-register

## descriptant

# corpus-register

Full file: corpus counting, pooled-prior ranking, thresholding and TSV/report output.

## Forward

Readable UTF-8 books -> events::book_body -> alphabetic runs with original byte length>1 -> lowercase counts. Union keys: alpha=1000*(ya+yb)/(na+nb); la=ln((ya+alpha)/(na+1000-ya-alpha)), lb analogous; z=(la-lb)/sqrt(1/(ya+alpha)+1/(yb+alpha)). Filter z>=zmin and raw_count_a>=min, sort descending total_cmp(z) then key; print counts and rates.

## Invariant

Corpus totals equal histogram sums without u64 overflow. Retained keys belong to the union vocabulary and satisfy the source z/count comparisons; NaN z is never admitted. Rows sort by descending total_cmp(z), then ascending word when scores compare equal. Equal counts alone impose no row order. Rows retain exact u64 counts and apply min to the original register count; f64 conversion affects only score/rate calculations and can lose precision above 2^53.

## Cost

O(B+tokens log V+V log V), memory O(V+largest book+rows), where B bytes and V union keys. IO cost additional.

## Composition

The counting child supplies maps and totals to ranking through run. Counting calls events::book_body directly; register does not invoke the event parser or sentence splitter. Ranked rows then feed TSV formatting. Reverse composition retains compatible maps, totals and thresholds; filesystem outcomes remain latent.

## Epoch memory

## Gaps

Syntax is source-bound and partial: macro expansion, names/types, cfg evaluation and external modules absent. Native fixtures exercise pure helpers and body slicing. Filesystem errors, invalid UTF-8 reads, formatted nonfinite rates and overflow remain unexecuted; large-count behavior has one targeted witness, not exhaustive coverage. No parser/annotator, sentence-splitting, live corpus, teacher or GPU execution is involved.

## Exact-count correction

## catalog

# corpus-register typed models

## nonbehavior

Type: behavior_complement.

U=(coherent maps,options,rows). Excludes row below z/count threshold, NaN z, outside-union key. Oracle native threshold/degenerate cases.

## antecedents

Type: reverse_reconstruction.

Output admits all coherent maps/totals and options satisfying score equations, retained counts/order and rejected tail predicates. Rounded TSV scores/rates expand fibers to formatting intervals; text/order unrecoverable. Oracle proposed finite fiber enumeration.

## byte-tokenization

Type: unicode_projection.

Original alphabetic runs admitted by UTF-8 bytes>1 before lowercase: ASCII A excluded, é and İ included, latter lowercases to i+combining dot. Oracle exact Unicode histogram.

## pooled-score

Type: numeric_relation.

alpha is 1000 times pooled frequency; variance sum reciprocal smoothed counts, not full logit variance. Oracle 64 interior integer pairs independently expanded log formula.

## swap-law

Type: equivariance.

Swapping corpora leaves alpha/variance fixed and negates finite z; selection need not be symmetric. Oracle native 64 swapped pairs.

## degeneracy

Type: ieee_partition.

Empty union has no rows. A singleton yields NaN only when the actual floating operations give both zero complementary denominators and positive numerators, making inf-inf. The small singleton fixture has this behavior. It is not universal: independently rounded counts and pooled total above 2^53 can leave positive denominators and finite z=0. One empty corpus with >=2 keys can have finite z; its printed rate is 0/0. Oracles: native small degeneracies and large-singleton witness; rate formatting source-only.

## threshold-order

Type: filter_relation.

Equality z=zmin and original u64 ca=min admitted; NaN zmin rejects all. Sort uses total_cmp then lexical key, including signed-zero distinction. Oracle identical maps at zero/EPSILON, min 2/3 and NaN.

## body-edge

Type: composition_relation.

Each read passes through book_body before counting. Outside-marker text absent; separate body deltas add; duplicate paths duplicate counts. Oracle native stripped body plus second chunk.

## io-errors

Type: failure_partition.

Read errors silently skipped; header book numbers still attempted paths. Output create/write/flush errors propagate; successful return says nothing about all inputs readable. Oracle source-only.

## precision-boundary

Type: representation_relation.

Totals add in u64 before f64 conversion. Original counts ca/cb are kept as u64 for the minimum gate and emitted rows; only score inputs ya/yb are cast to f64, whose precision above 2^53 is limited. Overflow excluded; no new smoothing/saturation policy. Oracles: native large-singleton witness plus exact count/min cases at 2^53+1 and 2^53+3. Both round-down exclusion and round-up inclusion by the former cast-back gate are rejected. Other high-count floating-score behavior remains unenumerated.

## tests

### Bounded native evidence

The singleton counts 2^53+1 and 2^53+2 yield alpha below 1000 and positive floating complementary denominators, admitting z=0. Returned counts now equal the original u64 values. Singleton NaN remains conditional on actual floating operations. Statistical scoring and NaN policy are unchanged.

The exact-count regression tests use ca=2^53+1 (rounds down as f64) and ca=2^53+3 (rounds up). At min=ca each row must be admitted with exact counts; at min=ca+1 it must be excluded. Before the correction, the two native large-count tests compiled and failed against production's former cast-back behavior. counts-old-behavior.log records this independent red gate. The fix keeps ca/cb before conversion, filters ca >= min and emits ca/cb; only z/rates use floating conversion.

Run cargo test -p world --lib corpus_models -- --test-threads=1 and the same with --release. Twelve tests (seven register, five events) pass twice per profile with identical normalized observations. evidence.json binds current source and observations; prior-evidence.json and rounding-evidence.json retain earlier source epochs.

Earlier compiled extraction controls rejected events AuxPass-to-Aux and score >= to >. The current correction additionally has an actual native Cargo old-behavior negative control. Controls and native tests use synthetic data and do not invoke filesystem orchestration or parser acquisition.

Output errors, nonfinite rate formatting, unreadable files, unenumerated floating-score behavior and overflow remain gaps. No live corpus IO, teacher or GPU execution.

## History

### 2026-10-30 · Epoch memory

2026-10-30. Native fixtures confirm Unicode byte admission, lowercase expansion, marker stripping, additive counts, pooled score/swap pairs, thresholds and lexical score ties. Singleton NaN is conditional on actual floating denominators; a new >2^53 fixture witnesses a finite singleton score. Numeric policy is unchanged. Review corrected inherited event-specific prose. No executable approximation/error-cap claim.

### 2026-10-30 · Exact-count correction

2026-10-30: independent large-count native witnesses fail on the former f64-to-u64 count round trip. ranked_rows now retains raw ca/cb, checks ca >= min and emits ca/cb exactly. The f64 z formula and NaN policy are unchanged. At ca=2^53+1 the old gate wrongly excluded equality; at ca=2^53+3 it could admit min=ca+1. The corrected native fixtures cover both directions.

### 2026-10-30 · document

Subject: source hash in manifest, epoch 2026-10-30. Scope: Full file: corpus counting, pooled-prior ranking, thresholding and TSV/report output.. Domain: terminating Rust calls without integer overflow, narrowed below. Models are partial source-derived relations, not proofs. Complement is an inner bound in the stated input/output universe. Reconstruction retains all compatible antecedents, not a unique inverse. Oracle status is explicit below; model.md#tests records executed coverage.

### 2026-10-30 · tests

2026-10-30. Seven inline register tests cover Unicode byte admission and lowercase expansion; 64 independently expanded log-odds pairs and swaps; score/count thresholds, lexical score ties and NaN threshold; empty/small-singleton/one-empty degeneracies; body stripping and additive counting; large-singleton floating behavior; exact large integer counts and minimum gates.
