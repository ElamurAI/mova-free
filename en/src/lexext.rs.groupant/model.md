# en-lexext

## descriptant

### en-lexext

The lexicon extension at run time: a second compiled blob (`lexext.bin`, made by `build.rs` from `data/lexicon-ext.tsv` and `data/acronyms.tsv`) with forms → (tag, source bits, lemma), lower-case lemmas → paradigm (tag, form) and acronyms → expansions. It answers only for what the dictionary (`dict`) does not know; the dictionary's blob, fingerprint and lemma numbers are untouched, so every trained model loads as before. `EN_LEXEXT=0` turns it off.

## approximant

### Forward approximant

probe(s) is open-addressing lookup with `hash::str_key` over a power-of-two table (slot = id+1, 0 empty), the same scheme as `dict`; it returns nothing when the extension is off. analyses(lower) = the records of the form, or, for a form ending in -s whose stem has an NNP record with an acronym lemma in capitals, one synthetic (NNPS, src, lemma) record (*gpus* → GPU). known = analyses nonempty. tags = sorted distinct tags. lemma(lower, tag) = the first record's lemma with that tag. form(lemma, tag) = the paradigm record of the lower-cased lemma with that tag. expansions(acronym) = the case-sensitive acronym's (expansion, source) pairs in file order. hint(word, inside_cap) filters records by how the word is written (case_gate) and returns their sorted distinct tags.

Domain/state: the compiled blob (immutable), EN_LEXEXT read once (OnceLock).

Executable surrogate: unsupported. Outputs are categorical (tags, strings); no percentage metric. No automatic substitution and no claimed 1/2/5/10 percent profile.

## invariant

### Invariant and cost

For every row of lexicon-ext.tsv, analyses(form) contains (tag, src bits of its sources, lemma or the form for `=`), and form(lemma, tag) returns some form of that lemma with that tag. Lookups never return a record for a key absent from the files except the acronym plural. With EN_LEXEXT=0 every function returns empty, so callers behave byte for byte as before the extension (measured: annotation of EWT and GUM dev, lexicon-cover on three corpora). Deterministic: build order is file order (sorted by the generator).

Cost: one probe O(1) expected (load factor ≤ 1/2), records O(k) per form; the acronym plural adds one probe. Blob ~ 20 MB in the binary; nothing read from disk.

Limits: case is not stored for forms (keys are lower case), so case-dependent decisions are the hint's; the lemma of a lower-case reading of a proper noun is lower-cased by the lemmatizer; no frequencies, so records of one form are unordered evidence, not a ranking.

## composition

### Composition

Consumers: `morph::known_form` (known words), `Morph::lemmatize_any_case` (a lemma for a form the dictionary lacks, and extension lemmas as candidates of the suffix rules), `Morph::inflect` (a form for a lemma the dictionary does not inflect), `derive::go` (extension words as bases below a form, extension classes added to a dictionary word's: *directional* JJ), `derive::analyses` (no derivation for an extension word), `ptag::dict_mask` and `tag::Tagger` (the hint for an unknown word), `main derive` (rows, sources, expansions). Producer: `build.rs::extension` from the files `lexgrow` writes.

## models

## behavior_complement

No record for: a form the dictionary knows with that tag (the generator leaves it out); a key in another case for acronyms (expansions("ai") is empty); a plural -s on a stem that is not an acronym in capitals (*lexicals*); any form when EN_LEXEXT=0. hint gives nothing for a capitalised word inside a sentence without a proper-noun record (*Stochastic* in a title, *Safed*), for an acronym's or two-capital name's record unless the word has two capitals (*amd*, *sry*), and for a name-list-only record or a name of three letters or fewer unless the word is capitalised (*goin*, *sh*).

## reverse_reconstruction

From a record (tag, lemma) the forms are recovered by form(lemma, tag) only up to the paradigm: several forms may share a cell (vertexes, vertices) and form returns the first; from a tag alone nothing is recovered. From hint's tags the written case is not recoverable (GPU and Gpu with two capitals give the same set).

## case_gate

hint(word, inside_cap): keep a record unless (it is an acronym record — lemma acronym_ok, or NNP/NNPS with ≥ 2 capitals — and the word has < 2 capitals) or (its sources are only Census/Names Corpus, or it is NNP/NNPS of ≤ 3 letters, and the word is not capitalised); if inside_cap and no kept record is NNP/NNPS, keep nothing.

## blob_layout

Sections (layout constants from build.rs): XF strings/offsets/index (forms), XL strings/offsets (lemmas, no index), XP strings/offsets/index (lower-case lemmas), XA strings/offsets/index (acronyms), XE strings/offsets (expansion\tsource), then group offsets and 8-byte records (tag, src, 0, 0, id) for XF, XP, XA. Source bits: OEWN 1, SPECIALIST 2, Census 4, Mova 8 (reserved), Names Corpus 16.

## tests

### Evidence

Checks: lexext::tests::grown_words; lexext::tests::acronyms.

cargo test --release -p en: all en tests passed, twice with identical outcomes. Negative controls inside the tests: misspellings and nonsense (happyness, noticable, occured, arguement, qwzxv) are not known; no comparative where the source gives none (lexicaler); lower-case acronym lookups are empty; lexicals is no acronym plural; hint gives nothing for gpu, aaronson, Stochastic inside a sentence, sh. Measurements (en/README.md): two runs identical; EN_LEXEXT=0 identical to the build before the extension. No teacher, API, GPU, training or held-out test set was used.

## History

### 2026-11-04 · descriptant

2026-11-04 modeling epoch (lexicon growth): new source; tests written with the code, each with negative controls; measurements in en/README.md («Measurements of the lexicon extension»). Prospective prediction made before measuring: unknown words on the Owls' papers fall by about a third, UD dev scores move by less than 0.1 point either way. Measured: 9.24% → 5.54% (a 40% cut), EWT dev LAS −0.02, GUM dev LAS ±0, XPOS +0.01/+0.03. A registered leaf is partial coverage, not a proof.

### 2026-11-04 · tests

Heading `### Evidence — 2026-11-04` stays in tests as `### Evidence`.
