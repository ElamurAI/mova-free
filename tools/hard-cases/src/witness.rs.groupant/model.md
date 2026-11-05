# mova-hard-cases-witness

## Descriptant

Stanza, Trankit and spaCy trf through the existing neural runner on a track's training trees under the shared GPU lease; # text attached only where id and every token form agree; agreed-input.conllu and parsers/out/*.conllu for the Eel's tree agreement. Source: `tools/hard-cases/src/witness.rs`.

## Forward approximant

if witness.json binds sha256(train) -> return it; else write in/ids.txt, in/tokens.txt; with flock(gpu_lock) run neural.py for the three parsers; align each output by index (same id and forms) -> keep the records all three align; write agreed input, filtered parser outputs, witness.json.

## Invariant

A record enters the agreed input only when all three parsers align on id and forms; parser output rows are unchanged except the inserted # text line; no witness is manufactured.

## Cost

Three neural parser runs (GPU); measured 38 s for 3 sentences including model loading.

## Composition

Feeds the Eel engine's agreement step (treeagree agree reads parsers/out). Parent hard-cases.

## behavior_complement

Does not run without the GPU lease (waits up to lease_wait_seconds), does not retry a failed parser, does not repair tokenization. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The raw runner output stays in witness-run-<hash>/out; the filtered files plus the missing list recover which records each parser aligned.

## Tests

witness::tests: text attached only to aligned records; red: a different tokenization and a wrong id do not align. Live: three sentences, all aligned; treeagree agree accepted the output. Finite fixtures and live samples; not a proof.

## History

### 2026-11-04 · Tests

witness::tests (2026-11-04): text attached only to aligned records; red: a different tokenization and a wrong id do not align. Live: three sentences, all aligned; treeagree agree accepted the output. Finite fixtures and live samples; not a proof.
