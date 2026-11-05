# Story admission before service filtering

## forward

Take cur; if pending text has nonblank content append its whitespace-normalized paragraph; sum paragraph byte lengths, append (title, paragraphs) iff total>=200. cur is None after taking even when the story is rejected.

## invariant

An empty cur cannot append output; total<200 cannot be admitted by this closure.

## behavior-complement

An empty cur cannot append output; total<200 cannot be admitted by this closure.

## reverse-reconstruction

For an appended story recover any whitespace variants of pending paragraph and existing normalized paragraphs with the same ordered strings and byte total>=200. Rejected cur may be absent or any subthreshold story; rejection loses title/content.

## parent-interaction

This consuming flush is invoked before replacing heading state and at EOF; rejected state is not available to a later story.

## boundary

Threshold is the sum without paragraph separators, not source byte count or Unicode character count.

## information

Admission output still contains bracket paragraphs; later service deletion cannot retroactively change the admission decision.

## cost

One pending-buffer tokenization and one paragraph-byte-length sum: O(buffer bytes + paragraph count), plus appended output allocation.

## checks

Historical 180..220 byte oracle and admitted illustration plus short retained prose.

Exact selected native boundary: `cargo test --offline --locked -p world --lib read::wave9_models::segmentation_threshold_and_service_filter -- --exact --test-threads=1`. Repeated in debug and release; see the current file-level wave34 evidence and external native-receipts.json.

## assurance

The behavior-complement is an inner exclusion hypothesis over admissible input/output pairs, derived from branch order; not the complement of an incomplete model of the entire file. Reverse reconstruction is the compatible antecedent relation, retaining every modeled ambiguity rather than a chosen inverse. Parent-interaction is a composition relation, boundary a constraint relation, information an information/state projection, and cost a resource relation. The named checks are finite falsifiers; unexecuted branches remain source-inspected gaps. No parser/model/teacher/corpus/frozen assessment was run.

## epoch-memory

New child in wave34; no prior child renamed or counted. Parent historical wave8/wave9 evidence remains historical. Current raw receipts are in the external wave34 reasoning run; source syntax is partial, macros opaque, and no semantic AST or general proof is supplied.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Subject: world-read-segmentation-admission, current source SHA 80ef1f69d776dcb5cd03bf925cc5c01e827d7919495ccda17d785eb3de459857. Domain: one terminating supplied-input helper transition; valid indices, sufficient memory and finite strings/trees/worlds. Source-derived partial relations, not a verified universal theorem. All views below share this domain and exact manifest scope; specialized assumptions are stated locally. No executable percentage approximant is claimed.
