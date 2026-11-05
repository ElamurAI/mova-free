# add-on-modules: modules installed apart from Mova and read by the dictionary store

## forward

The `mova` binary and `world modules` dispatch to `mova_modules::main_args`; install paths produce verified folders
`<root>/<name>/<version>`. The dictionary store (`world::dictstore`, child `world-dictstore` of lexical-semantics)
turns `module:` layer specs and `MOVA_MODULES` selections into layers through `mova_modules::find` and
`read_verified`, carrying `ModuleSource` provenance; `derived_licence`/`derived_header`/`guard_derived_write` mark and
fence what retained states (`world::knowledge::retention`) write. `torrent::make` publishes a verified folder.

## invariant

- Built-in dictionary layers are compiled from repository files only; no module data reaches them or a binary.
- A module layer's bytes equal its module's SHA-256 at every load; an installed folder verified at install.
- Share-alike / non-commercial derived outputs carry the module licence and are refused inside Git work trees.
- With `MOVA_MODULES` unset and no `module:` layer, the store, its fingerprint and retained states are byte-identical
  to the store before modules existed (the new state field is skipped when empty).

## cost

Install O(bytes) hashing and copy/download; load O(layer bytes) hashing per process; torrent O(bytes) SHA-1.

## behavior-complement

The composition does not: auto-load installed modules; ship, copy or link module data into the repository; trust
module content beyond hashes and declared licence; implement BitTorrent transfers; mark derived outputs other than
retained states and their meaning exports (other writers must call the guard).

## reverse-reconstruction

From a loaded store, every entry's module, version, licence and attribution are recoverable from its layer; from a
fingerprint, the module set only by search over candidate installs; from a marked output, the module names and
versions from its header line.

## licence-flow

Constraint relation over (licence_class, purpose, output path): nc-research ∧ release purpose ↦ refuse load;
taints(class) ∧ output inside a Git work tree or named as a built-in file ↦ refuse write; taints(class) ↦ header
mark, which `shippable` refuses for any built-in text.

## checks

`cargo test --release -p mova-modules`; `cargo test --release -p world --lib dictstore`;
`cargo test --release -p world --lib retention`. Red controls: guard disabled, NC header check removed,
research-only purpose check removed ⇒ dictstore tests fail; restored ⇒ pass.

## epoch-memory

`mova-modules-fetch` joins: `mova fetch` reads the release manifest in the repository and installs a
release's files (the model archive, the corpus, the dictionaries, add-on modules) by HTTPS, .torrent or magnet link;
a module archive lands in the module root and is verified as any installed module. The torrent maker gained
single-file torrents for release archives.

`mova-modules-oewn` joins: `mova oewn install` puts Open English WordNet 2025 into
(see History) through staging, an exact inventory bound to the zip SHA-256 and a load check;
`install.sh` calls it. `mova fetch` reuses a cached unpacked archive only after the same kind of exact inventory
check against the manifest contents. CI runs `cargo test --release --locked -p mova-modules`, and
`tests/ci-standalone-list.sh` goes red without that lane.

## History

### 2026-11-04 · checks

`cargo test --release -p mova-modules`; `cargo test --release -p world --lib dictstore`;
`cargo test --release -p world --lib retention`. Red controls 2026-11-04: guard disabled, NC header check removed,
research-only purpose check removed ⇒ dictstore tests fail; restored ⇒ pass.

### 2026-11-04 · epoch-memory

2026-11-04, first epoch: the mechanism exists for a Wiktionary dictionary module built separately; the two public
dictionaries were audited for CC BY-SA/NC text (none copied; a Pinocchio translation's status reported to the
operator).

### 2026-11-05 · epoch-memory

2026-11-05: `mova-modules-fetch` joins: `mova fetch` reads the release manifest in the repository and installs a
release's files (the model archive, the corpus, the dictionaries, add-on modules) by HTTPS, .torrent or magnet link;
a module archive lands in the module root and is verified as any installed module. The torrent maker gained
single-file torrents for release archives.

2026-11-05 (review round 2): `mova-modules-oewn` joins: `mova oewn install` puts Open English WordNet 2025 into
`$MOVA_DATA/raw/en-oewn/json` through staging, an exact inventory bound to the zip SHA-256 and a load check;
`install.sh` calls it. `mova fetch` reuses a cached unpacked archive only after the same kind of exact inventory
check against the manifest contents. CI runs `cargo test --release --locked -p mova-modules`, and
`tests/ci-standalone-list.sh` goes red without that lane.
