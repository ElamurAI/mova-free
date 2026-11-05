# algorithm-rabin-karp

## descriptant

### Combiner and epoch memory

Combiner inputs are raw Rust, its partial syntax projection, forward/invariant models, the ordered typed family and this memory. Output candidates are local recurrence, tie or shift changes, filtered by invariant and oracle agreement. It is a descriptive interface, not an implemented autonomous model generator.

Epoch evidence is retained in model.md#tests. Finite agreement narrows uncertainty but does not prove arbitrary input correctness. No useful faster executable approximant is supplied for these exact observations; categorical errors have no accepted percentage budget.

## approximant

### Forward model and invariant

M(t,p)=[i in increasing order: 0<=i<=|t|-|p| and t[i..i+|p|]=p]; empty p yields all boundaries.

O(n+m+c*m) byte/hash work, c hash candidates; O(n*m) worst case and O(1) auxiliary words plus output.

Inputs are immutable. Returned offsets/witness indices stay in bounds; returned edit cost is nonnegative. Public Rust docs define empty, element-unit, overflow and allocation behavior.

Karp and Rabin, Efficient randomized pattern-matching algorithms, IBM Journal of Research and Development 31(2), 1987, doi:10.1147/rd.312.0249. This implementation is original deterministic verified hashing; it does not claim their randomized expected bound.

## models

# Typed relations

Revision: source hash in manifest. Observation: complete single call, lawful Eq and successful allocation unless explicitly scoped otherwise. Model direction is an exact-target relational specification; correspondence to Rust is bounded evidence, not a proof. Each section names its distinct observation, fixed quantities and falsifiable constraint.

## behavior_complement

Universe: finite byte pairs and finite offset lists, assuming successful allocation. Excluded outcomes contain a nonmatching offset, omit a matching offset, duplicate an offset or violate increasing order. This is a source-derived exact-target specification, not a universal Rust proof; bounded oracle checks its finite restriction.

## reverse_reconstruction

Fix pattern p and output offsets y. The compatible text fiber is {t : for every boundary i, i belongs to y iff t[i..i+|p|]=p, and y is increasing}. Empty p fixes text length through |y|-1 but loses all byte values. Enumerating bounded texts retains all compatible texts in that finite domain; tests check original-input membership.

## empty_boundary

Hold text length n fixed and set p empty: output is [0,...,n]. Conversely an empty-pattern result must begin at zero, end at n and contain n+1 entries. The empty/empty result is [0], not an empty vector. Boundary fixtures falsify the off-by-one variant.

## overlap_period

For two reported starts i<j with j-i<|p|, p[j-i..]=p[..|p|-(j-i)]. This necessary overlap constraint does not imply existence of either occurrence. Repeated-byte and alternating-prefix fixtures observe actual overlaps.

## alphabet_bijection

For any injective byte relabeling f, M(f(t),f(p))=M(t,p). Noninjective maps may create new matches. The finite oracle domain uses binary bytes; XOR-one tests cover a concrete bijection.

## prefix_extension

Fix p nonempty and append suffix s to t. Old offsets at most |t|-|p| are preserved; newly possible matches start beyond that boundary. This models result stability under append without promising fixed storage or prefix hash identity.

## hash_collision

H(s)=sum s[k]*257^(m-1-k) modulo 2^64. Equal bytes imply equal hash, but not conversely. [249,27,201,69,201,27,249,1] and eight zeros both hash to zero. Byte verification excludes the former as a match for the latter; the compiled mutation removes that verification.

## rolling_state

At loop start i, window=H(t[i..i+m]); power=257^(m-1). Update (window-t[i]*power)*257+t[i+m] preserves this invariant modulo 2^64. All arithmetic explicitly wraps; there is no probability assumption. Bounds i<=n-m protect slice sums.

## tests

Independent bounded oracles, explicit empty/prefix/overlap or tie/transpose witnesses, and compatible-preimage checks. Commands and normalized repeated results are retained under (see History). Source hashes are bound in manifest.json. Debug and release are each run twice; syntax projections are repeated and compared. A compiled production mutation must fail an assertion, then the source is restored exactly. See model.md#evidence for final outcomes.

### Native checks

## evidence

### Validation evidence

Strict cargo clippy --offline -p mova-algorithms --all-targets -- -D warnings passed. Focused rustfmt --check passed for all eight owned Rust sources. Eight partial syntax projections were generated twice by mova-modelcheck syntax and compared byte-for-byte.

Four production mutations compiled and failed assertions: remove Rabin-Karp byte verification (actual wrapping collision), replace Horspool shift by full pattern length (overlap loss), reverse substring tie preference, disable OSA transposition. The exact source bytes were restored after each run. Compiler failures were not accepted as negative controls.

Independent bounded comparisons: each search checks 255 binary texts times 31 patterns (7905 pairs); substring checks 63 squared (3969 pairs); OSA checks 31 squared (961 pairs). Additional fibers, boundary and unit fixtures are separate. OSA uses recursive script enumeration, substring uses descending-length slice comparisons, and search uses direct candidate slices.

## History

### 2026-10-30 · descriptant

2026-10-30 prospective model: O(n+m+c*m) byte/hash work, c hash candidates; O(n*m) worst case and O(1) auxiliary words plus output. Predicted correctness follows the named recurrence or exclusion relation, subject to native checks.

### 2026-10-30 · tests

2026-10-30: independent bounded oracles, explicit empty/prefix/overlap or tie/transpose witnesses, and compatible-preimage checks. Commands and normalized repeated results are retained under $MOVA_DATA/runs/octopus-wave7-strings-2026-10-30/. Source hashes are bound in manifest.json. Debug and release are each run twice; syntax projections are repeated and compared. A compiled production mutation must fail an assertion, then the source is restored exactly. See model.md#evidence for final outcomes.

### 2026-10-30 · evidence

2026-10-30, CPU, native Rust. Four newly implemented algorithms contribute 14 tests. With temporary local library exports, the crate passed 279 unit tests and one doctest in debug twice and release twice. Sorting the 280 passed-test records produced byte-identical normalized results across all four runs. Timings and compiler banners are excluded; raw logs remain external.

Artifacts: $MOVA_DATA/runs/octopus-wave7-strings-2026-10-30/{debug1,debug2,release1,release2}.log and .normalized, controls.log, mutant-*.log, clippy.log and *-syntax2.json. controls.rs is the native external mutation/normalization driver. These finite checks neither establish a universal proof nor benchmark throughput. No live API, GPU, teacher or bus execution occurred.
