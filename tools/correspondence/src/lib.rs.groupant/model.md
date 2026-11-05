# correspondence-core

## descriptant

### Relation matching epoch36

The core distinguishes source identity, formula equality and finite-table equality. Exact matrix arithmetic provides small exhaustive witnesses while every Rust correspondence remains a separate obligation. Named endpoint theories prevent same-label Rational/Field false matches; converse and complement are different operators.

Useful findings: gcd implementations can share Euclidean recurrences; public big-number gcd may differ at zero-zero; Horner scalar value projection can succeed when derivative overflow rejects the algorithm pair; functional graph error observations must retain tails; assignment optimum differs from greedy matching; Gaussian elimination shares a field schema without cross-field solution equivalence.

Rejected shortcut: allow arbitrary nonempty effects to compose because strings agree. Independent relations can hide shared state; current core requires pure effects and equal observations. Rejected negative-test fixture: identity then empty over two values did not refute complement composition. The corrected identity-identity witness does. Preserve these constraints until an explicit typed state/observation adapter is implemented.

No automatic production substitution, learned numerical surrogate, unbounded formula prover, source-semantic extraction or general alpha-renaming is claimed. Canonical relation normalization is a small sound rule set, not a complete decision procedure for algebraic equality. Atomic model definitions carry exact finite tables and source/model provenance through the caller.

#### Epoch37

Canonicalization now covers composition associativity, converse/complement commutation and converse over union/intersection. The key design constraint was bounded output: right-associating chains could turn an accepted balanced AST into an over-depth chain, so chains use a deterministic minimum-height alphabetic tree whose height never exceeds the original association. A pre-existing defect surfaced: pushing converse through composition (already done before this epoch) could expand a valid 4,096-node converse chain to 6,143 nodes, so canonical JSON failed validation. The bounded no-push mode fixes this. Composition summaries gained totality and surjectivity with explicit vacuous empty-set behavior. Normalization remains a sound, incomplete rule set; complement over union/intersection (De Morgan) and Boolean lattice laws are not used.

## model

# Typed finite relation core

## Forward

Validate bounded formula metadata, declared named finite endpoint sorts, atom coordinates, atom identity consistency, recursive endpoint compatibility and root contract. Normalize only relation identities: double converse, converse of first-then composition, converse over union/intersection, converse/complement commutation on the full rectangle (converse placed innermost on atoms), identity units, composition associativity, double full-rectangle complement and associative/commutative/idempotent union/intersection. Evaluate exact finite Boolean matrices. Compare requires equal full contracts; return normalized_formula_match, finite_coincidence, finite_counterexample or incompatible_contract. Canonical keys omit contract intentionally: callers must index both.

Composition chains are flattened into ordered factor lists and rebuilt as the deterministic minimum-height alphabetic binary tree: the greedy maximal left part fitting height b-1 (memoized reach over factor heights). Any accepted association is one alphabetic tree over the same factors, so canonical height never exceeds the original and all associations of the same factor list share one tree. Pushing converse onto atoms can multiply converse nodes (a converse over an n-factor chain yields n converse nodes); when the fully pushed form exceeds MAX_NODES, a bounded mode that only cancels double converse is used. Both modes never increase height, the bounded mode never increases node count, and canonical JSON revalidates.

Laws reused from formal/RelationComposition.lean: composition_associative, converse_composition, converse_involution, identity_left/right and complement_double_scope (full rectangle). Converse/complement commutation and converse over union/intersection have finite checks only; no Lean theorem covers them yet. Not used: inverse cancellation (R;R° = Id), complement distribution over composition or union.

Finite atom tables are declared input models. Source/model hashes identify external artifacts but only the scanner checks them. The core does not extract or prove Rust behavior. Free atom names retain identity; bound intermediate states are existential composition coordinates, not textual variables. No arbitrary prose or unsupported numeric syntax is parsed.

## Invariant

Every admitted expression has endpoints matching its contract; composition preserves intermediate type identity, including named numeric theory and ordered labels. All matrix bits stay inside the codomain. Successful normalization preserves declared finite relation semantics, is idempotent, keeps atom names/tables, does not increase height and stays within MAX_DEPTH/MAX_NODES. Compare never turns finite observations into universal source equality. Lexicographically first differing pair is deterministic. Same-name atoms cannot bind conflicting tables inside one formula.

## Cost

At most 64 values per endpoint, depth32, nodes4096; at most4096 atom pairs. Matrix relation work is O(nodes*64^2), temporary matrices O(depth*64) excluding owned AST and normalized copies. Normalization uses ordered sets for union/intersection and a reach memo of n*(maxheight+64) entries per chain (n<=2048 factors), O(n*height) states; the full and bounded passes may both run for oversized converse expansions. Validation and structural ordering also traverse stored labels/metadata. Wide AST allocation has already happened before this API: callers must bound JSON input size/deserialization. Source IO and whole-catalog pair enumeration belong to the scanner. Test-suite timing (0.2 s release, 1.0 s debug for fifteen tests) is not a production measurement or speedup claim.

## Composition

compose_summary executes left then right only when intermediate Sort, observation, assumptions and effects agree; effects must be empty. Stateful models require an explicit common-state adapter and remain unsupported. Properties concern only the finite composite: cardinality; functional = univalent (at most one output per input); injective = converse univalent; total = every domain value has an output (vacuously true on an empty domain); surjective = every codomain value is hit (vacuously true on an empty codomain); identity. A nonempty domain with an empty codomain is not total but is surjective. An incompatible contract reports every property false. A noninjective function composed with its converse yields equal-output fibers, not identity. Empty universes and zero-width matrices are valid; 64-bit masks avoid shifting by64.

## nonbehavior

Type: behavior_complement. For a declared finite rectangle U=X×Y and finite relation R, complement(R) evaluates exactly U minus R. This says nothing about the complement of real Rust behavior when the atom table is only an observed projection. Invalid sorts, coordinates, atom definitions, budgets, metadata and typed composition are rejected before evaluation. Error observations are not inferred from missing atom rows.

## antecedents

Type: reverse_reconstruction. Converse(R)(y,x) iff R(x,y), preserving every compatible antecedent in the declared finite table. Endpoints swap. Converse of first-then composition reverses traversal; converse commutes with full-rectangle complement because the converse of X×Y is Y×X. Full reconstruction is guaranteed only relative to the table; omitted real inputs remain unknown. No single-valued inverse cancellation is implemented. Empty preimages prove absence only within this declared finite model.

## Checks

Fifteen focused unit tests. Earlier eight cover all256 pairs of Boolean relations for converse normalization, all512 rectangular relation pairs for reversed endpoints, noninjective fibers, rejected complement composition, distinct Rational/Field theories, effect and intermediate-type mismatches, free-symbol identity, malformed nodes/budgets, conflicting atom bindings, empty endpoints and64-bit masks. Added seven: 1,280 relation triples, each with four pairs (three-factor association, converse of a chain, converse/complement, converse/union; 5,120 comparisons), reach normalized_formula_match, and all five four-factor associations give one key; 256 relation pairs keep inverse cancellation, complement distribution and wrong De Morgan nonstructural, with finite counterexamples for each; 3,000 seeded typed random ASTs over 2/3/1-value sorts satisfy finite equality, idempotence, nonincreasing height and revalidation of canonical JSON; a depth-32 left-deep chain with an irreducible height-19 factor rebalances below the limit and equals its right-deep association; a 4,096-node converse chain uses the bounded mode idempotently; canonical JSON revalidates at that boundary; totality/surjectivity cover partial, constant, bijection round trip, empty domain, empty codomain and incompatible contracts.

## History

### 2026-10-31 · Checks

On 2026-10-31 the boundary test failed on the previous normalizer (canonical output rejected as over the node budget) and passes now. Four source mutants each failed tests: right-deep rebuild (4 tests), no bounded mode (2), total with any (1), converse chain without reversal (3). Final runs: debug twice and release twice, 15/15 each, identical sorted test identities. Older note: an initial complement control used identity then empty and accidentally satisfied the false law; the identity-identity witness replaced it. No theorem prover run, scanner/bridge execution or whole-Mova regression is claimed.

### 2026-11-01 · descriptant

#### Paired diagnostic memory — 2026-11-01

A forward normalization vidtvorant must track represented relation and admitted syntax resources as separate observations. Resource-boundary reasoning exposed a valid 4,096-node input whose converse distribution produced 6,143 nodes: semantic equality did not imply output-budget closure. The bounded fallback repairs that representation defect; native node/depth and idempotence witnesses support the scoped claim. This is distinct from the finite behavior-complement and converse tables, whose matching constructions are not independent source evidence. Preserve incompleteness and the fixed budget when composing with serialization or scanning. The [paired-review account](../../../../docs/mathematical-modeling.md) reuses the recorded wave37 evidence without a new test run.
