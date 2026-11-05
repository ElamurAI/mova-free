# qagen-report

## model

# qagen-report

## forward
Gather gated pairs, compute calls/distributions, then materialize QA, rejection and parameter artifacts; sample renders a seeded subset.

## invariant
All fallible preparation finishes before collect artifact replacement. Successful accounting fits u64/f64; rounding preserves finite totals.

## behavior-complement
Universe: finite supplied values and filesystem observations, return values, errors and writes in this scope. Malformed/overflowing calls cannot partially replace collect artifacts. Filesystem writes remain nontransactional; production v2 collection can load a model.

## reverse-reconstruction
F^-1(y) contains all compatible inputs and initial states yielding y. Reports/distributions admit many raw replies, paragraphs and histories. Accepted QA cannot recover rejected text or omitted metadata. No complete inverse solver or unique recovery claim.

## cost
O(raw bytes + gate/model costs + JSONL bytes + pair/anchor/render costs); BTreeMaps add log-key factors; sample O(N log N). Memory includes raw, calls and rendered strings.

## composition
Strict lookup feeds gather; gate owns coordinate validity. Accounting preflight precedes gather/model loading. Shared Collected feeds all stats; serialized params precede three ordered writes.

## validation-order
rule -> retry metadata -> calls -> gather -> optional lemma model -> rendering/serialization -> writes.

## numeric-domain
Four checked u64 totals; every sequential f64 prefix finite, even if later cancellation could rescue an overflow.

## publication
Preparation errors preserve artifact bytes; IO failure during writes may leave partial updates.

## scope-boundary
support/doubts require valid 1-based anchor and paragraph indices; externally malformed Pair/Collected may panic.

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

- qagen/src/report.rs: 4abf85ffbd32f22802a89ef625780b288d92e92616c247a4d591e37fe10977c2
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
