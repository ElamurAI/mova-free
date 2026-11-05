# fresh-process-retention-probe

## descriptant

### Fresh-process checkpoint probe

The probe builds a synthetic checkpoint through public construction/save and
explicit wire edits. It retains no learned lesson and performs no parser,
teacher, training, exam or answer call. An independently computed teaching hash
makes the public reuse query observable; literal weights/texts make transport
observable. The child uses only public restore/query/save operations.

The mathematical combiner joins fixture identity, process separation, restore
canonicalization and write-once effects through shared envelope bytes and
fingerprints. Equal child reports alone would be too weak: the test also checks
concrete lesson/successor expectations, preserved source bytes, output bytes,
independent checksum, exact rejection cause and existing-output contents.

## model

# Synthetic checkpoint process probe

## Forward

Let E=(P,h) be the envelope from the public constructor/save API, with a synthetic
lesson ledger, two literal source texts and one weight increment substituted.
H uses SHA-256 over the independently encoded declared wire field order.
For accepted E, R(E) loads into a fresh process, queries material identity and
allocation, saves canonical C(E), and reports (fingerprint, reuse, next, errors).
C sorts the text-vector keys by reconstruction into the retained ordered map.
For this fixture: reuse=7, next=8 and H(C(P)) is the reported fingerprint.
Three independent restores check C(C(E))=C(E). The input is never rewritten.
These are transport observations, not observations of trained knowledge.

## Invariant

Each process gets unique temporary paths through command-local environment values.
The child PID differs from the parent. Only the ignored exact helper is launched;
it requires input/output/report/mode and fails if called without them.
Successful observations preserve the fingerprint; duplicate output save returns
AlreadyExists and leaves output bytes intact. Hash rejection produces no output.
RAII removes the parent's fresh temporary tree. Deadline is 20 seconds per child;
scheduler stalls and pathological OS calls are outside this practical bound.

## Cost

For payload bytes B, text count T and ledger count L, fixture hashing/encoding and
save are O(B), restore text normalization O(T log T), reuse O(L times teaching
bytes). Each parent has a fixed number of process startups; process startup,
filesystem sync and dependency initialization costs are not benchmarked.

## behavior_complement

Universe U consists of finite synthetic input envelopes paired with child returns
and input/output byte effects. R is the forward relation above. An inner bound
of U minus R contains acceptance of an unresealed weight edit, replacement of
an existing output, changed source bytes, same-process receipts, a reuse result
other than 7 for matching teaching or successor other than 8. This relational
target precedes and is broader than the executed finite negative observations.

## reverse_reconstruction

Given canonical payload Q and logical report y, Pre(Q,y) contains every valid
envelope whose unique text entries reorder to Q and whose public observations
equal y. Both orders of the two text entries are compatible antecedents when
each carries its own correct checksum. PID, temporary path and presentation
metadata are not uniquely recoverable. Checksums are assumed collision resistant;
this is a compatible-preimage relation, not an inverse solver or proof of training.

## canonical_projection

For duplicate-free two-entry text vectors, C is the ordered-map projection.
C(C(P))=C(P), but P need not equal C(P). The reversed fixture has a different
wire checksum yet restores to the independently predicted canonical fingerprint.
Observed equality includes resaved bytes and weight values, not only a digest.

## process_boundary

Let parent memory M and child input file E be separate inputs. The child receives
paths/mode through its own Command environment and reconstructs solely by load.
Its report includes a PID receipt excluded from deterministic logical comparison.
Two fresh restores have equal logical reports; a third consumes the first output.
This establishes an OS process boundary for this fixture, not hermetic isolation,
absence of all inherited environment, cross-build or cross-model equivalence.

## rejected_transition_frame

For a valid envelope with one weight changed and old hash, load returns the exact
hash-mismatch error, source bytes stay equal and the output path stays absent.
For a valid restored state saved to an existing file, create_new rejects with
AlreadyExists and existing bytes stay equal. Partial writes, crashes, racing
writers and permission failures are not covered by these observations.

## trusted_checksum

Checksum consistency H(P)=h establishes the observed integrity gate, not trusted
origin. Changing a weight then resealing yields an accepted different fingerprint.
No signature, trusted teacher receipt or training lineage is inferred. Authentic
learning and transport consistency have distinct premises.

## reuse_identity

Teaching digest is H(course, stored-number=7, explanations, facts, exercises).
The candidate's placeholder number, title and assessment fields are not part
of this lookup identity. Modified explanation returns None; foreign course
returns the concrete separate-state error. Exercise labels ARE included in the
digest even though assessment labels are excluded. This probe has no exercises.

## information_loss

The canonical output loses text-vector ordering and JSON whitespace/key order;
the process report also omits paths and PID. Therefore output identity does not
identify original bytes or origin. The two accepted vector orders supply a
bounded collision of projection, not a cryptographic collision. Frame indexes,
lesson-focus reconstruction and nonempty graph semantics are unmeasured here.

## evidence

### Evidence

Scope: synthetic checkpoint construction and fresh-process transport only.
No training, parser, teacher, exam, answer-quality or cross-model experiment.

Commands:
- cargo test --offline -p world --test groupant_retention -- --nocapture
- cargo test --offline --release -p world --test groupant_retention -- --nocapture
- mova-modelcheck syntax REPO world/tests/groupant_retention.rs NEW_JSON

The owned sidecar has no structural diagnostics and declares eight typed models.
The changed-source registry check reports this one source and no missing entry.
The temporary one-node registry intentionally lacks unrelated source ownership,
so its global groupants report is not a full-registry pass; parent integration
must register this leaf and update ancestor bindings before claiming that pass.
Only focused tests are claimed; no full world suite is implied.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: prospective expectations were canonical text ordering, lesson 7,
next number 8, unchanged logical reports across processes, checksum rejection
before output creation, and acceptance after a trusted reseal. The checksum
encoder must respect serialized struct field order; ordinary JSON Value order
does not match the wire contract. Unknown fields and nonempty frames are outside
this fixture. The partial syntax artifact does not expand macros or resolve names.
Finite results do not establish answer quality, trained retention, universal
serialization correctness or cross-model compatibility.

### 2026-10-30 · evidence

2026-10-30: focused debug suite passed two parent tests, with one ignored
subprocess helper. Each suite launches five child helpers in distinct processes:
three canonical restores, one exact hash rejection and one resealed acceptance.
The helper is ignored in normal discovery and requires explicit paths/mode.
Source-bound syntax generated twice was byte-identical.

Logs are external under MOVA_DATA/runs/octopus-next-process-2026-10-30.
Two debug and two release runs each passed 2 tests with 1 ignored helper.
Sorted logical output lines were identical across all four runs. A compiled
negative control changed the independent expected successor from 8 to 9; the
complete report assertion failed with actual 8 versus expected 9. The original
source was restored and the focused debug suite passed again.

### 2026-11-05 · evidence

2026-11-05: the wire payload gained `dictionary` (always written, empty in older
states) and `licence` (written only when nonempty). The fixture encoder omitted
`dictionary`, so every reseal produced a checksum the real loader rejected
("retained state hash mismatch") and both parent tests failed. The encoder now
writes `dictionary` after `graph` and `licence` only when nonempty, matching the
struct order; retention code was unchanged. With the release model 2483516a the
two parent tests pass; the stale-checksum child still reports the exact
mismatch error (negative control), and the pre-fix encoder is the observed red.
