# world-types-values

## descriptant

# world-types-values

Pure identity/value operations and their source-bound partial syntax.

## Forward

Dispatch by operation and preserve Rust input/output variants. Parsing uses a restricted decimal grammar; names and displays project values; V::tags computes sorted support. Individual graphs are in model.md#catalog.

## Invariant

No operation changes a world store or validates entity existence. Canonical tag invariants hold for V::tags outputs, not arbitrary public V constructors. Constructor domains exceed parser domains. Strings are byte-exact and case-sensitive.

## Cost

Membership is linear in table size times string comparison cost. Normalization sorts n tags in O(n log n) comparisons then deduplicates. tag_list costs O(n) cloning for Tags. Formatting costs emitted bytes and temporary vector/string allocations. Id accepts at most seven ASCII bytes; is_word at most 32. No timing measured.

## Composition

Identity, vocabulary, routing and values share Reg, Field, Tag and V. key selects Field/Dom then key_ok constrains registry; external payload parsing is a dependency gap. Closed::tag supplies tags, V::tags normalizes, Display projects. Sequential R(x,z)=exists y: R1(x,y) and R2(y,z); reverse retains every compatible y. Parent complement is its own U minus R, not union of child complements. Fragment costs count actual calls once despite shared type dependencies. External language parsing, storage, branch execution and semantic identity remain unmodeled.

## Epoch memory

## catalog

# Typed models

## nonbehavior

U=finite tag vector times optional V. R=V::tags. U minus R forbids Some on empty, None on nonempty and duplicate or unsorted output. Direct V::Tags construction is outside this normalization graph.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in ../model.md#tests. Unsampled inputs remain unproved.

## antecedents

Canonical output support S admits all finite sequences containing each member of S at least once and nothing else; None admits empty only. Display recovery additionally retains variant and segmentation ambiguity.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in ../model.md#tests. Unsampled inputs remain unproved.

## idempotence

For canonical nonempty y, V::tags(y.tag_list())=Some(y). Direct Tags([]) and Tags([one]) are not fixed points; this law cannot extend to every V.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in ../model.md#tests. Unsampled inputs remain unproved.

## unknown-collision

Val default is Unknown displayed ?. Known(Words([?])) has identical display. Text wrappers do not escape payload. Display cannot recover variant identity.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in ../model.md#tests. Unsampled inputs remain unproved.

## word-language

Empty, digit-leading, uppercase, non-ASCII and 33-byte strings fail is_word. Lowercase-leading strings with lowercase/digit/_/- remainder pass through length 32. The 25-word text bound is not enforced here.

Approximation/evidence: exact mathematical target under the stated domain, source-inspected correspondence; bounded executed coverage only as recorded in ../model.md#tests. Unsampled inputs remain unproved.

## History

### 2026-10-29 · Epoch memory

2026-10-29: retained constructor/parser mismatch, Status/Stage alias, key aliases, Closed list's actual space separator and public noncanonical V::Tags. These are model observations; source unchanged. Partial syntax does not expand macros, infer types or prove semantics. Prospective falsification: extend probes to arbitrary custom Closed tables and formatter errors after widening the observation universe.

### 2026-10-29 · document

Subject/revision: world/src/types.rs at manifest SHA-256, epoch 2026-10-29. Each relation observes normal returns on finite typed inputs. Assumptions: fixed source, ordinary allocation, successful formatting sink; no downstream validation. Status: source-derived partial correspondence, not semantic proof. Every formula defines its operator and explicit domain below. For every entry the falsification oracle is a literal input/output comparison with the actual source-included Rust API. Only coverage named in ../model.md#tests was executed; remaining relations are inspected hypotheses.
