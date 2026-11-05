# Bounded source-composition evidence

## domain
Owned temporary Unix fixture trees, simple Rust/text/JSON files and explicitly created symlink/socket/oversize/depth controls. Process-specific unique fixture roots avoid overlapping tests. These fixtures are finite observations, not arbitrary source coverage.
## forward
Construct known originals and external replacement files, bind a snapshot, call actual public composer functions and compare observed bytes, metadata, model status and receipts. Mutation fixtures change content, base paths/modes, actions or filesystem kinds and require rejection before publication.
## invariant
Fixtures preserve separate upstream/custom/output roles; Drop removes only their owned temporary tree. Assertions compare exact expected replacement bytes and repeated receipts. Generic hash-matched model fixtures deliberately remain semantically unverified.
## cost
Finite small fixture IO except sparse oversize metadata and depth65 controls. Actual byte reads remain bounded by production limits; no corpus, network or model call.
## behavior-complement
Accepted negative-control outcomes are excluded: traversal, duplicate/colliding targets, symlink/special files, existing output and stale custom/base content must not silently succeed. A failed assertion is a rejected implementation candidate; passing fixtures do not characterize every invalid input.
## reverse-reconstruction
A pass report identifies named fixtures only. It cannot recover arbitrary source trees or prove all possible execution traces. Many implementations agree on these finite observations, including ones wrong on unseen paths.
## oracle-independence
Known literal output bytes, unchanged source snapshots, status categories and filesystem existence are observed outside the specific copying branch. SHA computation is shared with the library and is not independently reimplemented or formally verified. Counterexample: a defect outside tested filesystem names can remain invisible despite fourteen passing fixtures.
## checks
Fourteen native tests in this source. The CLI example separately composes an abstract Whale crate and copies an illustrative Octopus plan; bounded Cargo check/test commands execute only that tiny generated fixture when deliberately invoked during validation.
## epoch-memory

Preserve finite evidence labels; no synthetic fixture is a real Whale deployment, attribution review or training result.

## masked-drift-oracle-partition
Domain: two small native Fixture trees with a fixed replacement returning2; the old original returns1 and the changed original returns3, or an unoverridden README is changed. The tests call actual snapshot/check/stage and compare both the FileBinding map and independent known literal file bytes. In the overwritten case, old exact-base check refuses, old stage creates no directory, and an explicitly rebound synthetic base yields an equal source map but changed provenance. In the README case, rebinding yields a changed output. Thus source-map equality and old-base admission are deliberately different oracles.

The helper edits only owned temporary fixture bindings and is not operational rebase approval. A separate external source mutation suppresses drift rows at overlay targets; the new masked-drift test must fail on that variant. This finite negative control checks the regression's ability to reject a plausible weaker gate, not a claim that production had that defect. Shared hash code is not independently proved; literal byte assertions and preserved differing receipt fields supply additional observations. Fourteen native tests now include these two controls; no corpus, teacher, trainer, model call or live source update is involved.

## History

### 2026-11-01 · epoch-memory

2026-11-01: preserve finite evidence labels; no synthetic fixture is a real Whale deployment, attribution review or training result.
