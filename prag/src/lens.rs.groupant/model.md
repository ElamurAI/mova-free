# prag-lens

## model

# prag-lens

## forward

Domain X is tagged calls Lens(W;E) or Means(L), with finite supplied Word sequence W, fixed compiled expression snapshot E and arbitrary finite Reading/Lens values L. No live lexical learning occurs. Let M=expressions_in_sentence(lower(forms W),lower(lemmas W);E). Lens maps each match m in M order to (join(original forms W[m.start..m.end],space),phrase,kind,meaning,register); its domains are sorted unique union of m.domains. Means selects readings whose phrase contains ASCII space, renders text + ' = ' + meaning stripped of every trailing period, joins with '; ', or returns '-' when selection is empty.

## invariant

For Lens calls, reading count/order equal M count/order; text uses original surfaces, and domains=sort(unique(union domains(M))). For Means calls, only selected readings contribute, in order with multiplicity; empty selection yields '-'. Source-matcher span validity is a dependency premise.

## cost

O(total form/lemma bytes) normalization plus external matcher cost; D emitted domain occurrences require O(D log D) sorting and O(D) storage. Reading and meaning rendering cost is linear in emitted bytes; matching costs are not measured here.

## composition

Native supplied Words -> lens -> means uses compiled dictionary kick the bucket = To die. world::domains_with consumes means but independently filters expression domains; downstream world join is inspection only.

## epoch-memory

Initial source-bound epoch. Irony malformed-head native witness panicked before bounds repair. Missing parent evidence now abstains while linear evidence remains. Lens/roles production unchanged. Costs are prospective asymptotic models; no optimization, percentage guarantee or timing claim.

Documentation audit: root relations now retain tagged API inputs, concrete observation constraints and compatible intermediates. Runtime source, syntax and existing test evidence are unchanged.

## checks

cargo test -p prag --lib lens::wave12_models -- --test-threads=1. File-root model.md#evidence binds repeated profiles and mutants. No live parser/teacher/corpus or persisted exam evaluation.

## behavior-complement

For the tagged input domain X above and matching output schema Y, let J(x,y) be the concrete forward/invariant constraints above. C_inner={(x,y) in X×Y | not J(x,y)} is a source-derived inner model of absent behavior. In particular exclude candidate outputs that violate: For Lens calls, reading count/order equal M count/order; text uses original surfaces, and domains=sort(unique(union domains(M))). For Means calls, only selected readings contribute, in order with multiplicity; empty selection yields '-'. Source-matcher span validity is a dependency premise. The native finite checks support selected members/nonmembers; no general Rust correspondence proof or claim about malformed state outside X is made.

## reverse-reconstruction

B(tag,y)={tagged inputs and compatible matcher intermediate M satisfying the forward relation and yielding y}. Lens readings lose token boundaries inside joined surfaces and original lemma/head/POS fields; domain union loses occurrence multiplicity and order. Means loses all single-word readings, kind/register/domains and trailing-period multiplicity. Preserve E as a fixed provenance parameter; do not reconstruct one chosen dictionary or original sentence uniquely.

## coupling

Native supplied Words -> lens -> means uses compiled dictionary kick the bucket = To die. world::domains_with consumes means but independently filters expression domains; downstream world join is inspection only.

Typed entries bind the manifest revision, stated domain and observation, assuming stable compiled dependencies and bounded execution. Status is partial source-derived relation with finite named checks. Partial syntax leaves macro bodies opaque and is no semantic proof.

## evidence

### Native evidence

Old-code witness invalid_heads_abstain_from_ancestry failed with an index panic (head 3 on two words). Repair ignores invalid parent evidence while retaining linear explicit incongruity. Ctx summaries are temporary document state, not persistent lesson-to-exam learning. Lexical test requires the compiled expression snapshot without silent skip. Fixture expectations are hand-authored independently of output.

Production control: Replace matched meaning with wrong: supplied_lexical_observation rejects exact meaning.

Three file roots and four fragments declare 21 typed views. Syntax generation repeats byte-identically; structural checks validate bindings and bounds, not semantics. Compiled dictionary and frame dependencies are premises. No live parser, teacher, corpus, GPU, held-out run or performance claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. Irony malformed-head native witness panicked before bounds repair. Missing parent evidence now abstains while linear evidence remains. Lens/roles production unchanged. Costs are prospective asymptotic models; no optimization, percentage guarantee or timing claim.

Documentation audit, 2026-10-30: root relations now retain tagged API inputs, concrete observation constraints and compatible intermediates. Runtime source, syntax and existing test evidence are unchanged.

### 2026-10-30 · evidence

Heading `### Native evidence, 2026-10-30` stays in evidence as `### Native evidence`.

Source SHA-256: a84730ba30aaee9f91a8581f16544d5769a824f59b7a6ba43470d8aa6a641603. Filter: cargo test -p prag --lib wave12_models -- --test-threads=1, with --release for release. Eight tests pass twice per profile with identical normalized names/results; each compiled production mutant fails a named assertion, exact byte restoration and all eight tests pass after each. Two legacy irony and two legacy roles tests also pass. No full-prag/world suite or Clippy claim.

- old.log: 46bcb284bbab3f8b20e961d717dece7f8476037344fe7fa96d6f4fb63f0d628e

- debug1.log: b8501ec308cd3987361126d40582289f56a077fbacdf9f2aff6bec10153a886d

- debug2.log: 6a9e95190ac487a13b740be2b4199d1a836917aeafe3b27a5aa6471e0e680118

- release1.log: 85bdd13071c004df58a66ccfa36479f9a3aebeeb55db150b6424d09fba788e03

- release2.log: 68b8ca62cf4e7eb5c96f9e2a3bd13bf41461dd12693c70aa2fc0295fb9704b2a

- normalized.txt: 9111df7806a65f5cd8bde5017339ec3093f7c178943ff6d130e2e3c10f935ae6

- legacy-irony.log: aaf5ddfa86926e5d148c5d2eb91b892fedf0b619ac6bea99a2c0dd708cef5d8b

- legacy-roles.log: 1ecd20925e5a7ac5e36c1bcc55d7d0c56997d9bf9a8e8c548498e2615f6abcc0

- mutant-lens.log: 26e0a3b37d7edb65b9193414c011913bfe00c9e3ecc245079bcfc11db48641bf

- restored-lens.log: 6dabf6192b52da9c09b52b58fdcb3aa9566ff07d13f11be13e9e44291b262871

- lens-syntax1.json: 56d266b22e24cd5ccf21b0cd7b24f1fb7ab9dc42072745a75fc2ddfd9c8428ae

- lens-syntax2.json: 56d266b22e24cd5ccf21b0cd7b24f1fb7ab9dc42072745a75fc2ddfd9c8428ae
