# algorithm-lcp-array

## descriptant

### algorithm-lcp-array epoch memory

For finite byte text T of length n, return (A,L): A sorts the nonempty suffixes, L[0]=0 if n>0, and L[r]=max k with T[A[r-1]..A[r-1]+k]=T[A[r]..A[r]+k]. Empty input returns two empty vectors.

Existing suffix_array costs O(n log² n); the Kasai phase costs O(n) time and O(n) auxiliary space. Each successful byte comparison increases common and each outer iteration decreases it by at most one. This is not an overall linear suffix constructor. Allocation exhaustion is outside the terminating domain.

The prospective design retained comparison-sort construction costs and declined an unsupported linear total-time claim. Canonical BWT reachability is stricter than merely applying LF; byte wildcards deliberately have no escape grammar. Models distinguish information retained in full outputs from information discarded by projections. Test runs and compiled controls are recorded in model.md#evidence. No earlier modeling epoch is claimed. Partial Rust syntax is source-bound, with macro bodies opaque; it is neither a semantic AST nor proof. Future work may improve bounds only after prospective derivation and measurements.

## model

# algorithm-lcp-array

## Forward

For finite byte text T of length n, return (A,L): A sorts the nonempty suffixes, L[0]=0 if n>0, and L[r]=max k with T[A[r-1]..A[r-1]+k]=T[A[r]..A[r]+k]. Empty input returns two empty vectors.

## Invariant

A is a permutation of 0..n; rank[A[r]]=r. The carried prefix is a lower bound for the next comparison after subtracting one. All comparisons obey common<n-start and common<n-previous, bounding additions by n-1.

## Cost

Existing suffix_array costs O(n log² n); the Kasai phase costs O(n) time and O(n) auxiliary space. Each successful byte comparison increases common and each outer iteration decreases it by at most one. This is not an overall linear suffix constructor. Allocation exhaustion is outside the terminating domain.

## nonbehavior

Within U = finite input texts paired with arbitrary vector pairs, absent behaviors include unequal output lengths, nonpermutation A, L[0]!=0, and any adjacent L value differing from direct byte-prefix equality. This is a source-derived inner characterization of excluded outputs under successful allocation.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## preimages

For fixed (A,L), retain every length-n byte text whose suffix order is A and whose adjacent common-prefix lengths are L. The fiber is usually non-singleton: ([1,0],[0,1]) admits every equal-byte pair, including three pairs over {0,1,2}. Order/LCP do not recover byte labels.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## rank-state

Fix suffix order A. The inverse-rank vector is uniquely rank[A[r]]=r; an inconsistent rank is excluded. Fix rank without T and the compatible inputs remain all texts inducing that suffix order; rank does not supply byte identity.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## carried-prefix

At each start with positive rank, common is a lower bound on LCP with its lexicographic predecessor. After measuring h, the next candidate starts at max(h-1,0); encountering the least suffix resets the bound. This relies on suffix-order predecessor constraints, not arbitrary permutations.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## interval-minimum

For fixed ranks a<b, LCP(T[A[a]..],T[A[b]..]) = min L[a+1..=b]. The interval is nonempty and the suffix order must correspond to this exact text; swapping a suffix array from another text invalidates the premise.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## relabel

A strictly increasing byte map on the observed alphabet preserves both A and L. Injective decreasing maps preserve equality lengths only for a fixed pair of offsets, not the ordered arrays; noninjective maps can lengthen prefixes.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## distinct-substrings

For fixed n and L, the number of distinct nonempty substrings is n(n+1)/2 - sum L. This mathematical integer identity is tested with explicit substring sets at n=5; API does not perform the potentially overflowing product.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## constant-text

For n equal bytes, A=[n-1,...,0] and L=[0,1,...,n-1]. For n=0 both vectors are empty. Long constant fixtures exercise maximal carried prefixes and finite suffix bounds.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## evidence

### Evidence

Control: replace lcp[r]=common with zero; boundaries_and_carried_prefix rejects banana's expected LCP.

Syntax is generated twice and byte-compared. Strict algorithm Clippy and narrow rustfmt run separately. Structural validators check declarations, not semantic truth. Exhaustion, Unicode scalar matching, downstream application behavior and universal correctness remain outside the evidence.

Structural validation uses an external scaffold preserving all base registry owners and appending these six sources. A first minimal scaffold exposed existing owner checks; the corrected scaffold passes with no diagnostics and reports all six changed sources registered. The temporary missing scaffold-root sidecar and 473 preexisting missing sidecars are explicit backlog, not failures in these leaves.

## History

### 2026-10-30 · descriptant

2026-10-30: original implementation and fixtures; exact byte semantics and independent bounded oracles. Source SHA-256: 6d51acf8a319ef825660c6fc96d629c384239d949fa1fe36d6917c12aee639c4.

### 2026-10-30 · evidence

2026-10-30. Source SHA-256: 6d51acf8a319ef825660c6fc96d629c384239d949fa1fe36d6917c12aee639c4. Five module tests pass twice in debug and twice in release with identical normalized test identities/results. Raw logs: $MOVA_DATA/runs/octopus-wave10-text-2026-10-30/.

- lcp_array-debug-1.log: SHA-256 21d7e4bdc9b42c16e789ef9ab9012f786d26b0b18239be4e5ce2a18b85e97d88

- lcp_array-debug-2.log: SHA-256 0b8df33a6fee80e288458354d9f76bbfed1cfedf1ce2241ae8d4bcb5de8d0f05

- lcp_array-release-1.log: SHA-256 4c251490e30659849cc12b1a8159835a1f8ae209fa8ff53286c8bc12d3cf0639

- lcp_array-release-2.log: SHA-256 6d4bc8e0409c5df5e527130a704576b5028376e841013fd35a59fea73c4727e2

Compiled mutation log lcp_array-mod-rs-mutant.log: SHA-256 454a42d569fcd173a831c29d8fd2e886307e31ef8185a43dea79d9bf616e1657. Named assertion failure is required, not compilation failure; original source bytes restored and focused test passes.

Strict Clippy log SHA-256: ee26a23cab9e52eddcd50fa854be2183d0a2329a03f7c8cb922475aaf27ec62f. Source syntax was generated twice identically; syntax SHA-256: dda7d6b56fb51a9816c73915d1833a0d76f36d734204e08569d2a3ef345fb58a.
