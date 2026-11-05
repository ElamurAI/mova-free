# en-lexgrow

## descriptant

### en-lexgrow

Generator of the lexicon extension (`en lexgrow`): from Open English WordNet 2025 (JSON, CC BY 4.0), the SPECIALIST Lexicon 2026 (`LRAGR`, `LEXICON`; NLM terms), the 2010 U.S. Census surnames and the Names Corpus (Kantrowitz), with AGID's `dontuse` list as a block list, it writes `data/lexicon-ext.tsv` (form, PTB tag, lemma or `=`, sources) and `data/acronyms.tsv` (acronym, expansion, source), both sorted, keeping only what the in-code dictionary lacks.

## approximant

### Forward approximant

specialist_forms reads LRAGR rows `EUI|form|cat|agreement|base|citation`, keeps single-token rows of noun/verb/adj/adv, maps (cat, agreement) to a PTB tag (noun thr_plur → NNS, else NN, NNP when the base has a capital; verb infinitive VB, pres thr_sing VBZ, other pres VBP, past VBD, past_part VBN, pres_part VBG; adj/adv positive JJ/RB, comparative JJR/RBR, superlative JJS/RBS), drops non-noun rows whose base has two capitals, computes the covered units (an EUI any of whose forms the dictionary has in the same coarse class) and writes a covered unit's rows only for an NNS of a dictionary NN lemma. oewn reads entries-*.json: a capitalised noun → NNP; a noun → NN and, unless SPECIALIST has the lemma, its listed irregular plurals or the regular plural of `morph` (not for -s or -ism nouns); a verb → VB, VBP and, unless SPECIALIST has it or WordNet lists irregular forms, regular VBZ/VBD/VBN/VBG; an adjective → JJ and listed -er/-est forms as JJR/JJS when SPECIALIST lacks it; an adverb → RB. put writes (lower form, tag, lemma) → sources unless the form is in dontuse, the dictionary has the form with the tag, or the dictionary has the form with a tag of the same coarse class or the tag is a proper noun. census and first_names add NNP rows for names no source has as a common word and the dictionary lacks. Output drops common-word rows of ≤ 3 letters without a WordNet source and keeps one SPECIALIST EUI per row. Acronyms: WordNet synset pairs (an acronym_ok member and a multi-word member whose initials it is) first, then SPECIALIST acronym_of, at most 8 per acronym.

Domain/state: the source files as read; the in-code dictionary; no network, no randomness.

Executable surrogate: unsupported. Output is a categorical table; no percentage metric.

## invariant

### Invariant and cost

Limits: SPECIALIST is biomedical-heavy, so most rows are domain words (drug, gene, anatomy names); a few junk rows may remain (SPECIALIST's *unpossible* is kept: its unit is not in the dictionary); no frequency, so rare readings weigh like common ones in the hint; WordNet-only verbs with irregular forms get their base only; proper nouns are decided by spelling, not meaning; Mova's own trees are not a source yet (the released corpus is CC BY-SA and its trees were drafted by an EWT-trained model).

## composition

### Composition

Called by `main lexgrow`; reads `dict` (the compiled dictionary of the running binary, never the extension), `morph::Morph::inflect` for regular forms, `derive::class` for coarse classes. Its files are compiled by `build.rs::extension` and read by `lexext`. `acronym_ok` is shared with `lexext::hint`.

## models

## behavior_complement

Never written: Wiktionary or Wikipedia material (no such input exists); multi-word entries; forms with digits or other symbols; rows for closed classes (prepositions, determiners, pronouns, modals, auxiliaries); a second spelling or inflection of a unit the dictionary has (occured, noticable, occurences), except a noun plural of a dictionary lemma (vertices); AGID dontuse forms; adjectives/adverbs/verbs spelt with two capitals; comparison of an adjective SPECIALIST lists as non-comparable (lexical) or that only WordNet has without listed forms; surnames under 1,000 bearers; names that are common words in any source.

## reverse_reconstruction

From a row the source record is recovered by its source key (the SPECIALIST EUI points to the LRAGR/LEXICON unit; `o` to the WordNet entry of that lemma and class; `c`/`k` to the name list), but not the full unit: other forms of a covered unit, multi-word variants and the dropped rows are not recoverable from the output.

## source_selection

A SPECIALIST row passes iff single-token ∧ open class ∧ ¬(non-noun ∧ ≥ 2 capitals) ∧ (EUI not covered ∨ (NNS ∧ dictionary has the lemma as NN)) ∧ put's conditions ∧ (letters > 3 ∨ proper noun ∨ WordNet also has the row).

## provenance

Each row names its sources: `o`, `s:<EUI>` (one EUI kept per row), `c`, `k`; acronym rows `s:<EUI>` or `o:<synset>`. The TSV header carries the attribution of every source and the Names Corpus README.

## tests

### Evidence

Checks: lexgrow::tests::acronyms_and_words; lexext::tests::grown_words (on the generated files).

cargo test --release -p en: passed. Negative controls: acronym_ok rejects Smith, gpu, A, Ab; word_ok rejects a, x2, -ish and two words; grown_words checks that occured, noticable, arguement, happyness are not generated. The generator was run twice into the repository with byte-identical output. No teacher, API, GPU or training.

## History

### 2026-11-04 · descriptant

2026-11-04 modeling epoch (lexicon growth): new source. Observations from building it: SPECIALIST lists undoubled verb forms beside doubled ones (*occured*), variants of dictionary words (*noticable* as citation form), clipped forms and abbreviations as adjectives (*OA, sed, ves*) and gene names that collide with phonics chunks (*Sh*); each became a rule below, found by measuring EWT/GUM dev and the school course english before and after. A registered leaf is partial coverage, not a proof.

### 2026-11-04 · invariant

Every written row: word_ok form and lemma (letters, inner hyphens and apostrophes, ≥ 2 letters, ≤ 40 characters), a tag of an open class or NNP/NNPS/NNS, at least one source, and no (form, tag) the dictionary has. Output sorted by (form, tag index, lemma); acronyms by acronym then insertion order. Deterministic: BTreeMap/BTreeSet throughout, files read in sorted order; two runs gave byte-identical files (2026-11-04).

Cost: one pass over LRAGR (~1.9 M lines, 121 MB) held in memory as single-token rows, one over LEXICON (56 MB), the OEWN JSON (~30 files) and the name lists; about 3 s on 2026-11-04 (memory not measured).

### 2026-11-04 · tests

Heading `### Evidence — 2026-11-04` stays in tests as `### Evidence`.
