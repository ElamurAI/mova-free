# mova-whale-world-config

## Descriptant

The configuration mova-whale-world-v1: dossier and world roots, the optional library block of topic-corpus, corpora root, earlier-description site roots and files, learner state roots, Mova round choice, the public-originals switch and cache/context limits; unknown fields refused, every path absolute. Source: `tools/whale-world/src/config.rs`.

## Forward approximant

bytes -> serde (deny_unknown_fields) -> validate(schema, absolute paths, plain site file names, non-zero caches, context_bytes >= 1000) -> Config.

## Invariant

A loaded Config has the right schema and only absolute paths; public_originals defaults to false.

## Cost

Linear in the file size.

## Composition

Parent: whale-world; read by dossier, import, world and main.

## behavior_complement

Does not check that paths exist or are readable, and holds no deployment values (they live in a deployment's own file). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The Config re-serializes to an equivalent file (defaults filled in); no unique inverse of defaults.
