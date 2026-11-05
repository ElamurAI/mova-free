# Exclusive lexical evidence priority

## forward

For each query lemma class choose first overlap: event verb +3 and vhit=true, else participant +1.5, else text +1, else zero. Sum independently for repeated query classes.

## invariant

A class matching all fields contributes 3, not 5.5; vhit cannot be true without a verb overlap.

## behavior-complement

A class matching all fields contributes 3, not 5.5; vhit cannot be true without a verb overlap.

## reverse-reconstruction

For one class and score=3/vhit=true, all four participant/text membership assignments with verb=true are compatible. Aggregate scores lose assignment and query-order information; retain every class sequence producing both score and vhit.

## parent-interaction

ev_words constructs lexical classes before this helper; find_events_min receives both numeric score and vhit, and must retain the distinction.

## boundary

Duplicate query classes count repeatedly; duplicate alternatives inside a class do not multiply the existential overlap.

## information

Once verb matches, adding text or participant alternatives preserves this class contribution; moving text to verb can increase score.

## cost

O(sum over query classes of examined lexical alternatives times query-class membership length); early branch matches avoid lower fields.

## checks

Existing eight-mask priority/fiber test calls actual score_words without dictionary lookup.

Exact selected native boundary: `cargo test --offline --locked -p world --lib ans::wave8_models::score_precedence_and_fibers -- --exact --test-threads=1`. Repeated in debug and release; see the current file-level wave34 evidence and external native-receipts.json.

## assurance

The behavior-complement is an inner exclusion hypothesis over admissible input/output pairs, derived from branch order; not the complement of an incomplete model of the entire file. Reverse reconstruction is the compatible antecedent relation, retaining every modeled ambiguity rather than a chosen inverse. Parent-interaction is a composition relation, boundary a constraint relation, information an information/state projection, and cost a resource relation. The named checks are finite falsifiers; unexecuted branches remain source-inspected gaps. No parser/model/teacher/corpus/frozen assessment was run.

## epoch-memory

New child in wave34; no prior child renamed or counted. Parent historical wave8/wave9 evidence remains historical. Current raw receipts are in the external wave34 reasoning run; source syntax is partial, macros opaque, and no semantic AST or general proof is supplied.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Subject: world-ans-ranking-score-priority, current source SHA d8d94c891e347d2b640744b20c4662dc35064368e7d4d9253c4954b4297e1e09. Domain: one terminating supplied-input helper transition; valid indices, sufficient memory and finite strings/trees/worlds. Source-derived partial relations, not a verified universal theorem. All views below share this domain and exact manifest scope; specialized assumptions are stated locally. No executable percentage approximant is claimed.
