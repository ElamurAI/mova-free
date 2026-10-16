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
- **Licences of data** ([licence/DATA.md](licence/DATA.md)): non-commercial and research-only data only for tests; CC BY-SA data quoted and tested on, never shipped or modified into a release.

## Build and test

```sh
export MOVA_DATA=~/mova-data
./install.sh
cargo build --release
tests/run.sh
```

## Sending a change

- Small, logical commits, signed off (`git commit -s`, the [Developer Certificate of Origin](https://developercertificate.org/)).
- In the pull request, say what changed and what it measured before and after.
- Contributions are accepted under the licences of the repository ([LICENSE.md](LICENSE.md), [licence/](licence)).

Questions: open an issue, or write to ai@elamur.ai.
