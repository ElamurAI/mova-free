# enrichment-artifacts

## descriptant

### Enrichment artifacts descriptant

Mova — Combinatorial SLM uses this leaf to turn agreed source annotations into bounded review packets and authenticated draft artifacts. This groupant binds the existing enrichment-artifacts registry node; its authoritative Rust remains beside this directory. Forward/invariant/cost models are in model.md#core-model and twenty typed obernants in model.md#catalog. First is behavior complement over an explicit trace universe; second reconstructs ambiguous compatible antecedents.

## catalog

# Enrichment artifact obernant catalog

## absent-artifact-transitions

Type: behavior_complement. Domain: U is all finite one-call traces over calls prepare/review/stage/status/require_staged/refine/restage with supplied arguments, initial file maps, helper responses, ordered attempted/successful filesystem operations and return values. R is the actual Rust trace relation under the shared assumptions. Operator: C_hat = {(x,t) in U: t returns Ok from stage with published=true in its newly written receipt, or returns Ok from review after observing a preexisting teacher-reply.json or teacher-failure.json at its attempt gate, or returns Ok from require_staged with legacy_stage_requires_restage=true}. This is a source-derived candidate inner approximant of U\R, not a proven bound. It concerns these guarded observations only; it does not complement a successful-file checklist. Assumptions: no external writers and successful reads refer to the observed bytes. Evidence: stage, review, require_staged. Prospective oracle: mutate each guard in an isolated fixture and require the baseline to reject the associated forbidden trace. Failing review/stage traces may write failure-only artifacts or partial bytes and need not be literal successful prefixes.

## artifact-antecedents

Type: reverse_reconstruction. Domain: same trace relation R, observed projection y of return value and final named bytes. Operator: B(y) = {x,helper choices,trace | R(x,trace) and projection(trace)=y}; partial implementation is symbolic filtering by verified_packet, reviewed_bytes and status predicates. Retain all compatible antecedents, with no claim of completeness for filesystem failures. A staged receipt does not recover the original full corpus, unselected lessons, teacher reasoning or a unique call history: direct stage and restage may yield identical staged payloads. Assumptions: observation projection is fixed and any known provenance artifacts are included before filtering. Evidence: prepare truncation, restage, status. Prospective oracle: two source corpora differing only in unselected annotations should yield the same packet and staged observation; a receipt with a wrong review hash should have no admitted antecedent under the identity model.

## document-selection-fiber

Type: selection_preimage. Domain: parsed document groups, limit in 8..64. Relation: selected document maximizes sentence count among train-routed groups, breaking ties toward lexicographically smallest document; sort by numeric final ID suffix (unparseable becomes u64::MAX), truncate, then require at least eight. Reconstruction preserves arbitrary losing groups and omitted suffix sentences that do not change the winner/order. Assumptions: stable sort retains input order for equal numeric keys; route is fixed. Evidence: prepare. Prospective oracle: tie counts across two train documents; permute equal-suffix inputs and check the retained order rather than assuming a textual-ID tie break.

## source-join-boundary

Type: join_admissibility. Domain: selected tree IDs and prag/world tables. Relation: each selected ID joins a prag row with nonempty form/act/polarity/voice and a world row containing the world key. Empty world text is permitted by this gate. Whole input trees/TSVs must parse first; unused malformed records can still fail parsing. Assumptions: other prepare gates pass. Evidence: trees, table, prepare. Prospective oracle: remove one selected context row, then replace its world with an empty string; expect first rejection and second passage through this particular gate.

## lexical-ranking-projection

Type: ranking_projection. Domain: selected token rows and six-field dictionary index. Relation: eligible lowercase alphabetic NOUN/VERB/ADJ/ADV lemmas longer than two bytes rank unknown before known, then descending source-ID set size, then lexical order; retain at most 24 and at most three sorted source IDs each. The projection loses counts beyond the three displayed IDs and candidates after rank 24. Assumptions: token rows passed trees; Unicode alphabetic eligibility differs from later ASCII review admission. Evidence: prepare, validate. Prospective oracle: a Unicode eligible proposal can appear in packet but fail lowercase ASCII review gate; add a fourth source occurrence and inspect ranking independently of displayed provenance.

## lesson-retrieval-quotient

Type: retrieval_projection. Domain: eligible course lesson texts excluding books/dictionary. Relation: alphabetic tokens longer than three bytes are lowercased into sets; positive lexical-overlap lessons sort by descending overlap then path, retain one per course and at most four overall; excerpt is first 1800 characters. Different repeated token frequencies have equal set overlap. Assumptions: same enumerated course names and readable UTF-8. Evidence: tokens, prepare. Prospective oracle: duplicate words in a losing lesson without changing sets and verify ranking invariance; alter an unreturned lesson and verify school index hash changes.

## snapshot-freshness-fiber

Type: snapshot_identity. Domain: prepared packet plus current round/dictionary/school. Relation: check_source accepts exact dictionary-byte hash, nonempty school-index hash over all eligible lesson paths/hashes, and exact selected text/tree/prag/world projections. Unselected valid annotation changes may pass; school files outside enumerated courses or .lesson extension are outside its fingerprint. Assumptions: SHA-256 collision resistance and parser success. Evidence: check_source. Prospective oracle: change an unselected valid sentence versus a selected one, then change a nonretrieved eligible lesson; distinguish projection equality from whole-corpus equality.

## prompt-budget-boundary

Type: resource_admission. Domain: UTF-8 prompt and ENRICH_PROMPT_MAX_BYTES absent or parseable usize. Relation: admission iff limit>0 and prompt byte length<=limit; default 150000. prepare checks after packet.json and packet.sha256 writes, review checks before lock creation, refine checks only via its later review call after repair artifacts are written. Assumptions: environment fixed per call and preceding gates pass. Evidence: prompt_budget, prepare, review, refine. Prospective oracle: limit at n-1,n,n+1 bytes including multibyte text; inspect residual packet/repair files on failure. This is not a token or total-memory bound.

## review-attempt-boundary

Type: attempt_exclusion. Domain: verified packet, admitted prompt and run directory. Relation: create review-running directory, arm drop cleanup, write pid, reject existing success/failure attempt marker, then invoke teacher once. A stale lock blocks entry; process death may preserve it. Result Err writes teacher-failure.json when possible, whereas an Ok envelope can write teacher-reply.json before authentication/decoding fails. Assumptions: Rust Drop occurs on ordinary return, not guaranteed on process kill. Evidence: Lock, review. Prospective oracle: scripted transport error, malformed successful envelope and preexisting marker produce distinct residues; failure to write marker is not evidence teacher was never invoked.

## review-authentication-fiber

Type: provenance_relation. Domain: review.json and teacher reply envelope, optional repair parent. Relation: provider codex and is_error=false plus string result; decoded bytes must equal review.json exactly, with deterministic compose_repair only when repair-parent-review.json exists. This is local provenance consistency, not cryptographic proof of service origin. Assumptions: trusted artifact generation; editable files alone cannot authenticate a remote principal. Evidence: authenticated_result, decoded_review, reviewed_bytes. Prospective oracle: alter whitespace only in direct review.json; reject byte mismatch despite equal JSON meaning. For repairs compare generated pretty bytes instead.

## grounding-semantic-boundary

Type: evidence_projection. Domain: claims and questions against packet sentences. Relation: quote/evidence must be nonempty single-line verbatim substring of the referenced source; question answer must be contained in evidence and at most twelve whitespace words. This projection does not entail claim truth, answer relevance or plausible distractors. Assumptions: other validation gates handled separately. Evidence: grounded, validate. Prospective oracle: syntactically grounded but semantically unsupported paraphrase illustrates the unmodeled semantics; missing literal span must fail.

## dictionary-preservation-boundary

Type: preservation_contract. Domain: at most six unique proposed review headwords, six single-line fields and source examples. Relation: require lowercase ASCII headword, proposed source ID, example equal entire source sentence and included in field 4; if known record exists, old fields 2 and 4 remain substrings. It neither freezes all fields nor guarantees semantic preservation. Assumptions: known_record has the six-element shape produced by prepare; arbitrary malformed Packet vectors can invalidate this assumption. Evidence: validate. Prospective oracle: delete one prior meaning substring to fail, change another field to characterize the permitted boundary; retain contradictory extension as a semantic-quality counterexample.

## question-target-quotient

Type: equivalence_quotient. Domain: review questions. Relation: canonical removes nonalphanumeric characters and lowercases; canonical question keys must be unique, and (source_id,canonical answer,kind) targets unique across phases. Four canonical option values must differ. Distinct source IDs or kinds can distinguish otherwise equal answers. Assumptions: Unicode character operations match Rust and string equality. Evidence: canonical, validate. Prospective oracle: punctuation/case-only question duplicates fail; same answer on distinct sources does not fail the target gate by itself.

## phase-and-freeze-admission

Type: routing_constraint. Domain: questions and optional StudyBinding. Relation: exactly 12 exercise, 8 test, 4 exam rows, known kind/phase; new exercise source IDs avoid binding.frozen_source_ids. This does not hide frozen passages: validate emits every packet sentence as an explanation. Assumptions: context has passed verified_packet/study verification for stage. Evidence: validate, verified_packet. Prospective oracle: move a frozen test source to exercise while preserving counts and observe rejection; inspect lesson explanation text to avoid claiming assessment-source invisibility.

## repair-support-fiber

Type: restricted_reconstruction. Domain: authenticated invalid parent and Repair patch. Relation: refine supports only a first validate error equal to the exact extractive-span error, no existing repair parent, matching packet digest; compose_repair requires exactly all span_errors indices once, matching parent-byte hash, unchanged phase/kind/source_id and nonempty single-line reason. Other parent field values remain cloned; corrected review still needs full validation. Assumptions: indices come from the guarded exact index set. Evidence: refine, span_errors, compose_repair. Prospective oracle: parent with an earlier unrelated invalid field blocks refinement even if it also has span errors; wrong patch hash or extra index fails.

## stage-write-frontier

Type: failure_frontier. Domain: authenticated review staging with arbitrary filesystem failures. Relation: parse/validate failure attempts validation-rejected.json; verification/authentication failures happen outside that handler. Validated path creates staged.tmp, writes six files in fixed order, renames to staged, then writes receipt. write_new may create a file and fail after partial bytes; rename may succeed while receipt creation fails. Assumptions: serial ordinary filesystem operations, no rollback protocol. Evidence: write_new, stage. Prospective oracle: inject failure during each write and between rename/receipt; preserve partial file and failure-only branches. A failed trace is not generally a literal successful trace prefix, and an error-record write can mask the original error.

## status-observation-priority

Type: state_projection. Domain: verified packet with arbitrary artifact mixtures. Relation: absent receipt yields first matching state: staging_incomplete if staged or staged.tmp; rejected; failed; reviewed; review_incomplete; otherwise prepared. Presence-only states do not validate marker contents or authenticate reviewed bytes; review-running is ignored. Present receipt instead runs receipt validation and may error rather than fall back. Assumptions: verified_packet succeeds. Evidence: status. Prospective oracle: create both rejected and failed markers; expect rejected. Add partial receipt and expect parse error, not a presence-derived fallback state.

## receipt-output-closure

Type: artifact_closure. Domain: receipt, bound input bytes and staged files. Relation: schema 2 requires exactly six output keys, schema 1 exactly five excluding source-map; input hashes, staged/false/false flags and all recorded file hashes must match. Extra actual directory files are not inventoried by this key equality. Schema 1 is labeled legacy and require_staged rejects it. Assumptions: collision resistance, trusted files during reads; status does not rerun validate. Evidence: status, require_staged. Prospective oracle: remove/add an outputs key in the receipt and reject; add an unlisted staged file and characterize acceptance; legacy schema status can pass while require_staged fails.

## restage-provenance-preservation

Type: lineage_relation. Domain: verified authenticated old run and absent new directory. Relation: if old receipt exists it must pass status; copy context and six named inputs plus available repair-parent/plan files, write origin with teacher_called_again=false, then stage(new). Compatible restage outcomes preserve review bytes but can gain schema-2 source-map. Partial new directories survive errors and block simple retry. Assumptions: unchanged old files during copying; stage helper semantics fixed. Evidence: restage. Prospective oracle: restage repaired review and check its authentication chain; malformed existing old receipt must prevent directory creation. No teacher invocation is needed by this function.

## atomic-receipt-counterfactual

Type: transaction_counterfactual. Domain: actual stage trace and hypothetical stage* on the same validated payload. Changed rule: stage* writes payload and receipt inside one private staging directory, then atomically renames that whole directory to a new publication location; consumers read only that location. Relation: compare observable payload/receipt co-presence, leaving validation identical. Under atomic rename on the same filesystem and no concurrent writers, stage* eliminates the actual window where staged exists without a receipt, but not partial hidden writes, crash-durability uncertainty or publication policy. Evidence: source stage ordering is baseline; alternative is a modeling hypothesis only. Prospective oracle: fail immediately after current rename and at corresponding hypothetical boundaries, comparing only consumer-visible locations. No implementation or performance gain is claimed.

## tests

### Checks and evidence scope

Existing source tests available for future execution include source_validation_rejects_cycles_duplicates_and_partial_tables, stale_review_unsupported_examples_duplicates_and_erased_meanings_fail, partition_is_document_stable_and_changes_are_versioned, tampered_artifacts_and_prompts_cannot_retain_staged_status, compact_prompt_preserves_the_complete_annotation_payload, targeted_repairs_preserve_other_questions_and_reject_unrequested_edits, and restaging_a_repaired_review_preserves_its_authentication_chain. These are references, not claims that this epoch executed them or that they cover every catalog relation.

Structural procedure: run installed mova-modelcheck groupants against this worktree and models/contracts.json with fresh report paths twice; compare JSON bytes. Check manifest source digest, references and registry ownership; a Rust probe checks exactly twenty unique catalog headings, each matching manifest section/id and type. Deliberately mutate the probe input to a wrong heading and require rejection as its negative control. Run git diff --check and secret-guard before commit. Reports remain outside the repository. Structural validity establishes declared bindings and accounting only.

Prospective runtime priorities: partial writes in write_new; transport error versus bad successful envelope; rejection-file failure; failure between staged rename and receipt; duplicate presence markers; malformed receipt; legacy schema restaging; repaired provenance copied intact; prompt byte boundaries on multibyte text. All require isolated temporary runs and deterministic offline stubs, not live teacher access.

## core-model

### Enrichment artifact forward, invariant and cost model

#### Forward approximant

State X = (input arguments, source bytes, environment, filesystem map, teacher outcome). A filesystem value may be absent, complete or a partial byte string. Observations include attempted operations, written bytes, return result and optional status output. Work is serial with no external mutation; helper implementations are abstract relations rather than verified dependencies.

prepare parses all annotations, chooses the largest train-routed document with lexical tie break, selects up to limit sentences (8..64), joins selected pragmatics/context, derives lexical/role proposals, scans eligible school lessons, and hashes dictionary and school index. It creates a new run, writes packet and digest, checks prompt budget, and writes prompt/digest. Any later error can leave earlier or partially created files.

review verifies the packet/context and prompt, checks byte budget, creates a directory lock with PID, checks attempt markers and asks the teacher once. Transport Err attempts a failure record. Transport Ok writes the reply before checking envelope and decoding the review. Decoding can fail leaving a reply without review. Drop removes the lock on ordinary return; process termination can leave it stale.

stage verifies source/prompt and authenticated review bytes before its rejection handler. Inside that handler strict Review parsing and validate build a lesson or attempt a rejection record. On success it creates staged.tmp, emits dictionary, lesson, source map, questions, cross-course gaps and corrections, renames to staged, then writes schema-2 receipt. No publication occurs. Validation markers and transport failures are separate branches, not success artifacts.

status first verifies packet/context. Without receipt it classifies file presence using the catalog priority. With receipt it checks identity, flags, schema, exact outputs key set and recorded hashes (additional top-level receipt fields are accepted); it does not repeat semantic validation. require_staged rejects the legacy schema flag. check_source additionally checks current source projections and whole eligible-school/dictionary fingerprints, but is not implicitly called by every other public function.

refine allows one targeted span repair into a new run; it preserves immutable parent fields and provenance and then calls review/stage. restage authenticates and copies an old run, optionally validates its existing receipt, preserves repair provenance and invokes stage without teacher. Neither is a resumable transaction.

#### Invariants and boundaries

These are source-derived conditional constraints, awaiting scoped semantic checks in this epoch:

- Successful stage emits published=false and complete=false; draft output is not a completion or promotion claim.
- Packet/prompt identity binds bytes, not all external sources; check_source is the separate freshness operation.
- Known dictionary meaning/example substrings survive validated edits, under the six-field prepared-record assumption. This is weaker than semantic preservation.
- Question counts and canonical uniqueness are exact gates; source grounding does not establish answer quality.
- create_new prevents overwrite at each write, but can leave partial new files after write_all failure. A directory rename does not make a later receipt write atomic with its payload.
- Presence-based status without receipt is a diagnostic projection. A marker may be partial or preexisting; it does not reconstruct a unique history.
- Repair preserves all unedited Review fields by cloning; compatible provenance must include parent bytes and original reply, not merely a parent hash string.

#### Cost approximant

Let B be total source bytes read, N token rows, h_i maximum head-chain length per sentence, D document count, V candidate vocabulary, L eligible lesson count, W total lesson bytes, Q reviewed questions, P prompt bytes, O staged output bytes. Tree cycle checks cost O(sum_i n_i*h_i*log n_i) with ordered visited sets, quadratic in tokens up to a logarithmic factor for long chains. Other parsing and hashing are at least linear in their read bytes; ordered maps/sets add logarithmic insert/look-up costs. Document, word and lesson ranking sort bounded-by-input collections; lesson retrieval reads every eligible lesson even though at most four are returned. Quote and evidence checks repeatedly scan source strings; counts bound accepted output, not work on arbitrary oversized rejected inputs.

Prepare costs parsing(B,N) + ranking(D,V,L) + lesson tokenization(W) + serialization/hash(P). Review adds one transport cost T_teacher plus byte verification/serialization; byte limit does not bound model tokens or end-to-end time. Stage costs validation(Q,B) + O(O) serialization/write/hash. Status with receipt hashes named payload bytes and reads/authenticates review again. Restage adds copied input/context bytes and stage work. Failure cost is accumulated attempted operations, potentially including error-marker writes; it is not necessarily a successful-prefix sum. No timing or quality gain measured here.

## History

### 2026-10-29 · descriptant

#### Epoch memory — 2026-10-29

Source inspection retained the registry's useful observation: literal grounding and artifact fingerprints do not establish semantic correctness. The self-study interaction model already separates failure-specific artifacts and partial writes from successful-operation prefixes; this leaf makes that boundary explicit around review and staging.

A teacher transport failure writes teacher-failure.json, but a successful envelope can fail authentication/decoding after teacher-reply.json exists. Early stage packet/authentication failures occur outside the validation-rejected handler. A failed rejection-record write can replace the original error returned. Staging renames payload before writing receipt, so a visible staged directory need not be a completed stage. Status presence labels ignore review-running and do not authenticate review bytes until receipt validation. These details should survive future modeling epochs.

Preparation checks prompt budget after packet files exist; refine reaches that check after writing repair artifacts. Freshness checking is separate from packet self-consistency. The frozen-source gate restricts exercise routing while all packet passages can still appear in explanations. Receipt output-key closure is not directory closure: extra unlisted files are outside the check. Restaging a repaired review requires the parent authentication chain.

No Rust behavior was changed, no teacher was called and no training/GPU or task-quality result was measured. AST extraction, executable reverse solvers, full filesystem/concurrency/crash models, semantic quality and automatic combiner generation remain missing. Structural inventory results in model.md#tests do not turn these source-derived hypotheses into proofs. The atomic-receipt alternative is prospective only; preserve current evidence before any future implementation experiment.

### 2026-10-29 · document

Subject/revision: tools/enrich/src/lib.rs at the SHA-256 in manifest.json, inspected 2026-10-29. Each section is a distinct partial model for one serial call or the explicitly named composition. Unless narrowed below, fix source bytes, environment, helper behavior and initial filesystem; no concurrent mutation, trusted executable and ordinary filesystem semantics are assumptions. Hash comparisons assume collision resistance. Evidence for every entry is source inspection of the named functions, not an executed semantic test or proof. Every oracle below is prospective. Shared assumptions and evidence apply to all sections. Artifact bytes include malformed and partial files; return errors are observations, not rollback guarantees.

### 2026-10-29 · tests

Inspection epoch: 2026-10-29. Source mechanisms and existing test names were inspected; no semantic Rust tests, live teacher, training, GPU or full suite were run for this documentation-only addition. Every catalog oracle is prospective, including fault injection and the counterfactual transaction model.

Executed structural result, 2026-10-29: installed mova-modelcheck groupants passed twice with byte-identical JSON, zero diagnostics, seven file sidecars, one group sidecar, 524 missing file sidecars and three missing group sidecars. The Rust catalog probe passed all twenty unique ID/type/section references and rejected its deliberately wrong first heading. These counts apply to this isolated base checkout plus this leaf, before integration with parallel work.

### 2026-10-29 · core-model

Subject: tools/enrich/src/lib.rs, exact source digest in manifest.json; inspection epoch 2026-10-29. Partial source-derived specification, not an execution result or proof. AST is not materialized. Catalog relations share this scope.
