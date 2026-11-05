# Literal proposal generation

## forward

Let L(P,B) be the ordered raw rows from unblocked sentences satisfying ten-column token shape, nsubj/obj noun/proper-noun, verb head, unique literal spelling and alphanumeric boundaries. Forward output is take16(Pool(L(P,B))). This models token clozes, not semantic entailment.

## behavior-complement

In the universe of packet/blocklist/output triples, any output with a blocked source, invented literal answer or more than sixteen items is outside the generator relation. This inner complement does not classify every malformed parse.

## reverse-reconstruction

Given output Q, compatible inputs include exactly packets and blocklists whose ordered accepted raw rows yield Q after pooling/capping. Unemitted rows and blocked sentences remain ambiguous; empty output does not identify an empty packet.

## coupling

Blocking precedes the global pool: Pool(L(P,B)) = Pool(L(P without B,empty)). Removing a source can remove distractors for other sources, so filtering final outputs is not equivalent. The 64-subset oracle checks that coupling.

## composition

The pool child consumes only accepted raw rows in original sentence/token order. Parent adds literal eligibility, source exclusion and the sixteen-result stop. Shared canonicalization is assumed; the child cost already includes its raw scans, so do not count them twice. Parsing plus head searches costs O(sum tokens^2); pooling is O(R^2 log R) plus string work.

## invariant

Forward and reverse observations refer to the same fixed source snapshot and dependency behavior. Ordering is deterministic for fixed inputs; filesystem/environment stability is assumed where relevant. The complement is an inner bound relative to this fragment's input/output universe.

## checks

See the file-level ../model.md#tests for executed bounded checks and limits. The first three model relations are specifications; tests sample correspondence rather than proving all inputs. No expanded semantic AST or executable general inverse solver exists.

## epoch-memory

Retained the file-level 23 models and added this finer scope. An initial incidence oracle incorrectly treated three-byte fox as a token; the existing tokenizer rejects it. Corrected the independent membership matrix rather than changing production behavior. This is a model correction, not an algorithm defect.

## History

### 2026-10-29 · document

Partial source model, 2026-10-29. The raw Rust file remains authoritative. AST is not materialized; byte ranges are snapshot anchors, not semantic proofs.

### 2026-10-29 · epoch-memory

2026-10-29: retained the file-level 23 models and added this finer scope. An initial incidence oracle incorrectly treated three-byte fox as a token; the existing tokenizer rejects it. Corrected the independent membership matrix rather than changing production behavior. This is a model correction, not an algorithm defect.
