# mova-whale-main

## Descriptant

mova-whale CLI dispatching subcommands over one absolute JSON configuration. Source: `tools/whale/src/main.rs`. Public, domain-agnostic: concrete prompts, schemas, gate parameters and deployments come from pinned configuration.

## Forward approximant

Parse `COMMAND CONFIG [--once]`, require an absolute CONFIG path, load bounded JSON, dispatch to whale/swarm/agent/roles entry points, print JSON results, return failure with the error text.

## Invariant

Unknown commands, extra arguments and relative configuration paths are refused before any effect.

## Cost

O(config bytes) plus the selected entry point.

## Composition

Executable used by pool and swarm children (`whale`, `whale-research`, `whale-swarm`).

## behavior_complement

Excluded: relative CONFIG, unknown commands, trailing arguments other than --once. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

A printed result identifies the entry point; the configuration is not recoverable from output. No unique inverse is claimed.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (Whales in English)

Epoch 2026-11-04: serve CONFIG DEPUTY_CONFIG SOCKET and ask SOCKET ID [urgent] before the CONFIG subcommands.

#### Epoch 2026-11-04 (Whales reading papers)

Epoch 2026-11-04 (Whales reading papers): `paper CONFIG [--no-terminal]` (one Whale, one paper; asks through MovaTerminal or journals only) and `paper-check CONFIG...` (no shared run root or terminal profile, session, owner marker or root).

#### Epoch 2026-11-04 (learning Whales)

learn CONFIG [--no-terminal] (one Whale learns on one paper until its description passes), batch-plan CONFIG and batch CONFIG [--no-terminal] (one paper per Whale per batch, closed at its deadline).

#### Epoch 2026-11-04

New commands: `learn CONFIG --replay ROOT...`, `learn-report STATE_ROOT...`, `extraction-check TEXT...` (WHALE_PROSE=1 adds prose words).

### 2026-11-05 · document

#### Epoch 2026-11-05

Subcommands `epoch CONFIG [--epoch N] [--doc ID] [--no-terminal]` (epoch::run_cli) and `epoch-status CONFIG [--epoch N]` (read-only).

#### Epoch 2026-11-05 (rolling batches)

`batch-plan` closes finished assignments and gives each free Whale its next paper; `batch` is the rolling runner (one per run root, `batch::roll`); `--no-terminal` one dry pass (`batch::once`).
