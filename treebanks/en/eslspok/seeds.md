# Seeds: treebanks/en/eslspok

The seeds of this treebank, one section per seed. A section's anchor is the seed name: `seeds.md#errors`.

<a id="data"></a>

# ESLSpok — what these data are and which layers exist

**Gist.** Spoken language of Japanese learners of English: a random sample of utterances from language-proficiency interviews, the NICT JLE corpus. The sample is called SL2E. The interviewers' utterances were removed. The treebank has 2320 sentences and 21,312 words. XPOS (PTB) and dependencies were annotated manually from scratch by at least two annotators, and a third resolved disagreements. Agreement before adjudication: POS — 95.1%, LAS — 86.5%. UPOS was derived from XPOS and dependencies by a model with accuracy 0.9885 and then checked manually. Dependencies follow UD 2.0; the treebank has not changed since release 2.12.

**Conditions and exceptions.**
- Split: train 1856 / dev 232 / test 232 sentences.
- Learner errors are annotated literally, by form, not by intent. Tags follow the PTB guideline and Berzak et al. 2016 (TLE): *she feel very happy* — *feel*: VBP.
- The README metadata say «Features: manual native», but the data have no FEATS (`cut -f6` — «_» in 21,312 of 21,312).

**Examples.**
- `file01145.txt_29` *And when I go to my grandmother house , she feel very happy , so she give me many money .*
- `file00207.txt_69` *he runs every morning around here* — a sentence without a capital letter and a period, as in the transcript.

**In UD.** Layers:

| layer | state |
|---|---|
| lemmas | absent: «_» in 21,312 of 21,312 |
| FEATS | absent: «_» everywhere |
| DEPS, MISC | absent: «_» everywhere, even `SpaceAfter` |
| MWT | absent: *n't*, *'s*, *'m* — separate words without a `1-2` line (`tokenization.md`) |
| XPOS | PTB, 39 tags. No EWT tags ``` `` ```, `$`, `HYPH`, `NFP`, `ADD`, `AFX`, `GW`, `-LRB-`, `-RRB-`, `LS`, `SYM`, `WP$`. There is `"` (50) instead of the pair ``` `` ```/`''` and `X` (1) |
| `# text` | tokenized: spaces before *'s*, *n't* and punctuation |

Our annotator already trains on ESLSpok train together with EWT. It skips the missing layers: `Sentence::has_lemmas` and `has_feats` (`en/src/conllu.rs`). On the ESLSpok test: UPOS 95.23, XPOS 95.45, LAS 86.23 (`en/README.md`).

**Sources.** README `UD_English-ESLSpok`; `$MOVA_DATA/raw/ud-docs/docs/treebanks/en_eslspok/index.md`; Kyle et al. 2022, `2022.bea-1.7` (papers/k/ky/kyle-2022-dependency-treebank-spoken-second-language), sec. 3 (annotation, agreement, Berzak et al. 2016); digest the UD survey digest «english-banks», §1.11.

<a id="errors"></a>

# ESLSpok — known annotation errors

**Gist.** ESLSpok annotates learner errors literally, and that is not a fault but a custom (`data.md`). Faults are where the annotation contradicts its own guideline. The converter does not fix them. Counters — the rules of `../seeds-overview.md`, violations / matches.

**Examples and numbers.**
- ***to* as `obl` or `xcomp`** — 8 times: `obl` 5, `xcomp` 3. A PART cannot be an argument.
- **Passive subject as `nsubj`** (`tb.en.pass-subj`): 1 of 35 passives with `aux:pass`.
- **`obl` on a noun without a copula** (`tb.en.obl-fragment`): 7 of 20:
  - `file00084.txt_27` *Hokkaido city , for six years .* — a fragment; here EWT could also give `obl`;
  - `file00646.txt_48` *please the ticket to me .* — *me*: `obl` → *ticket*. For EWT this is `nmod`.
- **A demonstrative as a noun with UPOS DET** (`tb.en.dem-pron`): 1 of 121.
- **A sole `iobj` without `obj`** (`tb.en.sole-iobj`): 6 of 26, for example `file00054.txt_71` *I can not tell you about what you can do*. Under UD 2.0 this should have been `obj`. A sole `iobj` has been allowed only since UD 2.12 (the "Sole iobj" amendment). So these 6 are either errors by their own guideline, or an early switch to the new one.
- **`dep` on Japanese words** (26) — not an error but a custom (`old-ud.md`). But `dep` does not say what role the word has in the sentence.

**In UD.** There are few errors, because the annotation was done by two people with adjudication. ESLSpok's main problem for merging is not errors but old customs and missing layers.

**Sources.** UD `docs/_en/dep/iobj.md`; `docs/changes.md` No. 7 (Sole iobj, 2.12); Kyle et al. 2022, `2022.bea-1.7` (agreement 95.1 / 86.5 before adjudication).

<a id="old-ud"></a>

# ESLSpok — UD 2.0 customs: *when* as `mark`, *so* as CCONJ, Japanese insertions as `dep`, no new subtypes

**Gist.** ESLSpok was annotated following UD 2.0 (Kyle et al. 2022) and has not been updated since. So it lacks the changes that EWT adopted in 2.10–2.17:
- subordinate *when/where/how* are ADV `advmod` in EWT since 2.11; in ESLSpok *when* is SCONJ `mark`;
- "X, so Y" is annotated by EWT since 2.10 as `parataxis` with *so* — ADV `advmod`; in ESLSpok *so* is often CCONJ `cc`;
- EWT removed `flat:foreign` in 2.13; ESLSpok has it, and Japanese words in an English sentence have `dep`;
- no new subtypes: `obl:agent`, `nsubj:outer`, `advcl:relcl`, `nmod:desc`, `:unmarked` — 0.

**Conditions and exceptions.**
- *when/where/how/why* in `mark` — 41 of 103 (`tb.en.wh-advmod`). All 41 are *when* SCONJ.
- *so*:
  - ADV `advmod` — 271;
  - CCONJ `cc` — 40;
  - CCONJ `advmod` — 20.

  In EWT *so*: ADV `advmod` 531, SCONJ `mark` 75, CCONJ — none.
- Japanese insertions: UPOS X, XPOS FW:
  - `dep` — 26;
  - `flat:foreign` — 13, plus 1 `flat:foreign` that is not X;
  - `root` — 4.

  In EWT UPOS X is only on `goeswith` fragments and other cases with the tags FW, LS, XX, ADD, AFX, GW (`docs/_en/pos/X.md`).
- There are only 2 passive *by* phrases, both `obl`. Counted by the rule `tb.eslspok.by-agent` below: VBN + `aux:pass` + *by*, without FEATS, because ESLSpok has none. EWT has 205 such, of which 196 are `obl:agent`.

**Examples.**
- `file01145.txt_29` *And when I go to my grandmother house , she feel very happy…* — *when*: SCONJ, `mark`.
- *to Fuji jukai* — *jukai*: X/FW, `flat:foreign` → *Fuji*.

**In UD.**

| rule | ESLSpok | EWT |
|---|---:|---:|
| `tb.en.wh-advmod`: *when/where/how/why* in `mark` | 41 / 103 | 0 / 1049 |
| `tb.eslspok.so-cconj`: *so* — CCONJ | 60 / 339 | 0 / 657 |
| `flat:foreign` (`grep`) | 14 | 0 |

**Sources.** EWT README v2.10 (#313 `parataxis` for "X so Y"), v2.11 (#88 WH adverbs — `advmod`), v2.13 (#459 no `flat:foreign`); UD `docs/changes.md` No. 13 (foreign insertions, 2.14); `docs/foreign.md`; Kyle et al. 2022, `2022.bea-1.7` (UD 2.0).

```rule
rule: tb.eslspok.so-cconj
what: so as a coordinating conjunction CCONJ (EWT: ADV advmod or SCONJ mark)
match: s[form=so]
require: not s[upos=CCONJ]
severity: warn
source: EWT README v2.10 (#313); counter
```

```rule
rule: tb.eslspok.by-agent
what: by phrase on VBN with aux:pass — obl:agent (without FEATS, by XPOS)
match: v[xpos=VBN]; a[rel=aux:pass, head=v]; o[rel=obl|obl:agent, head=v]; b[form=by, rel=case, head=o]
require: o[rel=obl:agent]
severity: warn
source: EWT README v2.13; counter
```

<a id="tmod-npmod"></a>

# ESLSpok — the old subtypes `:tmod` and `:npmod` (before UD 2.15)

**Gist.** ESLSpok was annotated following UD 2.0 and not updated after 2.12. So NPs without a preposition acting as adverbials or modifiers have the old labels:
- `obl:tmod` — 77;
- `obl:npmod` — 50;
- `nmod:tmod` — 20;
- `nmod:npmod` — 4.

151 in total. In `mova`, as in EWT since 2.15, both subtypes are merged into `:unmarked`, and the temporal meaning is kept by `TemporalNPAdjunct=Yes` in MISC.

**Conditions and exceptions.**
- `:tmod` — temporal NPs: *week* 17, *day* 16, *time* 10, *today* 6, *tomorrow* 5, *night* 5, *morning* 5.
- `:npmod` — measures and quantity: *bit* 11, *years* 10, *times* 4, *little* 4.
- Only 4 of 151 have a preposition — real NPs without case, as the guidelines require. Compare with ATIS: 81% there.
- ESLSpok has not a single `:unmarked`.

**Examples.**
- `file00207.txt_69` *he runs every morning around here* — *morning*: `obl:tmod`.
- `file01070.txt_33` *last weekend my best friend in Tokyo came to Osaka…* — *weekend*: `obl:tmod`.

**In UD.**

| rule | ESLSpok | EWT |
|---|---:|---:|
| `tb.en.tmod-npmod`: old `:tmod`/`:npmod` | 151 / 151 | 0 / 0 |
| `tb.atis.tmod-case`: of them with a preposition | 4 / 151 | 0 / 2604 |

**Sources.** UD `docs/_en/dep/obl-unmarked.md`, `dep/nmod-unmarked.md`; docs#1028; EWT README v2.15 (`TemporalNPAdjunct=Yes`); README `UD_English-ESLSpok` (UD 2.0, changes — only 2.12).

<a id="tokenization"></a>

# ESLSpok — PTB tokenization without MWT and without `SpaceAfter`: the text cannot be reconstructed

**Gist.** ESLSpok words are the same as in EWT: contractions are split per PTB (*do n't*, *it 's*, *I 'm*). But:
- there are no MWT lines (`3-4 don't`);
- MISC is empty, so there is no `SpaceAfter=No`;
- `# text` is a tokenized string with spaces: *the man on the motorcycle 's fault .*

So the original text cannot be restored. EWT since 2.7 groups contractions into MWT, and `# text` there is the real string.

**Conditions and exceptions.**
- Separate words without MWT: *'s* — 209, *n't* — 179, *'m* — 110, *'ll* — 14, *'re* — 13, *'d* — 13, *'ve* — 4.
- `# text` with a space before *n't*: ESLSpok — 173 sentences, EWT — 0.
- *wanna* remained one word (VBP) — once. EWT splits it into *wan* + *na*: *wan* VBP 6, *na* TO 13.
- Segmentation is by the sample's sentences, without documents and paragraphs: there is no `# newdoc`.

**Examples.**
- `# text = And so this accident became the man on the motorcycle 's fault .`
- `# text = And then think about what we really wanna eat .`

**In UD.** For parsing and tagging there is no difference: `en` reads syntactic words and skips MWT lines (`en/src/conllu.rs`). The difference matters for the tokenizer (`tok.rs`) and for the full "text → graph → text" cycle: they cannot be trained on ESLSpok, because the spaces in `# text` are artificial. The converter into `mova` can restore MWT by the forms (*n't*, *'s*…), but can only guess `SpaceAfter` (see `convert.md`).

**Sources.** README `UD_English-ESLSpok`; UD `docs/_en/tokenization.md` ("new" PTB, MWT); EWT README v2.7 (MWT for contractions); Zeldes & Schneider 2023, `2023.udw-1.7`, sec. 3 (MWT in EWT 2.7, in GUM 2.8).

<a id="to-part"></a>

# ESLSpok — prepositional *to* is also PART/TO

**Gist.** The PTB guideline (Santorini 1990) gives the TO tag to every *to*: both the infinitive particle and the preposition. ESLSpok tags exactly that way. UPOS was derived from XPOS, so the preposition *to* in the `case` role got PART. EWT, following the "new" PTB, gives the preposition *to* XPOS IN and UPOS ADP. In `mova` *to* with a noun is ADP/IN, with an infinitive PART/TO.

**Conditions and exceptions.**
- All 576 *to* in ESLSpok are PART/TO:
  - `mark` — 341;
  - `case` — 224;
  - `obl` — 5, `xcomp` — 3, `fixed` — 2: these are errors, see `errors.md`.
- The role is annotated correctly: `case` with a noun, `mark` with a verb. So the converter can rely on it.

**Examples.**
- `file01070.txt_33` *…came to Osaka…* — *to*: PART, TO, `case` → *Osaka*.
- `file00498.txt_23` *And ballet is the decide to movement .* — *to*: `case` with *movement*.

**In UD.**

| rule | ESLSpok | EWT | CHILDES |
|---|---:|---:|---:|
| `tb.en.to-case`: *to* in `case` not ADP | 224 / 224 | 0 / 2029 | 24 / 1104 |

In EWT *to*: PART/TO 3993, ADP/IN 2211, SCONJ/IN 72.

**Sources.** Santorini 1990 (PTB), TO; UD `docs/_en/pos/ADP.md`, `pos/PART.md`; `docs/_en/tokenization.md` ("new" PTB, as in OntoNotes); Kyle et al. 2022, `2022.bea-1.7`, sec. 3 (PTB guideline for XPOS, UPOS derived from XPOS).
