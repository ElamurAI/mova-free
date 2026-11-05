# prag-invariants-frame-checks

## forward

sentence visits frames in order with one-based output rows. Missing subject on Fact Is/IsA or non-Bare verb be emits copula-no-who. A time_word place emits time-in-where. Equal nonempty ASCII-insensitive agent/patient IDs emit who-is-what except reflexive patient surface/pronoun or both unresolved bare entities. Checks are diagnostic hypotheses for teacher review.

## invariant

Bare be alone is not a copula. Different/empty entity IDs and recognized reflexives must not receive who-is-what. Multiple checks may flag one row; counts are hits, not distinct bad sentences. Existing time_word semantics distinguish named March from common march.

## cost

O(F) frame traversal plus total entity text/attribute-copy and lexical time_word costs. No corpus parsing, storage or model inference. Output storage O(H) hit records plus detail text.

## composition

Public APIs sentence, Spread::add/counts/hits consume the same ordered frames. Native fixtures construct Tree/Frame/Anchor directly. The papers frames-check caller invokes frames_of_tree and sentence before ctx.new_line, then Spread::add on retained frames; caller corpus/teacher/persistence paths are inspected only.

## epoch-memory

Initial source-bound epoch. No production repair was warranted on the scoped valid-input domain. Independent finite frequency and dependency oracles support the existing smaller-class tie, occurrence counts and anchor exclusions. Malformed cached tree/anchor state, heuristic false positives beyond fixtures and lexical dependencies remain open. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.

## checks

cargo test -p prag --lib invariants:: -- --test-threads=1; the wave12_models submodule contains new finite checks. File ../model.md#evidence binds old witnesses, repeated profiles and compiled production mutations. Syntax projections are partial and macro bodies opaque.

## behavior-complement

Domain: finite inputs satisfying this fragment's forward premises and the parent API tag/state constraints; output is its scoped observation. Let J_fragment(x,y) impose the concrete relations: sentence visits frames in order with one-based output rows. Missing subject on Fact Is/IsA or non-Bare verb be emits copula-no-who. A time_word place emits time-in-where. Equal nonempty ASCII-insensitive agent/patient IDs emit who-is-what except reflexive patient surface/pronoun or both unresolved bare entities. Checks are diagnostic hypotheses for teacher review. Also require: Bare be alone is not a copula. Different/empty entity IDs and recognized reflexives must not receive who-is-what. Multiple checks may flag one row; counts are hits, not distinct bad sentences. Existing time_word semantics distinguish named March from common march. The inner absent-behavior model is {(x,y): not J_fragment(x,y)}. This excludes wrong indices, selections or outputs, rather than treating a scope disclaimer as a behavioral predicate. Finite checks support selected cases; unknown external semantics remain outside this claim.

## reverse-reconstruction

A missing hit admits valid roles and suppressed ambiguous pronouns/reflexives, not necessarily a correct interpretation. A hit constrains kind/roles and displayed identities but cannot recover the original annotation, sentence, discourse state or true referents.

Formally retain every parent-tagged input and compatible intermediate state satisfying the forward relation with the observed fragment output. Do not merge antecedents merely because their projected result agrees; no unique inverse or complete executed solver is claimed.

## intervention

Negative-control partition enumerates 16 kind/verb/subject cases and five identity/reflexive cases. Existing tests separately cover named-time place and common march. A checker flag is not a label of real semantic impossibility.

All typed records bind the exact source revision and scope in manifest.json. Domains, observations and operators are above. Stable dependencies and successful termination are premises. These partial relations retain ambiguous antecedents; no universal solver, unique inverse or actual causal inference is supplied.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. No production repair was warranted on the scoped valid-input domain. Independent finite frequency and dependency oracles support the existing smaller-class tie, occurrence counts and anchor exclusions. Malformed cached tree/anchor state, heuristic false positives beyond fixtures and lexical dependencies remain open. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.
