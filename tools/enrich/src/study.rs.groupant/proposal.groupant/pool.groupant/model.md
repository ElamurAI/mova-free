# Canonical distractor pool

## forward

For each ordered raw tuple t, form map M_t from canonical answers to their last encountered spelling among same-role tuples with a different canonical answer. Emit t only when |M_t| >= 3 and its (source, literal answer, role) key is unseen. Wrong answers are the first three values in canonical key order.

## behavior-complement

Outputs containing fewer than three distinct canonical wrong answers, an answer-equivalent wrong option, or duplicate accepted keys lie outside this relation. Predicate equality is not required, so cross-predicate distractors are not excluded.

## reverse-reconstruction

An emitted wrong triple constrains the smallest three canonical keys and the final representative spelling for each. Arbitrary later larger keys remain compatible. Repeated canonical spellings mean neither source identity nor original pool length is uniquely recoverable.

## coupling

Canonicalization is a quotient on answers; BTreeMap insertion selects the last spelling, while iteration selects lexical key order. For six distinct title-case names all 64 blocked subsets yield n proposals iff n >= 4, otherwise zero; each triple is the first three surviving names other than the answer.

## composition

Raw row count R bounds map construction by O(R^2 log R), with at most sixteen emitted rows but potentially R rejected rows. The test covers distinct ASCII names; Unicode canonical collisions and cap saturation remain untested in this oracle.

## invariant

Forward and reverse observations refer to the same fixed source snapshot and dependency behavior. Ordering is deterministic for fixed inputs; filesystem/environment stability is assumed where relevant. The complement is an inner bound relative to this fragment's input/output universe.

## checks

See the file-level ../../model.md#tests for executed bounded checks and limits. The first three model relations are specifications; tests sample correspondence rather than proving all inputs. No expanded semantic AST or executable general inverse solver exists.

## epoch-memory

Retained the file-level 23 models and added this finer scope. An initial incidence oracle incorrectly treated three-byte fox as a token; the existing tokenizer rejects it. Corrected the independent membership matrix rather than changing production behavior. This is a model correction, not an algorithm defect.

## History

### 2026-10-29 · document

Partial source model, 2026-10-29. The raw Rust file remains authoritative. AST is not materialized; byte ranges are snapshot anchors, not semantic proofs.

### 2026-10-29 · epoch-memory

2026-10-29: retained the file-level 23 models and added this finer scope. An initial incidence oracle incorrectly treated three-byte fox as a token; the existing tokenizer rejects it. Corrected the independent membership matrix rather than changing production behavior. This is a model correction, not an algorithm defect.
