# mova-modules-torrent: BitTorrent v1 metainfo of a module folder

## forward

`Value` is a bencode tree whose dictionaries are `BTreeMap<Vec<u8>, _>` (byte-ordered keys); `encode` writes BEP 3
bencode. `decode` parses one value, refusing non-canonical integers, unordered keys and truncation. `make(dir,
trackers, web_seeds)` parses and verifies the module folder, checks URLs (web seeds `https://` folders ending in `/`),
orders module.json and the listed files by path, concatenates their bytes into pieces of `piece_length(total)` (power
of two, 256 KiB..16 MiB, at most about 2 000 pieces), SHA-1s each piece, builds `info` {files [{length, path}], name
`<name>-<version>`, piece length, pieces}, hashes the bencoded info (the info hash), adds a comment with licence and
attribution, optional `announce`/`announce-list` and `url-list`, and a magnet link with `dn`, `tr`, `ws`.

`single_file(path, name, trackers, web_seeds, xs, comment)` streams one file through `hash_pieces` (SHA-1 per piece of
`piece_length(size)`, the last piece shorter) into info {length, name, piece length, pieces}; web seeds and `xs` must
be `https://` file URLs (not folders); the magnet link carries `xt`, `dn`, `tr`, `ws`, `xs`, `xl`. `Magnet::link`
writes parameters in that fixed order; `Magnet::parse` reads `xt=urn:btih:` in 40 hex or 32 base32 (decoded),
`dn`, `xl`, `tr`, `ws`, `xs`/`as`, %-decoding values, ignoring others. `read_metainfo` decodes a single-file torrent,
hashes the info dictionary as re-encoded (canonical, so equal to its bytes in the file), refuses `files`, checks the
piece count against the length, and reads `url-list` and the trackers. `verify_file` re-hashes a file and names the
first piece that differs.

## invariant

- Same folder and options ⇒ same bytes and info hash (no dates, sorted keys and paths).
- The torrent covers exactly module.json and the listed files, all verified first.
- `decode(encode(v)) = v` for canonical values.
- `read_metainfo(single_file(f, …).bytes).info_hash = single_file(f, …).info_hash`; `Magnet::parse(m.link()) = m`.
- `verify_file(f, m)` succeeds iff f has m.length bytes and every piece SHA-1 equals m's (up to SHA-1 collisions).

## cost

B bytes: one SHA-1 pass O(B) plus the module's SHA-256 verification O(B); memory holds all files (modules are read
whole), pieces 20 bytes each. `single_file`, `verify_file`: one SHA-1 pass O(B) streamed, memory one piece (≤ 16 MiB)
plus 20 bytes per piece; `read_metainfo` O(torrent bytes).

## composition

Called by `main_args` (`make-torrent`); reads through `crate::Manifest::parse`, `verify_dir`, `read_verified`.

## behavior-complement

Does not: transfer data, contact trackers or DHT, exchange metadata (BEP 9), read multi-file torrents (read_metainfo),
make BitTorrent v2 or hybrid torrents, stream module folders larger than
memory, set a private flag, verify that a web seed actually serves the files.

## reverse-reconstruction

From the info dictionary, file names, lengths and order are recoverable exactly; file contents only up to SHA-1
piece collisions (search); the module's licence and attribution only from the outer comment, which is not covered by
the info hash.

## checks

`cargo test --release -p mova-modules torrent` (bencode_round_trip_and_canonical_form,
a_module_torrent_is_deterministic_and_hashes_its_pieces, piece_length_grows_with_size,
a_single_file_torrent_reads_back_and_verifies_its_pieces, magnet_links_parse_hex_and_base32_and_refuse_garbage).
Negative controls: a flipped byte in the third piece (named), a cut file, a changed info dictionary (another info hash),
a folder or plain-http web seed for one file, a magnet without btih, a short hash, a non-numeric xl; unordered
keys, a leading zero, a truncated string, an HTTP or non-folder web seed, a changed file.

## epoch-memory

Single-file torrents for release archives, magnet links with `xl`, `xs` and `ws`, and the reading side
for `mova fetch`. Interoperability observed: rqbit 9.1.0 loaded the four 0.5 release torrents made here, seeded them,
and a second rqbit fetched each one from a bare magnet link (metadata over BEP 9 from the seeder) with identical
SHA-256; `Magnet::parse` agrees with `basenc --base32` on a base32 info hash.

## History

### 2026-11-04 · epoch-memory

2026-11-04, first epoch. No external client was available to load the produced torrent; correctness rests on the
recomputed info hash and pieces in the tests. A multi-file web seed is fetched as `<seed><name>-<version>/<path>`
(BEP 19), so a server must expose a `<name>-<version>/` folder.

### 2026-11-05 · epoch-memory

2026-11-05: single-file torrents for release archives, magnet links with `xl`, `xs` and `ws`, and the reading side
for `mova fetch`. Interoperability observed: rqbit 9.1.0 loaded the four 0.5 release torrents made here, seeded them,
and a second rqbit fetched each one from a bare magnet link (metadata over BEP 9 from the seeder) with identical
SHA-256; `Magnet::parse` agrees with `basenc --base32` on a base32 info hash.
