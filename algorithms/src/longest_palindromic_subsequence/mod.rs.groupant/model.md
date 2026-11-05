# algorithm-longest_palindromic_subsequence

## descriptant

### Combiner and epoch memory

Prospective prediction: deterministic exhaustive agreement, boundary correctness and a compiled semantic mutant rejected. Initial scope deliberately excludes throughput and downstream language quality. Preserve observations when later epochs revise these models. Full semantic AST, macro expansion, executable approximate substitutes and universal proof remain absent.

## model

# longest_palindromic_subsequence mod.rs model

## Forward

Return the maximum cardinality of indices i1<...<ik whose selected values equal their reversal. Empty yields 0. Interval recurrence D(i,j)=D(i+1,j-1)+2 for equal endpoints, otherwise max(D(i+1,j),D(i,j-1)); D(i,i)=1 and empty interval=0.

## Invariant

At iteration (left,right), row[right] before update is D(left+1,right), row[right-1] is D(left,right-1), and diagonal is D(left+1,right-1). Copy below before overwriting; after the inner loop row gives the completed interval row.

## Cost

Exactly n(n-1)/2 endpoint equality evaluations; O(n²) word operations and O(n) words. Equality cost is additional. Result at most n so additions fit usize for allocatable input.

## nonbehavior

Type: behavior_complement. Domain: Finite slices T:Eq under reflexive symmetric transitive stable equality, sufficient memory; output usize.

Universe (input,k). Exclude k<0 or k>n, k=0 for nonempty input, values without a palindromic subsequence witness, and values below any feasible palindromic witness length. Witness existence and maximality jointly characterize the result.

## compatible_antecedents

Type: reverse_reconstruction. Domain: Finite slices T:Eq under reflexive symmetric transitive stable equality, sufficient memory; output usize.

Given length n and observed k, compatible inputs are equality patterns admitting a palindromic index subsequence of size k and none larger. For an alphabet with at least n symbols: n=0 allows only k=0; n>0 allows precisely 1<=k<=n, witnessed by k copies of one symbol and n-k distinct other symbols. With smaller fixed alphabets extra constraints apply. Actual symbols and order are not uniquely recovered.

## reversal

Type: metamorphic_relation. Domain: Finite slices T:Eq under reflexive symmetric transitive stable equality, sufficient memory; output usize.

Reversing the input bijects subsequences and preserves palindromicity, hence preserves the optimum.

## injective_symbols

Type: metamorphic_relation. Domain: Finite slices T:Eq under reflexive symmetric transitive stable equality, sufficient memory; output usize.

An injective symbol mapping preserves equality patterns and length. Collapsing symbols can increase the optimum, as abc mapped to aaa.

## wrapped_endpoint

Type: composition_relation. Domain: Finite slices T:Eq under reflexive symmetric transitive stable equality, sufficient memory; output usize.

Adding the same symbol at both ends yields length(s)+2 by the equal-endpoint interval recurrence, including the empty inner interval.

## boundary_values

Type: output_constraint. Domain: Finite slices T:Eq under reflexive symmetric transitive stable equality, sufficient memory; output usize.

Nonempty input yields at least one; constant inputs yield n; pairwise-distinct inputs yield one. These boundaries exercise diagonal initialization and equality matching.

## subsequence_monotonicity

Type: order_relation. Domain: Finite slices T:Eq under reflexive symmetric transitive stable equality, sufficient memory; output usize.

Deleting elements cannot increase the optimum. Inserting one element increases it by at most two: remove its mirrored mate too from any optimum containing it; the remaining subsequence is palindromic.

## length_information_loss

Type: information_loss. Domain: Finite slices T:Eq under reflexive symmetric transitive stable equality, sufficient memory; output usize.

The scalar omits the witness, its position and its symbols. abc and xyz both give one; multiple distinct optimal subsequences may exist. Reconstruction needs more than the returned length.

## Limits and reference

Direct interval dynamic programming formulation of longest palindromic subsequence; independently written recurrence, not a call to longest common subsequence. No external implementation imported. Finite evidence is not a proof; partial syntax omits macro expansion and semantic resolution. No executable approximation profile is declared for these exact routines.

## evidence

### Evidence

Passed: two debug and two release executions of the 220-test library, including two new tests for this algorithm. Ordered successful test records are byte-identical across all four runs after removing timing and harness-order noise. Strict cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings and rustfmt --check on all six owned Rust files pass. One initial Clippy finding in the Huffman oracle was corrected to isolate_lowest_one before the final four-run verification.

Scope: 9,841 ternary sequences through length 8 against exhaustive index-subset palindrome enumeration, with reversal checks. Explicit empty, singleton, equal/distinct pairs, known examples, generic &str symbols, length-1,000 constant input, wrapping and scalar collisions.

Negative control: Changing equal-endpoint diagonal + 2 to diagonal + 1 in a separate copy compiles with rustc --edition 2024 --test; tests::boundaries_generic_and_loss rejects it with an assertion failure and exit 101. The production source is unchanged by mutation checks. Compile failure does not count as rejection.

Finite evidence is not a universal proof or throughput measurement. Exact routines have no approximate executable profile, and no downstream school gain is claimed.

## prospective

## History

### 2026-10-30 · descriptant

2026-10-30: Original std-only implementation and independent oracle. The combiner jointly relates raw Rust, source-bound partial syntax, forward target, ordered typed obernants, invariant, cost and measured assertions. Tests and implementation remain separate leaves; passing leaves do not certify integration wiring. Direct interval dynamic programming formulation of longest palindromic subsequence; independently written recurrence, not a call to longest common subsequence. No external implementation imported.

### 2026-10-30 · evidence

Heading `### Evidence 2026-10-30` stays in evidence as `### Evidence`.

Both Rust files have syntax projections generated twice by installed mova-modelcheck syntax and compared byte-for-byte; manifests bind exact source SHA-256. No macro expansion or semantic resolution is claimed. Logs, normalized records and Rust gate helper are under $MOVA_DATA/runs/octopus-wave5-strings-2026-10-30. Temporary lib.rs exports were used for local checks and restored before commit; parent registry/wiring validation belongs to integration.

### 2026-10-30 · prospective

#### Prediction recorded before final gates

2026-10-30. Predict longest_palindromic_subsequence agrees with its independent oracle over the bounded domain; debug/release result records repeat exactly. Predict semantic mutant fails a real assertion. Cost prediction: Exactly n(n-1)/2 endpoint equality evaluations; O(n²) word operations and O(n) words. Equality cost is additional. Result at most n so additions fit usize for allocatable input.
