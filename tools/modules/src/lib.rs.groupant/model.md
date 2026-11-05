# mova-modules-lib: add-on modules (manifest, roots, verify, install, remove, command)

## forward

`Manifest::parse` reads module.json (serde; unknown fields ignored) and `validate`s it: schema 1; name and version
safe names (`[A-Za-z0-9._-]`, not hidden); kind, licence and attribution text non-empty; files non-empty, each a
safe relative path (segments safe names), not `module.json`, a 64 lower-hex SHA-256, unique; every `provides` path a
listed file; dictionary layers named, unique, state draft/admitted/derived, admitted only with a listed review file;
`requires(requires_mova, MOVA_RELEASE="0.5")`. `roots()` = `MOVA_MODULES_PATH` entries then `data_dir()/modules`.
`discover(roots)` walks `<root>/<name>/<version>/module.json` in sorted order (hidden folders skipped), keeps the first
copy per (name, version) in a `BTreeMap`, records later copies as shadowed and invalid folders as problems
(`read_installed` requires the folder to equal `<root>/<manifest name>/<manifest version>`). `find` picks the named
version or the highest by `cmp_version`. `read_verified` checks size then SHA-256 of one listed file; `verify_dir`
checks all and returns unlisted files. Install: `check_admission` (optional pinned module.json SHA-256; nc-research
needs `accept_nc`), `check_root` (no Git work tree), then `install_with` stages into `<root>/.staging-…`, writes each
file through `put`, verifies it, writes the manifest bytes, renames to `<root>/<name>/<version>`; an existing identical
module.json is a verified no-op, a different one is refused; failures delete the staging folder. `install_dir` copies
from a verified folder; `install_url` fetches the manifest and `<base>/<path>` files (`fetch`: `file://` read, only
`https://` through curl with `--proto =https --proto-redir =https`). `remove` deletes a found module folder only when
it equals `<root>/<name>/<version>`, then an empty name folder. `install_torrent` runs aria2c if on PATH into
`<root>/.torrent-<pid>` and installs the unique downloaded module folder; otherwise fails with
`torrent_instructions`. `main_args` parses options and dispatches list/info/verify/install/install-torrent/
make-torrent/remove.

## invariant

- An installed module's folder is `<root>/<name>/<version>` and every listed file matched its size and SHA-256 at
  install time; nothing is moved into place before all files verify.
- No module root inside a Git work tree is installed into; plain HTTP is never fetched.
- A non-commercial / research-only module is installed only with explicit acceptance.
- Discovery and selection are deterministic (sorted folders, `BTreeMap`, version order, first root wins).
- `remove` touches only a folder that is a valid module at its canonical place.

## cost

F files, B bytes: validation O(F log F); install O(B) hashing plus copy or download; discovery O(roots × modules)
directory reads and manifest parses; verify O(B). Network time unmeasured.

## composition

Used by `world::dictstore` (`roots`, `find`, `discover`, `read_verified`, `split_ref`, `inside_git_worktree`,
`LicenceClass`) and by the binaries `mova` (`main.rs`) and `world modules`. `torrent::make` reuses `Manifest::parse`,
`verify_dir` and `read_verified`.

## behavior-complement

Does not: implement BitTorrent transfers; sign or authenticate modules beyond hashes (a pinned module.json hash is the
user's trust anchor); check that a licence label is true; resolve dependencies between modules; upgrade or garbage-
collect old versions; load module contents (the dictionary store does); fetch over plain HTTP; install into a Git
work tree.

## reverse-reconstruction

From an installed folder, the manifest and every file are recoverable exactly (they are the bytes installed); from a
module.json SHA-256, the manifest only by search; from a `find` result, the set of candidates is not recoverable (only
the highest version is returned). A failed install leaves no installed folder, so the pre-install state is unchanged
except an empty root folder.

## licence-admission

Constraint relation: (licence_class, accept_nc) ↦ admit. nc-research ∧ ¬accept_nc ↦ refuse; otherwise admit. The
class's `notice` is shown on install and by `info`; `taints` (cc-by-sa, nc-research) feeds the dictionary store's
derived-output mark.

## checks

`cargo test --release -p mova-modules` (manifest_rules_are_enforced, versions_and_requirements,
install_verify_list_remove, nc_needs_acceptance_and_a_repository_is_refused,
torrent_without_a_client_prints_instructions). Negative controls inside: a changed byte, a wrong pinned hash, a path
with `..`, an absolute path, a bad licence class, an unmet requirement, plain HTTP, a Git work tree root.

## epoch-memory

(https/file URL), `safe_rel`, `sha256_hex`, `data_dir`, `Manifest::parse` and `verify_dir` unchanged.

## History

### 2026-11-04 · epoch-memory

2026-11-04, first epoch. No BitTorrent client is installed on the development box (aria2c, transmission, mktorrent
absent); `install-torrent` was exercised only on its instruction path. An end-to-end run in a scratch data folder
installed, listed, verified and made a torrent of a demo module; installing into the repository was refused.

### 2026-11-05 · epoch-memory

2026-11-05: declares `pub mod fetch` (a release's files, `mova-modules-fetch`); `fetch` uses this module's `fetch`

2026-11-05 (review round 2): declares `pub mod oewn` (Open English WordNet install, `mova-modules-oewn`), which
uses `fetch::set_aside`, now `pub(crate)`.

### 2026-11-05 · public export

Doc comment of MOVA_RELEASE: the workspace version is 0.5.0. No code change.
