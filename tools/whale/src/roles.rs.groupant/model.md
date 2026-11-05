# mova-whale-roles

## Descriptant

Crab ExternalRoleHandler adapter for Whale pool and swarm. Source: `tools/whale/src/roles.rs`. Public, domain-agnostic: concrete prompts, schemas, gate parameters and deployments come from pinned configuration.

## Forward approximant

status(config, swarm): swarm::status(swarm) when Crab has a swarm configuration, else pool::status(config); start(config, swarm): swarm::start or pool::start; marker: pool::marker. serve installs WhaleRoles through mova_crab::angel::serve_with_roles.

## Invariant

The adapter adds no path or program choice of its own; every effect is gated by the delegated module's pins and Crab's request/journal gates.

## Cost

O(1) dispatch plus delegated cost.

## Composition

Top of the public Whale tree; installed by an embedding Crab binary or `mova-whale crab`.

## behavior_complement

Excluded: any operation without trusted server-side configuration (relative or missing paths fail closed); swarm markers (handled by Crab itself). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

A returned status admits the delegated module's compatible states; the dispatch branch is recoverable from whether a swarm configuration was present. No unique inverse is claimed.
