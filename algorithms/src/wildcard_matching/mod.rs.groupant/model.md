# algorithm-wildcard-matching

## descriptant

### algorithm-wildcard-matching epoch memory

Return true iff the complete pattern consumes the complete text. A literal byte consumes the same byte, '?' consumes one arbitrary byte, and '*' consumes k>=0 arbitrary bytes. There is no escaping or Unicode decoding. The frontier stores exact prefix-language membership.

For text length n and pattern length m, O((n+1)(m+1)) time and O(m+1) storage; this is exact bounded-frontier DP rather than an unproved linear greedy claim. Valid byte slice lengths fit isize so m+1 fits usize; allocation failure may panic/abort. No resource cap or approximate mode.

The prospective design retained comparison-sort construction costs and declined an unsupported linear total-time claim. Canonical BWT reachability is stricter than merely applying LF; byte wildcards deliberately have no escape grammar. Models distinguish information retained in full outputs from information discarded by projections. Test runs and compiled controls are recorded in model.md#evidence. No earlier modeling epoch is claimed. Partial Rust syntax is source-bound, with macro bodies opaque; it is neither a semantic AST nor proof. Future work may improve bounds only after prospective derivation and measurements.

## model

# algorithm-wildcard-matching

## Forward

Return true iff the complete pattern consumes the complete text. A literal byte consumes the same byte, '?' consumes one arbitrary byte, and '*' consumes k>=0 arbitrary bytes. There is no escaping or Unicode decoding. The frontier stores exact prefix-language membership.

## Invariant

Before updating cell j+1, diagonal is the old row[j], above is old row[j+1], and row[j] is current-prefix membership. A star accepts left OR above; a nonstar accepts diagonal AND byte predicate. Initial row accepts exactly all-star pattern prefixes.

## Cost

For text length n and pattern length m, O((n+1)(m+1)) time and O(m+1) storage; this is exact bounded-frontier DP rather than an unproved linear greedy claim. Valid byte slice lengths fit isize so m+1 fits usize; allocation failure may panic/abort. No resource cap or approximate mode.

## nonbehavior

Within U of finite (text,pattern,bool) observations, exclude true when no whole-string token consumption exists and false when one exists. Anchoring forbids an interior literal match by itself. Bytes outside '?' and '*' remain literal, including backslash; Unicode scalar or path-separator semantics are absent.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## preimages

For fixed pattern P and result true, the fiber is exactly its byte language L(P); for false it is the complement within finite byte strings. Many distinct texts match '*'. A Boolean and pattern cannot recover a unique original text or its allocation history.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## frontier

At text prefix length i, row[j] equals membership of T[..i] in P[..j]. Fixing a row retains all text prefixes with the same vector of prefix memberships; in-place diagonal/above temporaries preserve the previous row observations until used.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## star-collapse

Replacing any nonempty run of consecutive stars by one star preserves the recognized full-string language, since all nonnegative compositions of consumed lengths have the same possible total. This law does not replace or remove question marks.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## length-language

If P has q nonstar tokens, every accepted text has length at least q; with no stars length must equal q. A pattern of q question marks accepts exactly length q. All stars accept every finite text, including empty.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## literal-identity

For a pattern containing neither '?' nor '*', L(P) is the singleton {P}. On this restricted pattern domain, a successful match fixes the input bytes; adding a wildcard generally restores ambiguity.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## byte-boundary

Under byte semantics a two-byte UTF-8 character needs two question marks; a scalar-aware counterfactual would require one. Backslash is literal even immediately before '*', so backslash-star accepts a literal backslash followed by any bytes, not a lone literal star.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## star-recurrence

For star at pattern position j, accepting (i,j+1) is equivalent to accepting (i,j) with zero new bytes OR (i-1,j+1) with one more byte, for i>0. Replacing OR by AND loses witnesses such as nonempty text matched by one star.

Domain: Finite byte slices and ordinary terminating Rust calls with successful allocation. Observation: one public call or the explicitly named internal state. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## evidence

### Evidence

Control: replace star OR by AND; full_bytes_no_escapes_and_empty rejects the lost *b* witness.

Syntax is generated twice and byte-compared. Strict algorithm Clippy and narrow rustfmt run separately. Structural validators check declarations, not semantic truth. Exhaustion, Unicode scalar matching, downstream application behavior and universal correctness remain outside the evidence.

Structural validation uses an external scaffold preserving all base registry owners and appending these six sources. A first minimal scaffold exposed existing owner checks; the corrected scaffold passes with no diagnostics and reports all six changed sources registered. The temporary missing scaffold-root sidecar and 473 preexisting missing sidecars are explicit backlog, not failures in these leaves.

## History

### 2026-10-30 · descriptant

2026-10-30: original implementation and fixtures; exact byte semantics and independent bounded oracles. Source SHA-256: 9400255d9b64fd788511b5e92ef26552b2d48d4a2e027aa941c3182f14efd08e.

### 2026-10-30 · evidence

2026-10-30. Source SHA-256: 9400255d9b64fd788511b5e92ef26552b2d48d4a2e027aa941c3182f14efd08e. Five module tests pass twice in debug and twice in release with identical normalized test identities/results. Raw logs: $MOVA_DATA/runs/octopus-wave10-text-2026-10-30/.

- wildcard_matching-debug-1.log: SHA-256 ddba4c7d5c7148e800af0bb8704b61b6b950680a2eda9ae34ce5c42ea031bd74

- wildcard_matching-debug-2.log: SHA-256 52927abb4c41a9fce418ec62ac5ef3e7c07dc4b50b2fde703f6f49707f578af6

- wildcard_matching-release-1.log: SHA-256 dc1b06b4b2917d9e8afbcf1ff1cf30e4f97d81e1a5e408ce60720baa13b04c69

- wildcard_matching-release-2.log: SHA-256 d64ba49b1f531d63e32877c53d2dca179bb5878411d4759e9db825d9fc290a9c

Compiled mutation log wildcard_matching-mod-rs-mutant.log: SHA-256 023d5729cd3a2447384220ba2ea95c35111354ac698181fca86cfb28650e2d0f. Named assertion failure is required, not compilation failure; original source bytes restored and focused test passes.

Strict Clippy log SHA-256: ee26a23cab9e52eddcd50fa854be2183d0a2329a03f7c8cb922475aaf27ec62f. Source syntax was generated twice identically; syntax SHA-256: ced66ace70199460c2d4711bcbd801d5a2e4ccc0846d1e257e81dc074aa72685.
