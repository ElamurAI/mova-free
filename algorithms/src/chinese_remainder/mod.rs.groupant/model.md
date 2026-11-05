# algorithm-chinese-remainder

## descriptant

### algorithm-chinese-remainder combiner

The combiner joins source-bound partial syntax, forward model, behavior complement first, reconstruction second, invariant and finite tests. Each disagreement is an open obligation; valid leaves alone do not prove wiring. Independently scan 1..product(moduli) for the least common positive period, then 0..period for the first simultaneous residue. Bounded fixtures keep this brute force finite without Euclidean coefficients or production merge recurrence.

Generalized Chinese remainder theorem and Bezout identity are classical; see Kenneth H. Rosen, Elementary Number Theory and Its Applications, chapter on congruences. The implementation composes the repository extended_gcd and original widened modular arithmetic; no external source code copied.

Epoch memory: After each successful prefix the normalized class is exactly the intersection of prefix solution sets and its modulus is the prefix LCM. Bezout x satisfies m*x+n*y=g, so (m/g)*x=1 modulo n/g. Signed difference fits i128. Normalize its quotient and x into [0,n/g); their product fits u128. When L fits u64, a+m*t<L because a<m and t<n/g, so addition fits u128. Zero preflight makes all divisions defined.

## model

# algorithm-chinese-remainder

## Forward

Preflight the entire list for the first zero modulus and return ZeroModulus{index}. Otherwise fold from (0,1). For current (a,m) and normalized next (b,n), set g=gcd(m,n). If g does not divide b-a, return None. Otherwise L=m*(n/g); if L>u64::MAX return ModulusOverflow. Let t=(((b-a)/g) * inverse(m/g modulo n/g)) modulo n/g, using the Bezout coefficient of m for gcd(m,n). Return updated ((a+m*t) modulo L,L), eventually Some. Modulus one uses the sole zero residue. This operational error policy is part of the target.

## Invariant

After each successful prefix the normalized class is exactly the intersection of prefix solution sets and its modulus is the prefix LCM. Bezout x satisfies m*x+n*y=g, so (m/g)*x=1 modulo n/g. Signed difference fits i128. Normalize its quotient and x into [0,n/g); their product fits u128. When L fits u64, a+m*t<L because a<m and t<n/g, so addition fits u128. Zero preflight makes all divisions defined.

## Cost

For k constraints and U=u64::MAX, O(k log U) word operations, O(1) auxiliary storage. Preflight O(k). No benchmark or numerical approximation claim.

## nonbehavior

Type: behavior_complement. Domain: Finite slices of u64 congruences and candidate Result<Option<Congruence>,Error> observations, with normal terminating execution.

U minus graph(S) excludes unnormalized Some, wrong LCM or wrong solution class, None for compatible reached prefix, accepting a zero modulus, wrong first zero index, and an overflow not selected by the left-to-right policy. Mathematical unsatisfiability of the entire list alone does not force None when an earlier merge overflows.

## constraint_preimages

Type: reverse_reconstruction. Domain: Finite slices of u64 congruences and candidate Result<Option<Congruence>,Error> observations, with normal terminating execution.

For Some(r,M), all compatible input lists whose total LCM is M and whose intersection is r+M*Z reconstruct the output, with r<M and M>0. Many lists fit: [(8,18)] and [(2,6),(8,9)]. Prefix LCM divides final M, hence no overflow occurs for these valid lists. Error and None preimages additionally obey the preflight and first terminal prefix policy; they do not uniquely recover inputs.

## solution_family

Type: algebraic_relation. Domain: Finite slices of u64 congruences and candidate Result<Option<Congruence>,Error> observations, with normal terminating execution.

For Some(r,M), integer solutions are exactly x=r+k*M for k in Z. This family uses mathematical integers; no arbitrary machine multiplication is implied.

## compatibility

Type: certificate_relation. Domain: Finite slices of u64 congruences and candidate Result<Option<Congruence>,Error> observations, with normal terminating execution.

Two positive-modulus constraints intersect iff their residue difference is divisible by gcd(m,n). For a successful list, output satisfies every input and output modulus equals LCM, jointly certifying equality of solution classes rather than mere membership.

## permutation

Type: metamorphic_relation. Domain: Finite slices of u64 congruences and candidate Result<Option<Congruence>,Error> observations, with normal terminating execution.

On positive-modulus lists whose total mathematical LCM fits u64, permutations preserve Some or None. Outside this domain, overflow can precede a contradiction in one order and follow it in another. Zero error indices also change under permutation.

## normalization

Type: quotient_relation. Domain: Finite slices of u64 congruences and candidate Result<Option<Congruence>,Error> observations, with normal terminating execution.

Replacing an input residue by any u64 value congruent modulo its positive modulus preserves output. All Some residues are canonical in [0,M). Residue addition must remain within u64 when used as a machine transformation.

## redundancy

Type: closure_relation. Domain: Finite slices of u64 congruences and candidate Result<Option<Congruence>,Error> observations, with normal terminating execution.

Within positive-modulus lists of representable total LCM, duplicate constraints and insertion of any residue modulo one preserve output. Adding the returned class to a successful list preserves it. Outside the domain error ordering must be rechecked.

## error_precedence

Type: boundary_specialization. Domain: Finite slices of u64 congruences and candidate Result<Option<Congruence>,Error> observations, with normal terminating execution.

Any zero modulus wins globally, choosing smallest index. With positive moduli each merge checks incompatibility before overflow, but an earlier overflow masks later contradictions. Empty input is Some(0,1).

## Limits

Exact algorithm; finite tests and partial syntax do not prove Rust semantics, expand macros, or resolve types. No approximate executable profile applies to these exact integer outputs. Resource exhaustion and operational timing remain unmodeled.

## evidence

### Evidence

Five tests in this module pass; the temporarily integrated crate passes 177 unit tests twice in debug and twice in release. Single-thread normalized records match within each mode. Strict all-target Clippy and owned-file rustfmt checks pass. A compiled production mutant is rejected at a real assertion. Partial syntax regenerated twice with identical bytes.

Current-checker validation: an isolated Git snapshot of the four owned source/sidecar leaves passes groupants with zero diagnostics, four sidecars, zero missing leaf sidecars; its temporary parent is explicitly one missing group sidecar. changed against the task baseline reports all four changed Rust files registered, none missing, using the external leaf registry. Full parent-tree integration remains the integrating agent's responsibility. An initial historical checker/full-repository minimal-registry probe was rejected for unrelated ownership and old component-schema gaps; it is not counted as validation.

## prospective

## History

### 2026-10-30 · descriptant

2026-10-30: Preflight the entire list for the first zero modulus and return ZeroModulus{index}. Otherwise fold from (0,1). For current (a,m) and normalized next (b,n), set g=gcd(m,n). If g does not divide b-a, return None. Otherwise L=m*(n/g); if L>u64::MAX return ModulusOverflow. Let t=(((b-a)/g) * inverse(m/g modulo n/g)) modulo n/g, using the Bezout coefficient of m for gcd(m,n). Return updated ((a+m*t) modulo L,L), eventually Some. Modulus one uses the sole zero residue. This operational error policy is part of the target.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: b15b049f54c0162ef8bf88a23e53ca99af3f2b9054e68498a74051433c494112.

External evidence: MOVA_DATA/runs/octopus-wave4-number-2026-10-30, containing the Rust validation helper, raw logs and mutants. No throughput measurement, live API, teacher, GPU or semantic proof. Root integration separately binds parent groups and registry.

- debug-records.txt SHA-256: fa902d23abe56879ca09d425ce49c7eabfedb520d9356ce92da1e8e1c22e11cb

- release-records.txt SHA-256: fa902d23abe56879ca09d425ce49c7eabfedb520d9356ce92da1e8e1c22e11cb

- chinese_remainder-mutant-build.log SHA-256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- chinese_remainder-mutant-rejected.log SHA-256: 9b10a61332944b5a85c33cb2c701442dcddc5968262f884446268b293b8bfca4

- syntax.json SHA-256: b1cb7ff45965cfb05b671720e2bac1efe7299f0cac3f2c2b0f636c05707af584

### 2026-10-30 · prospective

Before code, 2026-10-30: merge x=a mod m and x=b mod n. g=gcd(m,n), compatibility iff g divides b-a. For compatible pair t=((b-a)/g)*inverse(m/g) mod(n/g); x=a+m*t mod lcm. Normalize signed factors separately, multiply in u128. lcm=m*(n/g) checked against u64 before cast. Global zero-modulus preflight precedes sequential fold; incompatibility precedes overflow at each merge, earlier overflow can hide later incompatibility. Prefix invariant: normalized residue represents exactly common integer solutions, modulus is prefix lcm. O(k log U) word operations, O(1) auxiliary storage. Planned independent residue scans, full-width products, error ordering and safe-domain permutations.
