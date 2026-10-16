# vfs — file system for the execution sandbox

**Goal** (`docs/context.md`, "Execution sandbox"): guest programs, including untrusted and generated code, see only
virtual files in memory. Only an approved result goes outside. Every SLM (small language model) lesson starts from a
clean snapshot in nanoseconds. Design note: the small model could write a virtual file system itself and embed it by
rewriting code to WASM/WASI.

A crate without dependencies, std only. Three implementations of the `FileSystem` trait:

| type | what it is | when |
|---|---|---|
| `Vfs` | in-memory tree with snapshots, limits, a journal and a virtual clock | default: untrusted and generated code (`--fs mem`) |
| `DirFs` | a real directory, the root is the boundary (like chroot) | working with operator data (`--fs dir:<dir>`) |
| `RealFs` | unrestricted real FS | explicit only, for backward compatibility (`--fs real`) |

Alongside it — `wasi-host`: runs `wasm32-wasip1` modules (like `mlab.wasm`), and their WASI calls go into the same `Vfs`
(see "WASI host" below).

## What `Vfs` can do

- **In-memory tree:** directories, files, symlinks. Metadata: size, inode, permissions (`0o644`; the owner's read and
  write bits are checked), `mtime`/`ctime` from the virtual clock.
- **Paths:** absolute, and relative to the current directory (`chdir`), `.` and `..`, multiple `/` in a row. Links
  in the middle of a path are resolved physically, as in POSIX, up to 40 hops (then `ELOOP`). Bounds: `NAME_MAX` 255,
  `PATH_MAX` 4096, NUL byte — `EINVAL`, empty path — `ENOENT`.
- **No escaping the root:**
  - `..` above the root is an `ENOTCAPABLE` error with the explanation "path escapes the sandbox root: '…'", not a
    silent "sticking" to the root;
  - a link leading above the root (`../../..`, `/../x`) gives the same error at access time;
  - an absolute path is always inside this tree: `/etc/passwd` in memory is "No such file or directory", the host
    cannot be reached.

  Every attempt goes into the journal (the `denied` counter).
- **Copy-on-write snapshots:**
  - nodes are immutable and shared via `Arc`: directory keys are `Arc<str>`, contents are `Arc<Vec<u8>>`;
  - `snapshot()` and `restore()` are O(1): it is the root `Arc` plus counters;
  - the first change after a snapshot copies only the nodes on the path from the root. File contents are copied only
    if the file is appended to or edited in place; a file overwritten entirely is not copied;
  - `Snapshot` is `Send + Sync`: one snapshot can be restored in many threads (`Vfs::from_snapshot`).
- **Byte-exact restore:** `dump()` is a canonical byte-level description of the state (counters, nodes, metadata,
  contents), `fingerprint()` is FNV-1a 64 of it. Gate: snapshot → changes of all kinds → `restore` gives the same `dump()`.
- **Limits** (`Limits`: total volume, number of nodes, file size; defaults 256 MiB / 65,536 / 64 MiB):
  - checked before the change, so a violation does not change state;
  - the error is clear: "vfs limit: file '/big.csv' would be 12345 bytes, max_file_size = 2000" (`EFBIG`); for
    total volume — `ENOSPC`, for the node count — `EDQUOT`;
  - parsed from a string: `total=256M,file=64M,files=65536`.
- **Operation journal** (`journal`):
  - entries: number, virtual time, operation, path, bytes, result (`ok` or `ENOENT`, `ENOTCAPABLE`…);
  - TSV — `journal.tsv()`, summary — `summary()`;
  - the cap is 100k entries. Above the cap ordinary entries are dropped with the `dropped` counter, while errors get
    the same amount of extra headroom: errors never vanish silently;
  - the journal is history, not state: `restore` does not roll it back, it adds an entry.
- **Virtual clock:** starts at 2026-09-26T00:00:00Z. Every change advances it by 1 µs, `advance(ns)` does so
  explicitly (for the WASI clock and sleep). The same run gives the same `mtime` and the same `fingerprint()`.
- **Explicit import and export of a real directory:**
  - `import_dir(real, virtual)`:
    - limits apply: a too-large file stops the import with a clear error before reading;
    - real links are carried over as links of this tree, their targets are not read: a link to
      `/etc/passwd` leads to `/etc/passwd` in memory;
    - devices, pipes and non-UTF-8 names are skipped with a counter;
  - `export_dir(virtual, real)`:
    - links are not exported;
    - it does not write through an existing real link in the target directory — error.
  - Nothing happens implicitly.
- **POSIX semantics where they are visible:** `rename` with replacement (file over a directory — `EISDIR`, a directory
  into itself — `EINVAL`, over a non-empty directory — `ENOTEMPTY`), `rmdir` of a non-empty one — `ENOTEMPTY`, `unlink`
  of a directory — `EISDIR`, `mkdir` of an existing one — `EEXIST`, writing through a dangling link creates the target
  (like `O_CREAT`).
- **Error texts as in `std::io::Error` on Linux:** "No such file or directory (os error 2)". So program output is
  identical in memory, in a real directory and in a WASM guest (the "as if" rule). Numbers come in three numberings:
  `Errno::linux()`, `Errno::wasi()` and ours.
- `temp_name(prefix)` → `/tmp/<prefix><k>`: deterministic, skips taken names. `Vfs::new()` creates the root and
  `/tmp` (0o777), as on a clean machine; `Vfs::empty()` — root only.

## `DirFs` — a real directory with a confined root

- Program paths are virtual: `/` is the directory root, as in chroot; relative ones are from the root.
- `..` above the root is an error (lexically, before calling the OS).
- Links leading outside the root are an error:
  - the check uses the canonical path of the deepest existing ancestor;
  - a dangling last link pointing outside is an error too: otherwise a write would create a file outside the root.
- `temp_name` creates `<root>/tmp` if needed.
- **v1 caveat:** there is a window between check and access (TOCTOU). Another process may swap a directory for a link.
  The robust way is `openat2(RESOLVE_BENEATH)`, that is the next step. For untrusted code — `Vfs`.

## Negative controls

A gate that cannot show red is not a gate. `Vfs::fault` and `DirFs::fault` deliberately break the implementation, and
the test `gates_green_and_negative_controls_red` requires: without the fault — green, with the fault — red.

| fault | gate | red |
|---|---|---|
| `ClampDotDot`: `..` above the root silently stays at the root | root escape (`Vfs`, `DirFs`) | yes |
| `NoLimits`: limits are not checked | limits | yes |
| `RestoreForgetsClock`: `restore` does not roll back the clock | snapshot → byte-exact restore | yes |
| `NoConfine`: `DirFs` without the canonical path check | links pointing outside | yes |

## Snapshot speed (release, `nice`, one thread)

`cargo run --release --example snapbench`: N files of 1 KiB in directories of 1000 plus a 64 MiB file.

| files | size, MiB | snapshot | restore | lesson: restore + write 1 file | first appended byte to a shared 64 MiB file | full `dump()` |
|---|---|---|---|---|---|---|
| 1 000 | 65 | 19 ns | 20 ns | 34 µs | 16 ms | 10 ms |
| 10 000 | 74 | 20 ns | 18 ns | 34 µs | 10 ms | 31 ms |
| 100 000 | 162 | 20 ns | 20 ns | 37 µs | 9 ms | 62 ms |

- Snapshot and restore do not depend on size: ~20 ns.
- A lesson costs in proportion to the width of directories on the path: a root of 100 entries plus a directory of 1000.
  Volume does not matter here. `Arc<str>` keys instead of `String` cut a lesson from 40 to 34 µs.
- Appending a byte to a shared large file means copying its contents, once per snapshot. After that the file is edited
  in place.

## Tests

`cargo test`: 16 tests.
- Root escape — red tests: `..`, `/../`, links pointing outside, loops, absolute host paths.
- Limits with error texts; state is unchanged after a violation.
- Snapshot → changes → byte-exact restore, isolation of copies, parallel sandboxes from one snapshot.
- POSIX semantics, the journal and its cap, import and export (links, limit, writing through a real link).
- `DirFs` with links pointing outside and dangling ones; negative controls.

## WASI host (`wasi-host`)

Runs `wasm32-wasip1` modules in a sandbox. For the emulated code it looks like a real machine, for us it is fully
reproducible.

- **Engine — `wasmi` 2.0** (MIT/Apache-2.0), not `wasmtime`. Why it is lighter for us:
  - 8 third-party crates in the tree (wasmi, wasmi_core, wasmi_ir, wasmi_collections, wasmparser, spin, libm,
    bitflags — all MIT or Apache-2.0) versus ~106 packages in the `wasmtime` lock file with Cranelift;
  - pure Rust: no JIT, executable memory or signal handlers;
  - builds fast and on its own — also for WASM and Mini;
  - fuel is a deterministic instruction cap;
  - module compilation is lazy: the 2.5 MB `mlab.wasm` is ready in milliseconds.

  The price: an interpreter is slower than a JIT. For the lesson sandbox this is acceptable. Heavy modules on the
  server — `wasmtime` as a next step, with the same WASI layer over `vfs`.
- **WASI preview1** — all 46 functions of `wasi_snapshot_preview1`:
  - file functions (`path_open`, `fd_read`, `fd_write`, `fd_pread`, `fd_pwrite`, `fd_seek`, `fd_tell`, `fd_readdir`,
    `fd_filestat_get`, `path_filestat_get`, `path_create_directory`, `path_remove_directory`, `path_rename`,
    `path_unlink_file`, `path_symlink`, `path_readlink`, `fd_filestat_set_size`, `*_set_times`…) go into the same
    `Vfs`;
  - there is one pre-opened directory — `/`, the `Vfs` root;
  - an absolute path in `path_*` and a root escape give `ENOTCAPABLE`, with a journal entry.
- **Reproducibility:**
  - the clock is the `Vfs` virtual clock: every read advances it by 1 µs, `poll_oneoff` with sleep only
    advances it;
  - `random_get` — splitmix64 with a seed (`--seed`); the guest's `HashMap` keys derive from it too;
  - the same run gives the same output, the same `fingerprint()` and the same call counters.
- **No network:** `sock_*` → `ENOTSUP`. Also: `proc_raise` → `ENOTSUP`, hard links (`path_link`) → `ENOTSUP`.
- **Virtual RAM with limits:**
  - the cap on guest linear memory — `StoreLimits`, `--mem-limit`, default 1 GiB;
  - fuel cap — `--fuel`;
  - descriptor cap — 1024.
- **v1 caveat:** a descriptor is bound to the canonical path, not to an inode. A file renamed or deleted while open
  disappears for the descriptor (in POSIX it would remain).
- **Error texts in the guest carry only the number.** Only `errno` crosses the WASI boundary, so the explanation "path
  escapes the sandbox root" or "vfs limit: …" is seen by the guest as "Operation not permitted (os error 1)" / "File too
  large (os error 27)". The full text stays in the host journal.

```
cd mlab && cargo build --release --target wasm32-wasip1        # mlab.wasm
cd ../wasi-host && cargo build --release
../target/release/wasi-run [--fs-import DIR[:VIRT]]… [--fs-export DIR[:VIRT]]… [--fs-limits …] [--seed N] \
    [--fuel N] [--mem-limit 1G] [--journal j.tsv] [--stats] ../target/wasm32-wasip1/release/mlab.wasm \
    -e "writematrix(magic(4), '/tmp/m.csv'); disp(readdir('/tmp'))" --fs real
cargo run --release --example mlab_gate -- ../mlab/data/suite-fs.txt [--vm]    # gate with per-scenario timing
```

`--fs real` in the guest means: the guest's `std::fs` → WASI calls → the host's `Vfs`. Without it, `mlab` in the guest
uses its own `Vfs` inside the module, i.e. a double sandbox.

### WASI gate (`cargo test` in `wasi-host`, 12 tests)

- **File suite** `mlab/data/suite-fs.txt`: 25 scenarios × tree and VM — **50/50** identical output between native
  mlab with `--fs mem` and `mlab.wasm` in the host, plus a match with the suite's expected output. Time: 25 scenarios —
  0.03 s in the guest.
- **Frozen suites** v1 + v2: **249/250** identical output (206 scenarios — 0.12 s in the guest, 0.01 s native).
  The one difference is la-18, a suite erratum with a degenerate matrix: `norm` of `inv(A)*A` with Inf/NaN. Native faer
  (AVX2/FMA) gives a number, the wasm32 build (simd128) gives "svd: failed to converge". The cause is in the faer build
  for wasm, not in the host: V8 (Node 24, `node:wasi`) gives the same output, checked 26.09. The test requires exactly
  this difference: if it disappears, it must be removed from the list.
- **Guest bounds:**
  - root escape — an error, recorded in the journal;
  - `/etc/passwd` — "No such file or directory";
  - the vfs limit reaches the guest as `EFBIG`;
  - memory cap: `zeros(2000)` in 16 MiB — the guest fails;
  - fuel: an infinite loop is stopped by a trap.
- **Host ↔ guest:**
  - files placed by the host are read by the guest; what the guest writes is seen by the host;
  - `restore` of a snapshot gives a byte-exact clean machine;
  - a repeated run gives the same output and `fingerprint`;
  - `tic; toc` in the guest — virtual time.
- **WASI unit tests without the engine** (guest memory is a byte array):
  - `args`, `prestat`;
  - `path_open` with `O_CREAT`/`O_EXCL`/`O_TRUNC`/`O_DIRECTORY`/`APPEND`;
  - `fd_read` into several buffers, `fd_seek`, `pwrite` with a hole;
  - `fd_readdir` with a too-small buffer and a cookie;
  - clock, seed, `poll_oneoff`;
  - out-of-bounds memory — `EFAULT`, not a panic.
- `mlab.wasm` is built separately. If it is missing or older than `mlab/src`/`vfs/src`, the test fails with the build
  command — it is not silently skipped.

## Licenses

- Code — the Elamur Cosmic License (`LICENSE.md` at the repo root).
- `vfs` — no dependencies.
- `wasi-host` — `wasmi` 2.0.0, `wasmi_core`, `wasmi_ir`, `wasmi_collections` (MIT/Apache-2.0), `wasmparser` 0.228
  (Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT), `spin` 0.9 (MIT), `libm` 0.2 (MIT), `bitflags` 2 (MIT OR
  Apache-2.0). Licenses checked against crates.io metadata (`cargo metadata`) 26.09.
- For tests `wasi-host` also uses `mlab` (our own).

## What next

- `DirFs` on `openat2(RESOLVE_BENEATH)`, without the TOCTOU window.
- Descriptors on inodes: an open file survives `rename` and `unlink`.
- Hard links.
- Users and groups — for Linux emulation, "Access ladder".
- `wasmtime` as a second engine for heavy modules on the server, with the same WASI layer; compare speed.
- Snapshot to disk: tree serialization with zstd compression on save (the "as if" rule).
- Journal — in DuckDB next to the SLM training journal.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
