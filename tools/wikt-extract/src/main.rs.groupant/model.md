# wikt-extract-command

## descriptant

### wikt-extract-command

#### Descriptant

Written with the code (tools/wikt-extract, first version). page: index lookup of the given titles, their streams decoded, rows printed. build: every part's index read (MultiBzDecoder) → distinct stream offsets; a job queue of (part, offset) consumed by N threads, each decoding one bz2 stream (BzDecoder stops at the stream end), parsing its pages (quick_xml, unmatched end tags allowed) and turning ns-0 pages with an English section into rows with provenance; the first error stops the queue (fail-fast); rows sorted by (lower-case title, not-lower-case flag, title, index in page) and written after a CC BY-SA header; gaps.tsv sorted by count; report.json (counts, seconds, peak RSS); module.json with each file's SHA-256 and size. check: re-hashes the listed files. cover: streams a layer and reports a word list's exact and case-folded coverage.

The forward model below is source-derived; the tests in model.md#tests exercise the actual functions on small synthetic pages, and the full English pass (docs/data.md, «Wiktionary») is the corpus-scale observation. Executable percentage approximants are not supported: the outputs are categorical text records with no useful error metric.

## approximant

### Forward relation

page: index lookup of the given titles, their streams decoded, rows printed. build: every part's index read (MultiBzDecoder) → distinct stream offsets; a job queue of (part, offset) consumed by N threads, each decoding one bz2 stream (BzDecoder stops at the stream end), parsing its pages (quick_xml, unmatched end tags allowed) and turning ns-0 pages with an English section into rows with provenance; the first error stops the queue (fail-fast); rows sorted by (lower-case title, not-lower-case flag, title, index in page) and written after a CC BY-SA header; gaps.tsv sorted by count; report.json (counts, seconds, peak RSS); module.json with each file's SHA-256 and size. check: re-hashes the listed files. cover: streams a layer and reports a word list's exact and case-folded coverage.

#### Domain

Well-formed UTF-8 wikitext/XML of the English Wiktionary dump export format (2026-09-01 run); bounded resources.

## models

# Typed source models

Partial source-derived relations at this manifest snapshot; one call under the stated domain; no universal semantic proof.

## behavior_complement

Never writes decompressed XML, never overwrites a module.json, never writes into a Git work tree (nearest existing ancestor canonicalized), never retries a failed stream, never reads non-main namespaces into rows; report.json timing and RSS are not part of the layer's identity.

Domain: Well-formed UTF-8 wikitext/XML of the English Wiktionary dump export format (2026-09-01 run); bounded resources.

## reverse_reconstruction

From the module one recovers the dump name and files and, per row, the stream that held its page (file + offset): decompressing that stream and re-running entry::entries reproduces the row; the layer bytes are independent of the thread count (sorted output), so they do not reveal it.

Domain: Well-formed UTF-8 wikitext/XML of the English Wiktionary dump export format (2026-09-01 run); bounded resources. Preimages are sets, not unique inverses: the rendering is not injective.

## determinism

Layer and gaps.tsv bytes are a function of the dump bytes and the code only: rows are sorted after all threads join; observed identical for 6 and 3 threads on the 2026-09-01 English dump.

## fail_fast

An error in one stream stores the error, moves the queue index past the end and the run returns an error naming every stopped stream; no partial module.json is written.

## tests

### Native evidence

Package/bin: `wikt-extract`. Selected tests (`cargo test --release -p wikt-extract`), all passing:

- `tests::pages_of_a_stream`
- `tests::rows_sort_lower_case_first_and_header_is_cc_by_sa`

Negative control: rows_sort_lower_case_first_and_header_is_cc_by_sa checks the Git-tree guard on this crate's own folder (red) and on / (green), and that the header is refused by a CC BY-SA check.

Scope: actual functions on synthetic in-memory inputs; the corpus-scale pass is an observation, not a proof. Lua module semantics, full template coverage and the correctness of every printed sense are not established.

## invariant-and-cost

### Invariant and cost

#### Invariant

module.json lists exactly the files written with their current SHA-256 and byte size; the layer's first line declares CC BY-SA; rows are sorted by sort_key.

#### Cost

## History

### 2026-11-04 · descriptant

Heading `#### Epoch 2026-11-04` stays in descriptant as `#### Descriptant`.

### 2026-11-04 · determinism

Layer and gaps.tsv bytes are a function of the dump bytes and the code only: rows are sorted after all threads join; observed identical for 6 and 3 threads on the 2026-09-01 English dump (2026-11-04).

### 2026-11-04 · tests

Heading `### Native evidence — 2026-11-04` stays in tests as `### Native evidence`.

### 2026-11-04 · invariant-and-cost

Time ≈ (decompressed bytes / per-thread bz2 throughput) / threads + parse; measured on the English edition (2.01 GB bz2, 108 963 streams): 63–105 s wall at 6 threads, 100 s at 3 threads on a shared machine (2026-11-04). Memory: the sorted rows (≈ 340 MB of text) plus per-thread stream buffers; peak RSS 733–832 MB.
