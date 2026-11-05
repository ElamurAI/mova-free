# correspondence-corpus

## descriptant

### Source-bound inventory, typed comparison indexing and two-order finite composition exploration.

Earlier source/model formats are predominantly prose. Missing machine formulas remain gaps. An initial ownership check based only on lexical model paths was strengthened after review: canonical model files must remain within the declared owner sidecar, including symlink targets. The negative control uses an internal foreign-sidecar symlink.

Finite relation structural candidates, finite coincidences and two-order composition summaries are distinct observations. Even an identity on the supplied finite endpoints is not an unrestricted inverse theorem or a production replacement. Effects require an explicit common-state adapter; generic composition of effectful formulas is rejected by the core.

Independent review identified alias multiplication as an output-budget gap: the comparison cap does not constrain same-owner provenance. A separate 100000-edge provenance budget now rejects excess before constructing excess edge JSON; accepted reports remain complete and deterministic.

## model

# Correspondence corpus model

## forward
Let R be a validated unique-ownership registry, S the Git-discovered confined Rust sources, and F a bounded catalogue of typed finite relations. inventory(R,S) binds source bytes and declares coverage. Every registered algorithm implementation source algorithms/src/name/mod.rs enters A; every registered source outside algorithms enters L. Groups are classified by transitive source ownership: all nonalgorithm, mixed, or excluded algorithm-only. Every discovered algorithm implementation must be registered.

Each formula is bound to a known node, one owned descendant source and a model artifact under its declared sidecar. SHA-256 comparisons bind exact bytes. Source symbol and model section are review annotations, not resolved semantic assertions. Duplicate formula IDs reject. Each member of A,L and the two admitted group tiers receives explicit missing-role entries for absent forward, reverse_reconstruction and behavior_complement formulas.

The report counts |A|*|T| opportunities per target tier and four directional role pairs, plus a separate behavior-complement comparison. Actual finite comparisons use exact-contract and endpoint/assumption/effect indices. Their union is deduplicated by formula ID. No dense all-source-pair semantic test is implied. Core compare separates normalized structural candidates, finite coincidences and counterexamples. Core compose_summary explores each admitted pair in both orders, preserving endpoint incompatibility and finite scope. Complement-role entries do not enter reconstruction composition exploration.

## behavior-complement
For the declared one-process universe U of stable readable source snapshots, valid allocations and tagged success/errors, excluded behavior U minus R includes success for stale hashes, unknown owners, duplicate IDs, escaped paths or foreign-sidecar model symlinks; treating missing formulas as matches; collapsing mixed/root groups into pure application groups; reporting a finite candidate as a proof or substitution. Filesystem races, allocation aborts and adversarial concurrent mutation are outside this partial model.

## reverse-reconstruction
Given a report y, Pre(y) consists of every stable repository/registry/catalogue state producing those hashes, ownership classifications, formulas and finite outcomes. Hashes provide practical content identity but no logical uniqueness theorem. Report projections and formula semantics do not reconstruct all source execution histories. A finite identity or injectivity observation concerns only the explicit endpoint labels. Missing formulas preserve unknown behavior rather than selecting an antecedent.

## invariant
Registry ownership is validated before comparisons. Algorithm implementation membership excludes test files and executable approximants. Traversal and report ordering use BTreeMap/BTreeSet and sorted formula IDs. Group-level comparisons do not change ownership or runtime dependencies. All results retain proof=false and production_substitution=false. The canonical model path remains within the lexical declared owner sidecar, rejecting redirected sidecars and foreign nested links.

## cost
Registry/source preflight is O(N+B) metadata/hash work for N sources and B bytes. Descendant group classification can cost O(G*N) in an adversarial ownership tree. Formula validation is bounded by core expression/node/universe caps. Missing-role discovery is O((|A|+|L|+|G|)*F). Indexed candidate enumeration avoids the dense source Cartesian product, but can reach P<=100000 admitted formula pairs. Core finite relation evaluation determines pair cost; each admitted non-complement pair invokes two composition summaries. Output retains comparison and selected category records, so memory is O(B_inventory+F+P*summary_size), with at most 2P composition summaries.

Input files are capped at16MiB; registry/discovered-source counts at100000. Source preflight caps aggregate bytes at256MiB before reused inventory reads; concurrent growth after preflight is outside the stable snapshot assumption. Core limits bound formula count/depth/nodes and endpoint cardinalities. These are resource guards, not asymptotic performance guarantees or percentage accuracy profiles.

## checks

Six focused scanner tests pass: algorithm path membership; unconfined paths; valid source binding and stale source rejection; stale model/unknown owner/foreign model; external and internal-foreign-sidecar symlinks; deterministic full tiny Git scan plus duplicate catalogue rejection. The full fixture checks mixed tier separation and two-order finite composition records. A baseline empty-catalogue CLI scan ran twice with identical bytes:200 algorithm implementations,473 nonalgorithm source nodes,56 pure groups,3 mixed groups and2196 absent typed roles. Existing output reports reject with File exists. Scoped Clippy passes with --no-deps; dependency-inclusive strict Clippy is blocked by existing modelcheck warnings. No training, teacher, production workload or performance gain was measured.

## History

### 2026-10-31 · descriptant

2026-10-31, wave36. This file supports the first bounded typed correspondence scanner. The inventory found 200 algorithm implementations and 473 other registered source groupants at the baseline; 56 pure nonalgorithm groups and three mixed groups are reported separately. Root and mixed groups must not be mislabeled as ordinary application leaves.

2026-10-31, wave37: reverse and complementary gcd records are useful declared roles but their exact converse/complement construction is not independent evidence from forward comparisons. The scanner now exposes all exact forward bases with matching owner, full source binding and contracts. Unknown provenance stays unknown; source semantic evidence remains separate. Recognizing the construction does not prove the source implements it.

### 2026-10-31 · checks

2026-10-31: six focused scanner tests pass: algorithm path membership; unconfined paths; valid source binding and stale source rejection; stale model/unknown owner/foreign model; external and internal-foreign-sidecar symlinks; deterministic full tiny Git scan plus duplicate catalogue rejection. The full fixture checks mixed tier separation and two-order finite composition records. A baseline empty-catalogue CLI scan ran twice with identical bytes:200 algorithm implementations,473 nonalgorithm source nodes,56 pure groups,3 mixed groups and2196 absent typed roles. Existing output reports reject with File exists. Scoped Clippy passes with --no-deps; dependency-inclusive strict Clippy is blocked by existing modelcheck warnings. No training, teacher, production workload or performance gain was measured.

### 2026-10-31 · document

#### Declared derivation provenance (2026-10-31, wave37)

Prospective model: preserve existing coverage and comparison outcomes while exposing construction dependencies. For each reverse record, recognize only an exact top-level Converse of a declared forward expression; for each complementary record, recognize only an exact top-level Complement. Require identical owner and full source binding (path, hash, symbol), identical observation/assumptions/effects, and exact swapped/preserved endpoint sorts including coordinate order. Model artifact locators may differ: they describe the separate roles. Retain all matching forward IDs as sorted derivation edges. Count derived formulas separately from edges so aliases never inflate the formula count.

The accounting describes declared AST construction, not semantic proof or independent source verification. No normalization, atom-name inference, role-name inference or finite equality establishes an edge. Unmatched nonforward records remain not_recognized; forward records remain declared_forward. Source evidence stays separate. Existing role coverage, comparisons and composition opportunities retain their prior meanings and counts.

Additional behavior exclusions: reporting recognized reverse/complement records as independent evidence from forward comparisons; selecting an arbitrary base from several exact bases; interpreting not_recognized as independently verified. Reverse preimages of a provenance report include every catalogue with the reported exact dependencies and unresolved records; the report cannot recover source semantics or certify independent origins.

For F formulas and maximum expression/contract comparison size E, this conservative recognition adds O(F^2*E) worst-case comparison work and O(min(F^2,100000)) edge storage before rejection. A separate total derivation-edge cap of 100000 matches MAX_CANDIDATES; the formula cap alone does not constrain alias-product output sufficiently. It does not clone full contracts or expressions per candidate. BTree ordering makes input permutations deterministic. No performance gain is claimed.

Measured 2026-10-31: all 11 focused scanner tests passed in debug and release (six retained, five provenance tests). The real catalogue gcd fixture recognizes four derived formulas: two converse and two complement, with four edges. Controls reject same atom names with altered pairs, different source path/hash/symbol, owner, assumptions, effects, observation, coordinate order and endpoints, and unrecognized nonforward ASTs including a singleton union around a converse. Alias bases retain two sorted edges for one derived formula; input permutation produces identical provenance. The existing full tiny scan still verifies coverage and comparison/composition counts and determinism. Scoped Clippy with --no-deps and -D warnings, rustfmt, changed-file registration and diff whitespace checks pass. Source syntax was regenerated with mova-modelcheck. No whole-corpus source-equivalence claim or performance measurement is made.

Review correction, 2026-10-31: a catalogue can multiply forward and derived aliases without entering any algorithm/application comparison. Provenance therefore enforces its own total edge budget before inserting each edge or building that formula's JSON. Exceeding the budget returns an error through scan before comparison enumeration. Accepted reports retain every exact sorted edge; no truncation or arbitrary-base choice occurs. A small four-edge fixture checks exact-boundary acceptance, rejection at budgets zero through three, zero-edge acceptance and permutation determinism. This resource guard does not prove semantic correspondence.

Budget follow-up measurements, 2026-10-31: 12 scanner tests pass in debug and release. Strict scoped Clippy (--no-deps, -D warnings), rustfmt, changed-file registration and diff whitespace checks pass. The new boundary test exercises accumulation across two derived formulas and two exact forward aliases without a large fixture. Syntax and source hash are refreshed.
