# placement-metadata

## descriptant

### Placement descriptant

Existing Rust tests separate durable recommendation, usefulness eligibility and controversy. The standalone probe compares the complete finite relation, complement and converse against source. No persistence or RAM manager is implemented. Future revisions must refresh hashes and reconsider fibers. syntax.json stores a partial source-bound AST projection; semantic analysis remains absent.

## catalog

# Typed model catalog

Finite formulas are exact on the 48-value metadata domain by source inspection and bounded probe; they do not extend to storage. Counterfactual entries define changed rules. Proposed oracles remain unexecuted unless model.md#tests records them. Cost is symbolic and unmeasured.

## nonbehavior

Type: behavior_complement. Domain/observation: C x D single-call transitions.

Operator and approximation: B={(c,d):d != (1,i,(s!=Instance) AND m AND r AND NOT t,t)} = U minus R. Exact finite metadata complement; inner bound only if embedded into wider effects.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Enumerate 48 inputs and 16 decisions; exactly 48 admitted and 720 absent pairs.

## antecedents

Type: reverse_reconstruction. Domain/observation: one complete decision d.

Operator and approximation: F^-1(d)={c:g=1,i=h,t=a,k=((s!=Instance) AND m AND r AND NOT t)}. Empty if g=0 or k AND a; retain all scopes/flags satisfying the equation.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Enumerate fibers and round-trip every member; no unique inverse.

## output-image

Type: reachable_image. Domain/observation: all 16 Boolean decisions.

Operator and approximation: Image(F)={d:g=1 AND NOT(k AND a)}: six distinct decisions; residency h is free.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Enumerate image; reject ten other decisions.

## compile-fiber

Type: reverse_reconstruction. Domain/observation: decisions with k=1.

Operator and approximation: Compatible c have s in {General,Domain}, m=r=1,t=0,i=h. Each full output has two antecedents.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Enumerate both positive skeleton fibers.

## cold-fiber

Type: cardinality_preimage. Domain/observation: d=(1,0,0,0).

Operator and approximation: t=i=0; exclude the two (non-instance,m=r=1) assignments from 12 possibilities, leaving ten.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Count cold uncontested noncompiled antecedents.

## contested-fiber

Type: cardinality_preimage. Domain/observation: d=(1,h,0,1).

Operator and approximation: t=1,i=h; s,m,r are free, yielding 12 inputs per h.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Count both contested fibers.

## scope-quotient

Type: observational_equivalence. Domain/observation: scope-only substitution.

Operator and approximation: General and Domain are indistinguishable for fixed flags; Instance differs only if m AND r AND NOT t.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Compare three scopes at each flag tuple.

## importance-delta

Type: intervention. Domain/observation: toggle i only.

Operator and approximation: F(c[i:=NOT i]) differs exactly in h; g,k,a are invariant.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Check all 48 paired substitutions.

## conflict-delta

Type: intervention. Domain/observation: toggle t from zero to one.

Operator and approximation: a rises; k falls iff non-instance AND m AND r; g and h stay fixed.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Check all 24 uncontested starting inputs.

## review-gate

Type: sensitivity. Domain/observation: toggle r from zero to one.

Operator and approximation: k rises iff non-instance AND m AND NOT t; other decision fields unchanged.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Check 24 unreviewed inputs.

## foundation-gate

Type: sensitivity. Domain/observation: toggle m from zero to one.

Operator and approximation: k rises iff non-instance AND r AND NOT t; ordinary reviewed facts remain uncompiled.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Check 24 nonmechanism inputs.

## instance-barrier

Type: scope_restriction. Domain/observation: s=Instance.

Operator and approximation: For all 16 flag settings k=0, including reviewed important mechanisms.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Enumerate instance subspace.

## minimal-blockers

Type: prime_implicants. Domain/observation: failed skeleton proposal.

Operator and approximation: k=0 iff s=Instance OR NOT m OR NOT r OR t; every disjunct is independently sufficient.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Isolate each blocker against an otherwise eligible input.

## minimal-repair

Type: counterfactual_repair. Domain/observation: metadata Hamming edits for k=0.

Operator and approximation: Minimum edits to obtain k=1 equals [s=Instance]+[NOT m]+[NOT r]+[t]; importance unchanged; Instance has two optimal scope repairs.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Enumerate all candidate repairs; compare minimum distance.

## nonexclusive-flags

Type: joint_reachability. Domain/observation: g,h,k combinations with a fixed zero.

Operator and approximation: g=h=k=1 is reachable for two scopes; storing, activation eligibility and compilation proposal coexist.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Check the two fully positive candidates.

## batch-order

Type: permutation_equivariance. Domain/observation: independent calls on a finite list.

Operator and approximation: map(F,permute(L))=permute(map(F,L)); no shared mutable state is read by recommend.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Proposed: permute a list with duplicate candidates.

## repeat-call

Type: idempotent_observation. Domain/observation: two calls with identical c.

Operator and approximation: F(c)=F(c); repeated recommendation is not durable-store deduplication and F is not composable on Decision as input.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Repeat all finite inputs in probe.

## policy-cost

Type: resource_bound. Domain/observation: one metadata call.

Operator and approximation: Fixed comparisons/Boolean gates, no loops or allocations: O(1) time and auxiliary space; no numerical latency prediction.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Source inspection only; benchmark remains unmeasured.

## admission-boundary

Type: domain_boundary. Domain/observation: caller admits a Candidate.

Operator and approximation: F records all representable metadata. No truth, provenance or admission fields exist; cannot reconstruct approval evidence from Decision.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Proposed: compare equal metadata from incompatible provenance records outside API.

## conflict-policy-change

Type: counterfactual_policy. Domain/observation: hypothetical remove NOT t from skeleton gate.

Operator and approximation: F_alt changes only k for non-instance,m=r=t=1, either i: four inputs. Not current Rust behavior.

Assumptions: fixed revision and shared domain boundary above; external state only in prospective entries. Evidence: source/design inspection; model.md#tests records the executed subset. Oracle: Compare formula tables; expect four changed inputs.

## tests

### Evidence and checks

Executed: compiled global/src/placement.rs directly with rustc --edition=2021 --test and ran all three existing unit tests: three passed. This isolates the unchanged metadata policy; no full crate or system suite was run.

A separate Rust probe imported the same source and ran twice with identical output. It enumerated 48 candidates, comparing all four output fields with F; checked all 768 candidate/decision pairs against the complement and converse equations (720 absent); counted six output fibers of sizes 2,2,10,10,12,12; checked 48 repeated calls and importance toggles; compared the relaxed-conflict counterfactual (four differing inputs). A contested general foundation rejects the deliberately relaxed skeleton rule, providing a negative control. The probe also checked all 40 manifest section IDs against exactly one matching catalog heading per ID.

The probe was external review tooling, not production code or an added registered Rust source. The finite relations are fully stated in model.md#core-model and model.md#catalog, allowing independent regeneration. No timing measurements were made.

Executed structural checks: installed mova-modelcheck groupants with source bindings and component paths; changed inventory against base ab4ba4321; git diff --check and repository secret guard. Structural inventory does not validate mathematical adequacy. Global repository sidecar backlog remains visible.

Only the explicit finite probe and three named existing tests are executed evidence. Other catalog oracles (including minimal repair enumeration, scope substitution probes, batch permutations and all prospective persistence/RAM/promotion integration checks) remain proposed. The partial AST does not implement the surrounding storage/RAM engines; passing metadata tests do not verify those engines.

## core-model

### Placement metadata model

Subject is manifest source SHA. Raw code remains ../placement.rs; A partial source-bound AST projection is stored in syntax.json.

Domain C={General,Domain,Instance} x {0,1}^4 has 48 candidates. D={0,1}^4 has 16 decisions. Write c=(s,m,t,i,r), d=(g,h,k,a). Forward F(c)=(1,i,(s!=Instance) AND m AND r AND NOT t,t). R={(c,F(c))}; horizon is one pure call. No parser, IO or mutable external state participates.

Invariant: g=1, h=i, a=t and k iff non-instance AND mechanism AND reviewed AND not contested. Thus k AND a is impossible. Activation and skeleton proposal are nonexclusive and neither removes the durable recommendation.

The first catalog entry specifies U minus R for U=C x D. The second specifies its full converse, not a unique inverse. Six outputs are reachable: two have two preimages, two have ten, two have twelve; total 48.

Cost: O(1) time/auxiliary space, fixed Boolean work without loops, allocation or IO. Nanosecond cost is unmeasured. Materialized batch outputs require O(n) space. Storage, scheduling and training require separate models.

Limits: caller supplies admission, scope, review and usefulness. Flags are recommendations, not writes, residency, truth judgments or Rust edits. AST and automatic model generation remain missing.

## History

### 2026-10-29 · descriptant

2026-10-29 epoch: twenty scoped models describe metadata and its boundaries. General and Domain collapse under decision observation; Instance differs only at the skeleton gate. Complete outputs still have fibers of size 2, 10 or 12. Preserve these observations when revising metadata.

### 2026-10-29 · document

Revision: manifest source SHA, 2026-10-29. Each section is a distinct scoped relation, not an independent proof. C has scope s and flags m=mechanism, t=contested, i=important, r=reviewed; D=(g,h,k,a). Unless explicitly prospective, hold other metadata fixed. Relations describe transitions, not unbounded reachability. Rust valid values and the already-admitted caller contract are assumed. A partial source-bound AST projection is stored in syntax.json.

### 2026-10-29 · tests

2026-10-29, source SHA bound in manifest.json.

### 2026-11-04 · descriptant

#### Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.
