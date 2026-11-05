# corpus-events-extraction

## descriptant

# corpus-events-extraction

Event::show, noun, events, book_body and sentences.

## Forward

Select VERB or ADJ with Cop child. Passive roles: OblAgent,NsubjPass-or-Nsubj; active: Nsubj,Obj (VERB only). First matching child precedes noun admission. Recipient Iobj then to-Obl. Compose lowercased verb, optional be, first particle, not prefix.

## Invariant

First-match selection and sentence indices preserve source order. Count ties have canonical key order. Arithmetic assumes no integer overflow; IEEE NaN is filtered only by the source comparison. Counts retain role multiplicity, not unique entities.

## Cost

O(W^3) worst-case repeated child scans plus text bytes; linear output.

## Composition

Declared child relations compose through the same body, token arrays, maps and ranked records. Reverse composition existentially retains these intermediate values; complements cannot simply be unioned. Shared work is counted once. Parser and filesystem state remain latent dependencies.

## Epoch memory

## Gaps

Syntax is source-bound and partial: macro expansion, names/types, cfg evaluation and external modules absent. No live IO/parser/corpus/teacher/GPU run. Filesystem traces, thread scheduling, tokenizer/annotator/repair and external sentence splitting beyond bounded fixtures remain unverified dependencies. Models do not certify complete logical semantics.

## catalog

# corpus-events-extraction typed models

## nonbehavior

Type: behavior_complement.

U=(W,E). Excludes active adjectival patient and recipient-only emission without admitted agent/patient. Oracle native masking.

## antecedents

Type: reverse_reconstruction.

e permits all W satisfying role and normalization constraints; spelling/case/surface form and ignored fields are lost. First-match constraints remain essential. Oracle proposed ignored-token metamorphism.

## first-match

Type: selection_partition.

find(P)=noun(W[min{i:head=p+1 and P}]), not first admissible noun. PRON masks later NOUN. Oracle native masking fixture.

## voice-negation

Type: role_mapping.

AuxPass switches to passive sources; first NsubjPass or Nsubj wins in index order. Negation only Advmod lemma not/never/n't; first particle appends. Oracle native passive fixture.

## noun-unicode

Type: normalization_relation.

Only NOUN/PROPN with all lowercased characters alphabetic or '-' admitted. Empty lemma vacuously Some(empty), potentially admitting an event rendered '_'. Oracle Unicode/digit/empty assertions.

## recipient-choice

Type: fallback_relation.

Admitted first Iobj wins; rejected first Iobj allows first to-Obl fallback, which itself selects before noun admission. Oracle priority native; rejected-Iobj branch proposed.

## History

### 2026-10-30 · Epoch memory

2026-10-30 prospective prediction: native fixtures match marker reversal, first-role masking, Unicode byte admission, role multiplicity and degenerate NaN behavior. These are characterized, not policy changes. Only private register helpers were extracted; event production logic is unchanged. No executable approximation/error-cap claim is made.

### 2026-10-30 · document

Subject: source hash in manifest, epoch 2026-10-30. Scope: Event::show, noun, events, book_body and sentences.. Domain: terminating Rust calls without integer overflow, narrowed below. Models are partial source-derived relations, not proofs. Complement is an inner bound in the stated input/output universe. Reconstruction retains all compatible antecedents, not a unique inverse. Oracle status is explicit below; ../model.md#tests records executed coverage.
