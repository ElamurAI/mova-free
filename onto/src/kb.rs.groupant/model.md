# onto-kb

## descriptant

### onto-kb

BFS first shortest taxonomic predecessors; prop sorts ancestor seed indices, chooses first visible nonempty layer then descendant-minimal concepts. Mixed values Undetermined, equal values nearest depth then index. add gates precede insertion; TSV loaders retain valid prefixes.

## approximant

### Forward relation

BFS first shortest taxonomic predecessors; prop sorts ancestor seed indices, chooses first visible nonempty layer then descendant-minimal concepts. Mixed values Undetermined, equal values nearest depth then index. add gates precede insertion; TSV loaders retain valid prefixes.

#### Equations

A_l={i:assert[i].concept∈Anc(c), prop=p, layer=l}; l*=first l∈layer_chain(L) with A_l≠∅. M={i∈A_l*:¬∃j∈A_l*, c_i≠c_j ∧ c_i∈Anc(c_j)}. If values(M) singleton: win=argmin(depth(c_i),i); else Undetermined.

#### Domain

Valid indices/DAG/layer chain and private indices assumed. gates=false disables strict/cycle only, not duplicates/same-concept conflict. u16 depth unguarded.

Categorical behavior and effect boundaries have no validated percentage surrogate. Executable Rust approximant unsupported; 1/2/5/10% future caps require a useful metric first.

#### Visibility switch domain

The layer-chain equations, invisible-layer exclusion and invisible-seed preimage freedom require `leak_all_layers == false`. With the diagnostic switch true, prop instead scans all layer indices in reverse order, independent of the requested layer's parent chain; a nominally invisible layer can then decide. Index/acyclicity assumptions still apply. The executed fixtures keep the switch false; its compiled mutant deliberately sets true and fails the real-layer assertion.

## invariant

### Invariant and domain

Valid indices/DAG/layer chain and private indices assumed. gates=false disables strict/cycle only, not duplicates/same-concept conflict. u16 depth unguarded.

#### Limits

Finite synthetic fixtures are not semantic proof. Public invalid indices, allocation failures, huge counters, environment races and live effects outside explicit checks remain unverified.

#### Visibility switch domain

The layer-chain equations, invisible-layer exclusion and invisible-seed preimage freedom require `leak_all_layers == false`. With the diagnostic switch true, prop instead scans all layer indices in reverse order, independent of the requested layer's parent chain; a nominally invisible layer can then decide. Index/acyclicity assumptions still apply. The executed fixtures keep the switch false; its compiled mutant deliberately sets true and fails the real-layer assertion.

## cost

### Cost approximation

BFS O(Va+Ea); K candidate ancestry maps plus O(K²) dominance; loader full rows memory and repeated insertion traversal.

Symbolic costs only; no timing or throughput claim.

## composition

### Composition

A_l={i:assert[i].concept∈Anc(c), prop=p, layer=l}; l*=first l∈layer_chain(L) with A_l≠∅. M={i∈A_l*:¬∃j∈A_l*, c_i≠c_j ∧ c_i∈Anc(c_j)}. If values(M) singleton: win=argmin(depth(c_i),i); else Undetermined.

Graph builder→store shape check→Kb; Kb→query→bridge/describe/pron. hs Parsed→hsmap preference/scoring/gates. Core SynMap last-duplicate vs Graph first-duplicate requires unique synset keys for interchangeable lookup. Syntax children below share parent projection; exact byte ranges bind original source semantics.

- seed-gates: insert(a)=validate duplicate,same-node conflict,strict ancestry; then append+index

- property-resolution: A_l=visible ancestor assertions on layer l; l*=first nonempty layer; M=descendant-minimal concept candidates; Known iff values(M) singleton

## models

## behavior_complement

Invisible layer cannot decide; dominated seed cannot win same layer; incomparable mixed values exclude Known; rejected insertion excludes vector/index changes.

## reverse_reconstruction

Known constrains winner/minimal values/proof, admits invisible or other-property seeds; Undetermined requires both minimal values, many graph preimages. Failed load allows arbitrary suffix.

## layer_visibility

Layer precedence before concept specificity; fable ancestor can override real specific seed.

## minimal_antichain

Discard any seed concept ancestor of another different candidate; duplicate same-value nodes survive.

## gate_atomicity

duplicate→same-node contradiction→Strict before append; load is prefix-mutating.

## proof_coordinates

Cid u32/LayerId u16/depth u16; Chain holds selected shortest path and extra provenance.


## Visibility switch domain

The layer-chain equations, invisible-layer exclusion and invisible-seed preimage freedom require `leak_all_layers == false`. With the diagnostic switch true, prop instead scans all layer indices in reverse order, independent of the requested layer's parent chain; a nominally invisible layer can then decide. Index/acyclicity assumptions still apply. The executed fixtures keep the switch false; its compiled mutant deliberately sets true and fails the real-layer assertion.

## tests

### Actual-code checks

- `kb::wave29_tests::layers_precedence_and_rejected_cycle`
- `kb::wave29_tests::partial_layer_load_retains_valid_prefix`

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective model saved before edits; nine existing sources read fully. Added bounded inline tests; store reader overflow repaired only after compiled old-code failing witness. No live corpus, teacher, model or database effects executed. Source syntax generated twice identically; partial lexical projection only.

### 2026-10-31 · tests

#### Evidence 2026-10-31

Each selected test passed twice debug and twice release after exact mutation restoration (15 named suite checks; all four sorted outcome sets equal, exits0). Compiled source control: negative-kb.log (compiled source mutation assertion failure101). Controls compile successfully and fail an assertion with exit101. Store control is the original unsafe length addition before repair.

Finite graph/token/temp fixtures; public invalid states and unselected branches not exhausted

Whole CLI, live teacher/corpus/model/prior loading, filesystem failure/durability and huge inputs were not executed. No exact wrapper is advertised as a percentage surrogate. Added test blocks rustfmt-formatted independently; unrelated source formatting preserved. Strict broad Clippy not repeated: existing dependency/source warnings are outside this work.
