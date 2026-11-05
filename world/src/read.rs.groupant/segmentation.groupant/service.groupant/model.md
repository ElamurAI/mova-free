# Retained paragraph and title gate

## forward

For each admitted story retain paragraphs whose first byte is not '[' and whitespace word count>=4; then retain story iff paragraphs nonempty, title not in exact listed exclusions, and title does not end with Fairy Book.

## invariant

Output excludes bracket-starting paragraphs, fewer-than-four-word paragraphs, listed excluded titles and Fairy Book suffix titles.

## behavior-complement

Output excludes bracket-starting paragraphs, fewer-than-four-word paragraphs, listed excluded titles and Fairy Book suffix titles.

## reverse-reconstruction

Recover any admitted pre-filter list whose ordered surviving paragraphs equal output; arbitrary removed service paragraphs and rejected stories remain ambiguous subject to prior admission.

## parent-interaction

Paragraph filtering precedes empty-story removal. Retention preserves surviving input order and does not renumber titles.

## boundary

A bracket after the first character is not excluded by the bracket predicate; normalized paragraphs have no leading whitespace.

## information

Exact title matching is case-sensitive; exclusions do not implement general service-title understanding.

## cost

O(total admitted bytes + stories times fixed title-table size); retain reuses vectors and drops rejected strings.

## checks

Historical service/threshold test; source-only suffix/case cases remain gaps.

Exact selected native boundary: `cargo test --offline --locked -p world --lib read::wave9_models::segmentation_threshold_and_service_filter -- --exact --test-threads=1`. Repeated in debug and release; see the current file-level wave34 evidence and external native-receipts.json.

## assurance

The behavior-complement is an inner exclusion hypothesis over admissible input/output pairs, derived from branch order; not the complement of an incomplete model of the entire file. Reverse reconstruction is the compatible antecedent relation, retaining every modeled ambiguity rather than a chosen inverse. Parent-interaction is a composition relation, boundary a constraint relation, information an information/state projection, and cost a resource relation. The named checks are finite falsifiers; unexecuted branches remain source-inspected gaps. No parser/model/teacher/corpus/frozen assessment was run.

## epoch-memory

New child in wave34; no prior child renamed or counted. Parent historical wave8/wave9 evidence remains historical. Current raw receipts are in the external wave34 reasoning run; source syntax is partial, macros opaque, and no semantic AST or general proof is supplied.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Subject: world-read-segmentation-service, current source SHA 80ef1f69d776dcb5cd03bf925cc5c01e827d7919495ccda17d785eb3de459857. Domain: one terminating supplied-input helper transition; valid indices, sufficient memory and finite strings/trees/worlds. Source-derived partial relations, not a verified universal theorem. All views below share this domain and exact manifest scope; specialized assumptions are stated locally. No executable percentage approximant is claimed.
