# gutenberg-text-command-csv-dates

## descriptant

### gutenberg-text-command-csv-dates

#### Descriptant

Full source read preceded changes. Prospective model predicted branch/order, resource and information-loss behavior. Tests exercise actual helpers on bounded synthetic inputs; command workflows remain unexecuted. Production source prefix is preserved; only inline tests added.

Quoted char scanner handles doubled quote, comma/newline only outside quotes, ignores outside CR. EOF emits iff field or row nonempty. death_years splits semicolons; BCE->0; last hyphen numeric prefix>0, otherwise whole None.

Executable percentage approximation is unsupported: categorical parsing/effects have no useful certified metric here. Partial syntax excludes macro expansion and name resolution.

## approximant

### Forward relation

Quoted char scanner handles doubled quote, comma/newline only outside quotes, ignores outside CR. EOF emits iff field or row nonempty. death_years splits semicolons; BCE->0; last hyphen numeric prefix>0, otherwise whole None.

#### Domain

UTF-8 strings and dates fitting u32; malformed quotes accepted.

## models

# Typed source models

All models are partial source-derived relations at this manifest snapshot. Scope is one call/trace under the stated domain; no universal semantic proof.

## behavior_complement

Quoted comma/newline cannot split a field; an undated nonempty author excludes Some even if earlier entries dated.

Domain: UTF-8 strings and dates fitting u32; malformed quotes accepted. Observation: Quote state and field buffer determine separators; no syntax error result. Date vector retains person order and cardinality of nonempty entries. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## reverse_reconstruction

Rows admit quoted/unquoted spellings, ignored CR and optional trailing newline. A year vector admits arbitrary names and date prefixes; None cannot identify which entry failed.

Domain: UTF-8 strings and dates fitting u32; malformed quotes accepted. Observation: Quote state and field buffer determine separators; no syntax error result. Date vector retains person order and cardinality of nonempty entries. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## quote_escape_transition

Inside quotes two quotes append one literal and consume both; a single quote leaves quoted state.

Domain: UTF-8 strings and dates fitting u32; malformed quotes accepted. Observation: Quote state and field buffer determine separators; no syntax error result. Date vector retains person order and cardinality of nonempty entries. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## empty_eof_partition

Empty input and a sole empty quoted field produce no rows; comma produces two empty fields; newline produces one empty field.

Domain: UTF-8 strings and dates fitting u32; malformed quotes accepted. Observation: Quote state and field buffer determine separators; no syntax error result. Date vector retains person order and cardinality of nonempty entries. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## last_hyphen_date

rsplit_once chooses last hyphen; skip nondigits then take digits; suffix annotations are discarded and zero rejects.

Domain: UTF-8 strings and dates fitting u32; malformed quotes accepted. Observation: Quote state and field buffer determine separators; no syntax error result. Date vector retains person order and cardinality of nonempty entries. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.

## bce_short_circuit

Any entry containing literal BCE contributes0 before hyphen parse, including malformed surrounding date text.

Domain: UTF-8 strings and dates fitting u32; malformed quotes accepted. Observation: Quote state and field buffer determine separators; no syntax error result. Date vector retains person order and cardinality of nonempty entries. Assumptions: bounded resources and source branch semantics. Oracle: selected actual-helper tests in model.md#tests constrain only their finite fixtures; remaining branches are source-derived.


## tests

### Evidence

Selected exact tests: ["tests::csv_and_death_years","wave33::csv_boundaries_and_date_fibers"].

Each runs twice in debug and twice in release with normalized named outcomes compared. Compiled production-boundary mutants are recorded in the external wave33 text receipts with original/restored hashes and positive reruns. Existing negative assertions are separate from compiled mutants. No CLI, download, teacher, corpus or model execution. For this fragment, source-derived paths not reached by these tests remain unverified.

## invariant-and-cost

### Invariant and cost

#### Invariant

Quote state and field buffer determine separators; no syntax error result. Date vector retains person order and cardinality of nonempty entries.

#### Cost

O(B) Unicode scanning and allocated output; last-hyphen searches across entries sum O(B).

Arithmetic and allocation premises are conditional. No timing or percentage surrogate claimed.

## History

### 2026-10-31 · descriptant

Heading `#### Epoch 2026-10-31` stays in descriptant as `#### Descriptant`.
