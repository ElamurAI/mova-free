# spectral-linear-algebra

## model

# spectral-linear-algebra

## forward

Tagged dense matrix factorization and spectral observations: pivoted LU, symmetric Jacobi eigenpairs, modified Gram-Schmidt basis, dominant-eigen iteration and Lanczos tridiagonal projection. Reconstruction/residual observations differ from eigenvector accuracy. Real identities: PA=LU; A=QR and Q^TQ=I; AV=VD,V^TV=I; T=Q^TAQ. Native residuals measure departures.

## invariant

Dimensions, finite values, symmetry/rank conditions, sign/tie conventions and statuses remain leaf-specific. Permutation and basis coordinates must travel with results; a small residual does not by itself identify a unique eigenvector.

## cost

LU O(n³), O(n²) storage. Jacobi actually recomputes ||AV-VD||F via n matrix-vector products per rotation candidate; budget b costs O((b+1)n³), with repeated allocations. MGS m-by-n costs O(m n²); power O((b+1)n²); k-vector Lanczos O(k n²+n k²) with reorthogonalization and O(nk+k²) storage. Leaf caps/statuses remain binding.

## composition

LU solves require the returned permutation and nonsingularity policy. MGS Q supplies orthonormal coordinates only on its admitted rank domain. Lanczos T is a projection Q^T A Q, not the entire matrix; Ritz vectors require lifting by Q. Comparing Jacobi eigenvalues to power iteration requires matching selected eigenvalue and convergence status, especially magnitude ties and multiplicity. Floating residuals need conditioning and spectral-gap assumptions before any solution-error interpretation. Power dominance additionally requires a magnitude gap and nonzero initial dominant-eigenspace projection; small residual alone allows a nondominant eigenpair.

## limitations

Successful allocations and each leaf's finite domain/error precedence. These are tagged relations, not automatic production adoption. Floating residuals are not solution-error guarantees. No generic 1/2/5/10% composition-wide profile; finite evidence remains separate from analytical claims.

## epoch-memory

Wave26. Tagged library tools; no automatic training adoption, source-volume quality claim or generic numerical profile.

## checks

["cargo test -p mova-algorithms lu_decomposition::tests","cargo test -p mova-algorithms jacobi_eigen::tests","cargo test -p mova-algorithms modified_gram_schmidt::tests","cargo test -p mova-algorithms power_iteration::tests","cargo test -p mova-algorithms lanczos::tests"]

## wave26-executed-checks

# spectral-linear-algebra: wave26 checks

31 source-local test declarations across five modules. Every module has a compiled failing production mutation followed by exact restoration. The final whole library passed 877 checks twice debug and twice release. See [integrated evidence](../algorithm-library.groupant/model.md#wave26-evidence) for scopes and limitations. These tagged APIs are not an invented serial pipeline.

## catalog

# Typed family relations

## behavior-complement

Type: behavior_complement.

In the tagged family relation, exclude outputs violating the selected leaf's invariant, domain/error precedence or coordinate convention. In particular: Dimensions, finite values, symmetry/rank conditions, sign/tie conventions and statuses remain leaf-specific. Permutation and basis coordinates must travel with results; a small residual does not by itself identify a unique eigenvector. Cross-family equality without the adapter assumptions in composition is excluded.

## reverse-reconstruction

Type: reverse_reconstruction.

Exact complete LU reconstructs A=P^-1LU and full QR reconstructs A=QR. Eigenvector signs and repeated eigenspaces are ambiguous. Partial Q,T determines Q^TAQ but leaves action outside span(Q) undetermined. Rounded factors admit tolerance fibers.

## permutation_coordinate

Type: permutation_coordinate.

Ignoring LU permutation changes the solved equation; reconstructed coordinates require the exact pivot convention.

## basis_projection

Type: basis_projection.

MGS and Lanczos bases define coordinates; Q^T A Q can agree while omitted orthogonal directions differ.

## spectral_multiplicity

Type: spectral_multiplicity.

Power has no dominance guarantee without a gap and nonzero initial projection; signs and repeated-eigenvalue bases are nonunique.

## residual_conditioning

Type: residual_conditioning.

Residual thresholds are stopping observations. Eigenvalue gaps and conditioning are extra assumptions, not automatic guarantees.

## wave26-evidence

### spectral-linear-algebra: wave26 checks

31 source-local test declarations across five modules. Every module has a compiled failing production mutation followed by exact restoration. The final whole library passed 877 checks twice debug and twice release. See [integrated evidence](../algorithm-library.groupant/model.md#wave26-evidence) for scopes and limitations. These tagged APIs are not an invented serial pipeline.

## History

### 2026-10-30 · epoch-memory

Wave26, 2026-10-30. Tagged library tools; no automatic training adoption, source-volume quality claim or generic numerical profile.
