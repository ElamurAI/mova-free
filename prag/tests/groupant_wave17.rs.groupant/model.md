# prag-groupant-wave17

## model

# Wave17 native joins

## forward

Decode supplied events, project attempted-call accounting, append/read one isolated ledger, and gate independent numbered rows against the same Item batch.

## invariant

One-based row identity and source policy; attempted count differs from accepted labels; decode errors short-circuit before append.

## cost

Event and row bytes, one JSON record append/read and ordered accepted-source mapping; no process costs measured.

## composition

Each named test is a separate tagged relation. Composition retains the same ordered Item or Para batch, one-based row/anchor coordinates, closed labels/rule version, attempted cardinality, validated reply and isolated ledger. Independent branches do not share a live acquisition pipeline. Distinct tests do not constitute one end-to-end pipeline. Independent finite or literal oracles remain distinct from cross-implementation equality.

## epoch-memory

# Prag supplied client adapter joins — prospective model

Forward model: synthetic JSONL events E -> decode_events(E) -> interpret_reply(step,batch,attempted,secs,value) -> append_call(path,call) -> silver::gate(text,items). read_calls observes the persisted accounting. A supplied client helper places the fallible decoder before append using ?. This is a concrete adapter composition of production functions, not an execution of ask/run. Literal TSV and usage values are independent fixtures. Out-of-order TSV row numbers refer to batch positions, not source ord/para or answer arrival order.

First obernant, behavior_complement: decoder errors prevent this adapter from reaching append or gate; a valid suffix cannot recover an earlier rejected event. Duplicate TSV row numbers exclude all accepted labels while successful transport accounting remains. Missing TSV row 2 excludes only its label; attempted count is not accepted count. Source-dependent voice checks distinguish Tale narrator from unquoted Tatoeba none. The adapter excludes live processes, corpus/data acquisition, environment changes, retries, concurrency and crash recovery. Append success is not a durability or transactional guarantee.

Second obernant, reverse_reconstruction: accepted (row,labels) plus the ordered Item batch yields source identity via row-1; row alone admits every compatible source batch. Call.sentences gives attempted cardinality, not accepted rows, labels or source content. Full/partial/duplicate answers may have identical attempted count and token accounting. A decode failure with unchanged ledger admits many rejected event histories; unchanged bytes cannot identify the failure. Repeated/case-normalized label spellings and omitted optional events give further nonunique preimages. No unique inverse of event decoding or label parsing is claimed.

Prospective tests:

1. turn_rows_accounting_and_labels_keep_batch_coordinates: two out-of-order rows, Tale/Narrate at row 1 and Tatoeba/Assert at row 2; usage input 13, output 5, cache-read 7, cache-write 0, zero reported cost; batch 9, attempted 2, seconds 0.25. Read-back literal JSON observation and source-key/Act pairs must agree with batch order despite reversed arrival order.
2. attempted_counts_do_not_turn_partial_labels_into_acceptance: three supplied Items, only rows 1 and 3 -> accepted keys [1,3], missing row 2, ledger one call with sentences 3. A second isolated ledger for duplicate row 1 has one attempted call with sentences 3 but zero accepted labels. This joins accounting to gate cardinality without treating transport success as label acceptance.
3. failed_turn_stops_before_accounting_and_successful_suffix: create prior valid ledger, snapshot bytes, then feed success+failure, failure+success, repeated successful trace via success+success (rejected at the first postcompletion item), and error-prefix+success. Each helper returns Err and exact prior bytes remain; read-back remains one call. The valid suffix itself is the same valid trace used for the initial success. This is supplied adapter sequencing evidence only.

Invariants: local row numbers are 1-based; source mapping is row-1 into the supplied batch; attempted cardinality equals supplied Item count regardless of gate outcome; successful interpretation retains literal usage and caller metadata; decode errors short-circuit before append. The scratch owner uses temp_dir, PID and AtomicU64 plus exclusive create_dir, and RAII cleanup; no environment mutation. Exclusive creation fails on an existing path instead of sharing it. Cleanup is best-effort and does not establish crash behavior.

Cost: finite fixtures; general decoder and ledger read scale with event/ledger bytes, serialization with one record, silver row grouping O(r log r) plus label parsing, and source reconstruction O(accepted rows). One append per successful decode in this adapter; rejected decodes perform no ledger I/O. No speed/error bound or whole-pipeline proof.

Prospective compiled adapter mutation: in accepted_sources change items[k - 1] to items[items.len() - k]. It remains valid Rust and in range for valid rows, but test turn_rows_accounting_and_labels_keep_batch_coordinates must fail assertion W17-PJ1-SOURCE-LABEL-JOIN, observing reversed source keys paired with unchanged Narrate/Assert labels. Restore exact harness bytes before positive reruns. Root should record successful mutant compilation and named assertion failure, not count a compiler error as evidence.

Pending evidence: exact final source/harness hashes, API owner confirmation, root debug twice/release twice named outcomes, normalized comparisons, source restoration and mutation receipt. No execution outcomes are asserted here.

## checks

3 named tests pass twice in debug and twice in release. The compiled adapter fails its named assertion, with exact restoration. Partial syntax repeats identically; model.md#evidence retains native and structural receipts. Macro bodies remain opaque; semantic AST and whole-system proof are absent.

## catalog

# Wave17 supplied-input relations

Source-derived specifications; bounded native evidence and repeated partial syntax are recorded in model.md#evidence. No general proof or complete inverse engine is claimed.

## behavior-complement

Type: behavior_complement. Named check: all three prag joins.

Universe: supplied event traces, caller metadata, isolated ledger and numbered label rows over an ordered Item batch. Exclude accounting after decode failure, exchanged source/row labels, accepted labels for missing or duplicated rows, and treating attempted sentence count as accepted labels. A valid suffix cannot recover an earlier rejected trace. No process/crash/concurrency or teacher guarantee is modeled.

## reverse-reconstruction

Type: reverse_reconstruction. Named check: all three prag joins.

Retain all traces, metadata, rows and Item lists sharing the observed reply, accounting and labels. Row k plus the same batch recovers Item k-1; ledger cardinality alone cannot recover accepted rows or text. Full, partial and duplicate answers can share usage and attempted counts. Unchanged ledger bytes admit multiple failure histories; event decoding and label parsing lose spellings and discarded events.

## accounted-label-coordinates

Type: attempted_label_coordinate_transport. Named check: turn_rows_accounting_and_labels_keep_batch_coordinates.

Decoded usage (13 input,5 output,7 cache-read,0 cache-write) and caller (batch9,attempted2,0.25secs) survive interpret/append/read. Reversed TSV arrival order retains row1 Tale/Narrate and row2 Tatoeba/Assert through the same Item references. Exclude arrival-order indexing and reversed source mapping. Compatible source content remains unconstrained beyond the retained keys, source policy and supplied rows.

## partial-label-accounting

Type: attempted_acceptance_separation. Named check: attempted_counts_do_not_turn_partial_labels_into_acceptance.

Three attempted Items yield accepted row1 and row3, missing row2, and one accounting record with sentences3. A separate duplicate-row reply rejects all labels but still records one successfully decoded attempted call. Ledger data does not establish label coverage; accepted labels cannot reconstruct absent text. Costs are event/row parsing, one append and ledger read.

## decode-before-ledger

Type: failure_effect_exclusion. Named check: failed_turn_stops_before_accounting_and_successful_suffix.

The supplied adapter evaluates decode_events with ? before interpret/append/gate. A prior valid ledger is byte-identical after failed suffix, failed prefix with valid suffix, repeated successful trace, or error prefix with valid suffix. Failure histories have nonunique unchanged-ledger preimages. This sequential adapter property is not atomicity, process execution, durability or concurrency evidence.

## evidence

### Wave17 bounded evidence

212 selected Mova checks pass twice in debug and twice in release, with identical normalized named outcomes: 155 prior focused checks, 33 new source-unit checks, 17 additional existing synthetic QA checks and seven new joins (three pragmatic, four question-generation). Root qagen library runs explicitly skip lemma::tests::doubt_with_en_model. The 100-algorithm library and its prior evidence are unchanged and are not remeasured in this epoch.

Seven existing sources gain models, with 21 nested fragments and 81 typed views; the two harnesses add 11 views, the new question-generation group adds six, and existing parents add five. Final inventory: 329/762 Rust sources, 433 pending (350 public,83 closed), 659 nodes,16 groups,314 fragments,3676 views,1109 type labels,319 partial syntax projections. Filename coverage in prag is complete; semantic coverage is scoped. Six qagen sources remain in the backlog.

Witnessed repairs: exact u128 Hamilton quotient/remainders remove the >2^53 weight-order error and usize sum overflow; checked prompt adjacency prevents i64 wrap/panic; QA anchors reject empty list components and paragraph labels permit at most one P/p prefix; accounting refuses nonfinite fields before opening and propagates non-NotFound I/O errors; run reports identify Codex transport. No broad option-compatibility change or transport rename was made.

Owner evidence records eight old-code failure witnesses and 14 compiled production mutants, followed by exact restoration and positive runs. Two root compiled adapter controls additionally fail named assertions: reversed Item/label identity and a wrong sentence-source mapping. Compiler errors are not counted as mutation rejection. Stale group bindings and wrong child identities are structurally rejected and restored exactly.

Independent source review found no further production counterexample within declared domains and refined costs: quota initialization/cap scanning remains at zero rounds; string comparisons/cloning and general JSON-object lookup work are explicit. The small quota oracle covers 45,056 tuples; this finite evidence is not a Lean proof or complete input verification. Native code and syntactic projections remain separate from models.

A gate worker also invoked a broader 26-test legacy run containing an optional local-model test; captured evidence does not establish whether that model branch ran or returned early. It is documented separately and excluded from the root 212-check receipt. Root checks use supplied fixtures and isolated temporary files, with no live teacher, parser model, corpus/database acquisition, GPU, whole-school or adoption workflow. No speed gain or new approximation-error budget is claimed.

#### Supplied-path dependency snapshots

These inspected production files participate in the tested paths; listed hashes are execution snapshots, not complete semantic dependency closure.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective design retained below; no speed or accuracy gain is claimed.

### 2026-10-30 · document

Date: 2026-10-30. Read transport worktree teacher/src/lib.rs and prag/src/opus.rs, and base prag/src/silver.rs and prag/tests/groupant_wave16.rs before construction. Root owns installation, groupant/source/syntax binding, compilation and execution. Exactly three integration tests; no repository edits or cargo execution by this worker.

Construction receipt: external harness written; rustfmt --edition 2024 completed successfully. No compilation/test execution. Harness SHA-256 6daabf7b11474b3266be105a16669f19ad9fc5a00b72853aae4e35ac8a9ef94e. Inspected source SHA-256: transport opus.rs 0169f72bddab55f9e493b85a86918b5f0adbcf5153c7c12725bb249f2ed5c136; transport teacher/src/lib.rs 718e03bf89dcf8e3c69f3af6fa137d88c70d9797b04f21f1752d998e2a5b6b90; base silver.rs adf915c8d5e9059d3f58b20eca8d9b94721ee7fd9f39ac69f93775a6b5c856c9. Source signatures were inspected directly; root must rebind final integrated hashes.

### 2026-10-30 · evidence

Heading `### Wave17 bounded evidence, 2026-10-30` stays in evidence as `### Wave17 bounded evidence`.

3 supplied integration joins. Source SHA-256 6daabf7b11474b3266be105a16669f19ad9fc5a00b72853aae4e35ac8a9ef94e; twice-identical partial syntax SHA-256 a9eff47f7ff405c244bc8455114b38b866753cdfa406bf184fb73a0f622092bd.

Raw receipts: $MOVA_DATA/runs/octopus-wave17-integration-2026-10-30. native-evidence-hashes.json SHA-256 2aee840e096dcaf3d558e55c95b86f6a6623506aa620351c1e9d009ba29e94e5; mova-normalized.txt e7d315a6d937998d03e2cdc2555ba4fd96137ef61ef873104329ca632b697274; structural-controls/summary.json a50d4387189f34044579c0d068475201acf1f6e7ee8e3297c0db6909f3180975. Focused formatting passes. Cargo Clippy completes with warnings; this is not a warning-free strict-lint claim. Macro bodies remain opaque in partial syntax. External child dependencies are not inferred to have complete models from these joins.

- teacher/src/lib.rs: 718e03bf89dcf8e3c69f3af6fa137d88c70d9797b04f21f1752d998e2a5b6b90

- prag/src/data.rs: b3146b6af3ca37c5a64f82c60970bea0420e5f7f62cf38642c305e09b8ce4015

- prag/src/schema.rs: 7dade219cf710dd08df7a60853a6101058b90af22a564ac35c9b3c5d4b23eff8

- prag/src/silver.rs: adf915c8d5e9059d3f58b20eca8d9b94721ee7fd9f39ac69f93775a6b5c856c9

- prag/src/opus.rs: 0169f72bddab55f9e493b85a86918b5f0adbcf5153c7c12725bb249f2ed5c136

- qagen/src/gate.rs: 09328f711abf4b6453f95bed3d873deb205c8801d939841974416fe74e3e5806

- qagen/src/schema.rs: 03e113198711324f029d8e9e4d9fc899416e3144ea856240b9d962775c78c2fe

- qagen/src/prompt.rs: 2e30ac525e2bf8cf8432298ff35eaf2ec08933c17aff60e480017a2b7cca89f6

- qagen/src/select.rs: 0c35caf4eda91c0a5d36bce728f3846b360b483a66fa4ca3c884964839cd2554
