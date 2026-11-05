# 12. Mova and Modern Hebrew: where it stands, what it would need

Mova does **not** parse Modern Hebrew yet. This chapter says what Mova already has that touches Hebrew, what is missing, and a plan in steps that keeps to Mova's rules: deterministic Rust, measured before claimed, licences respected, a large model only as a teacher. It is a design note, not a commitment: a new language is a new direction and needs the operator's word before work starts (`AGENTS.md`).

[← README](README.md) · [← 11 UD analysis](11-ud-analysis.md) · [sources →](sources.md)

---

## 12.1 What exists today (2026-10-17)

| Piece | What it does | Hebrew today |
|---|---|---|
| `en` | tokenizer, tagger and transition parser (averaged perceptrons), lexicon compiled in | English only; its FEATS registry (`en/src/gram.rs`) has no `HebBinyan`, `HebExistential`, `Definite=Cons`, `Prefix`; features outside it are dropped on reading |
| `en::conllu` | CoNLL-U reader and writer | keeps multiword token lines (`3-4`) in the full view and drops them in the training view — enough to read Hebrew files without loss |
| `gparse` | GPU biaffine parser, trained from scratch, no pretrained weights | language-agnostic on **gold words**: it could be trained on Hebrew UD today; it has no segmenter, and its data reader skips MWT ranges (`gparse/src/data.rs`) |
| `prag`, `world` | frames and the world of the text | read English UD trees; their lexicons (VerbNet classes, verb classes in `global`) are English |
| `tools/macula` | Biblical Hebrew (MACULA) trees → UD, against UD Ancient Hebrew PTNK: test UAS 88.38 / LAS 82.23 (2026-10-17); also `macula prag`, Hebrew pragmatics by rules, compared with the English reading of the same verses | **Ancient** Hebrew (`hbo`) only, already segmented by MACULA; the closest thing Mova has to a Hebrew reader |
| `treebanks/cards.tsv` | fact rows for HTB, IAHLTwiki, IAHLTknesset, PostRab (UD 2.18) | metadata, sizes, paper ids; no converter (`convert.md`) and no seeds yet |
| `dialects/` | the Mova UD dialect and its seeds | no Hebrew seeds |
| data | `$MOVA_DATA` and the research library | UD Hebrew treebanks, Hspell, UniMorph Hebrew, Wiktionary (kaikki) Hebrew, Dicta wordlist (NC), Dicta suffixed verbal forms, Hebrew Wikibooks grammars, UD Hebrew guidelines — see [sources.md](sources.md) |
| witnesses | `docs/parsers.md` | UDPipe 2 models for all UD languages are downloaded (NC: tests only); Stanza and Trankit are installed and have Hebrew models available (trained on HTB; to be fetched; tests only until the data licence is checked); GF RGL has a small Hebrew resource grammar (`gf-rgl/src/hebrew`, about 1 400 lines) |

So the pieces that would carry a Hebrew pipeline exist in general form (CoNLL-U with MWTs, a language-agnostic GPU parser, a UD dialect with converters, a rule-based Hebrew reader for the Bible), but none of them reads Modern Hebrew text.

## 12.2 What Hebrew asks of a pipeline that English does not

From chapters [02](02-script-and-spelling.md), [04](04-nouns-adjectives-pronouns.md), [05](05-verbs.md) and [11](11-ud-analysis.md):

1. **Segmentation before tagging.** One token in three is a multiword token; prefixes ו ה ש ב כ ל מ and pronoun suffixes must be split, and the split is ambiguous in unpointed text (בצלם has five readings). English has nothing comparable beyond *don't*.
2. **The covert article.** בבית is "in a house" or "in the house"; the letters do not say. The answer lives in a feature (`Definite=Def` on the preposition, IAHLT scheme) and is decided by context.
3. **No vowels.** Most words have several readings: ספר *séfer* "book", *sapar* "barber", *safar* "counted", *siper* "told", *sfar* "frontier". Lemma and POS depend on context. Pointing (niqqud) appears only in poetry, children's books and dictionaries.
4. **Spelling variation.** Full spelling (ktiv male) is standard but variable: היתה/הייתה "was", אמא/אימא "mother". A lexicon must hold the variants or normalise.
5. **Root-and-pattern morphology.** A verb's lemma (the 3MS past) and its binyan predict voice and valency (pi'el active, pu'al its passive; hif'il causative; hitpa'el reflexive or reciprocal). This is exactly the kind of structure a frame builder needs (who did what to whom), and it is invisible in the letters unless analysed.
6. **Gender and number agreement everywhere** — on verbs, adjectives, numerals, pronouns. Agreement is a strong cue for attachment; a parser should use it.
7. **Verbless clauses.** Present-tense "be" is absent or a pronoun (הוא, זה). The predicate heads the clause; a frame reader must make "X is Y" frames without a verb.
8. **Existentials and possession** (יש/אין לי) instead of "have": in `world` terms, a holding expressed by an existential with a dative.
9. **Right-to-left text, mixed with digits and Latin.** Logical order in files, bidirectional display; never reverse strings. Gershayim ״ and geresh ׳ are often typed as `"` and `'`.

## 12.3 Licences of the Hebrew data (for building vs testing)

Following `licence/DATA.md` and the operator's rules (NC and research-only: tests only; CC BY-SA: as little as possible, never carried or modified into a release):

| Resource | Licence | May be used for |
|---|---|---|
| UD Hebrew-HTB | CC BY-NC-SA 4.0 | tests and measurements only |
| UD Hebrew-IAHLTwiki, -IAHLTknesset | CC BY-SA 4.0 | training (a model trained on it would be CC BY-SA, like `en.bin` from EWT), tests; not carried as data |
| UD Hebrew-PostRab | CC BY-SA 4.0 | tests of robustness on older Hebrew |
| Hspell 1.4 (word lists, morphology) | AGPL-3.0 | copyleft: tests and comparison; building from it needs a separate decision |
| UniMorph Hebrew (from Wiktionary) | CC BY-SA 3.0 | paradigm checks, tests; learning |
| kaikki.org extract of English Wiktionary, Hebrew | CC BY-SA (Wiktionary) | glosses, inflection tables for checks |
| Dicta wordlist | CC BY-NC 4.0 | tests only |
| Dicta Hebrew suffixed verbal forms | CC BY 4.0 | build (with attribution) |
| UD Hebrew guidelines | CC BY-SA (conservative reading) | quote, follow |
| Hebrew Wikibooks grammars | CC BY-SA | read, cite; not copied |
| Our own rules, seeds, paradigms written from grammar knowledge | Elamur Cosmic | build |

The paradigms of Hebrew are facts of the language: Mova can write its own generator of inflection (binyan × person × tense, weak-root rules) under its own licence and **check** it against UniMorph, Hspell and Wiktionary without copying them. That is the cleanest road to a lexicon that can ship.

## 12.4 A plan in steps

Each step has its measure and its negative control. Nothing here is started without the operator's word.

### Step 0 — data in the Mova dialect

- A Hebrew page in the dialect: base = the **IAHLT scheme** (concatenative MWTs, `Definite=Def` on the preposition, no inserted words), because it is the scheme of the CC BY-SA treebanks, it is simpler, and it matches Arabic. Keep `compound:smixut` as a subtype in the Mova dialect (it is more informative than `compound` and convertible without loss: § 11.11 rule 4).
- Converters both ways, as rules in the `convert` language: IAHLT ⇄ old HTB (deterministic, Zeldes et al. 2022), so HTB serves as a second **test** set.
- Seeds in `dialects/mova/seeds.md` for each Hebrew decision, with the evidence.
- A FEATS registry for Hebrew (`ud-registry-he.tsv`), including `HebBinyan`, `HebExistential`, `Definite=Cons`, `Prefix`, `ExtPos`.
- **Measure:** round trip IAHLT → old → IAHLT is byte-identical on all three IAHLT files. **Negative control:** a converter with one rule removed must fail the round trip.

### Step 1 — a segmenter

- Character-level: for each character, "does a new word start here?" (Zeldes 2018), plus the covert-article decision on ב/ל/כ. Deterministic features: the character window, prefix and suffix letter sets, a lexicon lookup of the candidate pieces.
- Lexicon from our own paradigm generator (§ 12.3), checked against UniMorph/Hspell.
- Trained on IAHLTwiki + IAHLTknesset train; decided on two-fold cross-validation on train; IAHLT test read once per release.
- **Target to beat:** MWT F1 of Stanza (92.9 on Wiki) on the way to HebPipe (98.8) (Zeldes et al. 2022: Table 4). **Negative control:** a segmenter that never splits must score far lower (it gets every MWT wrong: about one token in three).

### Step 2 — tagger and parser

- `gparse` trained on gold Hebrew words (it is language-agnostic); then end to end on the segmenter's output.
- The `en` perceptron tagger/parser generalised to a second language would need its lexicon and feature templates made per-language; agreement features (gender, number, person) should be template inputs.
- **Witnesses** for the agreement loop (`tools/treeagree`): Stanza and Trankit Hebrew, UDPipe 2 — as tests only, under their data licences.
- **Measure:** LAS on IAHLTwiki dev (cv), HTB test as an out-of-domain measure. Run twice; the two runs must be identical.

### Step 3 — Hebrew into frames

- `prag` rules for Hebrew trees:
  - verbless clause with `cop` or no verb → a state frame (X is Y);
  - יש/אין + ל → possession (a holding in `world`), with polarity;
  - binyan → voice and valency: nif'al/pu'al/huf'al → passive or unaccusative frame, hif'il → causer + causee, hitpa'el → reflexive or reciprocal;
  - את + definite noun → patient;
  - pro-drop: 1st/2nd person subject from the verb's person feature;
  - negation לא / אין / אינו / אל.
- A bilingual concept table Hebrew lemma → Mova concept (the English lexicon), built from our own seeds and checked against Wiktionary glosses.
- **Measure:** the same as `macula prag-compare` for the Bible: agreement of the Hebrew frames with the frames of an English translation of the same sentences, on a parallel set we may test on.

### Step 4 — answering

`world` and the readers are language-independent once frames exist. A Hebrew question (מי נתן לדני את הספר? "who gave Dani the book?") becomes a frame query like an English one. A large model may teach (check silver trees, propose seeds) but never answers.

## 12.5 What we would not do

- Train on HTB or ship anything derived from it (NC).
- Copy Wikibooks, Wiktionary or grammar tables into the repo.
- Let a teacher model segment or parse at test time.
- Normalise spelling silently: every normalisation is a rule with a reason, and the original form is kept.

## 12.6 Open questions (parked for the operator)

1. Is Modern Hebrew a language Mova should take on now, after English, or only after the abstract annotation (`docs/dialect.md`) is ready?
2. Hspell is AGPL: may it be used to *build* (e.g. as a lexicon source), or only to test?
3. Should the Mova dialect keep `compound:smixut` (HTB, PostRab, UD guidelines) or follow IAHLT's plain `compound`?

---

## References

- Mova repo: `AGENTS.md`, `docs/architecture.md`, `docs/dialect.md`, `docs/parsers.md`, `licence/DATA.md`, `en/src/conllu.rs`, `en/src/gram.rs`, `gparse/README.md`, `gparse/src/data.rs`, `tools/macula/README.md`, `tools/treeagree/README.md`, `treebanks/cards.tsv`.
- Zeldes, Amir. 2018. A Characterwise Windowed Approach to Hebrew Morphological Segmentation. SIGMORPHON 2018. https://aclanthology.org/W18-5811/
- Zeldes, Amir, Nick Howell, Noam Ordan and Yifat Ben Moshe. 2022. A Second Wave of UD Hebrew Treebanking and Cross-Domain Parsing. EMNLP 2022. https://aclanthology.org/2022.emnlp-main.292/
- Tsarfaty, Reut, Shoval Sadde, Stav Klein and Amit Seker. 2019. What's Wrong with Hebrew NLP? And How to Make it Right. EMNLP-IJCNLP 2019 demos. https://aclanthology.org/D19-3044/
- Licences: Hspell (AGPL-3.0) http://hspell.ivrix.org.il/; UniMorph Hebrew (CC BY-SA 3.0) https://github.com/unimorph/heb; Dicta datasets https://huggingface.co/dicta-il; UD treebanks per their `LICENSE.txt`.
