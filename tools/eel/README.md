# Mova Eel

Eel is Mova's learner (Rust on Mova), supervised by Crab; the Octopus — a large language model (LLM) — develops it. See [the sea animals of Mova](../../docs/animals.md).

Reusable cooperative idle-GPU continuation through Mova's native parser. This
crate owns the working controller and trainer: independent GPU leases, immutable
annotation export, actual treeagree invocation, native gparse baseline/train/
reload, finite CUDA evidence, developer gates, conditional weight installation,
crash reservations, durable receipts and bounded artifact retention. It has no
private-source dependency. It does not launch a teacher or another AI provider.

Eel trains only the native syntax (AST) model, gparse. Training of the
pragmatics model is not implemented in this crate.

A deployment implements `TrainingData::status` and `TrainingData::export` and
calls `mova_eel::run(&configuration, once, &mut data)`. Ordinary foreground
training uses `mova_eel::trainer::run`. Status is a polling hint; a concurrent
annotation writer may advance the generation before the immutable export. The
export's SHA-256 identity owns the reservation. Both identities are validated
before use in generation paths. Export supplies `train_path`, `dev_path`,
`agreed_input_train_path`, `parsers_dir`, `missing_training_witness_ids` and
`generation_sha256` using the existing versioned snapshot contract. Missing real
parser witnesses stay excluded; the engine cannot manufacture them.

The configuration injects absolute native `gparse`, `treeagree`, `model_dir`,
`run_root`, shared `gpu_lock`, pinned developer/protected data and runtime limits.
The native executable must be gparse itself, not a wrapper taking the same GPU
lock. `controller_executable` rejects an additional wrapper by path/content.
The legacy `peta_executable` alias, owner identifiers and installer lock filenames
remain accepted or retained to cooperate with older controllers during upgrades.
Those are compatibility protocol identifiers, never deployment paths or data.

Fresh training admits only ids with a `school:` prefix by default. A deployment that admits another kind of
reviewed annotation names it: `fresh_train_prefixes` (each a lowercase name and `:`, for example `["hard:"]` for the
hard-case lane of `tools/hard-cases`). Every other gate is unchanged; an id outside the named prefixes still fails
the round as nonteaching.

The idle policy remains one epoch, learning rate 0.00003, sampling 0.15 by default
(admissible 0.01..0.25), a new checked seed, bounded child deadlines, an explicit
fixed LAS floor and cooperative cooldown. A sample fraction is a work budget,
not an output-error percentage. Probe-idle does not guarantee the next trainer
lease. Deferred rounds preserve rejection streaks and may advance the counter
without training. Three rejections increase cooldown; they do not establish a
new rejection on each deferred round.

`evidence::inspect` accepts historical trainer/Eel receipts and separates
`deferred`, `below_threshold`, `rejected`, and `installed`. Recorded candidate or
installation evidence needs actual-parent/generation/output digest fields;
proposed-parent hashes cannot replace actual-parent lineage. Partial historical
receipts remain incomplete. This classifies declared local evidence, not artifact
authenticity, successful generalization or current deployed-weight identity.
New Eel summaries add this interpretation without changing existing fields.

Tests use finite synthetic CoNLL-U and fake native executables to exercise real
Rust callpaths, command arguments, locks, gates and installation. They perform no
operational training, teacher calls or protected-label assessment. A public
crate build and private adapter build are separate from deployment: an existing
running binary continues its previous implementation until explicitly replaced
at a verified quiescent boundary. See the source-bound groupants for limitations.

Optional urgent reviewed diagnostics use `TrainingData::priority`. `Reviewed`
bypasses ordinary polling/export and consumes a deployment-verified immutable
snapshot; `AwaitingReview` yields without ordinary training. An untrusted failure
queue must not be mapped blindly to AwaitingReview: only an operator-admitted
active urgent case may block ordinary work. Ambiguity can require root/consultant
review first. Backends rank deterministically and implement durable/idempotent
`priority_terminal`; the default acknowledgement refuses rather than silently
forgetting terminal work. Failed acknowledgement preserves the terminal native
receipt, and same-run retry cannot repeat compute. Eel propagates uncertainty and
pauses. Installation, queue acknowledgement and Eel summary are separate commits;
manual reconciliation may be required after a crash between them.
A matching data generation alone cannot acknowledge urgent work: the terminal
receipt must also carry the selected diagnostic identity. Historical ordinary
receipts therefore remain ordinary even if their data generation matches.

## Build and test

A standalone workspace (own `Cargo.lock`), not built by the root `cargo build`:

```bash
cargo build --release --manifest-path tools/eel/Cargo.toml
cargo test --release --manifest-path tools/eel/Cargo.toml
```

This crate is a library (`mova_eel`); a deployment links it and calls `mova_eel::run` as described above.
