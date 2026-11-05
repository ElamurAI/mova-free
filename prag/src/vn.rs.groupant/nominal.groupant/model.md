# prag-vn-nominal

## forward
Nominal TSV loading keeps first row for each noun, requires four columns and sets transitive iff column four equals t. nom performs global exact lookup. speech_verb holds iff member class list is nonempty and every class communication suffix is 37.; speech_verbs lists qualifying keys in BTreeMap order.

## invariant
First noun duplicate wins even if later verb/transitivity differ. Event file label is transported, not checked against an enum. Missing word returns None/false. Global wrappers are not invoked by supplied tests.

## cost
Nominal load O(N log N + B); lookup O(log N); speech enumeration O(M log M + total member associations).

## behavior-complement
A later duplicate replacing the first noun, transitive true for i, or vacuous speech true for absent member is outside the described relation. WordNet semantic quality is not inferred. Universe: supplied inputs and candidate observations in forward's bounded domain; R is the specified transition. This is a partial source-derived exclusion relation, not a proof.

## reverse-reconstruction
Loaded noun entry cannot recover discarded duplicate rows or comments; transitive=false includes every fourth-column string except t. Speech=true forgets which and how many communication classes supplied the evidence. The target is the full compatible preimage, not a selected example.

## specialization
Fix the first arrival->arrive event i row; any later arrival rows leave the observation unchanged. The fixture retains first verb and false transitivity despite a conflicting later t row.

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
