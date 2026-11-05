# annot — the Mova annotator subproject

`annot` is the teacher side of annotation: it sends sentences, with our drafts and context, to a large model in batches, checks the answers with Rust gates, and writes silver data for training. The large model teaches; it never answers for Mova at test time.

The hand-annotated corpus is small. So the teacher takes random sentences: it annotates them, looks at the output of the Rust annotator, checks that output, and checks whether its own initial annotation had errors. Such annotation can train all the task classes the modules otherwise learn from small annotated corpora.

The `en` modules learn from the UD English treebanks EWT and ESLSpok. The annotator grows this corpus with silver: teacher annotation checked by the Rust annotator and gates. How `annot` fits with the open parsers used as witnesses: [docs/parsers.md](../docs/parsers.md); the teacher transport and the training pipeline: [docs/training.md](../docs/training.md#current-teacher-transport).

## Full UD

**Annotation.** Both the teacher and Rust produce full CoNLL-U: UPOS, XPOS, FEATS, lemma, head, and relation with subtype. In Rust, UPOS and FEATS are computed by tables over the graph (`en::ud`), with a second pass that takes verb Number/Person from the subject, as EWT does (`en/README.md`, "Agreement").

**Gate — the UD 2.18 registry for English** (`en::udreg`, data `en/data/ud-registry-en.tsv`):
- features are allowed only for their own part of speech;
- relations come from the registry;
- AUX only from the 16 auxiliary verbs;
- `cop` only "be".

`parse_block` normalizes teacher output to EWT conventions (`ann::ewt_rel`): `det:poss` → `nmod:poss`, `:tmod`/`:npmod` → `:unmarked`; the report shows the number of replacements, and the prompt states these conventions.

**Metrics** — CoNLL 2018 (`en::ud::Scores`).

**Third opinion — UDPipe 2** (LINDAT, `english-ewt-ud-2.17`, NC), for cross-checking and measurement only (`annot udpipe-eval`).

**Silver** is appended to our own corpus `corpus/mova-silver-en.conllu`, with the source and license of every sentence. Description — [corpus/README.md](../corpus/README.md). Sentences come from two sources: Tatoeba (CC0) and open papers from the library (CC BY, `annot sentences`).

## Flows

`ANNOT_FLOW` selects how the teacher works with the Rust draft:

- **default — two looks:** the teacher's first look, then a second look that sees both annotations (see the cycle below).
- **`draft`:** Rust annotates first and its output goes to the teacher as an example; the teacher corrects it in a single pass.
- **`patch`:** Rust annotates first; the teacher returns only patches to the draft, Rust applies them and the gates check the result. Mass silver is produced in this mode with `ANNOT_FEATS=rust`. The batch flow (`annot batch-prepare`) supports only `patch`.

`ANNOT_FEATS=rust` (or `annot refeat` after the fact) replaces the teacher's FEATS by features from the `en::ud` tables on top of the teacher's tags and tree, without an extra call.

`ANNOT_HINTS=error|warn` adds hints from the seed rules (`ANNOT_SEEDS`, default `en/seeds`) in patch mode. Hints are off by default. The seed rules are precise but cover few of the teacher's errors (they catch the ungrammatical, not wrong attachments), so they are meant to check the teacher's patches after the fact rather than to steer the first look.

`annot batch-blind` repeats a patch batch without the draft, as a check that the teacher corrects rather than copies the draft.

Limit: EWT is public, so a teacher may have seen it in training; on sentences outside treebanks (Tatoeba, papers) there is no gold, and quality there is not established.

## Cycle (per batch of sentences)

1. **First look.** The teacher annotates sentences with fixed tokens in CoNLL-U: lemma, PTB tag, head, UD relation.
2. **Rust annotator** (`en`: tagger, morphology on the in-code dictionary, parser) annotates the same tokens.
3. **Second look.** The teacher sees both annotations and the list of differences and gives the final one: it chooses its own, Rust's, or a third option, and also checks places where both agree.
4. **Gates in Rust** (`ann.rs`, test with negative controls):
   - forms match the given tokens;
   - tag is from enum `Tag`, relation from enum `Rel`;
   - heads within the sentence, exactly one root, no cycles.

   A block that fails does not go into the corpus.

The teacher is Codex CLI through the shared [`mova-teacher`](../teacher) crate, authenticated with ChatGPT; there is no fallback to the Claude API ([docs/training.md](../docs/training.md#current-teacher-transport)). The model comes from `MOVA_TEACHER_MODEL`, else `ANNOT_MODEL`, else the shared default `gpt-6.1-terra`; only `gpt-` models are accepted. Effort is `ANNOT_EFFORT` (medium). The teacher is called in an empty directory with no tools. Time and token accounting — `calls.jsonl`.

## Calibration before trust

`annot calib` — the same cycle on sentences from the **UD test set**, where gold is known. It measures the accuracy of the teacher first look, the Rust annotator and the final annotation against gold. Only after that can annotation of random sentences (`annot silver`) be trusted.

## Commands

```
annot calib  <UD train>... -- <UD test.conllu> <n> <seed> <out-dir>
annot silver <UD train>... -- <sentences.tsv> <n> <seed> <out-dir>
annot report <out-dir> <test.conllu>                  what the check changed in the first look
annot refeat <UD train>... <out-dir> <test.conllu>    FEATS from Rust on top of teacher annotation
annot feats-diff <file.conllu> <test.conllu> [top]    where features diverge from gold
annot seed-examples en/seeds <out.tsv>                examples from seeds
annot bronze <UD train>... -- <sentences.tsv> <corpus.conllu>   bronze: Rust only
annot sentences <papers-dir> <out.tsv> [max]          random sentences from open papers
annot drafts <UD train>... -- <sentences.tsv> <n> <seed> <out.conllu>   Rust drafts only, no teacher
annot udpipe-eval <test.conllu>                       UDPipe 2 against gold (measurement only)
annot gate-check <drafts.conllu>...                   the draft gates without a teacher: refusals by reason
annot db-repair <in.conllu> <out.conllu>              the EModE repairs over a parse database, into a new file
```

Batch flow (patch requests; restartable local Codex batches, `codex_batch.rs`):

```
annot batch-prepare <UD train>... -- <test.conllu | sentences.tsv> <n> <seed> <out-dir>   drafts and batch-requests.jsonl
annot batch-run <out-dir>              run the requests through a local Codex batch worker (ANNOT_NO_WAIT=1: do not wait)
annot direct-run <out-dir>             the same requests, waiting in the foreground
annot batch-apply <out-dir> <results.jsonl>   patches, gates, report, corpus append
annot batch-cost <out-dir>             token counts of a finished batch (no dollar cost for Codex)
annot batch-blind <patch-dir> <out-dir>   the same sentences with no draft: the anchoring check
annot codex-worker <out-dir>           the worker process that batch-run starts; not run by hand
```

Output:
- `opus-a.conllu`, `rust-b.conllu`, `final.conllu` — annotations;
- `opus-*.txt` — raw responses;
- `calls.jsonl` — call accounting;
- `report.txt` — report.

Source of random sentences: English Tatoeba from the **CC0** subset (`$MOVA_DATA/annot/src/tatoeba-eng-cc0.tsv`, provenance alongside).

## Limits

- Silver grows slowly: batches of 20–50 sentences under a shared limit.
- The `en` modules are not retrained on UD + silver here; such a retraining needs a negative control (corrupted silver must give a worse result).
- Only UD trees: no query slots, forecast components or Ukrainian with the same scheme.
- UD English is the output form. The long-term direction is a language-independent abstract annotation (predicate, participants with roles, universal features, as in `kg` and in the `en` enums), with UD as one of its translations.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
