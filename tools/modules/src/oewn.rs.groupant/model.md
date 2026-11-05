# mova-modules-oewn: Open English WordNet 2025 installed by inventory and load

## forward

`Inventory::parse` reads `tools/modules/oewn-2025-json.tsv` (compiled in): a header `# zip-sha256 <hex>` and one
`path<TAB>bytes<TAB>sha256` row per file (73 flat JSON files). `Inventory::oewn_2025` requires the header to equal
`ZIP_SHA256` (the zip of the OEWN 2025 GitHub release), so the file list is bound to that zip. `check_dir(dir)`
requires the set of entries of `dir` to equal the inventory's names, every entry a regular file (no links, no
folders), each with the inventory's size and SHA-256. `check_load(dir)` reads the folder as `world::wordnet::load`
does — `entries-*.json` give lemma → part of speech → `sense[].synset`, the other JSON files but `frames.json` give
synset ids — and requires every file to parse as a JSON object, at least one sense, and every sense's synset present.

`install(inv, root, url, zip)`: if `root/json` exists and passes `check_dir` and `check_load`, it is reused.
Otherwise the zip is the one given, or `root/english-wordnet-2025-json.zip` if its SHA-256 is the inventory's, or
downloaded (`fetch::download_to`: https through curl, or file://) into `<zip>.part`, checked and renamed. Its
SHA-256 must equal the inventory's; `unzip -q -o` unpacks it into `root/.json-staging-<pid>`; the staging folder
must pass `check_dir` and `check_load` (else it is removed and the call fails, nothing installed); an existing
`json/` is renamed `json.before-oewn-2025[-n]` (`fetch::set_aside`); the staging folder is renamed to `json/`.

## invariant

- `root/json` is created or replaced only by one rename of a folder that passed both checks against the inventory
  bound to `ZIP_SHA256`; a partial unpacking never sits at `root/json`.
- An existing `root/json` is reused only when it passes both checks; an interrupted earlier unpacking (the old
  `entries-a.json` sentinel present, other files missing), a truncated, changed, missing or extra file each fail them.
- No folder is deleted except this run's own staging folder after a failed check.

## cost

OEWN 2025: zip 9 986 555 bytes, unpacked 72 404 635 bytes in 73 files. Download O(Z); SHA-256 O(Z) and O(U);
unzip O(U); `check_load` parses every file, O(U) time and peak memory of the largest file's JSON value plus the
synset id set (about 107 519 ids). Measured: `mova oewn check` on the real folder 0.6 s.

## composition

`main.rs` dispatches `mova oewn install|check|inventory`; `install.sh` runs `mova oewn install` (with `--url` from
`MOVA_OEWN_URL`). Uses `crate::fetch` (`download_to`, `sha256_file`, `set_aside`), `crate::safe_rel`,
`crate::sha256_hex`, `crate::data_dir`. The reader it guards is `world::wordnet` (world depends on mova-modules, not
the reverse, so the load check restates the reader's file roles rather than calling it).

## behavior-complement

Does not: install other OEWN editions or formats; repair a partial folder in place; delete earlier folders, the kept
zip or staging folders of killed runs (those are never read); verify the meaning of the data beyond resolvable senses;
use the hypernym links or definitions in the check (the reader tolerates missing ones); work without `unzip`.

## reverse-reconstruction

A passing folder conforms to the file inventory associated by the compiled manifest with ZIP_SHA256; its contents
alone do not reconstruct the archive bytes or establish their provenance. A downloaded ZIP is retained beside the
folder; an externally supplied ZIP remains at its supplied path, and reuse of a valid folder does not require a ZIP. An earlier `json/` survives as
`json.before-oewn-2025[-n]`. The URL used is not recorded. From a refusal the failing file or synset is printed.

## checks

`cargo test --release --locked -p mova-modules oewn`: `the_shipped_inventory_is_bound_to_the_zip_sum` (73 files,
round trip; an inventory of another zip is not taken as the 2025 one; a row without the header is refused);
`an_interrupted_or_damaged_extraction_is_never_taken_for_an_installation` (fixture zip of stored entries: an
interrupted unpacking with only `entries-a.json` plus a stale staging folder is replaced and kept aside; a complete
folder is reused with an unreachable URL; truncated, same-size changed, missing and extra files are each unpacked
again from the kept zip); `a_wrong_zip_or_one_that_does_not_load_installs_nothing` (a zip whose SHA-256 is not the
inventory's; a zip that matches its inventory but names a missing synset; an archive with an unlisted file — none
creates `json/`, no staging folder left). Red control: reuse on the old sentinel (`entries-a.json`
exists) turns the interrupted-extraction test red; restored, green.

## epoch-memory

First epoch (review round 2): `install.sh` accepted `entries-a.json` alone as an installed WordNet and
unzipped into the final folder, so an interrupted unpacking became a lasting incomplete installation that
`world::wordnet` loads silently (it skips unresolved senses). The install moved into Rust: staging, the exact
inventory generated with `mova oewn inventory` from the real zip (SHA-256 (see History)), the load check, one
rename. The real folder on this host: 128 009 lemmas, 185 129 senses, 107 519 synsets, all senses resolve.

## History

### 2026-11-05 · cost

OEWN 2025: zip 9 986 555 bytes, unpacked 72 404 635 bytes in 73 files. Download O(Z); SHA-256 O(Z) and O(U);
unzip O(U); `check_load` parses every file, O(U) time and peak memory of the largest file's JSON value plus the
synset id set (about 107 519 ids). Measured 2026-11-05: `mova oewn check` on the real folder 0.6 s.

### 2026-11-05 · checks

`cargo test --release --locked -p mova-modules oewn`: `the_shipped_inventory_is_bound_to_the_zip_sum` (73 files,
round trip; an inventory of another zip is not taken as the 2025 one; a row without the header is refused);
`an_interrupted_or_damaged_extraction_is_never_taken_for_an_installation` (fixture zip of stored entries: an
interrupted unpacking with only `entries-a.json` plus a stale staging folder is replaced and kept aside; a complete
folder is reused with an unreachable URL; truncated, same-size changed, missing and extra files are each unpacked
again from the kept zip); `a_wrong_zip_or_one_that_does_not_load_installs_nothing` (a zip whose SHA-256 is not the
inventory's; a zip that matches its inventory but names a missing synset; an archive with an unlisted file — none
creates `json/`, no staging folder left). Red control 2026-11-05: reuse on the old sentinel (`entries-a.json`
exists) turns the interrupted-extraction test red; restored, green.

### 2026-11-05 · epoch-memory

2026-11-05, first epoch (review round 2): `install.sh` accepted `entries-a.json` alone as an installed WordNet and
unzipped into the final folder, so an interrupted unpacking became a lasting incomplete installation that
`world::wordnet` loads silently (it skips unresolved senses). The install moved into Rust: staging, the exact
inventory generated with `mova oewn inventory` from the real zip (SHA-256 `7d749f6e…4b51`), the load check, one
rename. The real folder on this host: 128 009 lemmas, 185 129 senses, 107 519 synsets, all senses resolve.
