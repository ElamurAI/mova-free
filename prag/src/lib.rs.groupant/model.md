# prag-lib

## model

# prag-lib

## forward
The library exposes public module paths and the static teacher transport descriptor. The measured compositions are Labels -> labels_tsv -> parse_labels -> silver::gate, and supplied Word vectors -> roles/roles_docs and world_docs. The binary consumes TEACHER_TRANSPORT in its saved-run report. These observations concern public wiring, not execution of every exported module.

Domain: finite Labels with enum-valid values and means trimmed, nonempty, at most MEANS_MAX_WORDS whitespace-separated words, containing no TAB/CR/LF; numbered serialized rows paired with supplied Items; finite valid supplied dependency trees paired with sentence IDs. Tests use default means '-' and vary each field independently across its class set. Observation: reconstructed labels, accepted numbered label map/rejections, positional roles, pre-sentence world strings in input order, and report transport string. Operator: public APIs composed at this source revision, with dependency implementations as compiled. Approximation semantics: partial source-derived relations supported by finite fixtures, no numerical error cap or full semantic proof.

## invariant
Canonical TSV preserves every label field on the restricted means domain. The source/voice constraint is additional to schema validity: unquoted Tatoeba accepts voice=none and rejects narrator/character; tales reject none. Repeated or out-of-range row numbers reject the whole batch. roles_docs and world_docs preserve supplied ID order while reading each document in sorted document-key order. A new document has no earlier events from another document. TEACHER_TRANSPORT remains the descriptor consumed by main's report; it does not perform dispatch.

## cost
Module declarations and the descriptor introduce no input-dependent runtime work. For L serialized label bytes, schema serialization/parsing costs O(L), including means word counting. For R numbered rows and n Items, gating costs O(L + R log R + n log R) plus voice/quote checks and O(L+R+n) retained data. For N supplied sentences and W word bytes, roles/world retain input/output and sort O(N log N) keys, then incur delegated frame/context processing; no constant-time or linear bound for that delegated processing is claimed. Composing roles and world separately performs two reads; their costs add rather than sharing context. Descriptor observation is O(1) with fixed-size output.

## behavior-complement
Universe: supplied label/item or tree/ID inputs paired with candidate observable results in the restricted domain. A canonical valid TSV losing or changing a label field on parse is excluded. A schema-valid narrator/character row accepted for unquoted Tatoeba is excluded, as is a nonempty accepted map after duplicate or out-of-range numbered rows. Output world context carrying events from a different doc_key is excluded; output ID order sorted instead of restored to input order is excluded. A current report claiming Claude instead of the library Codex descriptor is excluded. These exclusions describe source behavior; finite checks cover named representative cases, not all trees, source kinds or malformed inputs. No exclusion is inferred for arbitrary means strings outside the round-trip domain.

## reverse-reconstruction
Given a canonical seven-field TSV in the restricted domain, parse_labels recovers its unique Labels value. Starting from a parsed Labels observation instead loses padding accepted by parser trimming; multiple whitespace-padded rows remain compatible. Given an accepted gate entry (k, labels), its numbered source row must parse to those labels and satisfy the associated Item's voice rule; the item's key, title, unrelated text and document metadata generally cannot be recovered from the accepted map. With voice=none, many unquoted Tatoeba texts are compatible; a tale source is not. Empty acceptance alone cannot distinguish missing rows, invalid labels, invalid voice or batch-wide duplicate alignment rejection; retained rejection messages reduce but do not remove that ambiguity.

Given role triples or world strings, compatible supplied trees must reproduce role positions or contextual summaries with the same shared doc_key/order relationships; they need not share all surface forms, unused modifiers or tree details. World summaries omit future sentences, and bounded recent events/salience omit older state, so these cannot be uniquely reconstructed. The constant transport observation admits every prompt, model, effort and credential state using that descriptor. Combining observations intersects these concrete constraints; no complete preimage solver is implemented.

## composition
A Labels round-trip does not imply gate acceptance: gate joins the parsed value with its numbered Item and source/quote condition. roles and world join on supplied ID and doc_key, preserving input ordering despite internal sorting; marginal output equality cannot establish cross-document correctness. The descriptor/report join requires the actual consumer to reference the exported constant, as main does. Options and command dispatch are main's domain, absent from this library slice.

## source-conditioned-fiber
Specialized view: source-conditioned label compatibility. Domain is one valid canonical label row and an Item plus optional other numbered batch rows. Observation is acceptance of that label at its original number. Operator first restricts the row-parser fiber by source/quote admissibility, then by batch-wide unique/in-range numbering. Thus adding a duplicate row can erase a previously accepted label without changing its own seven fields; changing only source from unquoted Tatoeba to Tale reverses admissibility of voice=none. This is a nonlocal gate interaction, not a label-format inverse. Evidence: schema_rows_compose_with_silver_closed_domains covers all single-field classes for unquoted Tatoeba and a duplicate-row witness; tale/quoted-source clauses are source inspection, not new measured coverage. Cost is the same gate pass, not an additional algorithm. No approximation or injectivity claim.

## checks
Two library wave17_models tests cover canonical schema serialization/gating and supplied words through public roles/world APIs. Five binary boundary tests additionally verify the descriptor consumer and file adapters. Each target passed twice debug and twice release with identical named outcomes. Compiled library descriptor substitution failed the named binary gate-report assertion, then exact bytes were restored. See model.md#receipts for source and evidence hashes.

## epoch-memory

Stale Claude provenance was observed in the binary report; the shared library descriptor now identifies Codex CLI without changing opus API names. Review separated format round-trip from source-conditioned acceptance and public module availability from implemented behavioral composition. No task-score or speed claim.

## limitations
Finite fixtures do not cover every exported API, cross-field Cartesian combinations, arbitrary malformed trees or all source/quote cases. No live teacher, parser inference, model loading, corpus, training or GPU work. Partial syntax omits macro bodies and semantic name resolution. Dependency behavior is compiled evidence at the recorded revision, not bound solely by the lib.rs source hash.

## receipts

### Boundary receipts

Five binary and two library tests passed twice in debug and twice in release, with identical named outcomes across modes. No live teacher, parser inference, model loading, database, corpus or GPU calls. Tests use synthetic Word/CoNLL-U/label inputs and isolated temporary files.

Old production witness `gate_report_records_codex_transport_and_saved_accounting` failed because via named Claude CLI; the corrected report reads public `prag::TEACHER_TRANSPORT`. Legacy opus API names remain unchanged.

Compiled controls: `mutant-main-lastwins` changes overwrite to first-wins and fails `options_preserve_compatibility_and_numeric_domain`; `mutant-lib-transport` replaces the constant with Claude CLI and fails the gate-report witness. Both failed assertions after successful compilation, and exact original bytes were restored before all final positives.

Commands: `cargo test -p prag --bin prag wave17_models -- --test-threads=1` and `cargo test -p prag --lib wave17_models -- --test-threads=1`, each also with `--release`, repeated twice. Syntax generated twice per source and compared byte-identically. `mova-modelcheck groupants` reports valid=true, diagnostics=[] against the existing registry; source registration is root integration work.

Existing warning: unused_mut in prag/src/frames.rs:1382. Finite fixture evidence, no full suite or semantic-proof claim, no timing or accuracy gain.

## History

### 2026-10-30 · epoch-memory

2026-10-30: stale Claude provenance was observed in the binary report; the shared library descriptor now identifies Codex CLI without changing opus API names. Review separated format round-trip from source-conditioned acceptance and public module availability from implemented behavioral composition. No task-score or speed claim.

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
