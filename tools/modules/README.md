# mova-modules

Installs Mova's add-on modules: hash-pinned data packages under their own licences (for example dictionaries), never part of the repository or the core binaries. A module goes into `$MOVA_DATA/modules`, each file checked against its SHA-256; `MOVA_MODULES=<name>` lets the dictionary store load a module's layers. The format and the published modules are described in [docs/modules.md](../../docs/modules.md).

The binary is `mova`, the user-facing entry, with three subcommands: `modules`, `fetch` (a release's files, [releases/0.5-files.md](../../releases/0.5-files.md)) and `oewn` (Open English WordNet 2025 for the world model). `install.sh` builds it and runs `fetch` and `oewn install`.

```bash
cargo build --release --locked -p mova-modules
target/release/mova modules list
target/release/mova modules install <module.json URL>
target/release/mova fetch 0.5 corpus
target/release/mova oewn check
```

The other `modules` subcommands are `info`, `verify`, `install-torrent`, `make-torrent` and `remove`. `mova fetch --page` prints the release-files page from the manifest.

An unpacked release archive must hold exactly the manifest's `contents` (sizes, SHA-256, no other files); a cached unpacked folder is reused only after the same check, otherwise the verified archive is unpacked again. `mova oewn install` checks the zip's SHA-256, unpacks it into a staging folder, requires exactly the files of `oewn-2025-json.tsv` (an inventory bound to that SHA-256) and a successful load (every sense resolves to a synset), and only then renames the folder to `$MOVA_DATA/raw/en-oewn/json`; an installed folder is reused only when it passes the same checks.
