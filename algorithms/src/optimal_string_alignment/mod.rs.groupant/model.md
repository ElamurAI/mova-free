# algorithm-optimal-string-alignment

## descriptant

### Combiner and epoch memory

Combiner inputs are raw Rust, its partial syntax projection, forward/invariant models, the ordered typed family and this memory. Output candidates are local recurrence, tie or shift changes, filtered by invariant and oracle agreement. It is a descriptive interface, not an implemented autonomous model generator.

Epoch evidence is retained in model.md#tests. Finite agreement narrows uncertainty but does not prove arbitrary input correctness. No useful faster executable approximant is supplied for these exact observations; categorical errors have no accepted percentage budget.

## approximant

### Forward model and invariant

D(i,j)=min(D(i-1,j)+1,D(i,j-1)+1,D(i-1,j-1)+[a[i-1]!=b[j-1]],D(i-2,j-2)+1 when crossed adjacent elements agree); boundaries D(i,0)=i,D(0,j)=j.

O(n*m) equality/arithmetic operations; three rows of O(min(n,m)) words.

Inputs are immutable. Returned offsets/witness indices stay in bounds; returned edit cost is nonnegative. Public Rust docs define empty, element-unit, overflow and allocation behavior.

Robert A. Wagner and Michael J. Fischer, The String-to-String Correction Problem, JACM 21(1), 1974, doi:10.1145/321796.321811, for edit DP; Frederick J. Damerau, A technique for computer detection and correction of spelling errors, CACM 7(3), 1964, doi:10.1145/363958.363994, for adjacent-transposition motivation. This restricted recurrence is explicitly not unrestricted Damerau distance.

## models

# Typed relations

Revision: source hash in manifest. Observation: complete single call, lawful Eq and successful allocation unless explicitly scoped otherwise. Model direction is an exact-target relational specification; correspondence to Rust is bounded evidence, not a proof. Each section names its distinct observation, fixed quantities and falsifiable constraint.

## behavior_complement

Universe: finite Eq slice pairs and nonnegative integer results under successful allocation. Exclude results below absolute length difference, above max length, nonzero identical-input costs and costs inconsistent with restricted edit scripts. These necessary bounds alone do not determine the distance.

## reverse_reconstruction

Fix b and cost k. Fiber is all a whose minimum restricted edit-script cost to b equals k; k=0 fixes a=b under lawful Eq. For k>0 several sequences qualify. Bounded recursive script enumeration checks finite fiber membership; no unique reconstruction or unbounded solver is claimed.

## restricted_script

The transposition term consumes two adjacent elements on both sides and moves to D(i-2,j-2). It cannot reuse the transposed substring for another edit. The independent recursive prefix oracle consumes disjoint suffixes, testing the same restriction without row swaps.

## triangle_failure

CA->AC costs one; AC->ABC costs one; CA->ABC costs three under OSA. Unrestricted edits admit transpose then insert for cost two. Thus composing optimal restricted scripts need not remain restricted and OSA is not a metric.

## symmetry_bound

Transposing both DP axes exchanges insertion/deletion and preserves substitution/transposition conditions. Distance is symmetric, at least abs(n-m), at most max(n,m). Input swap permits allocating the shorter axis; bounded checks exercise symmetry.

## row_age

Before row i, previous stores D(i-1,*), older stores D(i-2,*) when i>=2, and current is scratch. Two swaps preserve these ages. A transposition mutation disables the only older-row candidate and the CA/AC assertion rejects it.

## unit_and_empty

Empty-input distance equals the other slice length and allocates no rows. Scalar é->e costs one; its UTF-8 bytes to e cost two. No normalization or grapheme rules are implicit.

## arithmetic_resource

Nonempty lengths must be <usize::MAX, making row dimensions representable. Candidate additions saturate, so a dominated overflowing candidate cannot panic or wrap; a finite optimal cost <=max(n,m) remains. Vec allocation may panic/abort; enormous zero-sized Eq slices are not a practical iteration promise.

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

2026-10-30 prospective model: O(n*m) equality/arithmetic operations; three rows of O(min(n,m)) words. Predicted correctness follows the named recurrence or exclusion relation, subject to native checks.

### 2026-10-30 · tests

2026-10-30: independent bounded oracles, explicit empty/prefix/overlap or tie/transpose witnesses, and compatible-preimage checks. Commands and normalized repeated results are retained under $MOVA_DATA/runs/octopus-wave7-strings-2026-10-30/. Source hashes are bound in manifest.json. Debug and release are each run twice; syntax projections are repeated and compared. A compiled production mutation must fail an assertion, then the source is restored exactly. See model.md#evidence for final outcomes.

### 2026-10-30 · evidence

2026-10-30, CPU, native Rust. Four newly implemented algorithms contribute 14 tests. With temporary local library exports, the crate passed 279 unit tests and one doctest in debug twice and release twice. Sorting the 280 passed-test records produced byte-identical normalized results across all four runs. Timings and compiler banners are excluded; raw logs remain external.

Artifacts: $MOVA_DATA/runs/octopus-wave7-strings-2026-10-30/{debug1,debug2,release1,release2}.log and .normalized, controls.log, mutant-*.log, clippy.log and *-syntax2.json. controls.rs is the native external mutation/normalization driver. These finite checks neither establish a universal proof nor benchmark throughput. No live API, GPU, teacher or bus execution occurred.
