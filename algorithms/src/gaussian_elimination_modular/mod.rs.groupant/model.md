# algorithm-gaussian-elimination-modular

## descriptant

### gaussian_elimination_modular

Prospective model saved before implementation in model.md#prospective (relative from source directory). Original standard-library code. This epoch adds explicit supported domains and independent finite checks. See model.md for typed excluded-output and preimage relations. Partial syntax omits macro expansion and semantic resolution. No executable approximation profile is attached; useful cheaper metrics and evidence remain unsupported. No timing gain is claimed.

## model

# gaussian_elimination_modular mod.rs

## Forward

RREF over fixed prime998244353. First available nonzero row pivot; normalize and eliminate all other rows. Inconsistent zero coefficient row=>None; otherwise free variables0 yield particular, each free variable1 yields one nullspace vector. Explicit variable count handles no equations. rows/variables<=128. O(rows*(vars+1)*(min(rows,vars)+1)+vars²) work O(rows*(vars+1)+vars²) memory. Golub and Van Loan, Matrix Computations (4th ed.), Gaussian elimination; exact field adaptation.

## Invariant

Columns ascend, first available nonzero row chosen. Pivot row normalized; eliminate every other row. Free columns ascend, set own free coordinate1 and others0, yielding a deterministic basis.

## Cost

RREF over fixed prime998244353. First available nonzero row pivot; normalize and eliminate all other rows. Inconsistent zero coefficient row=>None; otherwise free variables0 yield particular, each free variable1 yields one nullspace vector. Explicit variable count handles no equations. rows/variables<=128. O(rows*(vars+1)*(min(rows,vars)+1)+vars²) work O(rows*(vars+1)+vars²) memory. Golub and Van Loan, Matrix Computations (4th ed.), Gaussian elimination; exact field adaptation.

## behavior_complement

For accepted affine solution (v,N), Av!=b or any column vector u with Au!=0 is excluded; missing nullspace dimension n-rank(A) is excluded. None is excluded for a consistent system.

## reverse_reconstruction

Observed (v,N) admits all matrices and rhs whose deterministic RREF selection yields that affine basis; multiplying a row and its rhs by nonzero c preserves the result. None has many inconsistent preimages, e.g. 0*x=1 and 0*x=2.

## pivot_ties

Columns ascend, first available nonzero row chosen. Pivot row normalized; eliminate every other row. Free columns ascend, set own free coordinate1 and others0, yielding a deterministic basis.

## solution_space

All solutions are particular+sum t_i nullspace[i]. Empty equation set yields zero particular and identity basis; zero variables returns empty solution iff every rhs is zero.

## finite_field

Fixed p998244353 supports Fermat inverses only for nonzero pivots. Normalize raw u64 before arithmetic; products<2^60 and subtraction expression uses+p to avoid underflow.

## validation_order

Rows>128 or variables>128 returns TooLarge before shape checks. Then rhs length and every row width must match. Standard allocations can abort; no error conflation with mathematical inconsistency.

## evidence

Two tests per algorithm cover independent finite oracles and boundaries. Receipts: (see History). Paired debug/release logs, compiled-mutant rejection and restored rerun logs are retained there. Exact positive counts and final verification are reported in the integration receipt. Syntax projections are generated twice and compared. Source bindings are SHA-256; typed models are conditional specifications, not proofs. Allocator failure is not injected.

### Evidence

## prospective

### Prospective model

RREF over fixed prime998244353. First available nonzero row pivot; normalize and eliminate all other rows. Inconsistent zero coefficient row=>None; otherwise free variables0 yield particular, each free variable1 yields one nullspace vector. Explicit variable count handles no equations. rows/variables<=128. O(rows*vars*min(rows,vars)+vars²) work O(rows*vars+vars²) memory. Golub and Van Loan, Matrix Computations (4th ed.), Gaussian elimination; exact field adaptation.

Validation precedes arithmetic. Modular inputs normalize before products; products fit u64. Resource caps are policy refusals, distinct from algebraic absence. Allocator exhaustion is not recoverable in this bounded std implementation. No callbacks. Independent small oracles, repeated debug/release and compiled mutants are planned. No executable approximation profile is supported: no validated useful cheaper metric yet. Expected asymptotic improvement is a hypothesis, not measured speed.

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Two tests per algorithm cover independent finite oracles and boundaries. Receipts: $MOVA_DATA/runs/octopus-wave25-algebra-2026-10-30. Paired debug/release logs, compiled-mutant rejection and restored rerun logs are retained there. Exact positive counts and final verification are reported in the integration receipt. Syntax projections are generated twice and compared. Source bindings are SHA-256; typed models are conditional specifications, not proofs. Allocator failure is not injected.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
