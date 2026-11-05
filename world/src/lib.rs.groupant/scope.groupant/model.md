# world-lib-scope

## descriptant

### Module visibility and conditional GPU declaration

Let D be the module declarations in this byte scope. Public exports are P(g)=all pub mod names in D except induce_gpu, union {induce_gpu} iff cfg(feature=gpu). parsecache is declared with mod, hence excluded from P(g). The projection describes declarations before the first cfg(test); test modules lie outside this fragment.

## approximant

### Module visibility and conditional GPU declaration

Let D be the module declarations in this byte scope. Public exports are P(g)=all pub mod names in D except induce_gpu, union {induce_gpu} iff cfg(feature=gpu). parsecache is declared with mod, hence excluded from P(g). The projection describes declarations before the first cfg(test); test modules lie outside this fragment.

Categorical source relation; no executable percent surrogate or approximation error bound is claimed.

## invariant

Declaring talk, nerves, ideas or vmm does not execute their functions. parsecache has no public module export from this file. Changing gpu alters the induce_gpu declaration, not the unconditional names.

Cost: Lexical examination is O(|D|) declarations. Actual compilation recursively resolves sibling modules and their dependencies; this fragment cannot bound those build costs.

## composition

This fragment contributes the parent library's production module surface. Scope excludes the trailing test-module declarations, which remain represented in the parent AST. Public exports permit callers to invoke effectful APIs but do not trigger those effects.

## models

## behavior_complement

Exclude a public parsecache export, a default-feature induce_gpu export, or a runtime teacher/memory operation attributed to evaluating these declarations. These are visibility exclusions, not a proof that public callees are effect-free.

## reverse_reconstruction

The observed unconditional export set is compatible with either gpu feature state after projecting away induce_gpu. Observing induce_gpu in the declared public set requires gpu=true under this file's cfg rule. Export names alone admit arbitrarily different sibling implementations with identical declarations; their runtime effects cannot be reconstructed.

## feature_projection

P(false)=P(true) minus {induce_gpu}; this equation concerns the declared module surface, not link-time dead-code elimination.

## private_namespace

parsecache is a private module declaration while neighboring talk/tune/types/vmm/world are public. Private child items may still be used within the crate.

## declaration_execution_boundary

Module declaration resolution compiles referenced source; it is not a call to main_args, graph, remember, or call_v3.

## test_scope_boundary

The byte scope ends before cfg(test) mod tests; no test-module visibility claim is inferred from this scoped declaration list.


## tests

Compile/visibility evidence only: selected world library and binary test targets compiled with default CPU features. No runtime test is credited to module declarations; gpu=true visibility has source-derived evidence only.

Commands and repeated outcomes: [parent evidence](../model.md#tests), [source evidence](../evidence.json). No new execution was required for this model-only refinement.

## History

### 2026-10-31 · descriptant

2026-10-31 review epoch: replaced generic fragment wrappers with scoped transitions, concrete exclusions and compatible antecedents. The exact byte range/hash and shared parent AST are retained. This fragment contributes the parent library's production module surface. Scope excludes the trailing test-module declarations, which remain represented in the parent AST. Public exports permit callers to invoke effectful APIs but do not trigger those effects.
