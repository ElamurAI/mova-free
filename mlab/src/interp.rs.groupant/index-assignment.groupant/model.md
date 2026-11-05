# mlab-interp-index-assignment

## descriptant

### mlab-interp-index-assignment

Assignment and index-context evaluation boundary.

## approximant

### Scoped forward relation

RHS Mat -> existing target Mat or undefined -> evaluate index args with pushed EndCtx -> index::assign -> store.

#### Domain and gap

Source-generated states within resource limits. Categorical relation has no executable percent surrogate.

## invariant

### Scoped invariant

A function-handle RHS is rejected before an invalid index is evaluated.

#### Cost

Index expression evaluation plus index::assign costs and target growth/copy. EndCtx stack adds one entry per active index evaluation; dimensions use unchecked products.

## models

# Scoped typed relations

## behavior_complement

Nonmatrix RHS excludes target/index evaluation; failed undefined-target assignment excludes inserting that target. End-context evaluation must not retain its temporary stack entry after a returned error.

## reverse_reconstruction

Successful target matrix admits initial empty or existing matrices and many index/value histories; error text constrains first rejected stage only.

## rhs_precedence

A function-handle RHS is rejected before an invalid index is evaluated.

## missing_target

Undefined target uses dimensions 0,0 and is inserted only after successful assignment.

## end_context

Each index evaluation pushes then pops its dimension context before propagating its Result.

## column_coordinates

Indices delegate to index module, not direct Rust offsets.


## tests

### Evidence scope

Parent selected tests: wave30_parse_runtime_and_output_state, wave30_bounded_state_and_fallback, parse_error_runs_nothing. This fragment shares parent syntax and finite checks; no separate test count or complete fragment coverage claim. Assertion-control association and unexecuted paths are in parent model.md#tests and source-test-map.json.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Half-open source bytes [17664,19980); bound by parent source hash and scoped byte hash. Model covers only this fragment; parent composition explains joins.
