# prag-irony

## model

# prag-irony

## forward

Domain X is finite supplied (Word sequence W, raw cue text T) under fixed lexicons. Situation selection yields disjoint negative spans; polar sequence P follows token order, skipping ADP/SCONJ lexical sentiment and span interiors, flipping recognized lexical polarity under negation except cannot-wait. Let I and E be existence flags for the first implicit and explicit eligible pairs, C the distinct cue-kind list, and s=3I+2E+|C|. Return None iff neither pair exists or s<THRESHOLD; otherwise return s, features [implicit if I,explicit if E] followed by C, implicit pair if I else explicit pair, counts of P signs, adjacent sign flips, maximum sign-run length and positive-minus-negative. Invalid parent links supply no ancestry or ended-parent evidence; linear explicit precedence remains usable.

## invariant

Candidate implies I or E, score=3I+2E+cue-kind count and score>=2. Positive+negative=|P|, polarity=positive-negative, flips count adjacent sign changes, longest_run is zero for empty P. Cue kinds occur once in fixed order. No candidate from cues alone. No out-of-range dependency parent is indexed by ancestry/situation checks; a cycle terminates at the bounded walk limit. Pair choice and feature flags must agree.

## cost

Conservative O(N^3) for candidate pairs times bounded ancestry/contrast scans, plus fixed-seed situation scans and raw-text normalization/cue scans. O(N) polar/span temporary entries plus string/output bytes. This is a prospective source cost, not timing or accuracy evidence.

## composition

Native supplied Words/text -> irony executes. CLI consumes candidates; hand-authored labels are finite assertions, not held-out assessment material.

## epoch-memory

Initial source-bound epoch. Irony malformed-head native witness panicked before bounds repair. Missing parent evidence now abstains while linear evidence remains. Lens/roles production unchanged. Costs are prospective asymptotic models; no optimization, percentage guarantee or timing claim.

Documentation audit: root relations now retain tagged API inputs, concrete observation constraints and compatible intermediates. Runtime source, syntax and existing test evidence are unchanged.

## checks

cargo test -p prag --lib irony::wave12_models -- --test-threads=1. File-root model.md#evidence binds repeated profiles and mutants. No live parser/teacher/corpus or persisted exam evaluation.

## behavior-complement

For the tagged input domain X above and matching output schema Y, let J(x,y) be the concrete forward/invariant constraints above. C_inner={(x,y) in X×Y | not J(x,y)} is a source-derived inner model of absent behavior. In particular exclude candidate outputs that violate: Candidate implies I or E, score=3I+2E+cue-kind count and score>=2. Positive+negative=|P|, polarity=positive-negative, flips count adjacent sign changes, longest_run is zero for empty P. Cue kinds occur once in fixed order. No candidate from cues alone. No out-of-range dependency parent is indexed by ancestry/situation checks; a cycle terminates at the bounded walk limit. Pair choice and feature flags must agree. The native finite checks support selected members/nonmembers; no general Rust correspondence proof or claim about malformed state outside X is made.

## reverse-reconstruction

B(y) retains all (W,T,P,situations,I,E,C,pair) satisfying the forward relations and observed fields, with W and T separately tagged because text is used for cues independently of annotations. None admits no sentiment, same-sign sentiment, contrast-excluded pairs and other absent-pair cases. Same score admits distinct flag/cue combinations; full candidate still loses unused words, head alternatives and raw cue spelling. Invalid-parent abstention does not identify whether the intended tree would have supported ancestry.

## coupling

Native supplied Words/text -> irony executes. CLI consumes candidates; hand-authored labels are finite assertions, not held-out assessment material.

Typed entries bind the manifest revision, stated domain and observation, assuming stable compiled dependencies and bounded execution. Status is partial source-derived relation with finite named checks. Partial syntax leaves macro bodies opaque and is no semantic proof.

## Epoch notes

## evidence

### Native evidence

Old-code witness invalid_heads_abstain_from_ancestry failed with an index panic (head 3 on two words). Repair ignores invalid parent evidence while retaining linear explicit incongruity. Ctx summaries are temporary document state, not persistent lesson-to-exam learning. Lexical test requires the compiled expression snapshot without silent skip. Fixture expectations are hand-authored independently of output.

Production control: Explicit score increment two to one: finite_polarity_and_cue_oracle rejects changed score.

Three file roots and four fragments declare 21 typed views. Syntax generation repeats byte-identically; structural checks validate bindings and bounds, not semantics. Compiled dictionary and frame dependencies are premises. No live parser, teacher, corpus, GPU, held-out run or performance claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. Irony malformed-head native witness panicked before bounds repair. Missing parent evidence now abstains while linear evidence remains. Lens/roles production unchanged. Costs are prospective asymptotic models; no optimization, percentage guarantee or timing claim.

Documentation audit, 2026-10-30: root relations now retain tagged API inputs, concrete observation constraints and compatible intermediates. Runtime source, syntax and existing test evidence are unchanged.

### 2026-10-30 · evidence

Heading `### Native evidence, 2026-10-30` stays in evidence as `### Native evidence`.

Source SHA-256: 220c528abc6a7f0daff9737377444e13ccff8f3acd5fddcd0b86c0b373b8e19c. Filter: cargo test -p prag --lib wave12_models -- --test-threads=1, with --release for release. Eight tests pass twice per profile with identical normalized names/results; each compiled production mutant fails a named assertion, exact byte restoration and all eight tests pass after each. Two legacy irony and two legacy roles tests also pass. No full-prag/world suite or Clippy claim.

- old.log: 46bcb284bbab3f8b20e961d717dece7f8476037344fe7fa96d6f4fb63f0d628e

- debug1.log: b8501ec308cd3987361126d40582289f56a077fbacdf9f2aff6bec10153a886d

- debug2.log: 6a9e95190ac487a13b740be2b4199d1a836917aeafe3b27a5aa6471e0e680118

- release1.log: 85bdd13071c004df58a66ccfa36479f9a3aebeeb55db150b6424d09fba788e03

- release2.log: 68b8ca62cf4e7eb5c96f9e2a3bd13bf41461dd12693c70aa2fc0295fb9704b2a

- normalized.txt: 9111df7806a65f5cd8bde5017339ec3093f7c178943ff6d130e2e3c10f935ae6

- legacy-irony.log: aaf5ddfa86926e5d148c5d2eb91b892fedf0b619ac6bea99a2c0dd708cef5d8b

- legacy-roles.log: 1ecd20925e5a7ac5e36c1bcc55d7d0c56997d9bf9a8e8c548498e2615f6abcc0

- mutant-irony.log: 2cdda599b6e8b35868705332d42f9b6de355245e0daa0b049179bd23801fa682

- restored-irony.log: 2bf9ddee0055233b97ce71ba7ccf3525a267947205750ed72ef96c477ff6740b

- irony-syntax1.json: 46b8b4e1c34ce28cf67c6e6bfdfffb836ca8cd721a029b8e038397daed4cd51e

- irony-syntax2.json: 46b8b4e1c34ce28cf67c6e6bfdfffb836ca8cd721a029b8e038397daed4cd51e

### 2026-11-04 · Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.
