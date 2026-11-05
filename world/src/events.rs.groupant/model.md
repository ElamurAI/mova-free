# corpus-events

## descriptant

# corpus-events

Full file: Event/show, noun/events, body/sentences, BookStats/book_stats, top/book_json, run/show.

## Forward

Text -> book_body -> sentences -> tokenizer/annotator/repair -> ordered events. Counts sum occurrences; postings retain original sentence index even when tokenization is empty. JSON summarizes; run pools successful books and writes canonical rows. show selects indexed sentence or '?'. IO behavior is conditional.

## Invariant

First-match selection and sentence indices preserve source order. Count ties have canonical key order. Arithmetic assumes no integer overflow; IEEE NaN is filtered only by the source comparison. Counts retain role multiplicity, not unique entities.

## Cost

W words: repeated scans, including nested oblique-case scans, O(W^3) worst-case; O(W) events. B occurrences and D distinct keys: O(B log D+D log D) plus parser, IO and string bytes; postings O(B). Workers=max(1,threads).

## Composition

Declared child relations compose through the same body, token arrays, maps and ranked records. Reverse composition existentially retains these intermediate values; complements cannot simply be unioned. Shared work is counted once. Parser and filesystem state remain latent dependencies.

## Epoch memory

## Gaps

Syntax is source-bound and partial: macro expansion, names/types, cfg evaluation and external modules absent. No live IO/parser/corpus/teacher/GPU run. Filesystem traces, thread scheduling, tokenizer/annotator/repair and external sentence splitting beyond bounded fixtures remain unverified dependencies. Models do not certify complete logical semantics.

## catalog

# corpus-events typed models

## nonbehavior

Type: behavior_complement.

U=(words,events). Inner complement includes emission at non-VERB/non-copular-ADJ, emission without agent or patient Option, and pronouns becoming noun roles. Oracle: native role fixtures.

## antecedents

Type: reverse_reconstruction.

For O, retain all arrays W with exactly O from eligible predicates in array order, first relation match then noun admission. Ignored forms/tags/features can vary. Earlier matching children cannot be inserted freely. Oracle proposed: form/tag metamorphism.

## body-window

Type: slice_relation.

s=byte after first newline after first START, default 0; e=first END byte, default length. Result=t[s..max(s,e)]. Reversed markers yield empty; START without newline defaults s=0. Oracle five native slices.

## sentence-index

Type: index_relation.

Normalize CRLF, split blank paragraphs, join whitespace, keep byte length>=20 and 2*uppercase-scalar-count<=bytes; dependency split then 3..60 words. Index refers to this list, before skipped empty tokenizations. Oracle normalization and 3/60/61 native boundaries; skipped-token gap source-only.

## count-fiber

Type: multiset_reconstruction.

C[e] equals posting multiplicity. Compatible postings are all ordered (sentence-index,event) sequences with these multiplicities; sentence count equals nonempty tokenizations, not posting length. Oracle aggregate total; full fiber enumeration proposed.

## actor-incidence

Type: weighted_projection.

A[n]=sum C[e]*(1[agent=n]+1[patient=n]+1[recipient=n]), excluding empty. Same noun in two roles contributes twice. Oracle three events yield crow six.

## interaction-incidence

Type: symmetric_projection.

For patient and recipient separately, add C[e] to sorted agent~other if both nonempty and unequal. Repeated other doubles count; self pairs absent. Oracle native crow~fox six.

## corpus-support

Type: support_count.

Across successful input book instances T[e]=sum C_j[e], D[e]=sum 1[e in C_j]. Cells collapse event keys per book before document increment. Duplicate input paths count separately. Oracle proposed two synthetic book maps.

## publication-failure

Type: failure_partition.

Annotator/create_dir/final writer errors propagate; read failure skips book; per-book JSON/index writes discard errors while totals/done advance. Successful return does not certify every file. Oracle source review; injected IO unexecuted.

## ranking-and-show

Type: order_projection.

top sorts count descending/key ascending and truncates; corpus events drop totals<2; cells keep all. show preserves requested index order/duplicates and substitutes '?'. Equal stems collide on disk while pooled instances remain distinct. Oracle native top ties; IO/collisions source-only.

## tests

### Bounded native evidence

Run cargo test -p world corpus_models -- --test-threads=1 and the same with --release. evidence.json binds exact source and normalized observations from two runs per profile. Ten tests total includes register's five. No source include file replaces native tests.

Compiled negative control: extract exact production pure functions and native tests into an external scratch harness linked to the real en and serde_json debug libraries; copy the actual babi::split_sentences body. Baseline passes all ten. Replacing AuxPass detection with Aux compiles and fails passive_negation_particle_and_recipient_priority. This is a real compiled predicate mutation, not a false assertion introduced into a test. control-summary.log and control-events.log hashes are recorded.

The extraction control excludes run, book_stats and filesystem show; native Cargo runs compile the actual complete module. No live annotator, corpus writes, worker scheduling or filesystem-failure injection is tested. Catalog proposals remain unexecuted. Partial AST generation repeats byte-identically; semantic AST and general proof remain gaps.

## History

### 2026-10-30 · Epoch memory

2026-10-30 prospective prediction: native fixtures match marker reversal, first-role masking, Unicode byte admission, role multiplicity and degenerate NaN behavior. These are characterized, not policy changes. Only private register helpers were extracted; event production logic is unchanged. No executable approximation/error-cap claim is made.

### 2026-10-30 · document

Subject: source hash in manifest, epoch 2026-10-30. Scope: Full file: Event/show, noun/events, body/sentences, BookStats/book_stats, top/book_json, run/show.. Domain: terminating Rust calls without integer overflow, narrowed below. Models are partial source-derived relations, not proofs. Complement is an inner bound in the stated input/output universe. Reconstruction retains all compatible antecedents, not a unique inverse. Oracle status is explicit below; model.md#tests records executed coverage.

### 2026-10-30 · tests

2026-10-30. Five new inline tests in the original events.rs corpus_models module use synthetic Word records and strings. They cover five marker slices, CRLF equivalence, paragraph and 3/60/61 word boundaries, passive/negation/particle/recipient priority, first-role masking and noun Unicode/empty admission, and count/actor/interaction/tie projections. Existing fox_crow_cheese remains in place.
