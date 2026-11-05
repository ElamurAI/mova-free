# mova-whale-world-main

## Descriptant

The mova-whale-world command line: dossier import/add/list/verify/text, world build/stats/entity/explain/related/neighbourhood/sentences/context and export-texts EXPORT_CONFIG OUT_REPO [DOC...], one JSON value per command (JSON lines for neighbourhood --content). Source: `tools/whale-world/src/main.rs`.

## Forward approximant

argv -> export-texts: export::Config::load -> its world Config -> library once -> export::export -> report ; otherwise group, command, CONFIG -> Config::load -> positional args and --options -> dossier or world call -> JSON on stdout; world build defaults its epoch to the highest dossier epoch + 1 and reports seconds.

## Invariant

A refusal is an error exit with nothing written; dossier verify exits 1 when a file changed.

## Cost

Startup plus the called operation; import loads the library once per command.

## Composition

Parent: whale-world.

## behavior_complement

Does not retry, fetch, ask a terminal or start other processes; no command deletes anything. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The printed JSON names every text id and citation; the command line is recoverable from the usage text only.
