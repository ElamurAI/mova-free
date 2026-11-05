# algebra-transforms

## model

# algebra-transforms

## forward

Compute a bounded complex radix-two DFT and four exact fixed-field operations: polynomial multiplication, disjoint-subset convolution, affine linear-system solution and least discrete logarithm. Complex f64 and field-residue outputs are separate semantic types.

## invariant

Modular inputs normalize to p=998244353, products remain within u64 bounds, and outputs retain canonical residues/shape/ties. FFT forward phase is negative and inverse divides by n. Polynomial coefficient index addition and subset disjoint union are distinct products.

## cost

FFT O(n log n)/O(n), n power2<=65536. Karatsuba O(N^log2(3))/O(N) simultaneously live memory for padded N<=4096. Subset length2^d<=4096: O(d²2^d)/O(d2^d). Gaussian m,n<=128: conservative O(m*n*min(m,n)+m*(n+1)+n²) work plus bounded fixed-prime exponentiation, O(m*(n+1)+n²) memory including zero-variable rows. BSGS31596 entries, O(sqrt(p)log p) ordered-map work/O(sqrt(p)) memory.

## composition

Each leaf is std-only; no production calls among these modules. Zero-padded FFT convolution may be compared with Karatsuba only on small representable integer fixtures with explicit rounding/error checks. Fixed-field subset convolution with invertible empty coefficient yields a triangular field system solvable by Gaussian elimination only when the2^d unknowns fit128. BSGS output can be checked by modular powering under the same fixed modulus.

## epoch-memory

Wave25. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

## checks

Named joins: fft_exact_field_fixture_join, subset_triangular_reconstruction_rejects_or_join, discrete_log_target_join. See ../algorithm-library.groupant/model.md#wave25-evidence and wave25-validation.md there for integrated receipts. These separately tagged tests do not imply a serial runtime pipeline.

## limitations

FFT inverse identities and convolution theorem are real/complex target identities, not universal f64 error bounds or an isomorphism to a finite field. Modular algebra is exact inside fixed prime; no caller-supplied composite modulus is supported. Allocator exhaustion follows std policy. No callbacks or executable percentage profiles.

## catalog

# Typed family relations

Each relation keeps its own domain and assurance.

## behavior-complement

Type: behavior_complement.

Exclude wrong modular product coefficients, overlapping-subset terms, noncanonical residues, invalid affine particular/nullspace certificates or a nonleast/incorrect discrete exponent. For FFT exclude wrong length, nonfinite output, wrong phase/normalization or wrong native butterfly result; do not impose exact complex-theory equalities on rounded values. Exclude replacing field residues with raw floating magnitudes or equating resource refusal with algebraic None.

## reverse-reconstruction

Type: reverse_reconstruction.

Ideal fixed-size DFT is invertible; rounded FFT can merge input bit patterns. Polynomial product admits factor scaling(a*c,b/c) and zero-product ambiguity. Subset output with fixed a and a[empty]!=0 uniquely recovers b in the field, while zero a admits every b. Gaussian output admits many row-equivalent systems. BSGS Some0 admits every base with target1, and modular normalization makes raw representatives nonunique.

## fft-polynomial-join

Type: conditional_convolution_transport.

Zero-pad real coefficient vectors to power2 length at least len(a)+len(b)-1, transform, pointwise multiply, inverse, and compare small integer coefficients with Karatsuba modulo p. Without padding the FFT gives circular convolution. A measured small-fixture error permits rounding only there; no blanket exact field recovery follows for arbitrary coefficients or cancellation.

## subset-product-rejection

Type: disjoint_union_semantics.

Subset convolution c[S]=sum_{T subset S}a[T]b[S\\T] counts disjoint unions. Multiplying ordinary unranked zeta transforms and inverting yields OR convolution including overlap. Rank bookkeeping followed by selecting popcount(S) is essential; plain polynomial index addition is also not interchangeable with subset union.

## triangular-reconstruction

Type: conditional_linear_system_join.

For fixed a, set M[S,U]=a[S\\U] if U subset S, else0, so c=M*b. With ascending mask order the matrix is triangular and diagonal a[empty]. If a[empty]!=0 the field solution is unique; if it is0 uniqueness cannot be assumed. Gaussian integration is limited to2^d<=128 and matching modulus, narrower than standalone subset convolution.

## affine-solution-space

Type: rank_nullity_certificate.

A successful Gaussian result gives A*particular=b, A*z=0 for every returned basis vector and exactly n-rank(A) independent vectors; all solutions are particular+sum t_i*z_i. Empty equations yield zero particular and identity basis. Zero variables remain consistent iff every reduced rhs is0; shape/resource errors remain distinct from None.

## discrete-log-conventions

Type: least_exponent_relation.

For nonzero base, scan increasing giant blocks and retain the least baby exponent to return least x in0..p-2. Base0 gives target1->Some0, target0->Some1, other targets->None; nonzero base cannot reach0. A valid exponent without leastness is insufficient, and field inversion must not run on0.

## finite-arithmetic-bound

Type: representation_contract.

Normalize raw u64 operands before multiplication; (p-1)²<2^60 and the source's bounded reduced sums/subtractions fit u64. FFT's component cap and length bound keep sums far from overflow but do not bound relative error near zero. These separate finite-arithmetic arguments cannot be combined into one numerical-accuracy percentage.

## wave25-evidence

# algebra-transforms: wave25 evidence

## checks (wave25-evidence)

Named joins: fft_exact_field_fixture_join, subset_triangular_reconstruction_rejects_or_join, discrete_log_target_join. See ../algorithm-library.groupant/model.md#wave25-evidence and wave25-validation.md there for integrated receipts. These separately tagged tests do not imply a serial runtime pipeline.

## History

### 2026-10-30 · epoch-memory

Wave25, 2026-10-30. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

### 2026-10-30 · checks

2026-10-30: this family adds 10 source-local tests and 3 independent composition observations. The full 175-module library passed 782 checks twice in debug and twice in release with identical normalized outcomes. All five production mutants in this family failed named assertions; exact restoration and positive checks followed. Root inspected the failure receipts. Tests are finite observations, not universal semantic or floating-point proofs.

### 2026-10-30 · checks (wave25-evidence)

2026-10-30: this family adds 10 source-local tests and 3 independent composition observations. The full 175-module library passed 782 checks twice in debug and twice in release with identical normalized outcomes. All five production mutants in this family failed named assertions; exact restoration and positive checks followed. Root inspected the failure receipts. Tests are finite observations, not universal semantic or floating-point proofs.
