# prag-roles

## model

# prag-roles

## forward

Domain X is tagged calls roles(W), roles_docs(S), world_docs(S,b), doc_key(id), with finite supplied annotations and fixed frame-reader dependencies. For documents, pi is stable sort by doc_key; C resets at document-key changes. At each pi(i), world_docs first emits summary(C,6) plus bounded prior event queue, then frames_of_tree updates C, event/speech verb frames append event strings and queue truncates to last b, then C.new_line. roles_docs reads in the same context order, projects frames, then restores original input row order. Projection chooses first unused eligible predicate token matching first verb lemma component and closest nonpredicate matching slot token (earlier tie), preserving Ent.pron for primary slots; sort/deduplicate triples.

## invariant

Role triples (d+1,p+1,role) are sorted unique and within supplied token positions, and no predicate is reused across selected frames. roles_docs/world_docs preserve original ID sequence including duplicates. Context and recent queue never cross document-key transitions. world_docs observation is before the current frame update; b=0 gives 'none yet' for recent events. doc_key parsing and token choice use the child relations, not semantic identity guarantees.

## cost

For S sentences, ordering O(S log S), plus annotation cloning and frame-reader/context-summary dependency costs. Per sentence F frames and N tokens: O(FN) scans per fixed primary slot, plus attributes/obliques and O(R log R) emitted-tuple sorting. Recent queue stores O(b) event strings; output summary bytes add their own cost.

## composition

Native Word -> Tree::of_words -> frames_of_tree -> roles/world_docs executes. CLI annotation/cache paths inspected only; temporary Ctx is not retained learned state.

## epoch-memory

Initial source-bound epoch. Irony malformed-head native witness panicked before bounds repair. Missing parent evidence now abstains while linear evidence remains. Lens/roles production unchanged. Costs are prospective asymptotic models; no optimization, percentage guarantee or timing claim.

Documentation audit: root relations now retain tagged API inputs, concrete observation constraints and compatible intermediates. Runtime source, syntax and existing test evidence are unchanged.

## checks

cargo test -p prag --lib roles::wave12_models -- --test-threads=1. File-root model.md#evidence binds repeated profiles and mutants. No live parser/teacher/corpus or persisted exam evaluation.

## behavior-complement

For the tagged input domain X above and matching output schema Y, let J(x,y) be the concrete forward/invariant constraints above. C_inner={(x,y) in X×Y | not J(x,y)} is a source-derived inner model of absent behavior. In particular exclude candidate outputs that violate: Role triples (d+1,p+1,role) are sorted unique and within supplied token positions, and no predicate is reused across selected frames. roles_docs/world_docs preserve original ID sequence including duplicates. Context and recent queue never cross document-key transitions. world_docs observation is before the current frame update; b=0 gives 'none yet' for recent events. doc_key parsing and token choice use the child relations, not semantic identity guarantees. The native finite checks support selected members/nonmembers; no general Rust correspondence proof or claim about malformed state outside X is made.

## reverse-reconstruction

B(tag,y) retains all supplied IDs/annotations, stable ordering pi, initial/reset contexts, frame sequences and queue histories satisfying the child transitions and the observed y. roles(W) fixes initial C=default; document APIs fix per-key resets. Tuple deduplication loses repeated emissions; lexical matching loses unmatched fillers and competing parses. Summaries lose entities beyond six and event prefixes beyond b; b=0 loses every event string. Equal parsed document keys, duplicate IDs and original vs sorted row order remain distinct tagged antecedents.

## coupling

Native Word -> Tree::of_words -> frames_of_tree -> roles/world_docs executes. CLI annotation/cache paths inspected only; temporary Ctx is not retained learned state.

Typed entries bind the manifest revision, stated domain and observation, assuming stable compiled dependencies and bounded execution. Status is partial source-derived relation with finite named checks. Partial syntax leaves macro bodies opaque and is no semantic proof.

## evidence

### Native evidence

Old-code witness invalid_heads_abstain_from_ancestry failed with an index panic (head 3 on two words). Repair ignores invalid parent evidence while retaining linear explicit incongruity. Ctx summaries are temporary document state, not persistent lesson-to-exam learning. Lexical test requires the compiled expression snapshot without silent skip. Fixture expectations are hand-authored independently of output.

Production control: Drop queue entries at >= max(events,1): prior_world_retention_and_document_reset rejects premature eviction.

Three file roots and four fragments declare 21 typed views. Syntax generation repeats byte-identically; structural checks validate bindings and bounds, not semantics. Compiled dictionary and frame dependencies are premises. No live parser, teacher, corpus, GPU, held-out run or performance claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. Irony malformed-head native witness panicked before bounds repair. Missing parent evidence now abstains while linear evidence remains. Lens/roles production unchanged. Costs are prospective asymptotic models; no optimization, percentage guarantee or timing claim.

Documentation audit, 2026-10-30: root relations now retain tagged API inputs, concrete observation constraints and compatible intermediates. Runtime source, syntax and existing test evidence are unchanged.

### 2026-10-30 · evidence

Heading `### Native evidence, 2026-10-30` stays in evidence as `### Native evidence`.

Source SHA-256: 30e3aba0c3fba51e326a90fb8da79fa28c009d58c10742100aa462a75d41a5df. Filter: cargo test -p prag --lib wave12_models -- --test-threads=1, with --release for release. Eight tests pass twice per profile with identical normalized names/results; each compiled production mutant fails a named assertion, exact byte restoration and all eight tests pass after each. Two legacy irony and two legacy roles tests also pass. No full-prag/world suite or Clippy claim.

- old.log: 46bcb284bbab3f8b20e961d717dece7f8476037344fe7fa96d6f4fb63f0d628e

- debug1.log: b8501ec308cd3987361126d40582289f56a077fbacdf9f2aff6bec10153a886d

- debug2.log: 6a9e95190ac487a13b740be2b4199d1a836917aeafe3b27a5aa6471e0e680118

- release1.log: 85bdd13071c004df58a66ccfa36479f9a3aebeeb55db150b6424d09fba788e03

- release2.log: 68b8ca62cf4e7eb5c96f9e2a3bd13bf41461dd12693c70aa2fc0295fb9704b2a

- normalized.txt: 9111df7806a65f5cd8bde5017339ec3093f7c178943ff6d130e2e3c10f935ae6

- legacy-irony.log: aaf5ddfa86926e5d148c5d2eb91b892fedf0b619ac6bea99a2c0dd708cef5d8b

- legacy-roles.log: 1ecd20925e5a7ac5e36c1bcc55d7d0c56997d9bf9a8e8c548498e2615f6abcc0

- mutant-roles.log: 544e5460896556a96dc1c567feebc506aa16487c9227fed14db9707f93e7c7e0

- restored-roles.log: 5f57f0f5da0f7d546d5f4bea261d28828c1f467d714cb150ea190f4f43b6ab5d

- roles-syntax1.json: b561c5979def851294ca601d107821f67033b3fe05e8a7dae04eca46c2342400

- roles-syntax2.json: b561c5979def851294ca601d107821f67033b3fe05e8a7dae04eca46c2342400
