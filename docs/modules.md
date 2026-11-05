# Add-on modules

Mova can load **add-on modules**: data packages that are not part of Mova itself, each under its own licence. The
first kind is the dictionary layer (for example a dictionary built from Wiktionary, which is CC BY-SA). A module is
installed into the data directory; it is never in the repository, a release's core files or a binary. The core and
its built-in dictionaries (the pupils' dictionary and the commands dictionary, CC BY-SA 4.0 data) contain no
module data.

Code: crate `mova-modules` (`tools/modules`; the `mova` binary) and the dictionary store `world::dictstore`.

## What a module is

A folder `<root>/<name>/<version>/` holding `module.json` and the files it lists:

```json
{"schema": 1, "name": "example-dict", "version": "2026.11.04", "kind": "dictionary-layer",
 "licence": "CC-BY-SA-4.0", "licence_class": "cc-by-sa",
 "attribution": {"text": "From Example, CC BY-SA 4.0", "sources": ["https://example.org/"]},
 "files": [{"path": "layers/senses.tsv", "sha256": "…64 hex…", "bytes": 1234}],
 "provides": [{"kind": "dictionary-layer", "layer": "senses", "path": "layers/senses.tsv"}],
 "requires_mova": ">=0.5"}
```

| field | meaning |
|---|---|
| `schema` | 1 |
| `name`, `version` | `[A-Za-z0-9._-]`, not starting with a dot; the folder must be `<root>/<name>/<version>` |
| `kind` | what the module is, e.g. `dictionary-layer` |
| `licence` | SPDX-like text, e.g. `CC-BY-SA-4.0` |
| `licence_class` | `ours`, `cc-by-sa`, `cc-by`, `public-domain` or `nc-research` (below) |
| `attribution` | a text, or `{"text", "sources": [URL…]}` |
| `files` | every file except `module.json`: relative path (`[A-Za-z0-9._-]` segments, no `..`), SHA-256, size |
| `provides` | what Mova can load: `{"kind": "dictionary-layer", "layer": NAME, "path": REL}`; optional `"state"` (`draft`, the default, `admitted` or `derived`) and, for an admitted layer, `"review"` (a listed file holding its review record) |
| `requires_mova` | e.g. `>=0.5` or `>=0.5, <1`; compared with the module mechanism's release (`mova_modules::MOVA_RELEASE`, 0.5) |

Unknown fields are ignored, so the format grows compatibly. A dictionary layer file has the columns of the school
layers (word, part of speech, meaning for a pupil, sense in the lessons, examples, note; `#` lines are comments).

## Where modules live

Roots, searched in order: each folder of `MOVA_MODULES_PATH` (`:`-separated), then `$MOVA_DATA/modules` (`MOVA_DATA`
defaults to `~/mova-data`). The same name and version in two roots: the first root's copy is used and the other is
listed as shadowed. A root inside a Git work tree is refused.

## The command

```sh
mova modules list                                   # installed modules, licences, layers, folders
mova modules info NAME[@VERSION]                    # licence notice, attribution, what it provides
mova modules verify NAME[@VERSION]                  # every file's size and SHA-256 against module.json
mova modules install https://…/module.json          # fetch the manifest and the files next to it, check, install
mova modules install DIR                            # or from a module folder on disk
mova modules install-torrent FILE.torrent|MAGNET    # download with aria2c, then install
mova modules make-torrent DIR OUT.torrent [--tracker URL]… [--web-seed URL]…
mova modules remove NAME@VERSION
```

`world modules …` runs the same commands. Options: `--dir ROOT` installs into ROOT (and is searched first),
`--sha256 HASH` pins the module.json bytes, `--accept-nc` is needed for a non-commercial / research-only module.

- **install** from a URL: only `https://` (through `curl`, redirects kept on HTTPS) or `file://`; plain HTTP is
  refused. The files are fetched from the manifest's folder (`<base>/<path>`) into a staging folder, each checked
  against its size and SHA-256, and only then moved into `<root>/<name>/<version>`. Installing the same module.json
  again is a no-op; another module.json under the same name and version is refused. The licence notice and the
  attribution are printed on install and by `info`; `list` shows each module's licence and class.
- **install-torrent** uses `aria2c` when it is on `PATH` (`--seed-time=0`); without it, it prints the exact commands
  to download with any client and install the folder. Mova does not implement BitTorrent itself.
- **make-torrent** checks the folder against its module.json and writes a BitTorrent v1 metainfo file (BEP 3) of
  module.json and every listed file under the name `<name>-<version>`, with optional trackers (BEP 12) and HTTPS web
  seeds (BEP 19, so a client can fetch from the HTTPS server when no peer is online), and prints its magnet link. The
  output is deterministic: no creation date or client name. The piece length is a power of two from 256 KiB to 16 MiB,
  keeping at most about 2 000 pieces.
- **remove** deletes one installed version (the folder must be a valid module at `<root>/<name>/<version>`).

Publishing a module: put the module folder (module.json and its files, same relative paths) under an HTTPS folder,
give users the module.json URL (and its SHA-256 for `--sha256`), and optionally a .torrent made with `make-torrent
--web-seed URL/`. A client fetches a multi-file web seed as `<URL/><name>-<version>/<path>` (BEP 19), so the web seed
is the folder that holds a `<name>-<version>/` copy (or link) of the module folder; `make-torrent` refuses a web seed
that is not an `https://` URL ending in `/`.

## A release's files: `mova fetch`

The same binary downloads the files that come with a release beside the code (the trained model, the corpus, the
dictionaries). Add-on modules such as `wiktionary-en` are separate, each with its own manifest when it is published;
release 0.5 ships none. What they are, their sizes, SHA-256, licences, HTTPS URLs,
.torrent files and magnet links are in the release's manifest in the repository: [releases/0.5-files.md](../releases/0.5-files.md)
and the same data for programs, `releases/0.5-files.json`, which the 0.5 build of `mova` carries inside. A test
(`fetch::tests::the_shipped_manifest_follows_the_policy_and_the_bone_agrees`) checks that the Markdown page is the
rendering of the JSON and that every row follows the data policy (the model and the corpus CC BY-SA 4.0, the
dictionaries CC BY-SA 4.0 with citations, Wikipedia or Wiktionary data only as a separate module, only the model
required).

```sh
mova fetch 0.5                           # the required files (the model) over HTTPS; install.sh runs this
mova fetch 0.5 all --via torrent         # every file, from its .torrent
mova fetch 0.5 corpus --via magnet       # from the magnet link
mova fetch 0.5 corpus --archive corpus-0.5.tar.gz   # a file a BitTorrent client downloaded
```

Options: `--manifest FILE` (another release's manifest), `--base URL` (a mirror holding the same files),
`--keep-archive` (keep the `.tar.gz` under `$MOVA_DATA/release/<release>/`, e.g. to seed it).

Each way ends in the same check: the archive's size and SHA-256 must equal the manifest's, or it is refused and
nothing is unpacked. `torrent` first checks that the .torrent's info hash is the manifest's and, after the download from
the torrent's web seed, every piece's SHA-1; `magnet` takes the .torrent from the link's `xs` source and checks that it
hashes to the link's info hash. Then the archive (one top folder) is unpacked under `$MOVA_DATA/<unpack_to>/` (the
release's folder `release/0.5/`, or the module root for a module, which is then verified against its module.json),
and each `install` entry is copied into place (the model: `en/models/en.bin`, `en/models/gparse/`,
`coref/gender.tsv`). An earlier different file is kept beside it as `<name>.before-0.5`.

**What of BitTorrent Mova implements:** bencode, single-file v1 metainfo, the info hash, magnet links (`xt` btih in hex
or base32, `dn`, `xl`, `tr`, `ws`, `xs`), piece verification and BEP 19 web seeds over HTTPS. It does not implement the
peer wire protocol, trackers, the DHT or metadata exchange (BEP 9): to download from peers, use a BitTorrent client
with the .torrent or the magnet link and give its file to `--archive`. Our seeder (a client that implements all of
those) serves every release file; each torrent also has the HTTPS file as its web seed, so a client completes without
peers.

## Loading dictionary layers

The dictionary store (`world::dictstore`, [building the dictionary](training.md#building-the-dictionary)) loads module layers in two ways:

- `MOVA_MODULES=all` or `MOVA_MODULES=name[@version],…` adds every dictionary layer of the selected installed modules
  (highest version unless named) after the built-in layers or the `MOVA_DICT` manifest's layers. **Off by default**:
  without it the store, its fingerprint and every result are unchanged.
- a `MOVA_DICT` manifest names a layer `{"id": "wikt", "path": "module:NAME[@VERSION]/LAYER", "state": "draft",
  "licence": "CC-BY-SA-4.0", "scope": "module"}`. The `licence` must repeat the module's licence (or its class); an
  optional `sha256` pins the module's module.json; `admitted` needs the module to declare that layer admitted.

On every load the layer file is checked against the module's SHA-256. Each entry keeps its provenance through its
layer: module, version, licence, class and attribution (`world dict lookup` prints them, `world dict stats` prints
the attribution). As with every layer, a draft is loaded and shown but is knowledge only with `allow_unreviewed`.
The store's fingerprint names each module and version, so a retained state learned with a module is examined only
with the same module.

A layer may carry columns after the six of the school layers. The store keeps them per entry as one text
(`Entry::extra`) and reads them by name (`Entry::col`, `Entry::terms`; `dictstore::EXTRA_COLUMNS`: labels, synonyms,
antonyms, forms, form of, gaps, provenance). The school layers have none, so every reader of the six columns sees the
same entries.

## Modules Mova makes

| module | made by | licence | layer |
|---|---|---|---|
| `wiktionary-en` | `tools/wikt-extract build` from the raw English Wiktionary dump ([data.md](data.md), «Wiktionary») | `CC-BY-SA-4.0 AND GFDL-1.3-or-later`, class `cc-by-sa`; attribution to the Wiktionary contributors with the dump date | `wiktionary-en` (draft): the English entries, one row per part of speech, with the extra columns |

```sh
wikt-extract build $MOVA_DATA/raw/wiktionary/enwiktionary/20260901 $MOVA_DATA/modules/wiktionary-en/20260901-1
mova modules verify wiktionary-en
MOVA_MODULES=wiktionary-en world dict lookup telly
```

## Licence classes and gates

| class | loads | what Mova derives from it |
|---|---|---|
| `ours`, `public-domain` | always | ours |
| `cc-by` | always; the attribution is shown | ours, crediting the source when shared |
| `cc-by-sa` | always (the user's run; the module is not shipped by Mova) | marked with the module's licence; never in the repository or shipped files |
| `nc-research` | only for internal runs and measurements (`MOVA_DICT_USE=internal`); install needs `--accept-nc` | marked; never shipped |

The gates, each with a red control in the tests:

- a built-in layer is compiled from the repository's own files and never from a module; `shippable` refuses a text
  whose header declares CC BY-SA or a non-commercial licence, and every built-in text passes it;
- what Mova writes from a store holding a share-alike or non-commercial module carries the header line of
  `Store::derived_header` (`# licence: derived with add-on modules under CC-BY-SA-4.0 (module NAME VERSION; cc-by-sa)…`),
  which `shippable` refuses, and `Store::guard_derived_write` refuses such a file inside a Git work tree or under a
  built-in dictionary's file name. Today this covers the retained states (`world knowledge retain train|practice`,
  whose state also records the licence) and their exported word meanings (`world knowledge retain meanings`);
- a module root inside a Git work tree is refused on install.

## Tests

`cargo test --release -p mova-modules` (manifest rules, versions, install/verify/remove, pinned hashes, HTTP refused,
the release manifest and its page, `mova fetch` by HTTPS, torrent and magnet with a changed byte refused by each,
NC acceptance, Git work tree refused, bencode, a deterministic torrent whose pieces and info hash are recomputed) and
`cargo test --release -p world --lib dictstore` (`module_layers_keep_provenance_and_licence_gates`,
`derived_output_of_a_share_alike_module_never_ships`). Red controls run on 2026-11-04: with `guard_derived_write`
disabled, with the non-commercial check of `shippable` removed, and with the research-only purpose check removed,
the dictstore tests fail; restored, they pass.

## Limits

- The module mechanism trusts a module's licence label; it checks hashes, not provenance.
- Only dictionary layers are loaded so far; other kinds can be installed and verified but nothing reads them.
- The derived-output mark covers the retained states and their meanings export; a new writer of dictionary-derived
  files must call `guard_derived_write` and write `derived_header` itself.
- Torrents are BitTorrent v1 only; `install-torrent` needs `aria2c`; `mova fetch` reads single-file torrents and
  downloads only from their web seeds.
- `requires_mova` is checked against the module mechanism's release (0.5, the workspace version 0.5.0).
