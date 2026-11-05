# Source-bound batch packing

## domain

Subject: this source file's `normalize`, `compose`, `pack`, explicit header adapter and implementation bindings. Inputs are a page of at most256 single-target native Tasks plus caller-supplied document-local ordinals/order scopes, declared source pins, exclusion sets, Profile and packing policy. Output observations are prepared native Tasks/requests, per-packet rejection reasons, packing boundaries and per-target provenance. The public native annotation library remains an imported dependency, not duplicate source ownership. No consultant or trainer runs here.

## forward

For each input p, N(p) verifies original typed-context, AST, raw pragmatic and witness hashes, rejects declared Evaluation/excluded targets, filters explicitly excluded context and records a header-only adapter. C(P) unions compatible context observations by exact ID/ordinal/value, retains every target plus at most eight most recent eligible non-target context lines at/before the last target, records other IDs removed, composes target-only AST/TSV observations and calls the actual native evaluate/render gates. Every admitted candidate must satisfy <=8targets, <=16context and the actual combined schema-plus-stdin100000-byte cap; the native per-artifact/tree/character/label gates also apply. A proposed combination that fails flushes the previously valid batch; the newly validated singleton remains available. No bytes are truncated to fit.

## invariant

For supported single-target input packets, prepared_targets + rejection-record count = input-packet count. Every prepared target appears in exactly one batch within the page; every rejected packet retains its target IDs/reason. Target set, source characters, canonical pragmatic labels and Unknown dataset/context observations are checked or preserved by imported native gates. Every output is review_only and training_authorized=false. A preparation rejection has model_invoked=false. Context exclusions/removals are explicit; input hashes and original candidate/witness bindings remain in provenance. Page-local uniqueness is not global no-replay ownership.

## behavior_complement

Type: behavior_complement. Domain: supported bounded pages and declared observations. Observation: whether a target is submitted in a prepared Task. The complement excludes a submitted target absent from its source context, source-ID/text alias exclusions, declared Evaluation use, raw binding mismatch, native malformed source/tree/label observations, incompatible source/producer/order scopes and a combination beyond native capacity. Rejected singletons cannot disappear from both prepared output and rejection records. Counterexample to treating all syntax disagreement as exclusion: a structurally valid disagreeing pair remains a review request with native findings. Evidence: eight focused synthetic native packing controls; not an exhaustive semantic characterization.

## reverse_reconstruction

Type: reverse_reconstruction. Domain: returned Task, ordered per-target input hashes, source pins and adapter/mosaic provenance. Compatible antecedents retain the declared source/document versions, target order and individual producer/witness revisions. They are not unique: different raw header forms can admit the same adapted table, and bounded context projection removes text that is not recoverable from its ID/hash alone. Reconstructing omitted context requires the separately retained pinned source packets; hashes are not a text decoder. Counterexample: two distinct excluded-context histories can yield the same submitted context. Evidence: deterministic packing, original-revision preservation and explicit context-removal tests; no global inverse theorem.

## capacity-frontier

Type: capacity_frontier. Domain: an already valid current batch and a valid next singleton. Observation: attempted native preparation of their combination. Formula: if C(current+next) fails, output C(current) and keep next, rather than removing bytes or targets until a proposal succeeds. This includes native100000-byte failures, context/source-pin capacity and compatibility boundaries. Counterexample: two individually valid30k-comment source blocks per producer fit separately but their composed request exceeds100000; the focused test retains both in separate batches and checks the actual rendered budget. Capacity success does not establish a latency or quality gain.

## artifact-mosaic

Type: provenance_fiber. Domain: explicitly enabled cached observation mosaics. The producer identity identifies a mosaic and revision=H(ordered target ID, ordinal, original producer/revision, raw AST/pragmatic hashes, adapted pragmatic hash). Witness revision similarly binds ordered original witness declarations. These lists preserve original observation provenance while allowing different round artifacts to be compared in one request. They cannot reconstruct one training checkpoint or establish statistical independence. Counterexample: identical producer labels from two different cached revisions are not one frozen model; strict mode splits them while explicit mosaic mode retains both original revisions. Evidence: mosaic/strict boundary test. Hash collision resistance and truthful external source-file admission are assumptions.

## exclusion-projection

Type: observation_exclusion. Domain: declared source-ID exclusions and SHA256 of whitespace-normalized exact texts. A target matching either set is rejected. Matching context is removed and recorded; it is never submitted as harmless surrounding text. There is no case, punctuation or word normalization. Counterexample: a target can be excluded through an exact-text alias despite a different source ID. Evidence: ID, text and context controls. Completeness of the caller's held-out/alias list remains outside this source.

## context-order-scope

Type: order_compatibility. Domain: opaque caller order scopes and exact context ordinal mappings. No numeric/test/exam suffix is parsed from IDs. Different scopes or non-increasing target order split a proposal; contradictory context-ID/ordinal/observation joins cannot be silently voted together. Bounded projection keeps targets and records removed context IDs. Counterexample: two kinds may both declare ordinal1; they cannot be merged by guessing chronology from names. Evidence: different-document/scope boundaries and bounded-context accounting. Correct supplied order is an external assumption.

## header-adapter

Type: representation_projection. Domain: LF-framed eight-column pragmatic tables with exact teacher/conf headers. The adapter changes only the final header label to opaque teacher; row bytes are unchanged and raw/adapted hashes are retained. Composition emits one identical header and original row fields. This does not convert confidence values into trusted teacher identities or independent evidence. Other headers and CR framing are excluded. Counterexample: changing any six semantic labels is outside this adapter even if the result would look more plausible. Evidence: exact row-preservation and rejected-header controls.

## authority-boundary

Type: effect_boundary. Domain: local preparation and declared pins. ModelInvoked=false, training_authorized=false; no DB/model/runtime effects occur. evaluate/render success is structural/source consistency, not gold, language understanding, source-phase proof, licence permission or a checked on-disk SourcePin. Counterexample: Unknown input can produce a useful request while native not_teaching blocks GPU-admission eligibility. Global duplicate work can still occur across separate pages without the caller's durable ledger. Evidence: Unknown/Evaluation controls and source inspection.

## cost

At most two compose attempts per supported input packet: a singleton plus a tentative combined proposal; at most eight packets participate in one composition. Cost scales with bounded native parse/render bytes, target rows, maps, pin sets and copied provenance. A page cap does not bound arbitrary opaque caller provenance memory by itself. No runtime acceleration or whole-corpus completion bound is inferred. Real multi-target model latency and complete-batch acceptance must be measured separately; goodput=workers*targets_per_batch*batch_acceptance/mean_batch_seconds.

## checks

Eight focused Rust tests pass offline/locked with two build jobs, including native four/eight-target preparation, deterministic serialized output, strict/mosaic provenance, different documents/order scopes, actual100000-byte split with next-target retention, ID/text/context exclusions, malformed hash/Evaluation rejection, bounded context-removal accounting and exact header-row preservation. A strict Clippy pass covers owned targets; an existing unrelated dependency warning remains. These are finite checks, not Lean proofs or semantic accuracy measurements. No model/GPU/training/DB/held-out assessment was run.
