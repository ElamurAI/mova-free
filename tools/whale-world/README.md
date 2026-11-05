# mova-whale-world — document dossiers and one global world for Whale epochs

🐋 Whales write long descriptions of documents epoch after epoch (see
[`tools/whale`](../whale/README.md)). Each epoch adds texts about a document: the original, Mova's
long draft, a consultant's description, earlier descriptions and pages found on the web. This crate
keeps those texts as an append-only **dossier** per document and reads all of them with Mova into
one **global world** — entities, statements, a word → sentence → document index and a document
graph. A writer works from a document's **neighbourhood** in that world: its own texts of earlier
epochs and the texts of its nearest documents, with all their content. Everything is internal by
default.

It reuses [`mova-topic-corpus`](../topic-corpus/README.md): the library reader and neighbourhood
graph (`library`, `graph`), licence classes (`licence`), Mova's annotator over the Whales' reader
(`corpus::annotate`), the acronym test and statement kinds (`world`), explanation scoring
(`corpus::explain_score`) and the per-paper topic corpora (their cached parses, web pages and
consultant drafts). No parsing is reimplemented here. A separate crate keeps the Whale epoch
driver's dependency small: a library and one binary over topic-corpus.

    cargo build --release --manifest-path tools/whale-world/Cargo.toml --offline
    WORLD_EN_MODEL=…/en.bin cargo test --manifest-path tools/whale-world/Cargo.toml --offline

Every command takes one configuration (`mova-whale-world-v1`, unknown fields refused, absolute
paths); a deployment keeps its configuration outside this crate. `config.example.json` shows every
field with placeholder paths: `dossier_root`, `world_root`, optional `library` (the topic-corpus
library block), `corpora_root`, `site_old {roots, files, licence}`, `mova_state_roots`,
`mova_rounds` (`last` | `all`), `public_originals` (default false) and `limits {cache_shards,
cache_buckets, neighbours, context_bytes}`.

## Commands

    mova-whale-world dossier import CONFIG DOC...       # all importers, idempotent
    mova-whale-world dossier add CONFIG DOC --epoch E --kind K --file PATH --source S
                     [--licence L] [--input TEXT_ID]... [--created UNIX]
    mova-whale-world dossier list CONFIG DOC [--epoch E] [--kind K]
    mova-whale-world dossier verify CONFIG DOC          # exit 1 when a file changed
    mova-whale-world dossier text CONFIG TEXT_ID
    mova-whale-world world build CONFIG [--epoch E] [DOC...]
    mova-whale-world world stats CONFIG [--epoch E]
    mova-whale-world world entity CONFIG TERM [--epoch E]
    mova-whale-world world explain CONFIG TERM [K] [--offset N] [--epoch E]
    mova-whale-world world related CONFIG DOC [K] [--offset N] [--epoch E]
    mova-whale-world world neighbourhood CONFIG DOC [--radius R] [--max-docs N] [--epoch E] [--content]
    mova-whale-world world sentences CONFIG WORD [K] [--offset N] [--epoch E]
    mova-whale-world world context CONFIG DOC [--radius R] [--max-docs N] [--max-bytes N] [--epoch E]
    mova-whale-world export-texts EXPORT_CONFIG OUT_REPO [DOC...]

Every command prints one JSON value on stdout (`neighbourhood --content` prints JSON lines). A
refusal (unknown document, changed file, existing text id, interrupted build) is an error with a
non-zero exit. Writes are incremental, not a transaction over the whole store: a build that fails
after it started writing can leave directories, shards, indexes and `pending.json`, and the next
build refuses that root; inspect the failure and rebuild into a new root. Defaults: `K` 5 (`explain`) or 10, `--offset` 0, `--radius` 1,
`--max-docs` 20, `--epoch` the store's built epoch, `world build --epoch` the highest dossier epoch
plus one (everything).

## Dossier store (`mova-whale-dossier-v1`)

    <dossier_root>/<doc-id>/
        texts.jsonl                      append-only index, one row per text
        texts/<epoch>-<kind>-<n>.md      a text (originals .md.zst)

`doc-id` is the library id of the document (a lowercase slug). A text id is
`<doc-id>/<epoch>-<kind>-<n>`; `n` counts from 1 within (epoch, kind). Row fields:

| field | meaning |
|---|---|
| `id` | the text id |
| `doc` | the document id |
| `epoch` | unbounded integer (`u64`); 0 for material that existed before the epochs, 1 for imported drafts |
| `kind` | `original`, `mova`, `sol`, `site-old`, `web` |
| `source` | where it came from: a path, a URL or a terminal call |
| `licence`, `licence_class` | the licence as stated and its class (`topic_corpus::licence::Class`); `ours (internal)` when none is given |
| `public_ok` | false by default; true only for an `original` library text under CC BY, CC0 or public domain that is not local-only, and only when the configuration's `public_originals` is true. `web`, `sol`, `mova` and `site-old` are never public, whatever licence they state |
| `sha256`, `words` | of the text (decompressed) |
| `created_unix` | when the text was made (the source file's mtime for imports) |
| `inputs` | text ids the text was written from |
| `citations` | `{count, unresolved}`: numeric (`[3]`, `[2, 5-7]`) and author–year (`(Smith et al., 2016)`, `Smith (2016)`) markers, and those without a reference entry (a line `[n]`, or a line after a References/Bibliography heading with the surname and year) |
| `file` | the text's file, relative to the dossier directory |

Append-only: a row is never rewritten and a file never replaced (files are created with
`create_new`; appends hold `lock` and are fsynced). Adding an existing id is refused; a file whose
hash differs from its row is refused by every reader (`verify`, `text`, the world build) until the
new content is added as a new text. Rows are selected by epoch and kind from the index alone.

Importers (`dossier import`, idempotent: a text of the same kind, hash and source is skipped):

- `original` (epoch 0) — the document's library text (`text.md`, else `text.local.md`/`eprint.md`,
  local-only) with the library's licence.
- `site-old` (epoch 0) — earlier descriptions: each file of `site_old.files` under
  `<root>/<a>/<ab>/<abc>/<doc>/` or `<root>/<a>/<ab>/<doc>/` for each `site_old.roots`.
- `web` (epoch 0) — `web` documents of the document's topic corpus (`<corpora_root>/<doc>/`), read
  through the corpus from Sea Spider's store; the source is the page URL. Nothing is fetched.
- `sol` (epoch 1) — consultant drafts registered in the document's topic corpus (`kind: sol`).
- `mova` (epoch 1) — the last round's (or, with `mova_rounds: "all"`, every round's)
  `description.txt` under `<state_root>/papers/<doc>/round-NNN/`; inputs: the original.

An epoch driver adds its own texts with `dossier add` (or `Dossier::add`).

## Global world (`mova-whale-world-v1`)

    <world_root>/
        world.json                       schema, reader, built epoch, counts
        docs.jsonl                       one row per document (rewritten on add)
        texts.jsonl                      append-only: t → text id, doc, epoch, kind, sha, counts
        edges.jsonl                      library graph edges (rewritten on add)
        shards/<t>.jsonl.zst             every parsed sentence of text t
        index/w-XX.json.zst              256 word buckets: word → [(t, s)]
        index/e-XX.json.zst              64 entity buckets: entity → kinds, forms, postings, expansions
        doc-entities/<doc>.json.zst      entity → text → (kind, mentions) of one document

- **All content.** The store keeps every sentence of every text, every entity mention and every
  edge; nothing is ranked away at build time. Storage is economical: zstd shards and hash buckets
  read lazily through an in-process LRU (`limits.cache_shards`, `limits.cache_buckets`).
- **Epochs.** `world(e)` is the graph over the texts of epochs `< e`; `world build --epoch E` adds
  the texts of epochs `< E` not yet in the store. One store serves every `world(e)`: each text keeps
  its epoch and a query at `--epoch e` reads only texts of earlier epochs, so old worlds stay
  retrievable and nothing is rewritten.
- **Incremental.** A text is parsed once and never again; a build parses only new texts, then
  merges their postings into the buckets they fall in and appends their rows. `pending.json` marks
  a build in progress (renamed `last-build.json` when it ends); a build finding one refuses. A store
  read by another reader version is refused too (build into a new root).
- **Documents** are dossier documents and the documents of their topic corpora whose parse is
  cached there (`neighbour` texts, epoch 0, `lib:<id>`; `limits.neighbours`). A neighbour that later
  gets a dossier keeps its old text.
- **Reading.** `topic_corpus::corpus::annotate` over the Whales' reader; a text in which the reader
  admits no section (a plain draft, a PDF extraction without headings) is read again with one
  `## Text` heading after its title (`world::annotate_text`).
- **Entities**: acronyms (also plural), runs of capitalized proper nouns, noun lemmas of four
  letters or more and noun compounds; keys case-folded and singular
  (`mova_whale::knowledge::norm_term`); acronym expansions are definitions.
- **Statements**: sentences with at least one frame; kind `claim`, `method`, `result` or
  `statement` (`topic_corpus::world::kind`). Each cites `{doc, text, s}`; `doc` is in `docs.jsonl`.
- **Edges**: `cites` (from cites to), `similar` (with the library's score), `same_author`, from
  the library graph of each dossier document and the roles of its topic-corpus neighbours.

## Queries

All answers are deterministic (ties by document, text, sentence) and page with `K`/`--offset`.
Citations are `{doc, text, s}`: a dossier text id or `lib:<id>`, and the sentence number in that
text's shard.

- `entity` — `{key, kind, kinds, forms, mentions, docs, definitions: [{doc, text, s, expansion,
  count}], by_doc: [{doc, mentions}]}`; expansions grouped case-insensitively, most frequent first.
- `explain` — `{explanations: [{doc, text, s, kind, epoch, sentence, score}]}`: the term's entity
  postings and its rarest word's postings, scored by `explain_score` plus a bonus by text kind
  (originals 0.5, earlier descriptions 0.3, pages 0.2, consultant 0.1, Mova 0); at most two per
  document; repeated sentences dropped.
- `related` — `{total, related: [{doc, title, score, edges: [{kind, weight}]}]}`, documents with
  texts in `world(e)`; strength = 3 per `cites`/`cited_by`, 2 `same_author`, 4 × score `similar`,
  5 × shared-entity share (idf of shared entities / idf of the document's entities).
- `neighbourhood` — the nearest documents, breadth-first over `related` up to `--radius`, at most
  `--max-docs` besides the document; order (distance, strength descending, id); a document at
  distance ≥ 2 has the best min(parent strength, edge strength). Each with its visible texts
  (originals first, newest epoch first). `--content` streams every sentence of those texts:
  `{doc, text, s, kind, statement, sentence, entities}`.
- `sentences` — `{sentences: [{doc, text, s, sentence}]}`: round-robin over documents, sentences
  of 6–40 words first, a sentence sharing 80 % or more of its words with a chosen one dropped.
- `context` — the writer's pack from the neighbourhood: `{entities: [{entity, kind, mentions,
  explanations}], related: [{doc, title, distance, strength, via}], statements: [{doc, text, s,
  kind, sentence}], cut, bytes}`. Every entity of the document (most mentioned first; up to two
  explanations from other neighbourhood documents for entities met twice or more) and every
  statement of the neighbourhood's texts. `--max-bytes` cuts an eighth of the larger list from its
  far end until the compact JSON fits; `cut` counts what was cut.

## Library API (for an epoch driver)

    use whale_world::{config::Config, dossier::{self, Dossier, Kind, NewText}, world::World, query};

    let cfg = Config::load(path)?;
    let d = Dossier::open(&cfg, "doc-id")?;                       // creates the directory
    let rows = d.select(Some(2), Some(Kind::Mova))?;              // no text read
    let text = d.read(&rows[0])?;                                 // hash-checked
    let row = d.add(NewText { epoch: 2, kind: Kind::Mova, text: &body, source: "…",
                              licence: None, local_only: true, inputs: vec![…], created_unix: None })?;
    dossier::import::all(&cfg, lib.as_ref(), "doc-id")?;          // lib: Option<topic_corpus::library::Library>
    let mut w = World::open(&cfg)?;
    w.add(&cfg, 3, &[])?;                                         // world(3): texts of epochs < 3
    query::neighbourhood(&mut w, "doc-id", query::RADIUS, query::MAX_DOCS, 3)?;
    query::neighbourhood_content(&mut w, "doc-id", 1, 20, 3, &mut |line| Ok(()))?;
    query::context(&mut w, "doc-id", 1, 20, 3, Some(60_000))?;
    query::explain(&mut w, "GEC", 5, 0, 3)?;  query::entity(&mut w, "GEC", 3)?;
    query::related(&mut w, "doc-id", 10, 0, 3)?;  query::sentences(&mut w, "edit", 10, 0, 3)?;

Answers are `serde_json::Value` in the shapes above.

## Texts export (`mova-whale-texts-v1`)

`export-texts` writes the finished descriptions into an editorial texts repository (main article
and shadows, as in a repository whose subjects have a main article and preparatory «shadows»). Its
configuration: `world` (a `mova-whale-world-v1` configuration: dossiers and library), `batches`
(the batch log), `learners` (learner name → state root), `from_batch`, `dir` (the directory in the
repository), `describer`, `visibility` and `licence` (stated in every file). Without `DOC` it exports
every paper assigned in a batch `>= from_batch` whose learner state says `passed`; the main text is
the last round whose `gates.json` passed.

    <repo>/<dir>/index.md                                   list of exported papers
    <repo>/<dir>/export.json                                manifest: papers, file → sha256 written
    <repo>/<dir>/<doc>/index.md                             main article
    <repo>/<dir>/<doc>/seo.json                             title, meta description
    <repo>/<dir>/<doc>/shadow/<epoch>-<kind>-<n>/index.md   dossier texts: mova, sol, site-old

- **Citations.** Each sentence's sources come from the round's `draft.json`: restated paper
  sentences cite the paper; knowledge items cite their provenance in the learners' knowledge
  (`terminal`/`reread` → that learner's consultation notes, `topic-corpus` → the corpus document;
  pod items `pod:<learner>:<id>` in that learner's knowledge); `Surname … YYYY` mentions that match
  the paper's reference list (`refs.json`) cite that reference. A footnote follows each run of
  sentences with the same sources; the bibliography gives authors, year, DOI/ACL/arXiv link and
  licence. When the draft does not reproduce `description.txt` exactly, each paragraph cites the
  paper instead (`citations: paragraph`).
- **Cited, never copied.** Dossier `original` and `web` texts are listed by text id, URL or file
  name and licence; their text is never written, whatever the licence. Shadows are the dossier's
  `mova`, `sol` and `site-old` texts, one per distinct hash, the main text excluded. Local paths are
  reduced to file names.
- **Edits are kept.** Every Markdown file has front matter (visibility, licence, provenance hashes)
  ending with `export_sha256`, the sha256 of the file with that value zeroed. A file is written only
  when its content changes; an existing file is replaced only when its seal (and the manifest's
  hash) still match. Otherwise it is skipped and reported as `edited` (written by the exporter,
  changed since) or `foreign` (not written by it). Nothing is deleted.
- **Deterministic.** No clock; sorted maps; a second run writes nothing and a fresh repository gets
  the same bytes.

The answer is `{papers: [{paper, words, batch, whale, round, shadows, notes, citations}], exported,
not_passed, report: {written, unchanged, edited, foreign}}`.

## Licences

The world is internal. `public_ok` travels with every dossier row and world text; nothing here
writes a public output. The texts export writes internal texts only and states their visibility and licence in every
file. Pages fetched by Sea Spider, consultant drafts, Mova drafts and earlier
descriptions are never `public_ok`, whatever licence they state (`dossier::public_ok`).

## Limits

- Statement kinds are lexical (`topic_corpus::world::kind`); nearly every sentence has a frame,
  so «statements» are nearly all sentences.
- Neighbours come only from topic corpora that were built, and only their cached parses; a
  document without a corpus is related through library edges and shared entities only.
- Shared-entity strength counts frequent general nouns too (idf over the world's documents); with
  few documents it links weakly related papers.
- Acronym expansions come from `mova_whale::knowledge::expansion` and include some noise; the
  most frequent comes first.
- The word index rewrites every bucket a new text touches, so a build's index cost grows with the
  store; parsing, the dominant cost, does not.
