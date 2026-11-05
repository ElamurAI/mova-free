# prag-main

## model

# prag-main

## forward
CLI options scan into ordered positionals and a last-value-per-key map; numeric access parses usize or supplies a default only when absent. Selected commands delegate to library operations. The measured file adapters read supplied CoNLL-U, normalize lemma case, emit positional roles or world-before-sentence summaries, and optionally restrict worlds by complete document before final ID selection. Sentence splitting groups punctuation runs and nonletter tails. Saved-run reporting collects already saved outputs/calls, sums accounting and writes metadata using the library transport descriptor.

Domain: finite argument strings; representable numeric option values or expected parse errors; isolated files containing valid supplied trees, ID selections and saved accounting with representable finite sums. Observation: parsed positionals/options/numbers, errors, sentence pieces, TSV rows and JSON report. Operators: opts/num, sentences, roles_file/world_file and gate_report at the bound source revision. Approximation semantics: partial source-derived relations and finite witnesses, not a complete command implementation proof. Live acquisition/teacher/model-loading dispatch is unexecuted.

## invariant
Interleaved options and --prefixed values remain accepted; duplicates retain their last consumed value. Invalid present numbers error rather than silently default. Roles/world headers remain even with no output rows. World ID restriction retains all context sentences of selected documents before filtering final rows. Saved calls are counted/summed without executing new calls; current report provenance is Codex CLI via mova-teacher.

## cost
For B argument bytes, K options and D distinct keys, parsing is O(B+K log D) time and retained-byte memory. Sentence segmentation is O(text bytes). File adapters parse the entire input, even under --only; for N retained sentences they add O(N log N) delegated ordering, delegated frame/context work, and O(output bytes) serialization. ID/document sets add ordered-set membership costs. Accounting scans saved calls/batch outputs and aggregates with O(call bytes + batch bytes + grouping work), retaining parsed records/output. Only the chosen command branch executes; external parser/model/teacher runtime remains unmeasured. No optimization or speedup claim.

## behavior-complement
Universe: stated argument/file inputs paired with candidate outputs. First-wins duplicate option maps, reordered positional strings, and a default returned for malformed present numeric input are excluded. A consumed --prefixed value becoming a separate option is excluded by the current scanner. Headerless empty roles/world files are excluded. Filtering to selected sentence IDs before computing their prior document context, leaking other documents' prior events, or reordering world rows into internal sorted order is excluded on this valid-tree domain. Reports that replace saved call totals with zero or describe current Codex transport as Claude are excluded. Extreme sum overflow, environmental races and arbitrary malformed-tree behavior are outside this model, not silently ruled out.

## reverse-reconstruction
A successful positional/map observation admits argument lists with arbitrary overwritten earlier values and different option interleavings, provided scanning yields the same positional order and final key values. Numeric output merges accepted textual spellings and an absent key whose default equals the explicit value; it does not determine the original spelling or presence. Sentence pieces cannot reconstruct stripped whitespace or the exact whitespace before merged nonletter tails.

Roles/world output does not identify original surface lemma casing, unscored tree details, filtered-out documents or older context omitted by bounded summaries. A selected world row constrains a compatible earlier document history, not merely the selected sentence. Header-only world output admits empty selected-ID sets, IDs absent from input, or no retained sentences. Report call counts and sums admit different per-call allocations/orders; counts do not reconstruct prompts, labels, raw outputs or historical model/effort environment. The current metadata environment is not a historical per-call inverse. Intersecting these restrictions at shared IDs, selected documents and transport value preserves composition correlations; no complete reconstruction solver exists.

## composition
Options select paths/ID filters; file adapters normalize input into the public roles/world APIs and wrap outputs with stable headers. world_file joins only-IDs to doc_key before contextual reading, then projects IDs afterward. gate_report joins silver collection, saved call records and the library descriptor, with independent nontransactional file writes. These interactions are distinct from library schema-format/source-voice composition.

## selection-commutation
Specialized view: for valid supplied sentences S and requested IDs I, let D contain doc_key(i) for i in I. The claimed scoped relation is filter_I(world_docs(S)) = filter_I(world_docs(filter_D(S))). Both sides retain original row order and the complete earlier history of every selected document. Replacing filter_D with filter_I before world_docs does not satisfy this relation in general: an earlier unselected sentence can contribute the event visible in a selected row. This model uses the assumption that context is reset by doc_key and internal ordering depends only on that document's IDs. Evidence: supplied_world_filter_preserves_document_context uses interleaved input, a selected later sentence, an unrelated document, duplicate/blank selection IDs, a missing ID and empty selection. It is finite evidence, not a general theorem or approximation bound. Cost reduction applies only to delegated retained-document work; full input parsing remains.

## checks
Five binary wave17_models tests cover options/numeric edges, Unicode punctuation segmentation, literal role positions, world restriction/full-output equality and saved-call provenance. Two library composition tests provide adjacent joins. Each target passed twice debug and twice release with identical named outcomes. Old code fails the Codex provenance witness; compiled main first-wins and library descriptor mutations fail named assertions. Original source bytes restored before positives. See model.md#receipts.

## epoch-memory

Stale report transport was witnessed and repaired through the shared descriptor; old opus API names and option compatibility remain intact. Review clarified that --only saves retained-document computation but still parses the full CoNLL-U input, and counts/current environment cannot reconstruct call history. No optimization, score gain or full-dispatch verification claim.

## limitations
No live parser inference, model loading, teacher, database, corpus, training or GPU execution. Environment model/effort and role-alone selection are not varied. Resource-extreme malformed inputs, arithmetic overflow and transactional output guarantees remain unverified. Partial syntax omits macro expansion and semantic resolution.

## receipts

### Boundary receipts

Five binary and two library tests passed twice in debug and twice in release, with identical named outcomes across modes. No live teacher, parser inference, model loading, database, corpus or GPU calls. Tests use synthetic Word/CoNLL-U/label inputs and isolated temporary files.

Old production witness `gate_report_records_codex_transport_and_saved_accounting` failed because via named Claude CLI; the corrected report reads public `prag::TEACHER_TRANSPORT`. Legacy opus API names remain unchanged.

Compiled controls: `mutant-main-lastwins` changes overwrite to first-wins and fails `options_preserve_compatibility_and_numeric_domain`; `mutant-lib-transport` replaces the constant with Claude CLI and fails the gate-report witness. Both failed assertions after successful compilation, and exact original bytes were restored before all final positives.

Commands: `cargo test -p prag --bin prag wave17_models -- --test-threads=1` and `cargo test -p prag --lib wave17_models -- --test-threads=1`, each also with `--release`, repeated twice. Syntax generated twice per source and compared byte-identically. `mova-modelcheck groupants` reports valid=true, diagnostics=[] against the existing registry; source registration is root integration work.

Existing warning: unused_mut in prag/src/frames.rs:1382. Finite fixture evidence, no full suite or semantic-proof claim, no timing or accuracy gain.

## History

### 2026-10-30 · epoch-memory

2026-10-30: stale report transport was witnessed and repaired through the shared descriptor; old opus API names and option compatibility remain intact. Review clarified that --only saves retained-document computation but still parses the full CoNLL-U input, and counts/current environment cannot reconstruct call history. No optimization, score gain or full-dispatch verification claim.

### 2026-10-30 · receipts

Heading `### Boundary receipts — 2026-10-30` stays in receipts as `### Boundary receipts`.

Raw logs: `$MOVA_DATA/runs/octopus-wave17-boundary-2026-10-30`.

#### source-identities
- prag/src/main.rs: SHA-256 170a91fdaf37227ef14b8d620f451d9d46987dde5546136e8153e7e368794d0b; 2110 partial syntax nodes.
- prag/src/lib.rs: SHA-256 01009a73cf1a1bc103374decd92328f31a226a3212d307ff9f7327bc95523d65; 157 partial syntax nodes.

#### evidence-identities
- old-debug.log: 4c57dbf31120d23dcc656eb91e36bf8c32992e2758c58343501b577a97fce147
- mutant-main-lastwins.log: 5290e6e46a40b46064ac9618b5706c416f00061dfec25ce9f8fa9ed171cebce4
- mutant-lib-transport.log: 7ef0ca84984f30811f0e93e1cef795a61df0b896811339b2f93c6fb963b343ff
- bin-normalized.txt: e2403aae952a884de91bc78e37a177473b019837f4983a52cb1a183e9c41e885
- lib-normalized.txt: e4940b383033123f9a120fa2b7a2c5741bc4802faca11323db18ec8c369e7dbc
- structural.json: 7dc83a41ad7adbccff3fc114dd10bacd7f91c589772bb586ea5a628725992ae3
