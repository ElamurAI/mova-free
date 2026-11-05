# mova-whale-world-lib

## Descriptant

Crate root of mova-whale-world: module list (config, dossier, export, query, world) and the crate's purpose: append-only document dossiers, one incremental global world for Whale epochs and the export of finished texts into an editorial texts repository. Source: `tools/whale-world/src/lib.rs`.

## Forward approximant

Declares the modules; re-exports anyhow::Result; the end-to-end tests module under cfg(test).

## Invariant

No logic; every public module is listed once.

## Cost

O(1).

## Composition

Parent: whale-world (models/whale-world.groupant).

## behavior_complement

No behaviour of its own; re-exports no topic-corpus or Whale types. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The module list is recoverable from the source; nothing else.
