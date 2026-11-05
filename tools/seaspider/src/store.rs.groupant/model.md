# mova-seaspider-store

## model

# mova-seaspider-store

## Descriptant

Compressed raw page store: zstd frames with checksums appended to rolling segments, an fsynced JSONL journal, links file, readers by URL or sha256, intent/uncertain bookkeeping. Source: `tools/seaspider/src/store.rs`.

## Forward approximant

open: create seg/, cut a torn journal tail, index complete lines (by URL incl. final URL, by sha256), continue the highest segment at its file length, append one 'uncertain' line per dangling intent. put_body: sha256; reuse an existing frame of the same hash; else compress with checksum, roll to a new segment once seg_len >= cap, append, fsync. append: line + newline, fsync. done(url): latest page record not 'error' and younger than refetch age. read_body: read frame, decompress with cap body_bytes+1, require sha256 match.

## Invariant

Every journal line with seg/off/len points at bytes fsynced before the line was written; a successful read returns bytes whose sha256 equals the record; a torn last line never becomes a record; a dangling intent produces exactly one uncertain line.

## Cost

open O(journal bytes); put_body O(body) compression plus one fsync; append one fsync; get O(frame) after the O(journal) index.

## Composition

crawl appends robots/intent/page records and links lines; main's get/stats use Reader.

## behavior_complement

Not provided: concurrent writers (no lock), deletion or compaction, per-domain dictionaries or deltas (Spider features not carried), encryption. Counterexample: a flipped byte inside a frame makes get fail instead of returning wrong bytes (corrupted_frame_detected). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a record and the segments the exact decoded body is recoverable and verified by sha256; the transfer encoding and bytes beyond max_body are not. From a sha256 alone any of the records sharing it is a compatible antecedent (bodies are deduplicated). No unique inverse is claimed.

## one_writer_lock

## tests

### Checks

Five unit tests: round trip with dedup and sha lookup, segment roll-over at the cap, torn tail with orphan bytes (negative control: torn line is no record), corrupt frame, dangling intent recorded once.

## History

### 2026-11-04 · one_writer_lock

2026-11-04: the writer holds an exclusive lock on `lock` in the store directory for its lifetime; a second Sea Spider on the same store refuses to start, so one crawler writes a store at a time (the runtime is single-threaded and requests sequential). Negative control: without the lock the second-writer test fails.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked (27 unit + 9 end-to-end tests against a local 127.0.0.1 HTTP server; two identical runs on 2026-11-04).

These are finite fixtures; the only live run was the 2026-11-04 smoke against the project's own site.
