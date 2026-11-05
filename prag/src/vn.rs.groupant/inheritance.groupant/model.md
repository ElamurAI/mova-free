# prag-vn-inheritance

## forward
chain(c) follows valid parent indices and emits each at most once, stopping before invalid or repeated indices. restr(c,r) returns the first ancestor role named r or empty. communication recognizes a first numeric suffix beginning 37..

## invariant
chain is a finite simple path of existing class indices; length <= classes.len(). Invalid starting index returns empty. Nearest role shadowing is preserved. communication still requires a valid index; arbitrary mutation of public classes invalidating private indexes is outside best's domain.

## cost
chain O(H log H) time and O(H) visited/path storage; restr O(H log H + H R). Before repair acyclic chain was O(H); bounded malformed safety is the purpose, not acceleration.

## behavior-complement
Any repeated or out-of-range index in chain output is outside the valid-prefix relation. Returning a farther role despite a nearer declaration is excluded, including an explicitly empty nearer restriction. Universe: supplied inputs and candidate observations in forward's bounded domain; R is the specified transition. This is a partial source-derived exclusion relation, not a proof.

## reverse-reconstruction
Observed chain prefix ending at k can arise from None, invalid parent or an edge back into the prefix. restr empty can mean no matching role or an explicitly empty nearest role. These antecedents remain distinct compatible cases. The target is the full compatible preimage, not a selected example.

## specialization
Fix a public classes vector; chain is maximal before the first absent or repeated coordinate. Tests include invalid parent/start and two-node cycle; no infinite trace is executed.

## composition
TSV codec feeds index identities, ancestry feeds nearest restrictions and frame coordinates, frequency feeds tie preference, and matching retains these same indices. Nominal lookup is a separate projection of the loaded tables. Shared identities and order must survive any joined preimage; summing overlapping fragment costs double-counts work.

## checks
Six vn::wave15_models tests cover supplied tables, normalization, inheritance, exact score/role coordinates, ties/animacy, core/complement and malformed ancestry. See evidence.json for raw-log hashes. Syntax generated twice with identical bytes.

## epoch-memory

Prospective model: preserve valid TSV/file ordering and scores; add only pure production seams. Public mutable ancestry permits invalid indices; old malformed_parent_stops_at_valid_prefix panicked at index 999 (five other tests passed). Repair stops at valid nonrepeated prefix. BTreeSet adds O(H log H) bounded visited tracking; no optimization claim. Compiled preference mutant changes unique bonus 2 to 3 and must fail exact score assertion; restored bytes precede twice-debug/twice-release checks.

## limitations
Finite fixtures are not proof or linguistic accuracy. Syntax is partial, macros opaque. No parser, teacher, corpus, GPU, global flag mutation or global recording lifecycle execution. Class vector mutation can invalidate private indexes; only ancestry prefix is hardened. communication and global Match accessors retain index preconditions. Extremely large score/count overflow and concurrent record_start/take races remain outside the bounded domain. No error budget, automatic substitution or measured performance claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective model: preserve valid TSV/file ordering and scores; add only pure production seams. Public mutable ancestry permits invalid indices; old malformed_parent_stops_at_valid_prefix panicked at index 999 (five other tests passed). Repair stops at valid nonrepeated prefix. BTreeSet adds O(H log H) bounded visited tracking; no optimization claim. Compiled preference mutant changes unique bonus 2 to 3 and must fail exact score assertion; restored bytes precede twice-debug/twice-release checks.
