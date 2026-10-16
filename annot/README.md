# annot — the Mova annotator subproject

`annot` is the teacher side of annotation: it sends sentences, with our drafts and context, to a large model in batches, checks the answers with Rust gates, and writes silver data for training. The large model teaches; it never answers for Mova at test time.

**The design note (25.09.2026):**

> The hand-annotated corpus itself is minimal. So Opus takes random sentences, and a separate annotator subproject trains an annotator in Rust; the dictionaries already exist, so it is just an annotator module. Opus annotates, looks at the output of the Rust annotator, checks that output, and checks whether its own initial annotation had errors. Such annotation can train all the different task classes that the modules previously learned from small annotated corpora.

The `en` modules learn from 14,400 UD sentences (EWT + ESLSpok). The annotator grows this corpus with silver: Opus annotation checked by the Rust annotator and gates.

## Full UD (since 25.09.2026)

The annotation is now done in UD.

**Annotation.** Both Opus and Rust produce full CoNLL-U: UPOS, XPOS, FEATS, lemma, head, and relation with subtype. In Rust, UPOS and FEATS are computed by tables over the graph (`en::ud`).

**Gate — the UD 2.18 registry for English** (`en::udreg`, data `en/data/ud-registry-en.tsv`):
- features are allowed only for their own part of speech;
- relations come from the registry;
- AUX only from the 16 auxiliary verbs;
- `cop` only "be".

Positive control: EWT (train and test) and GUM give 0 violations; ATIS gives 3.2% (the old feature `PronType=Int,Rel`).

**Metrics** — CoNLL 2018 (`en::ud::Scores`).

**Third opinion — UDPipe 2** (LINDAT, `english-ewt-ud-2.17`), for cross-checking only. Idea: also run open-source annotators and compare their output with ours.

**Silver** is appended to our own corpus `corpus/mova-silver-en.conllu`, with the source and license of every sentence. Description — `corpus/README.md`. Sentences come from two sources: Tatoeba (CC0) and open papers from the library (CC BY, `annot sentences`).

### Max vs high (25.09.2026, 20 EWT sentences, full UD)

Effort is now actually controllable (via `--settings`).

| mode | output tokens per sentence | time for 20 sentences | AllTags | LAS | MLAS |
|---|---:|---:|---:|---:|---:|
| max, final | ~27k | 85 min | 95.0 | 92.9 | 85.3 |
| max, first look + FEATS from Rust | ~10.6k | 32 min | 93.3 | 92.0 | 82.0 |
| high, final | ~1.6k | 4 min | 90.5 | 91.1 | 80.2 |
| high, first look + FEATS from Rust | ~0.9k | 2.5 min | 91.1 | 91.6 | 81.0 |
| UDPipe 2 (EWT, NC) | — | seconds | 95.0 | 90.8 | 86.6 |

- **High:** on the first look 3 of 20 sentences failed the gate (`det:poss` instead of `nmod:poss`). A second check on high changes almost nothing: 99.5% agreement with the first look.
- **Max:** this is where the second check helps most — UFeats +4.2, MLAS +7.9.
- **FEATS from Rust** — features from the `en::ud` tables on top of Opus tags and tree (`annot refeat`). Without any extra call they give +1–3 UFeats.
- **"Draft" mode** (`ANNOT_FLOW=draft`). Idea: run our Rust annotator before the initial annotation and feed its output to Opus as an example. Rust annotates first, Opus on high corrects in a single pass.
  - Same 20 sentences: 19k output tokens for the whole batch (~0.95k per sentence), 158 s.
  - AllTags 93.3, LAS 92.9, MLAS 84.0; all 20 passed the gates.
  - This is nearly the quality of max with a check (95.0 / 92.9 / 85.3) for ~3.5% of its tokens. Two-pass high gives 90.5 / 91.1 / 80.2 for 31k.
  - No anchoring effect is visible on this sample. The final annotation agrees with Rust at LAS 80.3, while Rust alone against gold is 78.2. So Opus corrects rather than copies.
- **Caution:** the sample is small (238 tokens, ±2 points). A decision needs ≥ 5k tokens.

### High on 300 sentences (25–26.09.2026, EWT test, full UD)

**Limitation of the measurement:** sentences were only 5–30 tokens. A filter dropped short sentences and long complex ones, so the measurement is easier than the real thing. Design decision (26.09): long complex sentences, like short ones, must not be filtered out. The filter was removed from calibration, silver and bronze; the next measurement uses sentences of any length.

Two-pass high: first look and check, batches of 20, `ANNOT_FEATS=rust`. Report — `$MOVA_DATA/annot/runs/calib-ud-high300.log`.

| annotation | UPOS | UFeats | AllTags | Lemmas | UAS | LAS | MLAS | BLEX |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Opus high, first look | 96.86 | 94.52 | 91.82 | 97.38 | 93.37 | 90.88 | 79.38 | 85.02 |
| Rust | 94.76 | 94.31 | 90.75 | 96.78 | 84.22 | 80.14 | 67.37 | 71.87 |
| Opus high, final + FEATS from Rust (25.09) | 97.14 | 95.68 | 93.19 | 97.59 | 93.71 | 91.25 | 81.49 | 85.65 |
| same, Rust FEATS with agreement (26.09) | 97.14 | **97.90** | **95.41** | 97.59 | 93.71 | 91.25 | **84.77** | 85.65 |
| UDPipe 2 (EWT, NC — cross-check only) | 97.43 | 98.38 | 96.20 | 97.46 | 94.39 | 92.58 | 86.29 | 86.90 |
| **patch mode high + FEATS from Rust (26.09)** | **97.61** | **98.14** | **96.10** | **98.04** | **95.04** | **93.00** | **86.65** | **88.16** |

- **Cost of two-pass:** 30 calls, 506k output tokens (~1.7k per sentence), 66 min sequentially (~13 s per sentence).
- **Patch mode** (`ANNOT_FLOW=patch`, 26.09) on the same 300 sentences. Rust annotates first, already with agreement. Opus high returns only patches, then FEATS from Rust. Report — `$MOVA_DATA/annot/runs/calib-ud-patch300.log`.
  - Cost: 15 calls, 158k output tokens (~530 per sentence), 24 min (~4.7 s per sentence). Three times cheaper and faster than two-pass.
  - Quality is higher than two-pass on all metrics; all 300 passed the gates.
  - Against UDPipe trained on the same EWT: higher LAS, MLAS, BLEX, UPOS and lemmas; slightly lower UFeats (98.14 vs 98.38) and AllTags.
  - Opus does not copy the draft: the final agrees with Rust only at LAS 82.8.
  - **Careful with the conclusion.** EWT is public, so Opus may have seen it in training. On sentences outside treebanks (Tatoeba, papers) quality may be lower, and there is no gold there. Independent verification is future work.
- **Conclusion:** mass silver is produced in patch mode high with FEATS from Rust.
- **Honest measurement on sentences of any length** (26.09). 300 random EWT test sentences, 3360 tokens, median 8, longest 63; 92 shorter than 5 tokens, 14 longer than 30. Report — `$MOVA_DATA/annot/runs/calib-ud-patch-all300.log`.

  | | UPOS | UFeats | AllTags | Lemmas | UAS | LAS | MLAS | BLEX |
  |---|---:|---:|---:|---:|---:|---:|---:|---:|
  | patch mode high + FEATS from Rust | 97.38 | 97.53 | 95.62 | 98.36 | 94.58 | 91.90 | 84.99 | 87.42 |
  | UDPipe 2, same sentences | 97.08 | 97.89 | 95.92 | 97.50 | 94.14 | 92.11 | 85.90 | 86.40 |
  | Rust | 94.49 | 95.21 | 92.14 | 96.40 | 84.88 | 80.77 | 71.04 | 73.32 |

  - **Conclusion:** on par with UDPipe.
    - LAS −0.2, UFeats −0.4, MLAS −0.9;
    - UPOS +0.3, lemmas +0.9, BLEX +1.0, UAS +0.4.
    - On the 5–30 token sample it was 93.00 vs 92.58: that sample is easier.
  - **Cost:** 157k output tokens (~520 per sentence), 24 min.
  - **Bar on the whole EWT test** (`annot udpipe-eval`): UDPipe 2 — LAS 92.37, MLAS 86.31.
- **Rule hints** (`ANNOT_HINTS=warn`, 26.09) did not help on the same 300 sentences:
  - 131 hints in 60 sentences;
  - LAS 92.17 vs 93.00 without hints, MLAS 86.04 vs 86.65; UFeats and AllTags +0.05–0.1.
  - The difference of ~30 tokens out of 3.9k is within Opus run-to-run variance. The variance was not measured separately.
  - Likely cause: hints draw attention to the flagged spots, and Opus checks the rest less. There were fewer patches: 150k output tokens vs 158k.
  - Hints are off by default. Rules are applied after Opus, not before: they check its patches, and only sentences with violations go to a second look. This is cheap and does not disturb the first look.
- **Gates:** 292 of 300 passed; 8 sentences were lost on `det:poss`. Now `parse_block` normalizes to EWT conventions (`ann::ewt_rel`): `det:poss` → `nmod:poss`, `:tmod`/`:npmod` → `:unmarked`. The number of replacements is shown in the report. The prompt states this explicitly, and `:tmod`/`:npmod` were removed from the list of relations.
- **Agreement.** Analysis with `annot feats-diff` showed that half of the wrong features are verb Number/Person, because EWT assigns them from the subject. The `en::ud` tables got a second pass using the subject (`en/README.md`, "Agreement"). The gap to UDPipe on features shrank from 2.7 to 0.5 points.
- **Seed rules on the final annotation** (`en expert-check en/seeds --gold …`): violations of error rules are almost always real errors, 36 of 36. But they cover only 6.4% of tokens with an error: Opus makes "reasonable" mistakes (attachment), while the grammar catches the ungrammatical. So the rules are useful as precise hints for patch mode, not as full control.

## Cycle (per batch of sentences)

1. **First look.** Opus annotates sentences with fixed tokens in CoNLL-U: lemma, PTB tag, head, UD relation.
2. **Rust annotator** (`en`: tagger, morphology on the in-code dictionary, parser) annotates the same tokens.
3. **Second look.** Opus sees both annotations and the list of differences and gives the final one: it chooses its own, Rust's, or a third option, and also checks places where both agree.
4. **Gates in Rust** (`ann.rs`, test with negative controls):
   - forms match the given tokens;
   - tag is from enum `Tag`, relation from enum `Rel`;
   - heads within the sentence, exactly one root, no cycles.

   A block that fails does not go into the corpus.

Model and effort — Opus on max, variables `ANNOT_MODEL` and `ANNOT_EFFORT`. Invocation — `claude -p` in an empty directory with no tools. Time, token and cost accounting — `calls.jsonl`.

## Calibration before trust

`annot calib` — the same cycle on sentences from the **UD test set**, where gold is known. It measures the accuracy of the Opus first look, the Rust annotator and the final annotation against gold. Only after that can annotation of random sentences (`annot silver`) be trusted.

### First measurement (25.09.2026)

20 sentences of EWT test (seed 7), 238 tokens. All blocks passed the gates.

| annotation | tag | lemma | UAS | LAS |
| --- | ---: | ---: | ---: | ---: |
| Opus, first look | 97.1 | 99.2 | 96.2 | 91.6 |
| Rust annotator | 89.9 | 96.6 | 80.3 | 72.3 |
| Opus, final | 97.9 | 99.6 | 96.2 | 92.4 |

- **Cost.** Each look costs ~$1.8 and ~12 min per 20 sentences, i.e. ~$0.18 and ~73 s per sentence for both. Output is 76–83k tokens per call, almost all of it thinking on max.
- **Second look** adds +0.8 tag and +0.8 LAS for the same price as the first. It did not change the trees at all (UAS 100% against the first look).
- **`annot report`** (no Opus calls) shows exactly what the second look changed. 6 changes, of which 5 corrections and no regressions. In 10 head+relation pairs Rust was right where Opus was wrong, but Opus took only one of them.
- **The "effort high" measurement is invalid.** The box-wide `~/.claude/settings.json` sets `CLAUDE_CODE_EFFORT_LEVEL=max` and overrides the `--effort` flag. So both runs actually went on max: $3.84 vs $3.60, LAS 93.3 vs 91.6 — the spread between two max runs.
  - Fixed 25.09: `claude -p` also receives the level via `--settings`. Check on the prompt "17×23": 180–250 output tokens before the fix, 3 after.
  - A real high measurement is still ahead.
- **First silver** (Tatoeba CC0, 20 sentences, seed 11, `silver-2026-09-25-a`):
  - $2.57, i.e. ~$0.13 per sentence; ~51 s per sentence;
  - 0 self-corrections;
  - Rust against the final annotation — tag 95.8, UAS 86.7, LAS 82.1: the sentences are simple.

**About prices.** The dollars above are the API equivalent from `total_cost_usd`, not actual payment: `claude -p` runs on a subscription. The real resource is tokens and the subscription limit.

## Commands

```
annot calib  <UD train>... -- <UD test.conllu> <n> <seed> <out-dir>
annot silver <UD train>... -- <sentences.tsv> <n> <seed> <out-dir>
annot report <out-dir> <test.conllu>                  what the check changed in the first look
annot refeat <UD train>... <out-dir> <test.conllu>    FEATS from Rust on top of Opus annotation
annot feats-diff <file.conllu> <test.conllu> [top]    where features diverge from gold
annot seed-examples en/seeds <out.tsv>                examples from seeds
annot bronze <UD train>... -- <sentences.tsv> <corpus.conllu>   bronze: Rust only
```

Output:
- `opus-a.conllu`, `rust-b.conllu`, `final.conllu` — annotations;
- `opus-*.txt` — raw responses;
- `calls.jsonl` — call accounting;
- `report.txt` — report.

Source of random sentences: English Tatoeba from the **CC0** subset (41,512 sentences, `$MOVA_DATA/annot/src/tatoeba-eng-cc0.tsv`, provenance alongside).

## Next

- Scale gradually (shared limit): batches of 20–50 sentences, a few per day.
- Retrain the `en` modules on UD + silver and measure on UD test. Negative control: corrupted silver must give a worse result.
- Other task classes with the same scheme: query slots, forecast components, Ukrainian — when we return to it.
- **Long-term direction:** over time, move where possible to our own global abstract annotation for all languages — a language-independent annotation: predicate, participants with roles, universal features, as in `kg` and in the `en` enums. UD English will become one of its translations rather than the final form.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
