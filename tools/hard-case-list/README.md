# The hard-cases list

Mova's animals and the Octopus, the large language model that develops Mova, are described in [the sea animals of Mova](../../docs/animals.md).

`mova-hard-case-list` is the one list of sentences Mova reads badly, shared by every corpus reader, in one format.
A Wikipedia corpus reader's hourly check (outside this repository) and the Whales' topic corpora
([`tools/topic-corpus`](../topic-corpus/README.md)) write to it; the Fish priority lane and the Eel's hard-case
rounds ([`tools/hard-cases`](../hard-cases/README.md)) read it. The deployment's list is
`$MOVA_DATA/corpora/hard-cases.jsonl`.

    cargo test --manifest-path tools/hard-case-list/Cargo.toml --offline

## Format: `mova-hard-case-v2`

One append-only JSONL file, one event per line, written under an exclusive `flock` and synced. A case's key is the
first 16 hex digits of the SHA-256 of its trimmed text, so a sentence met in two corpora is one case.

```text
{"schema":"mova-hard-case-v2","event":"found","key":K,"parser":"en","text":…,"source":"wikipedia",
 "id":"wiki:Kim_Wilde:12","document":{"id":"wiki:Kim_Wilde","n":12},
 "where":{"article":"Kim Wilde","page_id":…,"rev_id":…,"dump":"enwiki-20260901","stream_offset":…,"sentence":12},
 "why":["grammar","witness"],"licence":"CC BY-SA 4.0","model":"en.bin:31a93401fe7135b1","reader":"run-1762290000",
 "draft_conllu":"# sent_id = …","witness_uas":0.45,"review_only":true,"training_authorized":false}
{"schema":"mova-hard-case-v2","event":"learned","key":K,"parser":"gparse","model":"gparse:6e9564693e9307b6","reader":"hard-cases-recheck"}
{"schema":"mova-hard-case-v2","event":"open","key":K,"parser":"gparse","model":"gparse:…","reader":"hard-cases-recheck","why":["witness"]}
```

| field | meaning |
|---|---|
| `source` | the reader that found it: `wikipedia`, `topic-corpus` |
| `id` | the source's sentence id |
| `document` | the context pointer: the document's id in its source and the sentence's place in it |
| `where` | the source's locator (Wikipedia: article, page id, revision, dump, offset; topic corpora: corpus, doc, sentence index) — what the source's `bundle` command takes |
| `why` | the judges that flagged it: `parse`, `tree`, `frames`, `grammar`, `nonprojective`, `witness` (Wikipedia); `roots`, `head_outside`, `no_words`, `no_frames`, `no_skeleton` (topic corpora) |
| `licence` | the text's licence as the source states it (empty: unknown) |
| `model`, `draft_conllu`, `witness_uas` | what read it badly, its tree, Stanza's agreement with it |

**Views per parser.** `parser` says whose reading an event is about: `en` (the CPU model `en.bin`, which the corpora
read with) or `gparse` (the GPU parser the Eel trains). Each case has one status per parser: `open` from a `found` or
`open` event, `learned` after a `learned` event, `unchecked` when that parser never judged it. The corpora's own
rechecks move the `en` view; the gparse recheck (`mova-hard-cases recheck`) moves the `gparse` view. A case found
again after it was learned is open again.

**Reading v1.** The topic corpus's first lines (`mova-hard-case-v1`: no `id`, `document`, `licence`, `model`) are
read as v2 found records with an unknown licence, `id` `<doc>:<s>` and document `{doc, s}`. A later report of the
same open case that brings a licence is written again, so the case gets it.

## Operations

- `record(path, found, parser)` — appends the found records not already open for that parser (one per key).
- `sync_source(path, source, current, parser, model, reader)` — makes the list agree with one source's current set:
  new cases found, that source's open cases missing from the set learned. Cases of other sources are left alone. The
  Wikipedia corpus calls it after every check and recheck (`wikicorpus hard-sync` runs it alone; that tool is internal,
  not in the public tree).
- `recheck(path, source, parser, model, reader, all, check)` — judges the open cases (of one source, when given;
  with `all` also the learned ones) and records `learned`, or `open` for a learned case that fails again. A case the
  check cannot judge stays as it was.
- `state(path)` — the latest state of every case.

Three unit tests cover v1 reading, per-parser views, deduplication, licence completion, source-scoped syncing and
rechecks, regressions reopened, and unjudged cases left alone.

## Limits

The whole file is read for every operation (O(list)); at tens of thousands of cases that is still a fraction of a
second, but the list never shrinks: compacting it would be a separate, explicit step. A key collision of 64 bits
merges two texts. `flock` coordinates cooperating writers on a local filesystem. Nothing here admits anything to
training.
