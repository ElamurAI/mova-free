# mova-whale-dossier

## Descriptant

The epoch's one integration point with two stores it does not own: the document dossiers (`ROOT/<doc>/texts/<epoch>-<kind>-<n>.md` + `texts.jsonl`) and the global world's neighbourhood of a document (CLI `neighbourhood DOC --radius R --max-docs N`, a closed topic corpus, or a fixture). Source: `tools/whale/src/dossier.rs`.

## Forward approximant

register(NewText) -> TextRow: under texts.lock, an existing row with the same (epoch, kind, sha256) is returned; else n = rows of (epoch, kind) + 1, the file is created (create_new, 0600, synced) and the row appended (fsynced). public_ok = open && kind not in {mova, sol}. licence_class(kind, licence) in {internal, cc0, pd, by, restricted, unknown}. WorldConfig::neighbourhood(doc) -> [NearDoc] nearest first (distance, then the source's own order), the document itself left out; topic_corpus: graph.json lists references, similar, same_author, cited_by with registry texts, at most max_docs.

## Invariant

A text file is never rewritten; generated texts are never public; document ids are single safe path components; no neighbour order is alphabetical (the source's order is kept, a stable sort by distance only).

## Cost

register: O(rows) to read texts.jsonl plus one file write; neighbourhood: one child process (cli) or reading graph.json and documents.jsonl (topic_corpus).

## Composition

Parent: whale-workers. Used by epoch.rs only. The dossier contract is shared with the global world's builder; the CLI format is parsed leniently (`documents` or `neighbours`, `doc` or `id`, `relations` or `relation`).

## behavior_complement

Not done: building the world or the dossier's original/site-old/web texts (another tool registers them; the epoch registers only the original when missing, and its own mova and sol texts), deleting or rewriting texts, paging a neighbourhood, timeouts on the world CLI, locking across machines. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Every row names its file by id, its sha256 and its inputs; a text's provenance is recovered from `source`, `inputs` and `citations`; the neighbourhood is recomputed from the world, not stored.

## History

### 2026-11-05 · document

#### Epoch 2026-11-05 (created)

Coded against the stated dossier contract and an assumed neighbourhood JSON while the global world's CLI did not exist yet; measured with the topic_corpus adapter on five papers (12 nearest documents each).

#### Epoch 2026-11-05 (world tool)

The world tool appeared during the work: `DossierConfig` (`files` | `world`: its CLI `dossier list|text|add`; texts read through it because its originals may be zstd) opens a `Store` with rows/file/read/register (idempotent by (epoch, kind, sha256) before `add`; a generated text it marks public is an error). `WorldConfig::World` replaces the generic CLI: `world neighbourhood … --epoch N` (meta), `world related` (edge kinds), `world neighbourhood … --content` (sentences grouped per text id, rebuilt as Markdown under one `## Text` heading, five sentences per paragraph); the document itself stays at distance 0. `NearText` holds a path or content. Measured read-only on a world of 85 documents: 3 child calls per document, 8–10 s per paper including Mova's writing.
