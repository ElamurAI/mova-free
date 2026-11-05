# Heading recognition

## forward

For line i, accept iff blank-before and either >=5 alphabetic characters all uppercase with trimmed byte length <70, or >=3 leading spaces, uppercase first character, <=10 whitespace words, no final . , ; :, >=5 alphabetic characters, and a following blank line.

## invariant

Accepted centered headings exclude two-space indentation and missing following blanks unless independently accepted by caps; a nonblank previous line excludes both routes.

## behavior-complement

Accepted centered headings exclude two-space indentation and missing following blanks unless independently accepted by caps; a nonblank previous line excludes both routes.

## reverse-reconstruction

Recover every line/context satisfying the observed Boolean. Accepted uppercase headings do not identify indentation, punctuation or title length below the cap; rejected headings retain all failed-predicate combinations.

## parent-interaction

Original indentation is tested before trimming; paragraph normalization cannot be moved ahead of recognition.

## boundary

Capital branch counts letters as Unicode characters but length as UTF-8 bytes; these are different thresholds.

## information

A line may satisfy both branches; there is one heading transition, not two stories.

## cost

Classification scans the line, allocates alphabetic characters and counts words: O(bytes) time and O(letters) temporary space.

## checks

Centered heading fixtures plus historical marker/whitespace tests; recognition is not literary-title detection.

Exact selected native boundary: `cargo test --offline --locked -p world --lib read::wave34_models::centered_heading_boundaries -- --exact --test-threads=1`. Repeated in debug and release; see the current file-level wave34 evidence and external native-receipts.json.

## assurance

The behavior-complement is an inner exclusion hypothesis over admissible input/output pairs, derived from branch order; not the complement of an incomplete model of the entire file. Reverse reconstruction is the compatible antecedent relation, retaining every modeled ambiguity rather than a chosen inverse. Parent-interaction is a composition relation, boundary a constraint relation, information an information/state projection, and cost a resource relation. The named checks are finite falsifiers; unexecuted branches remain source-inspected gaps. No parser/model/teacher/corpus/frozen assessment was run.

## epoch-memory

New child in wave34; no prior child renamed or counted. Parent historical wave8/wave9 evidence remains historical. Current raw receipts are in the external wave34 reasoning run; source syntax is partial, macros opaque, and no semantic AST or general proof is supplied.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Subject: world-read-segmentation-heading, current source SHA 80ef1f69d776dcb5cd03bf925cc5c01e827d7919495ccda17d785eb3de459857. Domain: one terminating supplied-input helper transition; valid indices, sufficient memory and finite strings/trees/worlds. Source-derived partial relations, not a verified universal theorem. All views below share this domain and exact manifest scope; specialized assumptions are stated locally. No executable percentage approximant is claimed.
