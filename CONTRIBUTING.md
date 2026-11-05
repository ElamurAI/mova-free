# Contributing to Mova

Thank you for helping. Mova is small on purpose, so a good change is usually small too.

## Before you start

- Read [README.md](README.md) and [docs/architecture.md](docs/architecture.md). AI agents start with [AGENTS.md](AGENTS.md).
- Open an issue first for anything bigger than a fix, so we can agree on how it will be measured.

## How we work

- **English everywhere** in code, comments, docs and commit messages.
- **Mechanics in Rust.** Grep for what exists before writing new code.
- **Deterministic.** Use `BTreeMap`/`BTreeSet` where order matters. Run twice: the outputs must be identical.
- **Measure before you claim.** State every number with its command and date. Decide on the training splits and the two-fold check (`cv`); the held-out tests are run once per release.
- **Negative controls.** A gate that cannot show red is not a gate.
- **Licences** ([licence/DATA.md](licence/DATA.md)): the Elamur Cosmic License covers the code only; data products are CC BY-SA 4.0 (or a source's stricter terms), each folder with a `LICENSE.md` that cites its sources; non-commercial and research-only data only for tests, never shipped.
- **Commits** in logical steps, each after `./secret-guard.sh` (API-key patterns in the staged diff; optionally the strings of an untracked `local.md`). Its negative control is `tests/secret-guard-control.sh`.

## Build and test

```sh
export MOVA_DATA=~/mova-data
./install.sh
cargo build --release
tests/run.sh
```

The worker tools under `tools/` with their own `[workspace]` (Crab, Octopus, Eel, Whale, the Sea Spider, the Fish
crates, the agent tools) are not built by the root `cargo build`. Test the one you change as CI does, for example
`cargo test --manifest-path tools/crab/Cargo.toml --release --locked`; `tests/ci-standalone-list.sh` checks that CI
lists every such workspace.

## Sending a change

- Small, logical commits, signed off (`git commit -s`, the [Developer Certificate of Origin](https://developercertificate.org/)).
- In the pull request, say what changed and what it measured before and after.
- Contributions are accepted under the licences of the repository ([LICENSE.md](LICENSE.md), [licence/](licence)).

Questions: open an issue or write to ai@elamur.ai.
