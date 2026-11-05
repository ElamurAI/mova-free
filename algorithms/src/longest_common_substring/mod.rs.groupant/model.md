# algorithm-longest-common-substring

## descriptant

### Combiner and epoch memory

Combiner inputs are raw Rust, its partial syntax projection, forward/invariant models, the ordered typed family and this memory. Output candidates are local recurrence, tie or shift changes, filtered by invariant and oracle agreement. It is a descriptive interface, not an implemented autonomous model generator.

Epoch evidence is retained in model.md#tests. Finite agreement narrows uncertainty but does not prove arbitrary input correctness. No useful faster executable approximant is supplied for these exact observations; categorical errors have no accepted percentage budget.

## approximant

### Forward model and invariant

Select (i,j,l) with a[i..i+l]=b[j..j+l], maximum l then minimum (i,j); choose (0,0,0) when l=0.

O(n*m) equality operations, O(m) row words. Eq operation costs are external.

Inputs are immutable. Returned offsets/witness indices stay in bounds; returned edit cost is nonnegative. Public Rust docs define empty, element-unit, overflow and allocation behavior.

Dan Gusfield, Algorithms on Strings, Trees, and Sequences, Cambridge University Press, 1997, doi:10.1017/CBO9780511574931 (longest common substring problem). Original rolling-DP implementation.

## models

# Typed relations

Revision: source hash in manifest. Observation: complete single call, lawful Eq and successful allocation unless explicitly scoped otherwise. Model direction is an exact-target relational specification; correspondence to Rust is bounded evidence, not a proof. Each section names its distinct observation, fixed quantities and falsifiable constraint.

## behavior_complement

Universe is pairs of finite Eq slices and triples of natural indices. Exclude out-of-bounds, unequal slices, nonmaximal length, and later tied starts. A feasible witness alone does not certify optimality; independent decreasing-length enumeration tests the finite specification.

## reverse_reconstruction

Fix b and witness w. Fiber contains every a with a[w.a_start..w.a_start+w.len]=b[w.b_start..w.b_start+w.len], no longer equal pair, and no earlier equal-length pair. Empty witness additionally requires no equal element. Many prefixes and suffixes remain unconstrained; finite enumeration checks membership, not unique recovery.

## suffix_recurrence

D(i+1,j+1)=D(i,j)+1 if a[i]=b[j], else zero. Reverse j traversal preserves the previous-row diagonal while overwriting current cells. Row zero stays zero. Maximum suffix lengths enumerate all common substrings.

## tie_selection

For a fixed maximum positive length, select lexicographically minimal (a_start,b_start), independent of reverse-column visitation. Tied separated runs and repeated b occurrences distinguish this from last-visited selection.

## empty_fiber

If either input is empty, return default witness without allocation. More generally length zero iff the two slices have disjoint equality classes. Default indices are canonical even when many zero-length windows exist.

## swap_projection

Swapping a,b preserves maximum length but need not simply swap selected indices: lexicographic tie priority changes. Tests assert only length symmetry and direct witness validity.

## unit_projection

Eq acts on supplied elements. UTF-8 bytes for é yield length two, a scalar slice yields one. Returned byte indices may split scalar encodings for arbitrary byte matches; no text boundary promise follows.

## storage_bound

Every suffix cell is <=min(i+1,j+1), so increments remain within actual slice lengths. b.len()+1 uses checked_add before row allocation. Vec allocation failure panics/aborts by allocator policy; the API offers no recoverable allocation error.

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

2026-10-30 prospective model: O(n*m) equality operations, O(m) row words. Eq operation costs are external. Predicted correctness follows the named recurrence or exclusion relation, subject to native checks.

### 2026-10-30 · tests

2026-10-30: independent bounded oracles, explicit empty/prefix/overlap or tie/transpose witnesses, and compatible-preimage checks. Commands and normalized repeated results are retained under $MOVA_DATA/runs/octopus-wave7-strings-2026-10-30/. Source hashes are bound in manifest.json. Debug and release are each run twice; syntax projections are repeated and compared. A compiled production mutation must fail an assertion, then the source is restored exactly. See model.md#evidence for final outcomes.

### 2026-10-30 · evidence

2026-10-30, CPU, native Rust. Four newly implemented algorithms contribute 14 tests. With temporary local library exports, the crate passed 279 unit tests and one doctest in debug twice and release twice. Sorting the 280 passed-test records produced byte-identical normalized results across all four runs. Timings and compiler banners are excluded; raw logs remain external.

Artifacts: $MOVA_DATA/runs/octopus-wave7-strings-2026-10-30/{debug1,debug2,release1,release2}.log and .normalized, controls.log, mutant-*.log, clippy.log and *-syntax2.json. controls.rs is the native external mutation/normalization driver. These finite checks neither establish a universal proof nor benchmark throughput. No live API, GPU, teacher or bus execution occurred.
