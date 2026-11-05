# prag-vn-matching

## forward
score rejects expletives. Bare object count mismatch costs 4 per difference; singleton against two frame objects takes last role. Passive subject enters object coordinate MAX; by fills frame subject. Fit gives +1 fit,-2 clash,0 unknown and doubles contribution. Matched objects/PPs/complement +4; unmatched PP -1, unused frame PP -3, missing comp -3, unexpected comp -4; adjective pair +2, unexpected -2, missing -3. Stable class uses unknown animacy then lesson preference. Strict > updates preserve first ties; core filter is applied after choosing the top candidate.

## invariant
Every role assignment retains original clause coordinate. PP matching consumes first unused compatible frame PP. First recognized verb returns immediately even if core filter fails; lower-scoring core-valid candidates are not retried. Integer i32 score arithmetic requires bounded sizes, conservatively <=10^6 total argument/syntax slots across each tested candidate.

## cost
Per-candidate O(S+(A+P) H R+P Q+H log H); repeated stable and actual scoring approximately doubles candidate work. Output O(A+P) plus copied roles.

## behavior-complement
For fixed bounded Lex,C, a result using second compatible PP before first, an animacy-sensitive stable class, or later equal-score candidate is outside R. The complement does not assert linguistic truth or guarantee a match whenever some valid frame exists. Universe: supplied inputs and candidate observations in forward's bounded domain; R is the specified transition. This is a partial source-derived exclusion relation, not a proof.

## reverse-reconstruction
Match loses unchosen frames and fillers. With unknown versus unrestricted filler, identical roles/scores can result; stable class additionally erases animacy during class selection. Reverse relation retains all C and Lex states yielding the observed full tuple, not only same role names. The target is the full compatible preimage, not a selected example.

## specialization
Hold clause signature and Lex fixed; change filler animacy. Stable class remains fixed while within-class frame and score may vary; unstable selection may switch classes. Tied two-class fixture checks exactly this distinction.

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
