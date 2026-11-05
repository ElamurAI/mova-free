# mova-whale-lib

## Descriptant

Public mova-whale module boundary and test serialization guard. Source: `tools/whale/src/lib.rs`. Public, domain-agnostic: concrete prompts, schemas, gate parameters and deployments come from pinned configuration.

## Forward approximant

Declare the Result type and modules; re-export WhaleRoles; under test, provide a process-wide mutex for tests that spawn processes or observe lifetime leases.

## Invariant

No dependency on private code; the serialization guard exists only in test builds.

## Cost

None at run time.

## Composition

Root module of the crate.

## behavior_complement

Excluded: concrete specifications, prompts, models or deployment paths (none are compiled in). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The module list is recoverable from the source; behavior is in child modules. No unique inverse is claimed.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (Whales in English)

Epoch 2026-11-04: modules animal (English service) and answer (English realization).

#### Epoch 2026-11-04 (Sea Spider live: singleton and callers)

Epoch 2026-11-04: Exports the new `seaspider` client module.

#### Epoch 2026-11-04 (Whales reading papers)

Epoch 2026-11-04 (Whales reading papers): modules paper, reader and terminal.

#### Epoch 2026-11-04 (learning Whales)

Modules batch, clause, compose, gates, knowledge and learn.

#### Epoch 2026-11-04

New modules `pod`, `review`, `topic`.

### 2026-11-05 · document

#### Epoch 2026-11-05

New modules `claims`, `dossier`, `epoch`.
