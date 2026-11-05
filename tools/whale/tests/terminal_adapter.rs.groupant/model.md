# mova-whale-terminal-adapter-tests

## Descriptant

End-to-end tests of the Whale's terminal adapter against a fake bash LLM in a real tmux server on a private socket: an answer, a usage-limit screen, and a full Whale run through its own terminal. Source: `tools/whale/tests/terminal_adapter.rs`.

## Forward approximant

Start tmux -L <private socket> new-session running the fake script, set @mova_crab_owner, wait for TERMINAL READY, write a mova-terminal profile; MovaTerminal::ask -> answer file text; limit mode -> AskError::Limit within seconds, the next question refused as a replay; paper::run with MovaTerminal on a fixture paper -> one question answered, glossary from the answer, description without it.

## Invariant

Never starts a model or uses the default tmux server; skipped without tmux (and without MOVA_OCTOPUS_TEST_MODEL for the Whale run); kills its private server on drop.

## Cost

About 5 s (model load and tmux polling).

## Composition

Parent: whale-workers; exercises terminal.rs, paper.rs, reader.rs and mova-terminal.

## behavior_complement

Not covered: real Codex or Claude screens, folder-trust dialogs, real limit messages, concurrent Whales on one tmux server. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Failures name the expected and actual AskError or the description and glossary; the temporary directory and tmux server are removed. No unique inverse is claimed.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (learning Whales)

Fourth test: the slow fake LLM writes its answer file, shows a working line for 1.5 s, then the idle marker; the adapter returns only after that and the next question is accepted.

#### Epoch 2026-11-04

Question literals carry the new `items` field (empty).
