# kg-id-registry

## descriptant

### kg-id-registry

Lazy heap/key registry and suffix-save state.

## approximant

### Forward relation

Open the heap table, choose the checked next same-kind heap, reject tab/LF/CR keys before lazy loading or lookup, reuse/add admitted IDs, and append unsaved suffixes. find/key retain the unchanged legacy line decoder; invalid-key id now takes precedence over cached legacy entries and lazy file errors.

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

Key admission precedes lazy I/O and lookup. Admitted bytes pass unchanged from names/map into the appended LF row and through lines/split_once at reopen. The parent supplies the well-formed heap metadata and I/O premises for this composition. find/key still expose the legacy decoder, so admission cannot repair ambiguous old rows or reconstruct original histories. Saved suffix counters and their failure limitations remain unchanged.

## models

## behavior_complement

Over id-call transitions, keys containing tab, LF or CR cannot obtain an ID or mutate registry state, even when a matching legacy key is already cached. Key rejection also excludes lazy loading. Kind-name conflict, nonconsecutive loaded index and checked exhaustion reject under their existing paths; there is no transaction or global file-uniqueness validation. These are source-derived inner exclusions, checked on finite fixtures.

## reverse_reconstruction

Decoded saved files determine names only within their parser interpretation, not original raw key histories or the loaded-cache set. Empty key results admit sentinel/out-of-range IDs as well as a stored empty key. Legacy terminal-CR loss adds distinct compatible antecedents described below.

## heap_max

MAX global at an existing name remains retrievable; a new name errors unchanged.

## key_zero

names begins with empty string at 0; the first newly assigned key index is 1. A stored empty key is distinct from the reserved local index, though key() alone does not disambiguate them.

## failure_order

id validates tab/LF/CR before keys_of and before existing-key lookup. This deliberately changes error precedence: invalid-key errors precede malformed-key-file errors, and cached legacy separator keys cannot bypass id admission. find and key remain inspection paths with the old decoder. open still eagerly validates heaps but not keys. Native rejected_keys_precede_lazy_io_and_legacy_lookup exercises both branches without mutating the legacy file.

## save_suffix

Saved counters advance per stage; ignored buffered-drop errors weaken a durable-save claim. Successful actual I/O, well-formed unique heap metadata with admitted names and a single writer are required for the parent's admitted-key roundtrip. heap rejects unframeable names (LF, terminal CR); see heap_label_framing.

## legacy_decode_fiber

Type: legacy_decode_fiber (specialized reverse reconstruction). Subject: the unchanged lines/split_once decoder, observing only the resulting (local ID, decoded key). Let k contain no tab/LF/CR. In the historical admission domain, LF-encoded keys k and k+CR decode to the same k, so this observation has at least the compatible original-key fiber {k, k+CR}. Under the fixed old LF encoder and single-row separator-free-key assumptions, these are exactly its two compatible keys. Raw LF and CRLF rows can differ; discarding raw bytes loses that distinction. A raw CRLF row itself does not identify whether it came from an old trailing-CR key plus LF or a CR-free key in a CRLF-formatted import without writer provenance.

Consequently id's new rejection restricts future admission but supplies no automatic reconstruction of ambiguous persisted trailing-CR keys. It neither rewrites files nor trims/normalizes inputs. find/key continue to return the existing decoded key; source-derived reconstruction must retain both histories unless external provenance narrows them. Other malformed files and duplicate-key collisions introduce further ambiguities outside this simple fiber.

Approximation status: exact decoder fiber under the fixed encoder assumptions above, partial history relation when writer provenance is absent. Oracle: legacy_trailing_carriage_return_remains_decode_ambiguous writes the literal row 1 TAB alpha CR LF, observes alpha at the original local ID and no alpha+CR, rejects a new id request for alpha+CR, and verifies unchanged file bytes after save. The pre-fix actual-source witness separately demonstrated alpha+CR admission followed by this loss. These finite checks do not repair or survey historical stores.

## heap_label_framing

Type: framing_admission (specialized behavior complement). Subject: heap(kind, name) admission against the unchanged heaps.tsv codec. Encoder E(h,k,n) = decimal(h) TAB kind TAB n LF; decoder = BufRead::lines (removes LF and one preceding CR), then splitn(3, TAB), so the name is the whole third field including any tabs. Domain: all UTF-8 names. Observation: the decoded heap triples after save and open.

For a single appended row, decode(E(h,k,n)) = [(h,k,n)] exactly iff n contains no LF and does not end with CR. Outside that set: a terminal CR is dropped (n = m+CR decodes as m; n = CR decodes as the empty name); an LF splits the row, so the tail either fails open for the whole registry ("malformed heaps.tsv line") or, when the tail parses as a triple, injects an extra heap. heap() therefore rejects exactly this lossy class, before any lookup. TAB, interior or leading CR, NUL, U+2028/2029, padding and Unicode stay admitted and exact. Heap names and keys deliberately differ by admission policy: id() also rejects TAB and every CR. Both split_once(TAB) for keys and splitn(3, TAB) for heap labels preserve tabs in their final field; that key restriction is not a codec-loss claim.

Status: exact codec relation for a single well-formed appended row; partial for files edited outside the API. Baseline witnesses (actual source at 02f3d0931, external baseline-probe.log): 11 names. 7 round-tripped exactly; trail+CR and CR reopened as trail and the empty name and then received a second heap number; lf+LF+inside made reopen fail; x+LF+"134217728 TAB global TAB forged" reopened with a forged heap sharing number 134217728 with words. Native: lossy_heap_names_are_rejected_without_state_or_file_change, unusual_exact_heap_names_reopen_with_stable_numbers. A mutant that also rejected TAB and every CR failed the exact-names test.

## heap_admission_order

Type: first_error_state_preservation. Subject: heap(kind, name) error precedence and effect. Order: (1) name admission; (2) existing name: a kind mismatch errors, otherwise the existing number is returned; (3) next number in the kind's range, where checked_add overflow or a range overflow errors as exhausted; (4) push. An error at any step leaves heaps, saved_heaps and the heap file unchanged; heap performs no I/O. Consequence: an inadmissible name reports the admission error even when its kind is exhausted or another kind owns a similar name. Admitted names keep their previous messages and results.

Status: source-derived for all inputs, with finite native evidence: heap_name_admission_precedes_kind_conflict_and_exhaustion (exhausted Global at u32::MAX plus an existing Local name), plus the lossy test's unchanged-state assertions. This model is distinct from failure_order, which concerns id/keys_of and lazy key files.

## heap_label_roundtrip

Type: persistence_roundtrip. Subject: heap -> save -> open -> heap_by_name/heap_name/heap. Domain: names admitted by heap(); an existing heaps.tsv with unique names and numbers and well-formed rows; one writer; successful actual I/O. Observation: (name, kind, number). For admitted n, the single-row codec relation gives back exactly (h,kind,n), so after reopen heap_by_name(n) = h, heap_name(h) = n and heap(kind, n) returns h without allocating; repeated save leaves the file bytes unchanged and the next new name continues from max+1. Reverse: the observed triple fixes this name-number pair but not the call history or the order of in-session lookups.

Status: exact under the domain; finite evidence: unusual_exact_heap_names_reopen_with_stable_numbers (11 names: tabs, interior/leading CR, empty, Unicode, combining mark, padding, U+2028, NUL). Not covered: duplicate names or numbers already present in an imported file (open does not reject them; heap_by_name returns the first match), flush or crash faults, concurrent writers.

## legacy_heap_decode_fiber

Type: legacy_decode_fiber (specialized reverse reconstruction). Subject: the unchanged heap-row decoder on historical files, observing decoded triples only. A CRLF row decoding to name m has compatible antecedents {m written by a CRLF-formatted import, m+CR written by the old LF writer}; without writer provenance they cannot be separated. An old LF-containing name produced either a file that no longer opens or additional decoded triples, so an injected triple is indistinguishable from a genuine one without provenance. The admission fix prevents new members of these fibers; it does not rewrite, repair or reinterpret existing files. A CRLF file still opens and decodes as before, and heap(kind, m+CR) is now rejected instead of allocating a second number for the decoded m.

Status: exact decoder fiber under the old-writer assumption; partial without provenance. Native: legacy_heap_files_decode_unchanged_and_ambiguous_forms_stay_rejected (CRLF rows including a tab name, unchanged bytes after save, an LF-split row still a hard open error). No real store was inspected.

## tests

### Evidence

Parent source checks (a fragment inherits only the behavior they actually exercise):

- id::tests::id_parts
- id::wave29::global_heap_exhaustion_preserves_state
- id::wave29::registry_persistence_and_corruption

Selected kg tests: 15 named outcomes, twice debug/twice release, equal sorted outcomes. Standalone actual global/build.rs: 3 named outcomes per run, twice debug/twice optimized; build_child is normally no-op, parent fixture invokes it against isolated synthetic seeds. Eight compiled source mutations (one per source) each compiled successfully and exited101 on the focused assertion; exact source restoration followed by positive rerun.

This fragment's untested branches remain source-derived. No live graph, DB, corpus, teacher, GPU or held-out evaluation was used. Command specs and numeric exits are retained in the external wave29 graphlang receipt. Strict kg Clippy reports existing source lints; see receipt.

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

### 2026-11-01 · document

#### Imported duplicate rows and cached decoder state — 2026-11-01

For a successfully parsed key file with consecutive indices 1..N fitting u32, let its decoded keys be k_i. `keys_of` retains the vector (empty sentinel, k_1,...,k_N), then collects its map in row order: M(k)=max{i: k_i=k}. Hence `key(Id(h,i))=k_i` but `find(h,k)=Id(h,M(k))`. The inverse fiber of a decoded key contains every index bearing it, while find chooses only the last; the behavior complement does not exclude duplicate decoded keys. On first access a successful decode is cached; subsequent external file edits are not observed by that Registry instance. A failed decode inserts no cache entry, so a later access can retry. Save appends from the cached saved count; it is not a synchronization or duplicate-repair operation.

For parseable heap rows `(h_i,kind_i,name_i)`, `heap_by_name(n)` and `heap(kind,n)` choose the first row with name n, while `heap_name(h)` chooses the first with number h. Neither duplicate names nor duplicate numbers are excluded by open. Therefore the two lookups need not be inverse. Example rows `(7,service,first),(7,service,second),(8,service,first)` give heap_by_name(second)=7 but heap_name(7)=first. Rows `1:alpha,2:beta,3:alpha` yield find(alpha)=3 while key(1)=key(3)=alpha; beta is the distinct-row control. A synthetic actual-source probe exercises these observations and save/reopen; these are bounded fixtures, not a claim of transactionality or full import validation.
