# Direct modifier phrase projection

## forward

Collect direct NmodPoss, base Amod and Compound children plus head, fetch original form or empty string for missing aligned positions, sort by token index, join with single spaces.

## invariant

No other child relation or grandchild is included merely by descendant status; lexical forms are not substituted for missing orig entries.

## behavior-complement

No other child relation or grandchild is included merely by descendant status; lexical forms are not substituted for missing orig entries.

## reverse-reconstruction

All trees/original arrays with the same selected indexed strings belong to the output preimage. Determiners, deeper descendants and unused originals may vary; missing originals can collide with explicit empty strings.

## parent-interaction

skeleton calls this helper for subjects, predicate and selected roles. Tree indices and original strings must describe the same tokenization for linguistic interpretation.

## boundary

One direct CompoundPrt whose base is Compound can participate in phrase selection; base relation and exact possessive relation are intentionally distinct.

## information

Sorting provides source order, not dependency visitation order; equal visible strings erase source position information.

## cost

For d direct children O(d + k log k + output bytes), O(k + output bytes) allocation, k selected including head.

## checks

New direct-modifier/original-form test, finite supplied Tree only.

Exact selected native boundary: `cargo test --offline --locked -p world --lib read::wave34_models::phrase_direct_modifiers_and_original_forms -- --exact --test-threads=1`. Repeated in debug and release; see the current file-level wave34 evidence and external native-receipts.json.

## assurance

The behavior-complement is an inner exclusion hypothesis over admissible input/output pairs, derived from branch order; not the complement of an incomplete model of the entire file. Reverse reconstruction is the compatible antecedent relation, retaining every modeled ambiguity rather than a chosen inverse. Parent-interaction is a composition relation, boundary a constraint relation, information an information/state projection, and cost a resource relation. The named checks are finite falsifiers; unexecuted branches remain source-inspected gaps. No parser/model/teacher/corpus/frozen assessment was run.

## epoch-memory

New child in wave34; no prior child renamed or counted. Parent historical wave8/wave9 evidence remains historical. Current raw receipts are in the external wave34 reasoning run; source syntax is partial, macros opaque, and no semantic AST or general proof is supplied.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Subject: world-read-projection-phrase, current source SHA 80ef1f69d776dcb5cd03bf925cc5c01e827d7919495ccda17d785eb3de459857. Domain: one terminating supplied-input helper transition; valid indices, sufficient memory and finite strings/trees/worlds. Source-derived partial relations, not a verified universal theorem. All views below share this domain and exact manifest scope; specialized assumptions are stated locally. No executable percentage approximant is claimed.
