# prag-vn

## forward
Let T=(verbnet,nominals,frequencies) be supplied UTF-8 tables and C a finite clause. Load T in order, index classes, expand ancestor frames, score candidates, select stable class optionally, choose first maximum, then require every core slot assigned. Result is None or (class,frame-source,frame-index,score,verb,role vectors). Globals additionally read stable policy and recording flag; recording invokes the unchanged unique-winner update via a callback.

## invariant
Output role vectors preserve clause argument positions and length; inherited frame coordinates remain distinct from member class. Bounds and arithmetic representability below are domain premises.

## cost
O(B log K + E log E) load approximation; match O(V K H F (S + (A+P) H R + P Q)) with repeated ancestor scans, plus BTreeSet chain O(H log H). B bytes, K classes, E frequency rows, H depth, F frames per ancestor, S syntax tokens, A objects, P clause PPs, Q frame PPs, R roles. No speed claim.

## behavior-complement
For fixed T,C, candidate output y with wrong object vector length, wrong inherited frame coordinate, or non-first equal maximum is outside R(T,C). This relation is a source-derived partial inner exclusion, not merely a rejected test list. Universe: supplied inputs and candidate observations in forward's bounded domain; R is the specified transition. This is a partial source-derived exclusion relation, not a proof.

## reverse-reconstruction
B(y)={(T,C):F(T,C)=y} on the supplied domain. Filler values, unchosen classes and irrelevant nominal rows may vary; None includes missing verbs, expletive frames and core mismatch. Preserve shared class indices when composing child preimages. The target is the full compatible preimage, not a selected example.

## specialization
Hold selected frame identity fixed; role vectors constrain syntax positions but lose lexical fillers, unused PPs and restriction scores. A supplied Match must be interpreted with its originating Lex; class_id/vframe remain global-only.

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

### 2026-11-04 · limitations

Clause-taking verbs, 2026-11-04 (student-verb). `clausal(verb)`: the verb's preferred class (the first of `vnfreq.tsv`, else every class it has) has, with its parents, a frame with S_INF, that, whether, whether/if, S-Quote or a bare S not after a wh-word. Used by the student to exempt content-taking question verbs from the verb gate. Complement: a verb with one clausal class among several (found: base-97.1) and a wh-clause only (reject what S) are not clausal. Reverse: many class layouts give the same answer. Checked through world verb_tests (found, sell, destroy, build not clausal; want exempt as a carrier).
