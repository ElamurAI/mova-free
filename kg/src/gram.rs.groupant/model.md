# kg-gram

## descriptant

### kg-gram

Packed u32 VESUM grammar projection, implicit-subject constructor and dotted display.

## approximant

### Forward relation

First colon token sets POS(unknown13,empty0); recognized subsequent tags overwrite fields, unknown tags ignored; gender implies SG only when number0. implicit_subject copies number/gender/person into fresh noun+NAZ+PRON+IMPLICIT.

#### Executable approximant gap

Exact categorical structure, identifiers, parser decisions and effects have no useful percentage metric established here. No executable surrogate, automatic substitution or 1/2/5/10% guarantee is claimed.

## invariant

### Invariant and domain

Masked setters preserve disjoint fields. Parser-generated values fit display tables; arbitrary public Gram(u32) need not. flag/with require bit<32. Implicit projection clears verb tense/aspect/negation.

#### Cost and resources

O(tag bytes), bounded field lookup; O(1) packed state and bounded display names allocation.

## cost

### Cost and resources

O(tag bytes), bounded field lookup; O(1) packed state and bounded display names allocation.

## composition

### Composition

Packed u32 VESUM grammar projection, implicit-subject constructor and dotted display.

First colon token sets POS(unknown13,empty0); recognized subsequent tags overwrite fields, unknown tags ignored; gender implies SG only when number0. implicit_subject copies number/gender/person into fresh noun+NAZ+PRON+IMPLICIT.

The parent invokes scoped transformations through the actual functions named in their source ranges. Children partition useful observations, not a complete semantic proof; boundaries can omit helpers, tests and effects.

## models

## behavior_complement

Ignored tags cannot introduce features. Dot display is not VESUM input; NEG/IMPLICIT display names are not parsed back. Arbitrary u32 display can panic.

## reverse_reconstruction

Packed grammar admits ignored/overwritten tag sequences. implicit_subject permits any antecedent tense/aspect/flags/animacy matching copied coordinates. Display provides no unique source-tag reconstruction.

## tag_order

noun:p:f stays plural, noun:p:s becomes singular; repeated case last recognized wins.

## zero_unknown

Empty input yields POS0; alien yields other13. Unknown trailing tags leave state unchanged.

## subject_projection

Fresh noun/nominative plus copied number/gender/person and two flags; selected-coordinate projection is idempotent.

## representation_domain

POS5bits admits31 but display ends13; NUM3,TENSE7,ASPECT3 also exceed display tables. Bitwidth validity is weaker than enum validity.


## tests

### Evidence

Parent source checks (a fragment inherits only the behavior they actually exercise):

- gram::tests::vesum_tags
- gram::wave29::ordered_tags_and_implicit_projection

Selected kg tests: 15 named outcomes, twice debug/twice release, equal sorted outcomes. Standalone actual global/build.rs: 3 named outcomes per run, twice debug/twice optimized; build_child is normally no-op, parent fixture invokes it against isolated synthetic seeds. Eight compiled source mutations (one per source) each compiled successfully and exited101 on the focused assertion; exact source restoration followed by positive rerun.

This fragment's untested branches remain source-derived. No live graph, DB, corpus, teacher, GPU or held-out evaluation was used. Command specs and numeric exits are retained in the external wave29 graphlang receipt. Strict kg Clippy reports existing source lints; see receipt.

## prospective

### Prospective graph-language encoding model

Read all eight sources before edits. No optimization proposed. All executable fixtures are tiny invented strings, compressed temp files, or isolated subprocess directories; no repository corpus, live store, teacher, GPU or held-out execution.

- global/build.rs: walk sorted markdown paths; close a block on any fence, discard unclosed block; parse declarations, insert verb concepts, reject canonical duplicate codes, missing/duplicate links, unknown *_cats and duplicate principle ids; emit lexical concepts/index links, first-seen verb lemma. Auxiliary absurdity cells sort real entries but tale rows preserve input order. closed_data is a separate optional branch; failed reads become empty. Build effects are output writes, expressions before global.rs, not atomic. Model independently of kg. Tests will compile actual build.rs with rustc --test, helper tests plus a child test process driving main against synthetic seed/data directories. Bound cost O(B + C log C + LC + R log R), tables and generated text retained.
- kg/src/cols.rs: six parallel public vectors; push, sort tuple permutation, all-bit filter, binary search outgoing range requires sorted src; modulo-u64 delta/zigzag, exact byteplanes, zstd files written sequentially then count. Actual raw width is 32 bytes/edge, not header's 28. Empty mask retains all, parsed empty mask rejects. u32 sort indices truncate lengths >=2^32; malformed public column lengths can panic; LEB accepts trailing/noncanonical bytes, rejects >10 continuation groups but does not validate top-group overflow. Test endpoints, error branches and synthetic file roundtrip; no crash consistency guarantee.
- kg/src/gram.rs: u32 fields with last recognized tag wins, gender implies singular only when number zero, unknown POS=13 except empty POS=0, unknown tags ignored. implicit_subject copies number/gender/person then noun+nominative+PRON+IMPLICIT. Display can panic for arbitrary public bitpatterns; dotted display is not VESUM input. Tests order and implicit projection; O(tag bytes), bounded fields.
- kg/src/id.rs: heap/name vector, per-heap lazily loaded name arrays and hash maps, reserved local zero; append only suffixes. open creates keys directory; malformed heaps checked before key loading; duplicate serialized keys collapse map to last occurrence. New keys reject tabs/newlines after existing-key lookup. Heap max +1 can overflow; append buffering can report success before ignored drop flush errors. Test tiny persistence, kind conflict, corrupt consecutive indices, key rejection; no concurrent writer/crash test. Hash lookup expected O(1), heap scan O(H), key load/save O(bytes).
- kg/src/morph.rs: normalized form map and lemma/grammar synthesis map; two complete bz2 passes, vocabulary filters analyses then selected lemma/paradigm keys select synthesis. Normalization drops seven marks and maps four apostrophes then Unicode lowercase; split_line is literal space based. Government inherited only by verb/prep, xp form marker enters lemma key. Tiny bz2 fixture checks unseen vocabulary excluded but selected lemma's unrequested forms synthesized. O(2 decompressed bytes + retained analyses/forms).
- kg/src/uk.rs: tokenize Unicode with context joiners; punctuation first (input-order failure), empty/length/digit/conjunction/unknown then dash-with-verb gate. Proper/common filtering; enumerate finite predicate analyses, greedy attachment and case preferences; deduplicate equal Facts, accept exactly one. Scope children: token/gates, verbal/nominal construction, render completeness. Render compares normalized word multisets after synthesis and restored relation words; not roles/order. Costs bounded sentence words <=8, analysis alternatives unbounded; scans O(P*n^2*a), equality overhead for candidate facts. Test precedence, simple actual parse, agreement and normalization/check damage. No linguistic accuracy claim.
- kg/src/textgen.rs: frequency lexicographic pair conditional/global, dictionary-first tie; after-preposition third-person pronoun n-form pool; attributes depth <=4; 1..12 heads; subset DP O(n^2*2^n), O(n*2^n) memory; strict transition > retains first predecessor and final max tie chooses last. Empty heads rejects before private zero DP. Frequency u32 overflow profile dependent. Test form tie, head bound and real three-word generation, no corpus order training.
- kg/src/main.rs: pattern-matched CLI dispatch; read_tatoeba strict three fields but language ignored. tatoeba pipeline needs existing store before Registry open because names.tsv created first; parse_lines drops malformed lines/ends at first line I/O error; loads names and morphology before output truncate. sub validates mask before reading store, writes columns only without registry. stats equal-count order unstable HashMap ties. Test strict compressed reader, parse_lines synthetic integration, sub invalid mask precedence. Do not invoke tatoeba_uk or held-out scoring.

Composition hypothesis: Morph::load uses Gram then uk::parse produces Fact; render_check gives only multiset completeness; main encodes each member into registry-backed parallel columns with grammar and flags; textgen consumes Fact/Morph not encoded Edges directly. No read-edges-to-Fact reconstruction adapter exists here. global's lexical concept indices have no established Registry Id translation. Complement and preimage relations must preserve these separate tags. No meaningful percentage surrogate for categorical parse, identifiers, codec or effects is established; record executable approximant as unsupported.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Source read before tests and models; retained prospective notes accompany the parent. Current evidence uses bounded invented inputs only. This is partial modeling, not proof or corpus accuracy.

### 2026-10-31 · prospective

Heading `### Prospective graph-language encoding model — 2026-10-31` stays in prospective as `### Prospective graph-language encoding model`.
