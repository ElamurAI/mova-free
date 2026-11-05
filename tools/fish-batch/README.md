# Bounded Fish batch preparation

Mova's animals and the Octopus, the large language model that develops Mova, are described in [the sea animals of Mova](../../docs/animals.md).

This public Rust library packs source-bound single-target observations into native Fish tasks. It imports `annotation_review_worker::annotation::{Task, evaluate, render}` and never invokes a consultant, model, database or trainer. Private corpus selectors/materializers own source-file reads, source-phase provenance, full-corpus coverage and held-out admission. The packer verifies declared input byte hashes; it does not reread the files named by source pins.

`pack(Vec<PacketInput>, &Profile, &PackOptions)` returns `PackReport { batches, rejections, boundaries, ... }`. Every prepared batch contains its typed Task, configured Profile, native Evaluation, exact rendered request, target IDs, deduplicated declared source pins and provenance. Flags remain `review_only=true` and `training_authorized=false`. Unknown dataset/context observations remain unknown; explicit Evaluation targets are rejected.

A PacketInput contains schema=1, a caller-supplied source ordinal, an opaque order_scope (default empty), a single-target native Task, exact context-ID/ordinal mapping, source pins and object provenance. The packer never parses numeric, question or exam suffixes from sentence IDs. Different ordering scopes, source versions, document hashes, dataset-use values or producer order form packing boundaries. Context comparisons preserve the supplied observations; conflicting observations are not voted into one answer.

Defaults: four targets, at most eight additional context lines, no artifact mosaics. The maximum is eight targets and sixteen total context lines. The actual native AST/UD/source/TSV gates and combined schema-plus-stdin 100000-byte guard decide whether a proposed batch fits. The greedy algorithm validates the next singleton first; if a combined proposal fails, it flushes the prior valid batch and retains that next singleton. Invalid packets yield explicit per-packet rejections. Nothing is truncated to fit. Every excluded or bounded-window context ID removed from the submitted view is recorded with its reason; all targets are retained.

`allow_mosaic_artifacts=true` is an explicit deployment choice for cached observations from different producer artifact revisions. Candidate names identify the mosaic, and its revision hashes the ordered per-target original producer/revision, AST and raw/adapted pragmatic bindings. Witness revision is similarly composed. Original bindings and provenance remain separate. These are observational mosaics, not a claim of one frozen model checkpoint, independent producers or semantic agreement. Epoch uncertainty is retained in context unresolved observations; lack of an available native unresolved slot is an explicit rejection, never a dropped observation.

`adapt_pragmatics_header` accepts LF-framed native `teacher` or cached `conf` headers. It changes only the last header name to the opaque `teacher` provenance column and retains all row bytes; raw/adapted hashes identify that adapter. Other headers and CR framing are rejected. Composition preserves raw AST blocks and adds two LF separators; TSV composition uses one identical header and original row fields separated by LF. It does not decode a returned annotation again, strip fences, change words, repair trees or rewrite pragmatic labels.

Source pins are declarations for downstream checking. A supervisor must verify actual pinned files, source admission and global no-replay ownership. Per-page duplicate-target checks cannot establish full-corpus uniqueness. Prepared native shape/source consistency is not semantic correctness, source licensing, gold annotation or GPU-training permission.

Focused synthetic Rust tests cover real native four/eight-target preparation, deterministic packing, different documents and ordering scopes, explicit mosaic provenance, the actual combined 100000-byte split with next-target retention, ID/text/context exclusions, unknown/evaluation/hash handling, context-removal accounting and exact header-row preservation. No consultant calls or corpus assessment are part of these checks. No throughput gain or whole-corpus completion date is inferred from batching support.

## Build and test

A standalone workspace (own `Cargo.lock`), not built by the root `cargo build`:

```bash
cargo build --release --manifest-path tools/fish-batch/Cargo.toml
cargo test --release --manifest-path tools/fish-batch/Cargo.toml
```

This crate is a library (`mova_fish_batch`); a supervisor links it and calls `pack`.
