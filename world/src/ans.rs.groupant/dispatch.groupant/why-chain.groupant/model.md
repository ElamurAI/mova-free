# Why first-return fallback chain

## forward

Feeling-specific cause lookup precedes event ranking. If no event, try matching attribute cause, belief and target trait then None. If event exists, choose explicit why; causal-token how; causing event; agent belief; agent goal; fulfilled intent; otherwise None retaining event anchor.

## invariant

With matched event and Some why, lower causal how cannot win even when why is empty. Exhausted event paths produce None with empty text, not anchor text promoted to answer.

## behavior-complement

With matched event and Some why, lower causal how cannot win even when why is empty. Exhausted event paths produce None with empty text, not anchor text promoted to answer.

## reverse-reconstruction

Observed text may arise from multiple branches and antecedent worlds; note and anchor constrain but do not uniquely identify histories. Retain every world/frame/selected-event configuration yielding the same Answer projection.

## parent-interaction

find_events ranks before this chain; negated question stably prioritizes negated hits but does not hard-exclude positive hits. ans then numeric-normalizes chosen text.

## boundary

Causal how accepts any token because/since/so/as/for; this is a lexical gate, not a proof of causality.

## information

Missing cause target falls through via Option; matched event failures do not reopen no-event attribute fallback.

## cost

Event search plus branch-dependent history/entity scans and rendering; early return skips later queries. No global performance bound without World costs.

## checks

Existing supplied-frame why-before-how then None witness. Feeling, belief, goals and intents source-inspected only.

Exact selected native boundary: `cargo test --offline --locked -p world --lib ans::wave8_models::supplied_frame_why_precedence_and_anchor_only_failure -- --exact --test-threads=1`. Repeated in debug and release; see the current file-level wave34 evidence and external native-receipts.json.

## assurance

The behavior-complement is an inner exclusion hypothesis over admissible input/output pairs, derived from branch order; not the complement of an incomplete model of the entire file. Reverse reconstruction is the compatible antecedent relation, retaining every modeled ambiguity rather than a chosen inverse. Parent-interaction is a composition relation, boundary a constraint relation, information an information/state projection, and cost a resource relation. The named checks are finite falsifiers; unexecuted branches remain source-inspected gaps. No parser/model/teacher/corpus/frozen assessment was run.

## epoch-memory

New child in wave34; no prior child renamed or counted. Parent historical wave8/wave9 evidence remains historical. Current raw receipts are in the external wave34 reasoning run; source syntax is partial, macros opaque, and no semantic AST or general proof is supplied.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Subject: world-ans-dispatch-why-chain, current source SHA d8d94c891e347d2b640744b20c4662dc35064368e7d4d9253c4954b4297e1e09. Domain: one terminating supplied-input helper transition; valid indices, sufficient memory and finite strings/trees/worlds. Source-derived partial relations, not a verified universal theorem. All views below share this domain and exact manifest scope; specialized assumptions are stated locally. No executable percentage approximant is claimed.
