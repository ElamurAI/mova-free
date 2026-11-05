# math-understand-chunk

## descriptant

### math-understand-chunk

read_number builds Rat from digit forms or accumulates number words/multipliers; chunk tries signed/contextual number plus unit before bare unit before word. standins emits placeholder forms with each token mapped to its retained Item index.

## approximant

### Scoped forward relation

read_number builds Rat from digit forms or accumulates number words/multipliers; chunk tries signed/contextual number plus unit before bare unit before word. standins emits placeholder forms with each token mapped to its retained Item index.

This source-derived relation is partial; no executable percentage surrogate or replacement is supplied.

## invariant

### Scoped domain and invariants

Successful number/unit readers advance the consumed token index. Exact Rat/units live in Item, not parser standins. Failed Rat operations may surface as unrecognized lexical material.

#### Cost

Chunk advances monotonically, but read_number may scan multiword spans and Rat operations have arithmetic costs. standins formatting and output allocation scale with emitted forms.

## models

## behavior_complement

A recognized fractional word cannot absorb a unit in chunk's number branch. Successful standin emission cannot lose its owner index. Placeholder output does erase sign/unit details, so unique source reconstruction is excluded.

## reverse_reconstruction

Items -8 miles and +8 kilograms both emit ['8','units'] with owner[0,0], while retaining different Item values/units. Fractions1/3 and1/2 emit 'half'; placeholder-only preimage is set-valued.

## signed_context

A minus is folded into a number only when the previous emitted item is neither Qty nor Unit and the next token starts with an ASCII digit. Otherwise it remains a word/operator token.

## ambiguous_unit_context

Bare single-letter scales require to/into/in context; listed ambiguous UnitW abbreviations require a/per/each/every unless after_num=true. Same spelling thus has context-dependent chunk role.

## tests

### Scoped evidence

Word numbers, fractions, unit ambiguity, speed and negative quantity placeholder ownership.

Named actual source fixtures: understand::wave28::{chunk_units_and_placeholder_transport}.

Parent model.md#tests records commands and paired profiles. No additional coverage inferred from parent fixtures outside this byte range; source-scoped numeric/parse properties remain partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced duplicate catalog sentences with concrete exclusions, compatible-antecedent examples and two distinct scoped behavioral views. Source and prior execution evidence are unchanged; this revision ran no tests.
