# mova-modules-command: the `mova` binary

## forward

Collects the arguments after the program name; `modules …` calls `mova_modules::main_args` with the rest, `fetch …`
calls `mova_modules::fetch::main_args` (a release's files), `oewn …` calls
`mova_modules::oewn::main_args` (Open English WordNet); anything else is a usage error. An error prints `mova: <chain>` to stderr and exits 1.

## invariant

No other effect than the called `main_args`; exit status 0 only on success.

## cost

Argument collection O(arguments) plus the called command.

## composition

Leaf adapter over `mova-modules-lib`, `mova-modules-fetch` and `mova-modules-oewn`; `world modules …` reaches the same `main_args`;
`install.sh` runs `mova fetch 0.5 model` and `mova oewn install`.

## behavior-complement

Does not parse options itself, read configuration, or provide subcommands other than `modules`, `fetch` and `oewn`.

## reverse-reconstruction

From the exit status, only success or failure; the failing step is recoverable from the stderr chain.

## checks

The library tests cover `main_args`' commands; the binary was exercised end to end (install, list,
info, verify, make-torrent, install-torrent without a client, install into a Git work tree refused).

## epoch-memory

`mova fetch …` dispatches to `mova_modules::fetch::main_args`; run live by https, torrent and magnet and
through `install.sh` (twice) in scratch data folders.

`mova oewn …` dispatches to `mova_modules::oewn::main_args`; `install.sh` runs it after
`mova fetch` and always builds the binary through Cargo.

## History

### 2026-11-04 · checks

The library tests cover `main_args`' commands; the binary was exercised end to end on 2026-11-04 (install, list,
info, verify, make-torrent, install-torrent without a client, install into a Git work tree refused).

### 2026-11-04 · epoch-memory

2026-11-04, first epoch.

### 2026-11-05 · forward

Collects the arguments after the program name; `modules …` calls `mova_modules::main_args` with the rest, `fetch …`
calls `mova_modules::fetch::main_args` (a release's files, 2026-11-05), `oewn …` calls
`mova_modules::oewn::main_args` (Open English WordNet); anything else is a usage error. An error prints `mova: <chain>` to stderr and exits 1.

### 2026-11-05 · epoch-memory

2026-11-05: `mova fetch …` dispatches to `mova_modules::fetch::main_args`; run live by https, torrent and magnet and
through `install.sh` (twice) in scratch data folders.

2026-11-05 (review round 2): `mova oewn …` dispatches to `mova_modules::oewn::main_args`; `install.sh` runs it after
`mova fetch` and always builds the binary through Cargo.
