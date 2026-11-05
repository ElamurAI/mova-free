# algorithm-matrix-chain

## descriptant

### matrix_chain implementation groupant

Original code; no external implementation was copied. The mathematical recurrence is standard dynamic programming. Raw Rust is authoritative outside this sidecar. Syntax is partial and omits expansion and semantic resolution.

Composition: source identity, forward recurrence, invariant and typed relations jointly constrain observations. behavior_complement excludes outcomes; reverse_reconstruction retains compatible antecedents. They do not supply a global inverse; leaf checks do not prove caller wiring.

## approximant

### Forward, invariant and cost

C(i,i)=0; increasing interval length computes minimum child costs plus scalar product. None represents overflow/infinity. Positive monotone sums justify dropping overflowing candidates without rejecting representable alternatives.

O(n^3) time and O(n^2) space for n>=2 matrices; linear dimension validation also applies to short chains.

## models

# Typed relations

Subject: mod.rs, source SHA-256 in manifest.json. Successful-result relations assume storage success.

## behavior_complement

Let T(d) be all binary parenthesization trees of positive dimensions, cost equal to the sum of scalar products at internal nodes. Universe: dimension/result triples. Under storage success, Ok(c) with c != min T is excluded; CostOverflow is excluded if any tree cost fits u128. Any zero dimension excludes Ok even on short inputs. Empty chains and single matrices cost zero.

## reverse_reconstruction

Ok(c) admits every positive vector whose cheapest tree costs c. Dimensions and trees are generally not unique. Restricted d=[2,x,3], c=24 determines x=4; finite_preimages executes this converse. General preimage enumeration remains absent.

## reversal

Reversing dimensions preserves optimum: mirror trees and transpose matrices. Every scalar product and total cost is unchanged; the tree correspondence is bijective.

## uniform_scaling

Scaling dimensions by positive q scales every tree cost and the mathematical minimum by q^3. This assumes scaled u64 dimensions are representable. The machine result becomes CostOverflow if the scaled minimum exceeds u128::MAX.

## constant_dimensions

For n>=1 matrices and all dimensions p>0, every tree has n-1 internal nodes costing p^3 each. Thus minimum=(n-1)*p^3 and no tree is identifiable from the cost.

## overflow_pruning

Unrepresentable interval minima act as infinity. Positive cost addition is monotone, so overflowing candidates cannot improve a representable minimum. A proper overflowing subchain may be unused; only the final cell determines CostOverflow.

## interval_composition

C(i,i)=0; C(i,j)=min over i<=k<j of C(i,k)+C(k+1,j)+d_i*d_(k+1)*d_(j+1). Increasing lengths complete dependencies before use. A minimizing split recovers compatible children, not unique input dimensions.

## resource_partition

Zeros reject before short-chain handling. For n>=2, n*n cells and Option<u128> byte size must fit usize and isize::MAX or DimensionOverflow follows. Reservation refusal yields AllocationFailed. Costs use checked multiplication and addition.

## tests

### Checks and limits

Four tests passed in each of two debug and two optimized standalone rustc --edition 2024 --test runs. Single-thread outputs matched after removing timing from the footer. Strict clippy-driver --test -D warnings and rustfmt passed. Compiled copied production mutant (maximize candidate cost) failed unchanged assertion checks. No mutation remains in repository source. Syntax extraction ran twice and matched bytes; source hashes are bound in manifest.json.

No allocator-refusal injection, benchmark, machine-domain proof or workspace integration is claimed. Parent integration owns exports and central registry wiring.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: 3,280 dimension vectors of length 0..7 over 1..3 against exhaustive parenthesization costs. Tests cover empty and zero cases, product overflow, sum overflow, an overflowing split with exact u128::MAX optimum, classical cost 15125, reversal, cubic scaling, constant dimensions and singleton preimages. Four tests passed twice each in debug and optimized standalone builds, with equal normalized outcomes. Strict lint passed and compiled mutants failed real assertions. Source syntax was identical twice. Reviewer correction replaced a non-overflowing chain witness with [MAX,MAX,1,2], and expanded coin cost accounting to include initialization and validation. Initial smoke tests preceded this recorded evidence; no prospective speedup claim was made. Finite observations are not universal proof. Allocation refusal, huge actual slices and complete preimage execution remain gaps. Preserve these observations in later epochs.

### 2026-10-30 · tests

2026-10-30: 3,280 dimension vectors of length 0..7 over 1..3 against exhaustive parenthesization costs. Tests cover empty and zero cases, product overflow, sum overflow, an overflowing split with exact u128::MAX optimum, classical cost 15125, reversal, cubic scaling, constant dimensions and singleton preimages.
