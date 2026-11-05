# prag-vn-codec

## forward
Element tokens map V/ADJ/ADV to tags, quoted tokens to lowercase lexical words, brace alternatives to sorted unique preposition sets (including named spatial expansion), and other tokens to (role,restriction). syntax splits whitespace. shape takes first V, first preceding NP, expletive flag, sequential objects/complements and adjacent prep-NP pairs.

## invariant
Preposition expansion is sorted/deduplicated. Restriction handling is literal token matching, not a logical formula evaluator; unknown restriction names expand to empty. Malformed absent V uses index zero, so general malformed grammar correctness is unclaimed.

## cost
O(B + sum p log p) per syntax with p expanded prepositions; shape O(S+expanded bytes) allocations.

## behavior-complement
Candidate Prep with duplicate alternatives or omitted literal to for {to}[+dest] is outside the token observation relation. Unknown types and permissive unmatched braces remain accepted according to explicit parser rules. Universe: supplied inputs and candidate observations in forward's bounded domain; R is the specified transition. This is a partial source-derived exclusion relation, not a proof.

## reverse-reconstruction
A Prep({from,of,off,out}) admits {+src}, {+src_conf} and literal alternatives with permutations/duplicates; case/ordering normalization is noninjective. Shape forgets semantics, examples and ignored lexical tokens. The target is the full compatible preimage, not a selected example.

## specialization
For fixed expanded set P, all token strings whose expansion sorts and deduplicates to P remain compatible; no original spelling is recoverable.

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
