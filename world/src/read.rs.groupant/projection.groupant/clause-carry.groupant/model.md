# Selected clause closure and subject carry

## forward

Start at root, close Conj/Advcl/Ccomp/Parataxis children with VERB/AUX/ADJ/NOUN POS once, sort indices. For each selected node find first explicit subject else prior nonempty subject; update carry; choose copula, speech, or action rendering. Nonverb without Cop emits nothing but may update carry.

## invariant

Empty root has no clauses. Speech branch cannot include its direct object through action-role rendering; nonverb without copula cannot emit an action clause.

## behavior-complement

Empty root has no clauses. Speech branch cannot include its direct object through action-role rendering; nonverb without copula cannot emit an action clause.

## reverse-reconstruction

Recover all selected clause trees, originals and carried-subject histories giving the observed ordered strings. Speech objects and omitted modifiers remain free, but later inherited subjects constrain earlier nonempty subjects.

## parent-interaction

phrase supplies subjects and roles; carry is local per skeleton call. Sorting happens before carry, so reachability discovery order does not determine inherited subject.

## boundary

Negation detects any direct child's listed lemma/form and renders canonical not; modal uses first qualifying Aux only in action branch.

## information

Subject carry is updated before nonverb/copula checks and speech branch; it is not restricted to emitted clauses.

## cost

v selected nodes, n tree nodes: conservative O(n*v) membership plus per-role phrase sorting and output bytes; overlapping phrase work cannot be charged twice to parent and child.

## checks

Existing Boolean and speech fibers plus new modal/subject-reset fixture; malformed cyclic Tree inputs excluded.

Exact selected native boundary: `cargo test --offline --locked -p world --lib read::wave34_models::modal_negation_and_subject_reset -- --exact --test-threads=1`. Repeated in debug and release; see the current file-level wave34 evidence and external native-receipts.json.

## assurance

The behavior-complement is an inner exclusion hypothesis over admissible input/output pairs, derived from branch order; not the complement of an incomplete model of the entire file. Reverse reconstruction is the compatible antecedent relation, retaining every modeled ambiguity rather than a chosen inverse. Parent-interaction is a composition relation, boundary a constraint relation, information an information/state projection, and cost a resource relation. The named checks are finite falsifiers; unexecuted branches remain source-inspected gaps. No parser/model/teacher/corpus/frozen assessment was run.

## epoch-memory

New child in wave34; no prior child renamed or counted. Parent historical wave8/wave9 evidence remains historical. Current raw receipts are in the external wave34 reasoning run; source syntax is partial, macros opaque, and no semantic AST or general proof is supplied.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Subject: world-read-projection-clause-carry, current source SHA 80ef1f69d776dcb5cd03bf925cc5c01e827d7919495ccda17d785eb3de459857. Domain: one terminating supplied-input helper transition; valid indices, sufficient memory and finite strings/trees/worlds. Source-derived partial relations, not a verified universal theorem. All views below share this domain and exact manifest scope; specialized assumptions are stated locally. No executable percentage approximant is claimed.
