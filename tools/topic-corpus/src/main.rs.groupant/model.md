# mova-topic-corpus-main

## Descriptant

CLI `mova-topic-corpus`: graph, missing, build, explain, world, seed, web-plan, web-ask, web-import, cite, sol-prompt, sol-draft, sol-import, sol-claims, public-export, hard-cases-recheck. Source: `tools/topic-corpus/src/main.rs`.

## Forward approximant

Load the config (schema mova-topic-corpus-v1) -> dispatch by command -> library/graph/corpus calls -> pretty JSON on stdout; errors print `mova-topic-corpus: …` and exit 1. Output files must be new (cite, web-plan); sol-draft takes the terminal holder lease before asking.

## Invariant

Every command reads only configured absolute paths; no command starts the download driver or a live terminal session on its own.

## Cost

Dominated by the library load (~10 s for 186k export rows) and lazy parses.

## Composition

Parent: `topic-corpus`.

## behavior_complement

Does not start stream drivers, does not start terminals, does not fetch pages itself (Sea Spider does), does not write into a Whale's store while it learns (seed refuses). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The printed JSON names the files written; the configuration is not recoverable from output. No unique inverse is claimed.
