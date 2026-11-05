# en — Mova's English graph compressor

`en` is Mova's English core. It tokenizes, tags and parses text into full Universal Dependencies (UPOS, XPOS, FEATS, lemma, head, relation) with a deterministic Rust pipeline trained on UD like a model, but without neural nets, and it generates text back from the graph. The rest of Mova reads English through it.

**How it is built.** Everything is Rust structures. Words are numeric constants built into the binary, so a rule compares integers. The tables (every form of a word → its class and features) are compiled in. The tokenizer is a table-driven automaton: (state × character class) → (command, new state). English becomes a compact, deterministic code. The parser keeps the structure in a graph, and the generator gives the sentence back from it: not always word for word, but in the same style.

A step-by-step walk through the pipeline on one sentence (dictionary, tokenizer, tagger, morphology, parser, graph, generator) is in [docs/english.md](../docs/english.md#how-the-compressor-works-step-by-step); the same page describes the [expert system of seeds and rules](../docs/english.md#the-expert-system-seeds-and-rules), [trees by logical derivation](../docs/english.md#trees-by-logical-derivation) and the [round trip](../docs/english.md#the-round-trip-what-each-layer-keeps-and-loses). The open parsers used as witnesses and test references are in [docs/parsers.md](../docs/parsers.md). This README is the crate reference: structures, data files, commands, environment variables.

## Rust structures

- **`gram.rs` — types instead of strings.**
  - A PTB tag (53) and a UD relation (57) are one-byte enums. An unknown relation in a treebank is a read error with a line number.
  - `Case` is an enum with a `Raw` escape.
  - A lemma is a `Sym` number: in the dictionary heap, or, with the high bit set, in the graph's local heap (new words).
- **Graph (`model.rs`).**
  - A `GNode` vertex is 12 bytes without strings: lemma, tag, case, relation, head, space.
  - Sentence bits: "interrogative", "first word capitalized".
- **Tables.**
  - Parser features are numeric keys (template number + word and tag codes).
  - The tagger is a dense 54³ transition array with exact Viterbi.
  - The order generator's keys are structures (`Class`, `SideKey`, `Link`).
  - The hash is deterministic (`hash.rs`), so training is reproducible.

## Dictionary in code (`data/`, `build.rs`, `dict.rs`)

- **`en lexicon <agid infl.txt> <train.conllu>...`** generates two files:
  - `data/lemmas.tsv` — the lemma heap: number = line, **append-only** (numbers are frozen: a graph written today is always readable). First by UD frequency, then alphabetically.
  - `data/forms.tsv` — full expansion of forms: entries "form → tag, lemma, UD frequency, source" (UD / main form of an AGID cell / clean for reverse lookup / stem).
- **`build.rs`** compiles both files into static open-addressing hash tables (the same hash as at runtime). Nothing is built or read from disk at runtime.
- **Word constants:** the 5000 most frequent lemmas plus `data/consts.txt` (weather: `w::CELSIUS`, `w::FAHRENHEIT`, `w::RAIN`…). A rule in code is an integer comparison.
- **Morphology (`morph.rs`)** — stateless, on the dictionary in code: UD frequencies → AGID paradigms → suffix rules. For a generator, `inflect_feats` inflects from lemma, tag and FEATS (Case=Acc pronouns, am/are/were by Person and Number, these/those), and `tree_feats` reads those FEATS from an unordered tree by agreement (the subject's case, the subject of «be»).
- **Tagger.** Unknown words get a class hint from the full expansion (weighted ×100 over the suffix model).
- **Derived words (`derive.rs`).** A form the dictionary does not know is read as a known word with productive affixes: *unsupervised* = un- + supervised, *tokenised* = the British spelling of token + -ize + -ed, *incrementally* = increment + -al + -ly, *self-attention*. `morph::known_form` counts such a word as known, the lemmatizer takes a derived lemma (*downsampled* VBN → downsample), and both taggers get the derivation's tags as their dictionary hint for a lower-case unknown word (`EN_DERIVE_TAGS=0` turns the hint off, the reference). The rules are data: ```` ```derive ```` blocks in the word-formation seeds (`seeds/morph/wf-negative-prefixes.md`, `wf-prefixes-neutral.md`, `wf-adj-suffixes.md`, `wf-noun-suffixes.md`, `wf-verb-suffixes.md`, `adv-ly.md`, `wf-spelling-variants.md`), compiled in. Each line is `prefix <affix> <stem classes> <shortest stem> [options] | gloss`, `suffix <affix> <stem classes> <result tags> [options] | gloss`, `inflect <ending> <class> <tags>` (only on a derived word) or `variant <British ending> <American ending>`; options are `after=`, `except=` (stem endings), `block=` (rival affixes that already make a dictionary word: *impossible* blocks *unpossible*), `start=`, `nostart=` (in-/im-/il-/ir- assimilation) and `part=` (un- + participle is JJ). The analysis is conservative: only a form the dictionary does not know; the bottom word is a dictionary word of an open class with at least 4 letters (5 if UD does not have it); a form is accepted only when the spelling rules (`derive::join`: e-dropping, y → i, doubling, -able → -ability, -le → -ly) give it back, so misspellings (*happyness, noticable, arguement, basicly*) are not derivations; at most 4 steps; an Early Modern English ending on a known verb is left to `ending_verb`. `en derive <word>` prints each analysis with its reason; `en lexicon-cover <text>...` counts unknown words before and after derivation.

- **Lexicon extension (`lexext.rs`, `lexgrow.rs`, `data/lexicon-ext.tsv`, `data/acronyms.tsv`).** Ordinary words the dictionary lacks: plain adjectives without comparison (*grammatical, lexical, artificial, stochastic, continuous*), the adjective reading of a word AGID has only as a noun (*directional*, so *bidirectional* is JJ), nouns with their plurals (*vertices*), verbs with their forms, domain words, proper nouns and acronyms. It is a blob of its own, so the dictionary's fingerprint, the lemma numbers and every trained model stay as they are; it answers only for what the dictionary does not know. `en lexgrow --oewn <OEWN json dir> --specialist <LEXICON> <LRAGR> --census <Names_2010Census.csv> --names <names dir> --dontuse <AGID dontuse> --out data` makes both files, deterministically, from:
  - Open English WordNet 2025 (`o`, the JSON edition, not `-plus`, which takes entries from Wiktionary): lemmas and their irregular forms;
  - the SPECIALIST Lexicon, 2026 release (`s:<EUI>`, U.S. National Library of Medicine): every listed inflected form of single-word nouns, verbs, adjectives and adverbs (an adjective gets -er/-est only when SPECIALIST lists them), and acronyms with their expansions;
  - 2010 U.S. Census surnames borne by 1,000 people or more (`c`) and the Names Corpus given names (`k`), only for forms no source has as a common word.

  The rules are conservative: a row the dictionary has with the same tag, or a form the dictionary has in the same coarse class, is not written; a SPECIALIST unit the dictionary already has under any spelling keeps the dictionary's forms (no *occured*, *noticable*), except a noun's extra plural; a SPECIALIST adjective, adverb or verb spelt with two capitals (an abbreviation's use) and a common-word reading of three letters or fewer that WordNet lacks are left out; a WordNet lemma SPECIALIST lacks gets the regular forms of `morph` (no plural for -s and -ism nouns), or only its irregular forms when WordNet lists some; a noun spelt with a capital is NNP. At run time `morph::known_form`, the lemmatizer (*vertices* NNS → vertex), `inflect`, the bases of `derive` and both taggers' hints for an unknown word read it. The hint (`lexext::hint`) reads the word as written: an acronym's rows only for a word with two capitals or more (*GPU*, not *amd* in a chat), name-list rows and names of three letters or fewer only capitalised, a capitalised word inside a sentence only as a proper noun. The plural of an acronym (*GPUs*) is NNPS. `en derive <word>` prints the extension's rows with their sources, an acronym's expansions, and the derivation of the form when there is one. `EN_LEXEXT=0` turns the extension off (the reference).

Dictionary sources: AGID 2016.01.19 (Kevin Atkinson, permissive license), UD English EWT + ESLSpok train 2.18 (CC BY-SA 4.0; ESLSpok has no lemmas, so no row comes from it). The dictionary is distributed under CC BY-SA 4.0; full citations are in [data/LICENSE.md](data/LICENSE.md), compiled in as `dict::LICENCE` outside the fingerprinted blob and printed by `en licence`. Lexicon extension: Open English WordNet 2025 (CC BY 4.0), SPECIALIST Lexicon 2026 (NLM, SPECIALIST NLP Tools Terms and Conditions), 2010 U.S. Census surnames (public domain), Names Corpus 1.3 (Mark Kantrowitz, free with credit; its README is in the file's header); see `licence/DATA.md`.

## Layers

The production model trains on UD English EWT + ESLSpok train.

| Layer | How |
|---|---|
| `tok.rs` tokenizer | table automaton: character class × state → command and new state; dictionaries from train |
| `ptag.rs` PTB tagger | averaged perceptron: word and neighbours ±2, suffixes and prefixes, word shape, dictionary in code, two previous tags; beam 8. For unknown words, a TnT prior (`tag.rs`: trigram Markov model, suffixes + dictionary) |
| `morph.rs` lemma ↔ form | dictionary in code; encoding = lemma + case or escape; position rules (a/an, capitalized first word); lossless by construction |
| `parse.rs` tree | arc-standard transition automaton; "feature → action weights" tables (type + label), averaged perceptron; **beam 8** with early update (Zhang & Clark 2008), 12 epochs |
| `lin.rs` order generator | side of the dependent (table with backoff; "heavy" feature — the dependent has its own complement, so a modifier with a complement goes after the noun) + a Markov chain on each side, dynamic programming over subsets |

`EN_TRAIN_FRAC` trains on a uniform fraction of the training data. `EN_JACKKNIFE=10` makes the parser learn on tags assigned by a tagger that has not seen those sentences; it is off by default.

**Training data and licences.** Treebanks converted into the meta-dialect `mova` ([docs/dialect.md](../docs/dialect.md)) can be added to training; GUM, for example, is converted by [`treebanks/en/gum/convert.md`](../treebanks/en/gum/convert.md). GUM is CC BY-NC-SA 4.0, so a model trained with it is research only. The production and open models train only on the source identifiers `en/data/train-licenses.tsv` admits (non-NC UD treebanks such as EWT, ESLSpok, ATIS and UD English-CHILDES, which is CC BY-SA 4.0) and on our silver ([licence/DATA.md](../licence/DATA.md)); the `childes` identifier means that UD treebank, not the TalkBank CHILDES database. Teacher silver should come from the text the module will work on (for RAG — documents and queries).

**Context tag rules (`tagrule.rs`).** Before the tagger runs, a few words whose class a short context fixes get their tag, as the EModE table does (`emode::force`). R1: «so» right after a word (no comma or coordinator), then a pronoun subject or a determiner and a noun, then *can* or *could* not negated, introduces a clause of purpose and is IN (SCONJ, `mark`): *I left the key under the mat **so** she can get it*. The rule and its expert check `en.verbal.so-purpose-mark` are in `seeds/verbal/purpose-clause.md`. The rules act in `annotate::tags` (annotation from a model file); the in-memory evaluations (`ud-eval`, `parse-eval`) tag without them.

**School results (2026-11-04).** Derivation on: *english* 234 of 666 before and after, *chemistry* 211 of 392 before and after (one more frame remembered: *unboil*, *un-fries* now read as verbs); two runs identical; `EN_DERIVE_TAGS=0` is the reference. Lexicon extension off → on (`EN_LEXEXT=0` is the reference; two runs identical): *english* 234 → 234 of 666, *chemistry* 211 → 211 of 392, *geography* 359 → 358 of 699, *biology* 2559 → 2563 of 4631.

## Examples are not truth

Training takes into account that examples are not the truth: they may contain errors, so the trained algorithm is not reduced entirely to covering the examples.

- **Tagger** (`ptag.rs`). If the model disagrees with a word's label in every epoch starting from the 3rd, there are no more updates on it. The word goes to a list of suspects — candidates for annotation errors: `EN_SUSPECTS=file en tag-eval …`. Negative control: `EN_NOISE=0.05` replaces 5% of training tags with random ones, and the suspects must find the corrupted labels.
- **Parser:** averaged perceptron and beam; epochs are chosen on dev, not on training accuracy.
- **Tables** for UPOS, FEATS and the order generator: a key fires only with ≥ 2 examples (singletons are not considered a rule), otherwise back off to a coarser key.
- **Silver** (`annot`, `corpus/`) weighs less than gold. The yardstick is only independent gold, never our own annotation.

## Direction

Train the UD meta-dialect for English to the level of top neural parsers and beyond: all English treebanks through converters into the meta-dialect ([docs/dialect.md](../docs/dialect.md)), LLM silver in large volumes (the SLM is a student of the LLM, not a distillate, [docs/context.md](../docs/context.md)), a stronger parser (the GPU parser [`gparse`](../gparse)), and one consistent meta-dialect instead of contradictory treebanks. The yardstick is EWT test, other English treebanks through reverse converters, and CPU speed. School results are in [docs/school.md](../docs/school.md#results); release-to-release comparisons are in the release notes ([0.4](../releases/0.4.md), [0.5](../releases/0.5.md)).

## Document context features (`ctx.rs`, experimental)

The idea is in [docs/context.md](../docs/context.md#multi-pass-parsing). Context features come in four groups: document genre; sentence type (GUM `s_type`), current and previous; paragraph start and speaker change; token entity (type and new/giv/acc status of the deepest mention containing it). They are conjoined with existing templates: tagger — class shift, × word shape, × first word; parser — × tag of the stack top and × tag of the buffer front; label — × tags of head and dependent. Without the `EN_CTX*` variables the annotation is bit-identical to the parser without context features.

A second pass derives context from the first (`ctx::derived`): genre, paragraph and speaker from the document passport; sentence type by a rule over the tree (`ctx::stype_rule`; in GUM `sub` is modality); entities only as noun status (the lemma has already occurred in the document → giv). First-pass annotation for training sentences comes from jackknifing by document (`en folds`); without it the model learns to copy gold UPOS.

- `EN_CTX=gold|shuffle` — training context;
- `EN_CTX_EVAL=gold,shuffle,pred` — measurement contexts, each a separate row;
- `EN_CTX_GROUPS=genre,stype,doc,entity` — feature groups;
- `EN_CTX_ENT=span|type|info` — part of the entity;
- `EN_CTX_SRC=rule` together with `EN_PASS1_DIR` and `EN_CTX_TRAIN_PASS1` — second pass;
- `EN_CTX_EWT=1` — genre for EWT from the `sent_id` prefix;
- `EN_SEED` — shuffle order;
- `EN_PRED_DIR` — test annotation for `en ctx-report`, which reports metrics by seed, a paired bootstrap over sentences and a breakdown.

## Full UD annotation (`en ud-eval`)

The annotator gives full UD; the other English treebanks are its tests.

**The output is full, as in CoNLL-U:**
- UPOS;
- XPOS (PTB);
- FEATS;
- lemma;
- head and relation with subtype.

UPOS and FEATS (`ud.rs`) are backoff tables over what the graph already knows: PTB tag, form, relation, child flags (subject, aux, passive, fixed). They are not written into the graph, since they are derived from it, so the vertex stays 12 bytes.

**Agreement.** EWT 2.18 writes Number and Person on VBZ, VBP and VBD according to the subject, and only VerbForm=Fin on MD. The subject "you" gives Number=Sing|Person=2. So verb FEATS are computed in a second pass (`Ud::feats_sentence`). `agreement` finds the subject:
- its own nsubj or csubj, failing those — expl;
- for aux and cop — the head's subject;
- for conj — the subject of the first conjunct;
- for a relative pronoun — the antecedent.

The table key gets the subject's class, its Number and Person, and a coordination feature. Two more changes:
- a conjunct without its own subject or aux takes them from the first conjunct: "has seen and heard";
- "10.2" is a separate number form (NumType=Frac).

The target is EWT conventions: ATIS writes the imperative as VerbForm=Inf and no agreement on VBP. `annot feats-diff` shows where wrong features are.

**How `ud-eval` measures.** Metrics are CoNLL 2018 on gold tokens: LAS is computed without subtypes, LAS with subtypes is a separate column; CLAS, MLAS and BLEX are F1 on content words. "—" means the treebank does not annotate this layer: ESLSpok — FEATS and lemmas, CHILDES — FEATS, ATIS, CTeTex, LinES and ParTUT — PTB tags. LinES and ParTUT have their own annotation conventions, so some discrepancies are not errors. GUMReddit is distributed without text (Reddit words are replaced with "_"). GUM, GENTLE and ParTUT are NC-licensed: tests only, not training.

### Beam width (`en beam-curve`)

There is one model (the parser is trained with beam 8); only the width at parse time changes, narrower for real-time answers and wider for batches. `en beam-curve` prints tag accuracy, LAS with subtypes and speed for a range of tagger and parser widths. The tagger default is beam 2. A parser trained with a beam loses accuracy without one, so the fastest parsing needs a separate greedy model, not a narrow beam.

### Parser diagnostics (`en parse-diag`)

`en parse-diag` reports where the parser's errors are, on dev, with our tags and LAS with subtypes:

- **Distance to the root** (McDonald & Nivre 2007, `papers/en/missing4-2007-mcdonald-characterizing-errors-data-driven-dependency.md`, I1): labelled recall by the gold depth of the dependent, precision by the predicted depth; LAS by arc length.
- **Projectivity** (same card, I4): sentences with a non-projective gold arc and their LAS against the rest.
- **Beam oracle** (Charniak & Johnson 2005, `narrative-2005-charniak-coarse-fine-n-best-parsing`, I2): the LAS of the best tree against the closest-to-gold tree among the distinct trees left in the final beam, the room for a reranker.
- **Cascading errors** (Ng & Curran 2015, `aclbulk-2015-ng-identifying-cascading-errors-constraints-dependency`, I1 and I4). One class at a time (punct; core arguments; function words; root; clauses; noun phrase; PP; modifiers; coordination; other), the gold arcs of the class are forced in the beam (head and label). Then the errors outside the class are counted again: the drop shows how many of them the class's own errors caused. "No tree" counts the sentences where every beam hypothesis died under the constraints; they keep the unconstrained parse.
- **PP attachment from the predicted tree** (Atterer & Schütze 2007, `cl-2007-atterer-squibs-prepositional-phrase-attachment-without`, I1 and I2; floors after Volk 2006, `aclbulk-2006-volk-how-bad-is-problem-pp`, I2 and I3, and Rosenthal 2010, `lingbulk-2010-rosenthal-semi-automated-annotation-prepositional-phrase`, I2). A PP is a gold `obl` or `nmod` word whose `case` child is tagged IN or TO and whose gold head is a noun or a verb. The parser **misses** a PP when its own tree does not have the same preposition as `case` of that word, with the word as `obl` or `nmod`. Each accuracy is given three ways, as the paper asks (missed = error, missed → noun, missed discarded), with the always-noun floor; rows for all PPs, *of*, other prepositions, and gold head a noun or a verb.

**Complete trees and the beam curve in `en parse-eval`** (Zhang & Clark 2008, `missing-2008-zhang-tale-two-parsers-investigating-combining`, I2 and I3). `parse-eval` also prints the share of sentences parsed fully right. `EN_BEAM_CURVE=1` parses the same model (trained with beam 8) at widths 1–16.

### Parser feature groups from the paper cards

`EN_PX` switches these feature groups for ablations (`EN_PX=none` turns them off). A model trained without them parses as it did, since the new keys have no weights in it.

| group | card |
|---|---|
| conjunct symmetry: same tag, same coarse tag, same 3-letter suffix, same capitalisation, a coordinator between (action and label features) | `aclbulk-2017-ficler-improving-strong-neural-parser-conjunction` I1 |
| subcategorisation: which core dependents (subject, obj, iobj, ccomp, xcomp, cop) s0, s1 and the label's head already have | `cl-2003-collins-head-driven-statistical-models-natural` I2 |
| distance: verbs (and modals) and punctuation between s1 and s0, binned 0/1/2+, with their tags and length; the same between head and dependent for labels | `cl-2003-collins-head-driven-statistical-models-natural` I1 |

The shipped `en.bin` (0.4) does not have these features: it also learns from the school trees and must be retrained to get them.

### Construction gate (`en construct-eval`)

LAS on a treebank can rise while a rare construction breaks. After Rimell, Clark & Steedman (2009, `papers/en/aclbulk-2009-rimell-unbounded-dependency-recovery-parser-evaluation.md`, I1 and I2), [`data/constructions.tsv`](data/constructions.tsv) holds 110 sentences of our own in 11 constructions, 10 each, never trained on. They are taken from the *Not yet* lists of `languages/en/21–29`. Each sentence names its two key arcs in EWT 2.18 conventions. The measure is the recall of those arcs (head and relation with subtype) per construction.

```
en construct-eval <train.conllu>...|<model.bin> -- data/constructions.tsv [<floor.tsv>]
```

- With a floor file it is a gate: the command fails if any construction falls below its floor.
- `EN_WRITE_FLOOR=1` records the measured recalls as the new floor.
- `EN_CONSTRUCT_MISSES=1` lists every missed arc with what the parser gave.
- There is one floor per model: `data/constructions-floor-en-bin.tsv` for the shipped `en.bin` (release 0.4) and `data/constructions-floor-ewt.tsv` for the in-memory model trained on EWT + ESLSpok train.
- Each construction has 20 arcs, so one arc is 5 points. The gate is strict on purpose: a red construction asks for a look, not an automatic rejection.
- **Negative control:** `EN_BREAK_TAGS=1` gives the parser every tag shifted by one word (a broken tagger); the gate must go red.

| construction | example |
|---|---|
| subject relative | *the dog **that** bit the postman* |
| object relative | *the book **that** she wrote* |
| extraposition | ***It** is hard **to solve** this problem* |
| *wh*-adjunct question | ***When** did the train leave?* |
| zero relative | *the cake we **baked** tasted sweet* |
| reduced relative | *the bread **baked** this morning* |
| *tough* adjective | *This problem is hard **to solve*** |
| free relative | *I like **what** you cooked* |
| *wh*-object question | ***Which book** did she choose?* |
| object complement | *They painted the door **red*** |
| gapping | *John likes tea and **Mary** coffee* |

## Full cycle `en rt-eval` (text → graph → text)

`en rt-eval` runs text → graph → text in two modes and reports exact-match share and chrF:

- **lossless** — the graph keeps vertex order; the original comes back exactly;
- **generative** — the order is recovered from the tree alone ([docs/english.md](../docs/english.md#step-6-generator-from-the-graph-back-to-text)).

## Commands

```
en train <train.conllu>... -- <model> [<check.conllu>...]                full UD annotator → model file (with check files — load back and the same annotation)
en annotate <model> <in.conllu|in.txt>...                               CoNLL-U from a model file: .conllu on gold tokens, anything else as text
en construct-eval <train.conllu>...|<model.bin> -- <constructions.tsv> [<floor.tsv>]   construction gate
en tok-eval|tag-eval|morph-eval|parse-eval|parse-diag|lin-eval|rt-eval|ud-eval|beam-curve <train.conllu>... -- <test.conllu>...
en rt-text <train.conllu>... -- <dom-train.txt> <dom-test.txt> [weight] general model vs one fine-tuned on the domain
en explain <train.conllu>... -- "sentence"...                           every step, human-readable
en graph <train.conllu>... -- <file.txt>...                             line graphs: lemma/tag/head/relation (version comparison)
en lexicon <agid infl.txt> <train.conllu>... [-- <data-dir>]            dictionary into code (data/)
en derive <word>...                                                     derivation of words the dictionary does not know, with the reason
en lexicon-cover <text>...                                              unknown words before and after derivation (EN_COVER_LIST=N lists them)
en licence                                                              the dictionary's licence and citations (data/LICENSE.md)
en ctx-report <gold.conllu> <base>=<f1>,<f2>… <system>=<…>… [--focus <name>] [--boot N]
                                                                        context experiment: metrics by seed, paired bootstrap, breakdown, examples
en folds <in.conllu> <k> <dir>                                          documents into k parts (first-pass jackknife)
en compare <ref.conllu> <pred.conllu>                                   UD scores and UPOS/lemma/UFeats agreement by class
en explain-json <train.conllu>... -- "sentence"...                      the same as explain, as JSON
en tag-cmp <train.conllu>... -- "sentence"...                           TnT and perceptron tags side by side
en ud-check <in.conllu>...                                              UD 2.18 registry violations (gold = positive control)
en expert-check <seeds dir> [--gold <gold.conllu>] <in.conllu>...       seed rules: fires and violations (EN_EXPERT_LIST=1 lists all); see docs/english.md
en xpos-fill <in.conllu> <out.conllu>                                   fill missing or non-PTB XPOS from form, UPOS and features
en convert <rules dir> <from> <to> <in.conllu>...                       UD dialect converter → CoNLL-U on stdout; see docs/dialect.md
en convert-check <rules dir> <dialect> <in.conllu>...                   X → mova → X: what changed, what did not come back
```

The library (`lib.rs`) is for domain crates: they link `en` by path and reuse its tokenizer, tagger, parser and generator.

## Limits

- The generator does not invert questions ("I should…" instead of "Should I…").
- Prepositional phrase attachment ("fog after 7am") stays weak.
- Gapping is not learned (EWT train has few `orphan` arcs; the parser makes *Mary coffee* a compound), and object complements become compounds or objects.
- Document context features are experimental; a second pass derived from the first does not supply mention boundaries.
- Density: the graph is not compressed; the format for scale is in place (id heaps, frozen numbers, 12-byte vertices).

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
