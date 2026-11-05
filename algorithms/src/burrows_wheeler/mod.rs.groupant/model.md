# algorithm-burrows-wheeler

## descriptant

### algorithm-burrows-wheeler epoch memory

Sort all n cyclic rotations lexicographically with equal rotations ordered by ascending source offset. Return their last bytes C and the row p whose source offset is zero. Empty T returns ([],0). Inverse checks p then computes an LF candidate and accepts it iff its canonical transform equals (C,p).

Comparison-sort cyclic doubling and final sort take O(n log² n) time and O(n) storage. LF candidate decoding takes O(n+256); inverse's canonical retransformation adds O(n log² n), so complete inverse is O(n log² n+256), not linear. Allocation failure may panic/abort. No benchmark gain is claimed.

The prospective design retained comparison-sort construction costs and declined an unsupported linear total-time claim. Canonical BWT reachability is stricter than merely applying LF; byte wildcards deliberately have no escape grammar. Models distinguish information retained in full outputs from information discarded by projections. Test runs and compiled controls are recorded in model.md#evidence. No earlier modeling epoch is claimed. Partial Rust syntax is source-bound, with macro bodies opaque; it is neither a semantic AST nor proof. Future work may improve bounds only after prospective derivation and measurements.

## model

# algorithm-burrows-wheeler

## Forward

Sort all n cyclic rotations lexicographically with equal rotations ordered by ascending source offset. Return their last bytes C and the row p whose source offset is zero. Empty T returns ([],0). Inverse checks p then computes an LF candidate and accepts it iff its canonical transform equals (C,p).

## Invariant

Every doubling order is a permutation. Ranks identify and order cyclic prefixes of the current width; pair sorting doubles width. shifted avoids overflowing i+width. Final (rank,offset) sorting makes periodic ties deterministic. LF maps each byte occurrence to the same ordered occurrence in the sorted first column.

## Cost

Comparison-sort cyclic doubling and final sort take O(n log² n) time and O(n) storage. LF candidate decoding takes O(n+256); inverse's canonical retransformation adds O(n log² n), so complete inverse is O(n log² n+256), not linear. Allocation failure may panic/abort. No benchmark gain is claimed.

## nonbehavior

In the universe of text/result pairs, exclude altered byte multiplicities, wrong sorted-rotation last columns, and noncanonical primary rows. Inverse rejects out-of-range p before any reachability check, and rejects unreachable in-range (C,p) pairs. Source-derived excluded behavior is not a semantic proof.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## preimages

The full canonical pair (C,p) has exactly one input text when reachable, recovered by inverse; otherwise the fiber is empty. Projection to C alone retains ambiguity: abab and baba both produce bbaa, with primary rows 0 and 2. Neither column-only reconstruction nor accepting every LF cycle recovers the original.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## periodic-ties

Equal rotations of a periodic text are ordered by original offset. If the least cyclic period is d (so d divides n), each equal-rotation class has n/d members; source offset zero is first in its class. For bbaa only primary 0 or 2 is canonical; 1 and 3 are rejected despite decoding periodic candidates.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## lf-occurrence

LF(i)=number of column bytes less than C[i] plus count of equal bytes strictly before i. Stable occurrence numbering gives a permutation; equal symbols alone do not identify which occurrence was meant. Following LF n times from p reconstructs a candidate, not reachability certification.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## doubling

At width w<n, sorting (rank[i],rank[(i+w) mod n]) refines cyclic prefixes to length 2w. Once width reaches n, order of those prefixes agrees with full rotations, including equality. Saturating doubling cannot stall before reaching valid slice length n.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## alphabet-relabel

For a strictly increasing map f of byte support, transform(f(T))=(f(C),p). Sorting equivalence and stable tie offsets survive; mere injectivity without byte-order preservation does not preserve row order.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## error-partition

For n=0, p=0 is the sole valid metadata and decodes empty. For n>0, p>=n gives InvalidPrimaryIndex. For remaining metadata, canonical retransformation equality chooses Ok(candidate) versus InvalidTransform. These precedence partitions cover every byte column and usize primary.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## multiset

For each byte b, count_b(T)=count_b(C), since predecessor offsets form a permutation. This necessary condition does not certify a valid encoded pair. The inverse finite search oracle enumerates all possible input texts separately.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## evidence

### Evidence

Control: return primary zero; periodic_ties_and_boundaries rejects baba's primary two.

Syntax is generated twice and byte-compared. Strict algorithm Clippy and narrow rustfmt run separately. Structural validators check declarations, not semantic truth. Exhaustion, Unicode scalar matching, downstream application behavior and universal correctness remain outside the evidence.

Structural validation uses an external scaffold preserving all base registry owners and appending these six sources. A first minimal scaffold exposed existing owner checks; the corrected scaffold passes with no diagnostics and reports all six changed sources registered. The temporary missing scaffold-root sidecar and 473 preexisting missing sidecars are explicit backlog, not failures in these leaves.

## History

### 2026-10-30 · descriptant

2026-10-30: original implementation and fixtures; exact byte semantics and independent bounded oracles. Source SHA-256: af429214a1f01bd496b53a02b74a132504afd6606c9b8e58df71a24159443a45.

### 2026-10-30 · evidence

2026-10-30. Source SHA-256: af429214a1f01bd496b53a02b74a132504afd6606c9b8e58df71a24159443a45. Five module tests pass twice in debug and twice in release with identical normalized test identities/results. Raw logs: $MOVA_DATA/runs/octopus-wave10-text-2026-10-30/.

- burrows_wheeler-debug-1.log: SHA-256 4819eaea7c81fac155dbaa8d05a34495c63d94d4161da2f028ecea6eb90e5f31

- burrows_wheeler-debug-2.log: SHA-256 fd104938fbabc76cc45744b20b2d00afaeb1fe651a7cdf4ae30d07eadb2ced29

- burrows_wheeler-release-1.log: SHA-256 6eb3830dc86658a05077b1768431f65a104df9bd21028158d686a7efdcbf399f

- burrows_wheeler-release-2.log: SHA-256 5cdea5dd1b9749e2d1bf0f331164586c47cd0962b88c701e2b8cf4e58ee43e94

Compiled mutation log burrows_wheeler-mod-rs-mutant.log: SHA-256 e8458407a3ef3931aed66452bfb56f78b573be9884eae3baf7b3bd176b7e084a. Named assertion failure is required, not compilation failure; original source bytes restored and focused test passes.

Strict Clippy log SHA-256: ee26a23cab9e52eddcd50fa854be2183d0a2329a03f7c8cb922475aaf27ec62f. Source syntax was generated twice identically; syntax SHA-256: 3dcfa7aca0d79efe39221a28a0e0c15357536b58afde96030179fa40a02a7eee.
