# qagen-run

## model

# qagen-run

## forward
Group paragraphs, render/check a Job, inspect resume state, then schedule capped Codex calls. try_final_raw exposes retry-first lookup errors; final_raw retains lossy Option behavior.

## invariant
Batch keys ascend and paragraph order is stable. Each processed first response receives at most one format retry. Metadata errors stop need preflight.

## behavior-complement
Universe: finite supplied values and filesystem observations, return values, errors and writes in this scope. Strict lookup never substitutes original after retry metadata error. No arbitrary Job batch uniqueness validation, crash-safe accounting or cancellation of in-flight work.

## reverse-reconstruction
F^-1(y) contains all compatible inputs and initial states yielding y. A selected path determines candidate and batch name but not contents or unselected file state. Successful runs admit multiple schedules and preexisting states. No complete inverse solver or unique recovery claim.

## cost
P paragraphs, B batches: grouping O(P log B) time/O(P+B) references. Resume O(B) probes plus raw/check costs; selection O(B*W) for W selected indices. J=max(1,jobs) threads; no explicit resource ceiling.

## composition
Batching/Job and lookup stages feed todo; existing accounting initializes reservations. Rename precedes append; child correctness cannot establish durability or multi-process isolation.

## ordering
BTreeMap sorts keys; Vec retains paragraph order. Duplicate arbitrary Job batches remain caller responsibility.

## resume
need checks raw first: absent ->1 even if retry exists; existing retry ->0; otherwise format rejection ->2 or accepted ->0.

## cap-boundary
Atomic reservation checks prior count before ask. No cross-process cap or usize-wrap proof is claimed.

## persistence
Raw rename precedes accounting append; crash/append error can leave unaccounted raw.

## checks
Finite synthetic Rust tests in owned sources; broad offline qagen lib suite twice per profile, explicitly skipping lemma::tests::doubt_with_en_model. Compiled production mutations reach named assertion failures. Exact outcomes and hashes are in the file groupant model.md#evidence.

## epoch-memory

Prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.

## limitations
Partial source models and finite observations, not proof of all concurrent/filesystem states. Syntax projection is partial; macros opaque. Allocation failure, races and crashes may prevent completion. No live teacher/model/corpus/database was used. No useful certified metric for a numerical surrogate of these error-sensitive operations was identified; executable approximant is absent, with no invented error budget or speed claim.

## evidence

### Evidence

Prospective model saved before production repair. Old debug and release: nine tests, three passed and six failed; release failures reach assertions for all six defects, including sentinel qa.tsv replacement. Final debug twice and release twice: 48 passed, one explicitly filtered model-loading test. Seven compiled production mutations reached named assertion failures; original bytes restored before positive runs. Normalized repeated per-test outcome lists matched per profile. Syntax generated twice and compared byte-identically: partial syntax, not semantic AST or proof.

No live teacher/model/corpus/database operations. Scheduler source-modeled only, not transport-tested. Writes remain nontransactional and resume/accounting crash gap remains. No numerical executable approximant claimed for this error-sensitive interface.

## prospective

### Prospective bounded repairs

Domain: finite synthetic calls JSONL, local filesystem states, supplied paragraph values. No teacher, model, database or corpus execution.
Report accounting computes ordered f64 totals and checked u64 totals. Nonrepresentable totals return Err; rounding must not turn finite large totals into JSON null. Preserve historical negative finite values and ordinary rounding. Cost O(record bytes + calls), memory O(record bytes + calls); no new asymptotic costs. collect must finish accounting/retry metadata validation before artifact writes, with no cross-file transactional promise.
Run lookup chooses retry first, raw second; strict lookup propagates IO errors, compatibility final_raw remains lossy Option. need returns 1 for absent raw, 0 for existing retry or acceptable raw, 2 for format rejection. Replace existence masking with Result. Cost at most two metadata probes plus raw read/gate. Concurrency and cap algorithm remain unchanged; crash gap between raw rename and call append is explicit.
Complement: no overflow wrap/panic, no accounting-derived JSON null, no collect artifact mutation on preflight accounting error, no strict metadata error treated as absence. Reconstruction: aggregate outputs admit many call lists with same totals, ordering/record attribution unrecoverable; raw choice admits all unobserved original contents if retry exists. Validation finite witnesses do not prove full scheduler behavior or transactional durability.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

#### Source and external log hashes

External logs: $MOVA_DATA/runs/octopus-wave18-qagen-2026-10-30/.

- qagen/src/run.rs: a34e297bdb69e178791983d8b9f6b35b1df34f223f15f1f13bc65d29153f2abd
- clippy.log: bc4e76e835f062ec5509db09ba039723fcdb557afe3dfa1c6df916b1011321a4
- control-collect_accounting_failure_preserves_artifacts.log: 1fa8882a2a8727f0db02238ddb43041a99bc0747b97a56cf4ef65ff6e4b80e42
- control-floating_total_overflow_is_error.log: a7e5441ec32fb8f8c32e4d155b8ea2812ec990690b9575a3f0885519b3102962
- control-large_finite_rounding_stays_numeric.log: 7e782221559111acad618ad06b802db5b618b8b324ed7b71bb2f2da6229a410e
- control-need_metadata_error_is_not_absence.log: 296a41ae046a2647a4b2af091f6f19c366abfa489d6b8a44288046fd88922b16
- control-need_retry_metadata_error_is_not_completion.log: abae58dac4de1f491355c953c9e7fb95a33540941382e5a1f8cf25d9da109fb6
- control-retry_error_is_not_hidden_by_valid_original.log: 4f67750d81b05575842f502cbf6ef82cb3c5042df411610712026ed5d998548d
- control-token_overflow_is_error_in_every_column.log: 4a7e49f61107404129394149e4f42bb99d203271bf5b124005f4618dc4d2b2a4
- debug-1.log: 625333791ae30f991dedd6e25d0020e9f906a1350355556f0cc624d9f46158a5
- debug-2.log: f264df8a5e584be794cc465819924b9eae07a92ede243df2b80837cd629fd89a
- groupants.log: 8e76094c144766b8ebc0d112b906ee8f705579d8142a28ceae10669773698f3b
- old-debug.log: 02607881a5523247c64d1e94faeb424366f29345957fe29ec77c51f61ab405a2
- old-release.log: 9f1f6249c2a18163d9c7101bb8175610c022ec07e586683f7623d7054109a231
- prospective.md: 7dd6933bbf69bea9355096d3b0a6328bbddebd798d886968e6b8b69b9e532b83
- release-1.log: 7085e8bb0fc67c869739ec9d2e0b89fc9eca9d72c1a92e6450e94f1094a98824
- release-2.log: 12df9c5e424628381036ea530fb7040e6f61f8dfa6f921b60f5ca2eb78a16c06
- verification.log: 7072d93e35e7c8de6fb03b39e073100f4f8aecc6229b4e61001a2a6021a081b8
