# Event eligibility and rank boundary

## forward

Compute score_words then reject missing required verb before participant/section bonuses. Role bonus is 3 matching requested role, 1 other role, -2 absent when who nonempty; section bonus 3/0.5/-1.5. Keep iff content>=min or who nonempty and total>=4; sort descending total, ascending sentence, stable encounter ties.

## invariant

Section bonus alone cannot rescue content below min with empty who. Required missing verb excludes even otherwise rescued total. Total below 4 cannot rescue subthreshold content.

## behavior-complement

Section bonus alone cannot rescue content below min with empty who. Required missing verb excludes even otherwise rescued total. Total below 4 cannot rescue subthreshold content.

## reverse-reconstruction

Given ordered hits recover all worlds/query-role-section configurations satisfying each hit gate and ranking; absent events may fail either gate. Equal totals retain role/section/content tradeoffs, not unique evidence.

## parent-interaction

find_events uses min=1 strict call first, retry min=2 without verb only when strict is empty and need_verb true. Nonempty strict suppresses relaxed alternatives.

## boundary

Participant in other role plus own-section bonus gives exactly 4 even with zero content; requested role plus own-section gives 6.

## information

Source assumes valid one-based event sentence index before section lookup; this is a panic boundary not an admission rejection.

## cost

E ev_words materializations plus comparison work and O(E log E) sort; ev_words scans P/B entities per event. Strict retry may double materialization; no measured speedup.

## checks

New zero-content role-rescue test plus existing section/tie/relaxed-fallback fixture; supplied synthetic World only.

Exact selected native boundary: `cargo test --offline --locked -p world --lib ans::wave34_models::participant_rescue_requires_total_four -- --exact --test-threads=1`. Repeated in debug and release; see the current file-level wave34 evidence and external native-receipts.json.

## assurance

The behavior-complement is an inner exclusion hypothesis over admissible input/output pairs, derived from branch order; not the complement of an incomplete model of the entire file. Reverse reconstruction is the compatible antecedent relation, retaining every modeled ambiguity rather than a chosen inverse. Parent-interaction is a composition relation, boundary a constraint relation, information an information/state projection, and cost a resource relation. The named checks are finite falsifiers; unexecuted branches remain source-inspected gaps. No parser/model/teacher/corpus/frozen assessment was run.

## epoch-memory

New child in wave34; no prior child renamed or counted. Parent historical wave8/wave9 evidence remains historical. Current raw receipts are in the external wave34 reasoning run; source syntax is partial, macros opaque, and no semantic AST or general proof is supplied.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Subject: world-ans-ranking-eligibility, current source SHA d8d94c891e347d2b640744b20c4662dc35064368e7d4d9253c4954b4297e1e09. Domain: one terminating supplied-input helper transition; valid indices, sufficient memory and finite strings/trees/worlds. Source-derived partial relations, not a verified universal theorem. All views below share this domain and exact manifest scope; specialized assumptions are stated locally. No executable percentage approximant is claimed.
