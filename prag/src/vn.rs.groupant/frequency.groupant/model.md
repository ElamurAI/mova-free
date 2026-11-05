# prag-vn-frequency

## forward
Frequency rows resolve existing class IDs and parse usize counts, skipping invalid IDs/counts, sorting descending count then ascending class index. lesson_pref=2 only for unique first count, 1 for other present class, 0 absent. Recording ignores preference, counts only one raw-score winner if core match succeeds and multiple classes considered. Global start clears counts then enables; take disables then drains.

## invariant
Frequency values are parsed without arithmetic; malformed/overflow numeric tokens skip. Duplicate rows are not aggregated and zero counts remain present (legacy behavior). Recording concurrency is not a transaction and count +=1 requires representability.

## cost
Frequency sort O(E log E); preference lookup O(Ev); recording one BTreeMap update O(log N). Public global lifecycle synchronization is not benchmarked.

## behavior-complement
A unique strict maximum receiving preference 1, tied maximum receiving 2, or unknown class row entering loaded frequencies is outside the table preference relation. No exclusion claim about concurrent record_start/take interleavings. Universe: supplied inputs and candidate observations in forward's bounded domain; R is the specified transition. This is a partial source-derived exclusion relation, not a proof.

## reverse-reconstruction
Preference 2 loses the exact count and margin; every strictly-leading count pair is compatible. Recorded count loses original clauses and order; counts alone cannot reconstruct the lesson set. The target is the full compatible preimage, not a selected example.

## specialization
A leading count of 9 over 3 and 90 over 30 induce identical preferences. Equal 4 and 4 changes both to 1; count scale is erased, tie order remains index order. Recording branch is source-reviewed, not globally executed.

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
