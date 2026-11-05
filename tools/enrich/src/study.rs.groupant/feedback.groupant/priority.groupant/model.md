# Weak feedback incidence

## forward

Define W_i = !(correct_i AND support_i), I_iw = source_i in sources_w OR headword_w in tokens(question_i), and H_w=sum_i W_i I_iw. tokens keeps alphabetic pieces whose UTF-8 byte length exceeds three, then lowercases. Sort rows by (-H_w, known_w, headword_w); preserve source vectors.

## behavior-complement

A row whose hit count exceeds the number of weak feedback items, double-counts source and token membership, or depends on understood lies outside this relation. Unknown dictionary words at equal hits precede known words.

## reverse-reconstruction

For observed hit vector H and fixed incidence matrix I, compatible weak masks are all binary W with I-transpose W=H. Every zero W position requires correct=support=true; every one allows the other three flag pairs. Projection commonly has multiple antecedents.

## coupling

The bounded independent incidence oracle enumerates all 64 correctness/support assignments for three feedback items and both uniform understood values: 128 executions. Short fox is excluded from token-only matching but admitted by its source; repeated source IDs and overlapping source/token matches contribute at most once per item.

## composition

For n words and m feedback items, current code repeatedly tokenizes questions per membership check: cost includes O(n*m*question_bytes), source-vector lookup and O(n log n) sorting. Pre-indexing is a prospective optimization, not implemented.

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
