# mova-hard-cases-eel

## Descriptant

mova_eel::TrainingData over a track's harvested hard-case data and one round of the public Eel engine (trainer::run once) with its gates unchanged; the internal track must not install into the deployed model directory. Source: `tools/hard-cases/src/eel.rs`.

## Forward approximant

ready() = (sha256 over the six learn files' hashes, missing ids) when train and dev are nonempty and witness.json binds the current train file; status -> generation; export -> immutable copy + generation re-check; round(track) = refuse internal==deployed model_dir and missing hard: prefix, else trainer::run(eel[track], once) and read its receipt.

## Invariant

An export's generation equals the copied files' generation; not ready -> no engine call; the engine's agreement, held-out, dev, EWT and install gates apply unchanged.

## Cost

File hashes O(bytes); the round is the engine's CUDA cost.

## Composition

Implements the public Eel TrainingData contract; the engine admits hard: ids through fresh_train_prefixes. Parent hard-cases.

## behavior_complement

Does not train by itself, run in a loop, install into the deployed directory for the internal track, or count unwitnessed records as training data. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The engine's generation directory keeps the export, receipt and candidate; from a receipt the exported files are recoverable by their hashes.

## Tests

eel::tests: readiness red controls (nothing, no dev, no witnesses, witnesses of another train file), status/export generation equality, a changed train file not ready. Finite fixtures and live samples; not a proof.

## History

### 2026-11-04 · Tests

eel::tests (2026-11-04): readiness red controls (nothing, no dev, no witnesses, witnesses of another train file), status/export generation equality, a changed train file not ready. Finite fixtures and live samples; not a proof.
