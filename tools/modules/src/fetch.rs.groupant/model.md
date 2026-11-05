# mova-modules-fetch: a release's files from its manifest

## forward

`ReleaseFiles::parse` reads `releases/<release>-files.json` (0.5 compiled in) and `check`s it: schema 1; an https
folder `base_url`; per file a unique id and name ending `.tar.gz`, 64-hex SHA-256 and 40-hex btih, `url = base_url +
name`, `torrent_url = url + ".torrent"`, `magnet = Magnet{btih, dn = name, xl = bytes, tr = trackers, ws = [url], xs =
[torrent_url]}.link()`, safe relative `top`/`unpack_to`/install paths, every install source among the contents,
`raw_bytes = Σ contents.bytes`; and the data policy: model, corpus and dictionaries `CC-BY-SA-4.0` (data, not code), no data file under
the code licence (`LicenseRef-Elamur-Cosmic`), a module not under it, not required, unpacked into `modules` with no install entries, no
`wiki`/`wikt` name outside a module, `required ⇔ kind = model`. `render_bone` makes the Markdown page from it (its usage block calls `target/release/mova fetch …` from the
repository root and gives the PATH line); `mova fetch [RELEASE] --page [--manifest FILE]` prints it.

`fetch_one` downloads into `$MOVA_DATA/release/<r>/<name>.part` by one way: `https` (curl, `--base` rebases URLs);
`torrent` (fetch the .torrent, require `info_hash = btih`, `length = bytes`, `name = name`, download from its first
`url-list` web seed, `verify_file` every piece); `magnet` (parse the link, require its btih to be the file's, take the
first https/file `xs`, then as torrent). `--archive FILE` skips the download. Then size and SHA-256 must equal the
manifest's, else the part file is removed and the call fails. `unpack_and_install`: `tar -tzf` members must lie in
`top` with no absolute or `..` path. A cached `$MOVA_DATA/<unpack_to>/<top>` is reused only when its stamp
`<top>.fetched` names the archive SHA-256 **and** `check_inventory` passes: its regular files are exactly the
manifest's `contents` (no missing, no extra anywhere in the tree, no links or special files), each with the manifest
size and SHA-256. Otherwise the verified archive is unpacked into a staging folder, which must pass the same
`check_inventory` (else the staging folder is removed and the call fails), then moved to `$MOVA_DATA/<unpack_to>/<top>`
(an existing different one, e.g. a tampered cache, renamed `.before-<r>`), stamp `<top>.fetched`; a module is
verified against its module.json and its licence must equal the manifest's; each install entry is copied (a file or a
folder) unless its tree hash already matches, an existing different target renamed `.before-<r>[-n]`.

## invariant

- Nothing is unpacked or installed unless the archive's size and SHA-256 equal the manifest's.
- Every installed byte comes from a folder whose inventory equals the manifest's `contents` exactly (paths, sizes,
  SHA-256, no extra files): a fresh extraction and a reused cache pass the same `check_inventory`.
- By `torrent` and `magnet`, additionally: the .torrent hashes to the manifest's btih and every piece's SHA-1 matches.
- No existing file under the data directory is deleted: a replaced one is renamed `.before-<release>`.
- The shipped manifest passes `check`, and `releases/0.5-files.md` = `render_bone(manifest)` byte for byte.

## cost

B archive bytes: download O(B) network; SHA-1 pass O(B) (torrent, magnet); SHA-256 O(B); tar list + unpack O(B);
content SHA-256 O(raw); installs O(installed bytes) copy plus tree hashes of source and target. Memory: one piece
(≤ 16 MiB) or 1 MiB buffers; the .torrent in memory.

## composition

`main.rs` dispatches `mova fetch`; uses `crate::torrent` (Magnet, read_metainfo, verify_file), `crate::fetch`
(the .torrent over https), `crate::safe_rel`, `crate::sha256_hex`, `crate::Manifest::parse` and `crate::verify_dir`
(modules), `crate::data_dir`. `install.sh` runs `mova fetch 0.5 model`. an internal release tool (not in the public tree) writes the manifest
and the page with `render_bone`.

## behavior-complement

Does not: speak the peer wire protocol, contact trackers or the DHT, exchange metadata (BEP 9), read multi-file or v2
torrents, resume a partial download, check signatures beyond the hashes in the repository's manifest, delete earlier
installs, or decide licences (it checks the manifest's declared licences against fixed policy rules only).

## reverse-reconstruction

From an installed `$MOVA_DATA/<to>` with a `.before-<r>` sibling, both the new and the earlier content are present, so
the install is reversible by renaming. From the stamp `<top>.fetched` the archive's SHA-256 is recovered exactly, hence
(by the manifest) its name, size and btih (the stamp alone does not vouch for the bytes beside it; the inventory
check does); a cache that failed the check survives as `<top>.before-<r>`; the way it was fetched is not recorded. From a refusal the expected and
observed size/SHA-256 are printed; the bad bytes are not kept.

## checks

`cargo test --release -p mova-modules fetch` (the_shipped_manifest_follows_the_policy_and_the_bone_agrees,
policy_rows_go_red, fetch_by_each_way_installs_and_a_tampered_byte_is_refused,
a_tampered_cached_extraction_is_unpacked_again_never_installed). Negative controls: a model under our
licence, dictionaries under the code licence, a module under our licence, an optional model, Wiktionary contents outside a module,
a magnet without `xs`, a changed SHA-256 in the page, one flipped byte of the served archive (refused by https,
torrent and magnet, nothing installed), a manifest SHA-256 that does not match, a magnet of another info hash; a cached extraction with changed bytes (same
size and other size), a missing file, an extra file in a copied directory and an extra top-level file (each with the
installed copy removed, so trusting the cache would install its bytes) is unpacked again and only manifest bytes are
installed; an archive holding an unlisted file is refused on a fresh unpack. Manual
mutations: one licence cell of the page and one licence row of the JSON each turn the shipped-manifest
test red; restored, green.

## epoch-memory

First epoch. Live run against https://research.elamur.ai/download/0/0.5/: `mova fetch 0.5 all` by https,
torrent and magnet each installed the four files in 7–9 s (local network), module verified; `install.sh` twice
(second run: up to date); `--archive` with a file a BitTorrent client (rqbit) downloaded from our seeder. The
web-seed way needed byte ranges on the server only for real BitTorrent clients; `mova fetch` downloads the whole file
from the web seed and checks pieces afterwards.
The operator put data under CC BY-SA 4.0 and only code under the Elamur Cosmic License; the dictionaries row of releases/0.5-files.json moved from `LicenseRef-Elamur-Cosmic` to `CC-BY-SA-4.0` together with this gate, and the negative control flipped (dictionaries under the code licence are refused). `cargo test -p mova-modules`: 13 passed.

A cached extraction was trusted by its archive-hash stamp alone, so changed cached
bytes could be installed after a genuine download. `check_inventory` now gates both a fresh extraction and reuse
(exact file set, sizes, SHA-256; checked: the three 0.5 archives hold exactly their manifest contents, 10/35/4 files).
Red control: reuse on the stamp alone (`stamped || …`) turns `a_tampered_cached_extraction_is_unpacked_again_never_installed`
red; restored, green. The page's usage now names `target/release/mova` and the PATH line; `--page` renders it.
`cargo test --release --locked -p mova-modules`: 17 passed, twice.

## History

### 2026-11-05 · forward

`ReleaseFiles::parse` reads `releases/<release>-files.json` (0.5 compiled in) and `check`s it: schema 1; an https
folder `base_url`; per file a unique id and name ending `.tar.gz`, 64-hex SHA-256 and 40-hex btih, `url = base_url +
name`, `torrent_url = url + ".torrent"`, `magnet = Magnet{btih, dn = name, xl = bytes, tr = trackers, ws = [url], xs =
[torrent_url]}.link()`, safe relative `top`/`unpack_to`/install paths, every install source among the contents,
`raw_bytes = Σ contents.bytes`; and the data policy: model, corpus and dictionaries `CC-BY-SA-4.0` (data, not code; 2026-11-05), no data file under
the code licence (`LicenseRef-Elamur-Cosmic`), a module not under it, not required, unpacked into `modules` with no install entries, no
`wiki`/`wikt` name outside a module, `required ⇔ kind = model`. `render_bone` makes the Markdown page from it (its usage block calls `target/release/mova fetch …` from the
repository root and gives the PATH line); `mova fetch [RELEASE] --page [--manifest FILE]` prints it.

`fetch_one` downloads into `$MOVA_DATA/release/<r>/<name>.part` by one way: `https` (curl, `--base` rebases URLs);
`torrent` (fetch the .torrent, require `info_hash = btih`, `length = bytes`, `name = name`, download from its first
`url-list` web seed, `verify_file` every piece); `magnet` (parse the link, require its btih to be the file's, take the
first https/file `xs`, then as torrent). `--archive FILE` skips the download. Then size and SHA-256 must equal the
manifest's, else the part file is removed and the call fails. `unpack_and_install`: `tar -tzf` members must lie in
`top` with no absolute or `..` path. A cached `$MOVA_DATA/<unpack_to>/<top>` is reused only when its stamp
`<top>.fetched` names the archive SHA-256 **and** `check_inventory` passes: its regular files are exactly the
manifest's `contents` (no missing, no extra anywhere in the tree, no links or special files), each with the manifest
size and SHA-256. Otherwise the verified archive is unpacked into a staging folder, which must pass the same
`check_inventory` (else the staging folder is removed and the call fails), then moved to `$MOVA_DATA/<unpack_to>/<top>`
(an existing different one, e.g. a tampered cache, renamed `.before-<r>`), stamp `<top>.fetched`; a module is
verified against its module.json and its licence must equal the manifest's; each install entry is copied (a file or a
folder) unless its tree hash already matches, an existing different target renamed `.before-<r>[-n]`.

### 2026-11-05 · checks

`cargo test --release -p mova-modules fetch` (the_shipped_manifest_follows_the_policy_and_the_bone_agrees,
policy_rows_go_red, fetch_by_each_way_installs_and_a_tampered_byte_is_refused,
a_tampered_cached_extraction_is_unpacked_again_never_installed). Negative controls: a model under our
licence, dictionaries under the code licence, a module under our licence, an optional model, Wiktionary contents outside a module,
a magnet without `xs`, a changed SHA-256 in the page, one flipped byte of the served archive (refused by https,
torrent and magnet, nothing installed), a manifest SHA-256 that does not match, a magnet of another info hash; a cached extraction with changed bytes (same
size and other size), a missing file, an extra file in a copied directory and an extra top-level file (each with the
installed copy removed, so trusting the cache would install its bytes) is unpacked again and only manifest bytes are
installed; an archive holding an unlisted file is refused on a fresh unpack. Manual
mutations 2026-11-05: one licence cell of the page and one licence row of the JSON each turn the shipped-manifest
test red; restored, green.

### 2026-11-05 · epoch-memory

2026-11-05, first epoch. Live run against https://research.elamur.ai/download/0/0.5/: `mova fetch 0.5 all` by https,
torrent and magnet each installed the four files in 7–9 s (local network), module verified; `install.sh` twice
(second run: up to date); `--archive` with a file a BitTorrent client (rqbit) downloaded from our seeder. The
web-seed way needed byte ranges on the server only for real BitTorrent clients; `mova fetch` downloads the whole file
from the web seed and checks pieces afterwards.
Epoch 2026-11-05: the operator put data under CC BY-SA 4.0 and only code under the Elamur Cosmic License; the dictionaries row of releases/0.5-files.json moved from `LicenseRef-Elamur-Cosmic` to `CC-BY-SA-4.0` together with this gate, and the negative control flipped (dictionaries under the code licence are refused). `cargo test -p mova-modules`: 13 passed.

Epoch 2026-11-05 (review round 2): a cached extraction was trusted by its archive-hash stamp alone, so changed cached
bytes could be installed after a genuine download. `check_inventory` now gates both a fresh extraction and reuse
(exact file set, sizes, SHA-256; checked: the three 0.5 archives hold exactly their manifest contents, 10/35/4 files).
Red control: reuse on the stamp alone (`stamped || …`) turns `a_tampered_cached_extraction_is_unpacked_again_never_installed`
red; restored, green. The page's usage now names `target/release/mova` and the PATH line; `--page` renders it.
`cargo test --release --locked -p mova-modules`: 17 passed, twice.

### Undated · reverse-reconstruction

From an installed `$MOVA_DATA/<to>` with a `.before-<r>` sibling, both the new and the earlier content are present, so
the install is reversible by renaming. From the stamp `<top>.fetched` the archive's SHA-256 is recovered exactly, hence
(by the manifest) its name, size and btih (the stamp alone does not vouch for the bytes beside it; the inventory
check does); a cache that failed the check survives as `<top>.before-<r>`; the way it was fetched is not recorded. From a refusal the expected and
observed size/SHA-256 are printed; the bad bytes are not kept.

### 2026-11-05 · school data outside the repository

kind school (CC-BY-SA-4.0, like model, corpus and dictionaries) is admitted; render_bone lists a file with more than RENDER_CONTENTS_MAX (200) contents per top folder (files, bytes), the JSON manifest keeping every file with its SHA-256. The 0.5 manifest test checks the school row, the summary, that no per-file row of it is rendered and that the school under the code licence is refused.
