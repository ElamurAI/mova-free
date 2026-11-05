# math-qread-affine

## descriptant

### math-qread-affine

add((c,x),(d,y),s)=(c+s*d, prune(x+s*y)); prune keeps exactly |a|>1e-12. subst replaces each original key once with its supplied Lin, preserving unsolved keys.

## approximant

### Scoped forward relation

add((c,x),(d,y),s)=(c+s*d, prune(x+s*y)); prune keeps exactly |a|>1e-12. subst replaces each original key once with its supplied Lin, preserving unsolved keys.

This source-derived relation is partial; no executable percentage surrogate or replacement is supplied.

## invariant

### Scoped domain and invariants

Finite map arithmetic is assumed for the pruning interpretation. Constant c is not pruned; map coefficients are. No global finite-value guarantee.

#### Cost

Each addition clones the left map, updates right keys through ordered-map operations, then scans retained entries. Substitution repeats additions for each replaced key; cost depends on total substituted support, not just original key count.

## models

## behavior_complement

For finite coefficients, canceled entries with |a|<=1e-12 cannot remain in add's output; subst cannot recursively substitute a newly introduced key within that same call. Numeric text parsing does not exclude nonfinite f64 values.

## reverse_reconstruction

Output x={LEAF:1} admits both the original map and addition of equal-and-opposite unknown terms. subst({u:1},{u:v}) yields v even if the solution map also maps v to a leaf; one pass cannot identify a unique input equation.

## one_pass_substitution

Let sol(u)=v and sol(v)=leaf0. subst(u,sol)=v, while a second call yields leaf0. The preimage relation therefore depends on substitution depth.

## key_classification

unknowns selects keys<LEAF independently of coefficient value; var(LEAF) is a numeric leaf while var(LEAF-1) is an unknown. add may remove an unknown through pruning without solving an equation.

## tests

### Scoped evidence

Lin cancellation/subst, compound zero denominator and provenance only.

Named actual source fixtures: qread::wave28::{affine_provenance_and_gate_order; empty_query_and_compound_provenance}.

Parent model.md#tests records commands and paired profiles. No additional coverage inferred from parent fixtures outside this byte range; source-scoped numeric/parse properties remain partial.

#### Wave34 direct-helper refinement

Earlier source-only gaps and wave28 receipts above are historical. Current descendants add actual helper fixtures; see ../model.md#wave34-evidence and each descendant's Tests section. Full English parsing and general solver correctness remain gaps.

## wave34-composition

# Affine descendant composition

one-pass-images invokes support-pruning repeatedly through add. It iterates original keys in BTreeMap order; replacement keys are inserted into accumulated output but never become iteration inputs. Each addition can discard a coefficient, so a later replacement operates on the already pruned map. The constant follows a separate unpruned accumulation. unknowns then classifies keys below LEAF without validating leaf backing storage.

Inverse composition must retain every intermediate map satisfying the rounded addition and pruning relation. A dropped residual cannot be reconstructed uniquely. Replacing this ordered process by simultaneous exact substitution would change the observed relation. These are absolute pruning thresholds, not percentage error guarantees.

## Cost

For each original key, count its replacement support, accumulated map clone and ordered updates/retention. Summing these add costs is appropriate; adding the full parent cost again is not. Map growth and cancellations determine actual storage.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced duplicate catalog sentences with concrete exclusions, compatible-antecedent examples and two distinct scoped behavioral views. Source and prior execution evidence are unchanged; this revision ran no tests.
