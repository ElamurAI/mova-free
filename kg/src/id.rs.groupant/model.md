# kg-id

## descriptant

### kg-id

Heap/local u64 IDs and append-oriented lazy registry with checked heap exhaustion.

## approximant

### Forward relation

Id(h,l)=(u64(h)<<32)|l. open creates keys/ and reads heap triples. heap first rejects names containing LF or ending in CR, then resolves an existing name/kind or checked(max same-kind+1), checking range before push. id first rejects any key containing tab, LF or CR, before lazy I/O, cached lookup or mutation. For an admitted key, keys_of loads consecutive indices 1..; id reuses its ID or appends the next name/map entry. save appends unsaved suffixes. find/key retain the existing line decoder and can inspect legacy keys that id now rejects.

#### Executable approximant gap

Exact categorical identifiers and persistence effects have no useful percentage metric established here. No executable surrogate, automatic substitution or 1/2/5/10% guarantee is claimed.

## invariant

### Invariant and domain

Let D be UTF-8 keys without tab, LF or CR. A rejected id input leaves cache membership, maps, name vectors, saved counters and heap state unchanged and does no file I/O. For consecutive unique existing key files whose keys lie in D, well-formed unique heap metadata whose names are admitted by heap(), one writer, representable indices and successful write/read completion, save/reopen preserves each admitted key and its ID. local 0 stays reserved. Exhaustion at MAX returns Err before heap push.

heap() rejects names containing LF or ending in CR before lookup, kind check or exhaustion, without changing heap state; tab, interior CR and other UTF-8 names round-trip exactly because the name is the third splitn field. Heap files are still not checked for duplicate names or numbers, and legacy CRLF heap rows decode without their CR. Legacy trailing-CR key loss is not repaired: the unchanged decoder strips a terminal CR, and original histories cannot be reconstructed uniquely without provenance. Filesystem failures, ignored buffered-drop flush errors, crashes and concurrent writers remain outside the roundtrip claim.

#### Cost and resources

Heap scan O(H); admission scans O(key bytes) before any key-file load, and hashing also reads key bytes (expected O(1) table probes). Key load is O(K bytes); save is O(new bytes)+loaded heap iteration. Rejected separator keys allocate an error but retain no registry state. O(H+loaded keys) retained; no lock or atomic durability. This repair has no measured speed claim.

## cost

### Cost and resources

Heap scan O(H); admission scans O(key bytes) before any key-file load, and hashing also reads key bytes (expected O(1) table probes). Key load is O(K bytes); save is O(new bytes)+loaded heap iteration. Rejected separator keys allocate an error but retain no registry state. O(H+loaded keys) retained; no lock or atomic durability. This repair has no measured speed claim.

## composition

### Composition

Id(h,l)=(u64(h)<<32)|l. open creates keys/ and reads heap triples. heap first rejects names containing LF or ending in CR, then resolves an existing name/kind or checked(max same-kind+1), checking range before push. id first rejects any key containing tab, LF or CR, before lazy I/O, cached lookup or mutation. For an admitted key, keys_of loads consecutive indices 1..; id reuses its ID or appends the next name/map entry. save appends unsaved suffixes. find/key retain the existing line decoder and can inspect legacy keys that id now rejects.

The registry child connects key admission, lazy loading and append counters. Stable IDs require the child admission domain and the successful persistence assumptions in model.md#invariant together; neither a correct packing function nor isolated lookup proves that connection. Duplicate heap rows, partial-write recovery and cross-writer serialization remain outside this model.

## models

## behavior_complement

Universe: id-call transitions on an existing Registry and arbitrary UTF-8 input keys, including invalid keys. Source-derived inner exclusion: an input containing tab, LF or CR cannot return an ID, load a key cache, change a counter/map/name/heap, or perform file I/O through id. Validation precedes keys_of and cached lookup. For admitted keys, malformed nonconsecutive files still fail before cache insertion. Heap exhaustion excludes wrap-to-zero and heap push. Duplicate serialized names/keys are not fully rejected; save can partially persist. The finite rejection tests are evidence for this relation, not a universal runtime proof.

## reverse_reconstruction

Id uniquely splits to (heap,local); key reconstruction needs registry state. key missing/out-of-range/zero returns empty string ambiguously. A decoded saved table admits different cache sets, saved counters and histories. Legacy trailing-CR observations can additionally lose original key identity; the registry child's legacy_decode_fiber retains that ambiguity. No unique historical repair follows.

## exhaustion_repair

Old debug witness at u32::MAX panicked addition overflow; checked_add now returns exhausted before push, regression requires absent new name and unchanged heap count.

## lazy_failure

open validates heaps first. find/key load one key file on demand; id does so only after admission. An invalid key therefore returns its key error even if the corresponding file is malformed, with no cache insertion. An admitted key still reports the lazy file error. First serialized index 2 fails because names length starts 1.

## append_epoch

saved counters delimit suffixes; no-change save appends no keys. BufWriter drop flush errors are ignored; a returned Ok is not an fsync or complete-write guarantee. The roundtrip model assumes actual write completion and successful subsequent reading.

## identity_domain

Service 1..2^26-1, Local 2^26..2^27-1, Global 2^27..MAX. keys_of/id can use undeclared heap numbers. heap() now rejects names whose bytes heaps.tsv cannot frame (LF anywhere, terminal CR); tab and interior CR stay admitted and exact. Unique, well-formed heap files remain a premise for imported data; see the registry child heap_label_* models.

## admitted_key_roundtrip

Type: persistence_roundtrip. Subject: current source-bound id -> save -> open/find/key composition. Domain D is all UTF-8 keys containing none of tab, LF or CR, with consecutive unique existing key rows also in D, well-formed unique heap metadata with heap()-admitted names, representable local indices, one writer and successful actual I/O completion. Observation fixes the heap and allocated local ID and compares exact key bytes after reopen; no Unicode normalization or whitespace trimming is allowed.

For k in D, write E(i,k)=decimal(i) + TAB + k + LF. BufRead::lines removes LF and an optional preceding CR; because k has neither CR nor LF, split_once(TAB) returns exactly (decimal(i),k). Thus the scoped composition preserves (h,k,i), and distinct byte strings in D remain distinct even when canonically equivalent Unicode. In reverse, observing the retained (h,i,k) fixes that key/ID pair within D but leaves all compatible cache sets, saved counters and allocation histories in the preimage; it is not a unique registry-state inverse. If k is outside D, id returns Err before this composition and leaves registry state unchanged. The invariant also requires valid preexisting rows and heap metadata; it does not certify arbitrary imported files, flush failure, concurrency or imported heap rows with duplicate names or numbers.

Approximation status: exact source-derived relation under the stated domain, with finite native correspondence evidence rather than a formal Rust proof. Oracle: admitted_unicode_keys_reopen_with_exact_ids checks nine keys including Unicode, decomposed/composed forms, whitespace, an empty string, Unicode line separators and NUL; rejected_key_separators_preserve_registry_state checks admission and allocation continuity. The baseline-red trailing-CR witness refutes extending this relation to the old admission domain. Historical bytes are not migrated.

## tests

### Evidence

Parent source checks (a fragment inherits only the behavior they actually exercise):

- id::tests::id_parts
- id::wave29::global_heap_exhaustion_preserves_state
- id::wave29::registry_persistence_and_corruption

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

#### Key roundtrip repair — wave37, 2026-10-31

The actual baseline source admitted alpha+CR, wrote an LF-terminated row and lost the terminal CR through BufRead::lines on reopen. A temporary-directory Rust unit test failed the stable-key assertion in both debug and release (exit 101). This refutes the earlier unqualified stable-ID claim over the entire old admitted-key domain.

The repair rejects tab, LF and CR before lazy loading or cached lookup. This intentionally changes invalid-key error precedence and prevents cache/counter/map mutation on rejection. The successful roundtrip claim now explicitly requires admitted keys, valid unique consecutive files, well-formed delimiter-free heap metadata, one writer and successful actual I/O. heap-name delimiter admission remains an uncovered runtime boundary; this bounded repair does not change it.

Four new native tests cover cold/warm rejection, legacy lookup and lazy-error precedence, exact Unicode/empty/whitespace key persistence, and unchanged ambiguous legacy CRLF decoding. Seven id tests pass twice per debug/release profile with equal normalized named outcomes. Source-derived admission/roundtrip and legacy decoder-fiber models explain why rejection prevents future loss but cannot automatically reconstruct ambiguous old keys. No trimming, normalization, file migration, historical repair or speed gain is claimed. Earlier wave29 evidence remains tied to its original source.

Evidence: $MOVA_DATA/runs/octopus-wave37-key-roundtrip-2026-10-31/; receipt.json binds the baseline witness and current native logs. An initial --lib command failed because kg is a binary crate; it is retained as setup-no-library.log and is not counted as a negative control. Tests use isolated temporary files only.

#### Heap-name admission — wave38, 2026-10-31

The key fix left heap labels unchecked. Baseline probes confirmed which names the heaps.tsv codec loses: LF anywhere (registry fails to open or a forged heap is injected) and a terminal CR (name merges with its CR-free form and receives a second number). Tabs and interior CR are exact because the name is the third splitn field, so the minimal policy rejects only LF and terminal CR, before lookup, kind check or exhaustion. Keys and heap labels therefore have intentionally different admission sets. Legacy CRLF heap files decode unchanged; nothing is rewritten. Four typed child models (framing, first-error order, round trip, legacy fiber) record the relations with their witnesses. Evidence: $MOVA_DATA/runs/octopus-wave38-opus-2026-10-31/.

### 2026-10-31 · tests

#### Wave37 key persistence evidence — 2026-10-31

Actual baseline witness: id::wave37::trailing_carriage_return_roundtrip_witness, retained in external id-baseline-witness.rs, failed in debug and release after successful compilation: reopen returned None for alpha+CR instead of the originally allocated ID. baseline-red-debug.log and baseline-red-release.log each record exit 101. This is an implementation counterexample, not a compile-error control.

Current checks add:

- id::wave37::rejected_key_separators_preserve_registry_state
- id::wave37::rejected_keys_precede_lazy_io_and_legacy_lookup
- id::wave37::admitted_unicode_keys_reopen_with_exact_ids
- id::wave37::legacy_trailing_carriage_return_remains_decode_ambiguous

Run cargo test -p kg --bin kg --offline --locked -j 2 id:: -- --nocapture, adding --release for the release profile. All seven id tests pass twice per profile; sorted named outcomes are identical across four runs. Fourteen rejection observations cover seven separator variants in cold and warm states. The persistence fixture covers nine admitted byte strings, unchanged repeated-save bytes and next-ID continuity. Legacy fixtures use literal synthetic rows; no real historical store is inspected or modified.

The child's evidence covers Registry behavior; id_parts alone is a parent packing check. Native passing fixtures are finite evidence, not proof of all inputs, heap-name admission, flush/crash recovery or concurrent safety. Two existing kg warnings remain; no strict whole-crate Clippy claim. Current receipt.json hashes the external witness and logs while retaining the historical wave29 receipt.

#### Wave38 heap-name evidence — 2026-10-31

Actual baseline probe (external baseline-probe.log, temporary test removed before the fix) at source 9df511fe: 11 heap names saved and reopened. Seven round-tripped exactly (plain, tab, interior CR, leading CR, empty, Unicode, NUL). trail+CR and CR reopened as trail and the empty name; a repeated heap call then allocated a second number. An LF name made every later open fail; an LF followed by a triple injected a forged heap that duplicated the number of words.

Current checks add:

- id::wave38::lossy_heap_names_are_rejected_without_state_or_file_change
- id::wave38::heap_name_admission_precedes_kind_conflict_and_exhaustion
- id::wave38::unusual_exact_heap_names_reopen_with_stable_numbers
- id::wave38::legacy_heap_files_decode_unchanged_and_ambiguous_forms_stay_rejected

Baseline-red: the first, second and fourth tests failed on the baseline source (exit 101): a lossy name was accepted, the exhaustion error came first, and words+CR got a new number. The third passed on the baseline too, which pins the admitted exact domain. After the fix, all eleven id tests pass once in debug and once in release (exit 0, identical sorted identities); they were not repeated because nothing changed after that success. A mutant that rejected TAB and every CR failed only the exact-names test (exit 101), so the test pins the minimal policy. Fixtures are isolated temporary directories; no real graph store was opened.

### 2026-10-31 · prospective

Heading `### Prospective graph-language encoding model — 2026-10-31` stays in prospective as `### Prospective graph-language encoding model`.

#### Wave37 admission prediction and bounded repair — 2026-10-31

Before editing source, the external model.md#prospective predicted that alpha+CR would be accepted, serialized with LF, then decoded as alpha. It predicted rejection of tab/LF/CR before lazy loading without state mutation and exact admitted Unicode byte/ID preservation. The baseline debug/release witness confirms the failure, and the focused native fixtures support the repair under the explicit successful-I/O domain.

The historical note above describing validation after lookup is superseded for id: admission now precedes both lazy file errors and legacy cached-key reuse. find/key preserve their decoder. Valid delimiter-free heap metadata is a required premise because heap still does not validate its name; no source expansion into that remaining boundary was made. Cost remains a linear scan/hash in key bytes with unchanged lazy load/save behavior; no runtime speed forecast was made.

### 2026-11-01 · descriptant

#### Paired diagnostic memory — 2026-11-01

The persistence vidtvorant predicts admitted key/name identity after save/reopen. Reverse decoder fibers exposed terminal-CR histories that collapse to the same decoded string; admission and state-preservation obernants then identified where rejection must precede mutation. The heap parent required a different minimal domain because TAB and interior CR survive its third-field codec. Preserve that distinction: copying the key policy to heap labels would reject exact inputs unnecessarily. Old ambiguous files remain ambiguous, and successful I/O, well-formed metadata and one writer remain premises. The wave37/wave38 receipts above are historical evidence, not newly rerun checks. See the [paired-review account](../../../docs/mathematical-modeling.md).

### 2026-11-01 · document

#### Imported duplicate identity fibers — 2026-11-01

The packed identifier splits uniquely into `(heap, local)`; this numerical bijection does not make imported metadata bijective. With duplicate admitted decoded key rows at i<j, both full identifiers return the same key and lookup selects j, so `id(h,key(Id(h,i)))` need not equal Id(h,i). Duplicate heap numbers similarly make `heap_name(heap_by_name(n))` fail to recover n. The nested registry model records exact first/last selection, the lazy cache boundary and the successful-I/O assumptions. No imported file is repaired, and behavior-complement exclusions remain distinct from the converse fiber.
