# qagen-main-parsing

## forward
opts consumes interleaved positionals and --key value pairs; num parses optional typed values; batches_opt parses comma-separated usize; validate_options applies command/subcommand whitelist.

## invariant
Successful opts never accepts missing value or consumes another option as value. Unknown keys error before main dispatch.

## behavior-complement
No key=value grammar, no abbreviation matching, no duplicate rejection; negative numeric tokens are values and typed parser decides validity.

## reverse-reconstruction
A parsed map has multiple preimages differing in option order and overwritten duplicates. Defaults erase distinction between absence and explicit default value at num output.

## cost
O(argument bytes + K log K) parsing with owned clones; batches list allocates one usize per comma component.

## composition
Whitelist includes batches for both run branches and replaces only fix gate; command arity remains validated by existing match.

## missing-value
Both last option without next token and option followed by --token error.

## typed-domain
num delegates FromStr; unsigned negatives/overflow reject, float NaN remains permitted by existing downstream contract.

## whitelist
Selection, run, sample, fix-select and fix-gate get separate allowed key sets; prompt/gate accept no options.

## precedence
Lexical parse errors precede whitelist errors; whitelist precedes command arity and typed conversion.

## checks
Synthetic embedded Rust tests, 1024 independent coverage cases, preflight sentinel artifacts, callback trace and compiled branch/arithmetic controls; debug and release twice, model-loading test explicitly skipped. See parent ../model.md#evidence for observed outcomes, not proof.

## epoch-memory

Prospective model was saved before repair. Old source failed four library witnesses and one CLI witness. The tie control initially survived because [2,1,2] selects 2 for both first and last ties; the corrected [2,1] witness distinguishes them. Preserve this observation: duplicate-endpoint fixtures can hide tie regressions.

## references
Original project implementation and rule v2: docs/context.md, qagen/src/gate.rs and qagen/src/run.rs. Model conventions: docs/groupants.md, docs/mathematical-modeling.md and docs/mathematical-modeling.md. No external solution code copied.

## limitations
Finite local tests and partial syntax projections, no semantic AST or universal proof. Allocation failure and callback panic are outside successful-return model; filesystem races and write failures can interrupt publication. No live model/teacher/database/corpus used. No useful certified numeric surrogate is attached to these categorical/effectful interfaces; no invented error budget, substitution or speed claim.

## History

### 2026-10-30 · epoch-memory

Prospective model was saved before repair on 2026-10-30. Old source failed four library witnesses and one CLI witness. The tie control initially survived because [2,1,2] selects 2 for both first and last ties; the corrected [2,1] witness distinguishes them. Preserve this observation: duplicate-endpoint fixtures can hide tie regressions.
