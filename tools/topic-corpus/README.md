# mova-topic-corpus — topic graphs and closed topic corpora for paper-describing Whales

A scientific paper is a set of topics, authors, references and citing works; the texts of that
neighbourhood are the topic's corpus. `mova-topic-corpus` builds that neighbourhood from a paper
library, asks the library's own downloader for the most-cited missing open copies, reads the
neighbourhood with Mova into a closed cross corpus that a Whale consults natively before it asks its
consultant, adds pages fetched by [Sea Spider](../seaspider/README.md), cites every sentence of a
description and turns a consultant's draft article into claims whose citations are followed to the
originals. It is a standalone crate beside [`mova-whale`](../whale/README.md); the Whale needs no
code change.

    cargo build --release --manifest-path tools/topic-corpus/Cargo.toml --offline
    WORLD_EN_MODEL=…/en.bin cargo test --manifest-path tools/topic-corpus/Cargo.toml --offline

Every command takes one configuration (`mova-topic-corpus-v1`, unknown fields refused, absolute
paths): the library files (export JSONL, citation edges, resolved reference lists, the text root with
per-paper `refs.json`, optional topic taxonomy and author positions), optional `downloads` (the
library's `papers` binary for snapshot lookups, the stream driver and claim script, the runs and raw
roots, `max_requests`), `corpora_root`, `limits`, optional `seaspider` (socket, store) and an optional
shared `hard_cases` list.

| command | stage | does |
|---|---|---|
| `graph CONFIG PAPER` | 1 | the neighbourhood as JSON |
| `missing CONFIG PAPER [--sol DRAFT] [--dry]` | 2 | ranks and routes the missing cited works; writes a new request run directory (not started) |
| `build CONFIG PAPER` | 3 | registers the cross corpus, parses the hot neighbourhood |
| `explain CONFIG PAPER TERM [K]` | 3 | sentences of other documents that explain a term |
| `world CONFIG PAPER` | 3 | the paper's small world (`world.json`) |
| `seed CONFIG PAPER WHALE STATE_ROOT [TERM…]` | 3 | the Whale hook (below) |
| `web-plan`, `web-ask`, `web-import` | 4 | Sea Spider's queue, the English request, the import of stored pages |
| `cite CONFIG PAPER ROUND_DIR STATE_ROOT WHALE OUT [--public]` | 5 | a cited description with a bibliography |
| `sol-prompt`, `sol-draft`, `sol-import`, `sol-claims` | 6 | draft articles → claims → sources |
| `public-export CONFIG PAPER OUT [DOC…]` | — | annotations of public-admissible documents only |
| `hard-cases-recheck CONFIG` | — | marks this reader's hard cases the parser now reads |
| `bundle CONFIG CORPUS DOC` | — | a document's sentences and trees as a hard-case bundle (JSON) |

## Stages

1. **Topic graph** (`graph.rs`). Topics with their taxonomy names, concepts, authors (scholarly
   homepage, h-index), the extracted bibliography resolved against the library by DOI, arXiv id, ACL
   id or title (year ± 1), library works citing the paper (most cited first), similar works (an
   idf-weighted cosine of title and abstract keywords, plus shared topics and concepts) and works of
   the same authors. Every node carries ids, the licence as stated, its class, whether it may feed a
   public output and which text we hold (`open`, `local`, `none`).
2. **More papers** (`missing.rs`). External entries of the paper's bibliography and of its
   references' and similar works' bibliographies are looked up in the local OpenAlex snapshot (one
   `papers openalex lookup` call; title-only entries by title, re-keyed by the record's ids when the
   year agrees), ranked by citations and routed: ACL Anthology PDF (CC BY 4.0 from 2016, CC BY-NC-SA
   3.0 before), `arxiv.org/pdf/<id>`, or OpenAlex's best open-access PDF; a shadow-library host is
   never a route, a work without a legal copy is list-only. The request is a new run directory for the
   library's own stream driver: `chunk-topic-00.jsonl` (`tools/sources` rows), `listonly.tsv`,
   `request.json` (every candidate with its route and reason) and `run.sh` with the robots.txt gate,
   host skip and PDF drop. Nothing starts; the driver keeps its pacing, disk guards and claims. A
   request is refused when the raw root has less than 300 GB free, and a request directory is never
   rewritten.
3. **Closed cross corpus** (`corpus.rs`), lazy. The registry (`documents.jsonl`) names each document
   — the paper, its references, similar, citing and same-author works with a text, a note for each
   author with no other work in the library, pages, drafts — and where its original is: library
   texts stay in the library, pages in Sea Spider's zstd store, drafts and notes zstd-compressed under
   `originals/`. A document is parsed on request and whole: the Whales' reader gives the parts and
   admitted sentences, the pragmatic reader parses them in order with one discourse context, so each
   sentence is read in the world of its document (UD words with the text's own forms, frames). The
   hot neighbourhood (`limits.hot` first documents) is parsed at build time and pinned; other parses
   live in `cache/` (zstd JSONL) under `limits.cache_max_bytes`, least recently used evicted first. A
   changed original is refused until registered again. `explain` parses only documents whose text
   contains the term. `world` builds the paper's small world: entities (acronyms, names, frequent
   nouns) with mentions, the paper's own expansions, the best neighbour explanation and the documents
   that speak of them, and statements sorted into claims, methods, results and others.
4. **Web pages** (`web.rs`). Author homepages, pages named by claims and given URLs go to Sea Spider
   only: a queue file and English requests of at most 16 URLs, asked as caller `whale`. Imported pages
   keep the licence the page states (`rel="license"`, a Creative Commons link, a rights meta tag, a
   copyright line) and the site's terms link.
5. **Citations** (`cite.rs`). Each written sentence of a Whale round (`draft.json`) ends with its
   sources: the paper for restated sentences, the corpus document for knowledge seeded from it, the
   consultation for terminal knowledge; a bibliography with stable ids (`lib:<library id>`,
   `web:<hash>`, `sol:<draft>`, `consult:<whale>`) closes it.
6. **Drafts → claims → sources** (`sol.rs`). One question to the Whale's own private terminal asks
   for a long draft article on the paper's topics with numbered citations and a reference list. The
   draft is registered as an internal, low-quality document. Its sentences become claims; each
   reference entry is resolved: a library text, a library work without text, a download request (by
   id, else by title, for stage 2), a page for Sea Spider, or unresolved. A claim is `sourced`
   (with the best-overlapping sentence of the held original as support), `pending`, or kept `from_sol`
   with the draft as its only source. A terminal error registers nothing; nothing is retried. The split into
   cited sentences and a reference list is `mova_whale::claims::split`, shared with the Whale
   epochs' citation check.

**Hard cases** (`hardcases.rs`). Sentences the parser reads badly — no words, a tree without one
root, a head outside the sentence, no frame for six words or more, nothing the Whales' reader could
restate — go to the [one shared hard-cases list](../hard-case-list/README.md) (`mova-hard-case-v2`,
the same file as the Wikipedia corpus's cases) with their document's licence and a pointer to their
document, until `hard-cases-recheck` finds them read well; it re-judges only this reader's cases.
`bundle CONFIG CORPUS DOC` prints a document's sentences and trees as a hard-case bundle for the Fish
priority lane ([`tools/hard-cases`](../hard-cases/README.md)).

**The Whale hook** (`seed.rs`). Between rounds (refused while the Whale holds `learner.lock`), the
terms the Whale would ask about and has no facts for are explained from neighbour papers: only
defining sentences (an expansion, «T is/means …», «… (T)»), only defining facts (`stands_for`,
`is_a`, `means`, read by the Whale's own answer reader), and an expansion from a document that is
neither a reference nor a similar work only when all its words occur in the paper. The facts go into
the Whale's knowledge store with provenance `topic-corpus` (corpus, document, sentence); the Whale's
writer explains those terms and its question selection skips them.

## Licences

Everything in a closed corpus is internal (files 0600 in 0700 directories). A public output may take
only library texts under CC BY, CC0 or public domain that are not local-only. Pages Sea Spider fetched
and their annotations, consultant drafts and our notes are never admitted, whatever licence they
state: `licence::public_admissible`, `corpus::assert_public`, `public_export` and `cite --public` all
refuse them.

## Limits

- Library citers only: the slim OpenAlex snapshot keeps no citing or referenced works.
- Similarity is lexical; title lookups need equal normalized titles.
- Author blogs are found only through scholarly homepages and given URLs (no web search).
- Claim support is word overlap with the cited text, not entailment.
- Seed precision is kept by strict filters, so recall is low.
