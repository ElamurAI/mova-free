# prag-vn-index

## forward
from_tsv scans rows. Each class row appends class index i, resolves parent only among earlier IDs, appends i to each member list, and replaces ID lookup on duplicate IDs. Frame rows attach to the current ID lookup; orphan/short rows are ignored. classes_of is exact string lookup.

## invariant
Table member order is retained; no lowercase normalization of member keys. Duplicate IDs are permitted and by_id points to latest row. Parents must precede children for resolved inheritance; not a general TSV validator.

## cost
O(B + (K+member-occurrences+frames) log K); memory O(B) retained strings and indexes.

## behavior-complement
A frame attached to a later unseen class or members sorted away from file order is excluded from the specified row-fold relation. This does not forbid duplicate IDs or malformed ignored rows. Universe: supplied inputs and candidate observations in forward's bounded domain; R is the specified transition. This is a partial source-derived exclusion relation, not a proof.

## reverse-reconstruction
Observed classes and indexes omit comments and ignored rows; many tables have identical loaded state. Identical final by_id does not identify earlier duplicate-ID rows; full classes vector supplies that missing coordinate information. The target is the full compatible preimage, not a selected example.

## specialization
Permuting parent and child rows can turn resolved parent into None; permuting equal candidate classes changes tie selection. Order is an explicit input, not an irrelevant formatting property.

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
