# retention-material

## model

# Retained material reuse and allocation

## forward

For fixed retained course and ordered ledger L, define d(l,n)=H(JSON(course,n,explain,facts,exercises)). retained_lesson rejects course mismatch, otherwise returns min{n in keys(L) | L[n]=d(l,n)}, or None. next_lesson_number returns checked(max(keys(L) union {0})+1), unless reserved.

## behavior-complement

Type: behavior_complement. Domain and observation are the forward relation above unless narrowed here. U is state/lesson/result tuples for reuse, or ledger/result tuples for allocation. R is the ordered-match/successor relation. An inner bound U minus R includes returning a larger matching key, reusing across courses, allocating a lower hole, or successfully returning KINDS_SRC/overflow. Assessment changes alone cannot alter d with other fields fixed.

## reverse-reconstruction

Type: reverse_reconstruction. Domain and observation are the forward relation above unless narrowed here. Some(n) admits all candidate lessons whose teaching tuple hashes to L[n] and whose earlier keys do not match; placeholder l.n and metadata omitted from the teaching tuple, including assessment questions and labels, are unconstrained by d; exercise questions, right answers and wrong-option order remain digest inputs. None requires no matches. A successful allocation k admits ledgers with max-or-zero=k-1 and arbitrary lower membership/digests, subject to reserved/overflow gates.

## identity-projection

Type: identity_partition. Domain and observation are the forward relation above unless narrowed here. Reuse substitutes each ledger key n into the teaching identity; it does not compare the staging placeholder l.n. Title/test/exam changes preserve the digest input. Facts, exercises and course remain identity inputs. Equality of hash inputs implies equality of hashes; converse requires a collision assumption.

## frontier

Type: allocation_boundary. Domain and observation are the forward relation above unless narrowed here. For an empty ledger or {0}, allocation is 1. A sparse ledger {2,7} gives 8, not 1. max=usize::MAX fails checked_add; successor=KINDS_SRC fails the reserved gate. This function does not validate every existing key or guarantee its successor fits downstream policies.

## scan-cost

Type: resource_bound. Domain and observation are the forward relation above unless narrowed here. For m ledger entries and serialized teaching size B, worst-case reuse serializes/hashes at every candidate, O(mB), with early exit at the least match. BTreeMap ordering supplies the priority, not insertion order. Prospective optimization would cache invariant tuple parts but must preserve n-dependent hashes; none is implemented.

## invariant

Reuse is read-only and does not allocate. Allocation is a separate query; no atomic reservation is implied. Parent wiring must decide whether to reuse or allocate, and this local fragment does not claim to model every caller.

## checks

oracle_reuse_fiber_selects_smallest_of_multiple_antecedents independently builds identity tuples and checks 3/7/19, placeholders, facts/course changes. oracle_allocation_frontier_ignores_holes_and_rejects_boundaries checks five literal successes and reserved/overflow errors. No parser or learning runs are required.

Shared evidence is in ../model.md#tests. Referenced older mutation controls are historical evidence, not rerun here. General semantic correspondence and unsampled behavior remain unknown.

## epoch-memory

Decomposition preserves the twenty file-level models and makes this observation boundary explicit. The prospective expectation is that all twelve existing focused CPU retention tests remain green because Rust is unchanged. Executed results are recorded in the root ../model.md#tests after checking. AST is not materialized for this range; no syntax selection, expanded AST, inverse solver or automatic combiner is supplied.

## wave34-composition

### Wave34 composition

Reuse and frontier are independent read-only queries sharing the ledger. Reuse first rejects course mismatch then returns the least matching substituted-key digest. Frontier ignores values and chooses checked maximum successor, rejects the reserved value, and does not reserve it. Caller control flow decides which query to use; no unconditional reuse-then-allocation transition is claimed.

Reverse composition preserves compatible intermediate values and their shared ledger/course/cache constraints. Parent behavior complement is relative to its own coupled trace universe, not a union of child complements. Costs include each executed call once. Existing model.md/composition.md retain historical models and evidence.

## History

### 2026-10-29 · epoch-memory

2026-10-29: decomposition preserves the twenty file-level models and makes this observation boundary explicit. The prospective expectation is that all twelve existing focused CPU retention tests remain green because Rust is unchanged. Executed results are recorded in the root ../model.md#tests after checking. AST is not materialized for this range; no syntax selection, expanded AST, inverse solver or automatic combiner is supplied.

### Undated · document

Subject: retention.rs, SHA-256 89715719216c1a94f9e701e24d4efea0b962bf04f5178d4916d19dc54536fb34. The manifest records the exact source interval. These source-derived relations are partial specifications; their evidence is bounded.
