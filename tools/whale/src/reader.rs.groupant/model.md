# mova-whale-reader

## Descriptant

A Whale reads a paper through Mova (sections, sentences, UD parse, clause skeletons, unknown words) and writes its own plain-text description bound sentence by sentence to the source; coverage, traceability and verbatim-run measures. Source: `tools/whale/src/reader.rs`.

## Forward approximant

read(markdown): headings -> sections (references/bibliography/acknowledgements/appendix skipped); paragraphs -> world::quant::sentences; skip 4..80-word bounds, alpha share < 0.6, residue (table rows, proceedings, citation lists); Tree::parse + world::read::skeleton; unknown = !en::morph::known_form and not a cited author name; noun lemma counts. write(r, max): opening from counts and top nouns; per section header + skeleton sentences (deduplicated, 4..40 words), every section keeping the same share under the word budget, evenly spaced, whole sentences only. measure: section coverage, traceability (content words of a written sentence within its source sentence), longest/median verbatim run. gaps: unknown terms in >= 2 sentences (acronyms first), longest sentences without a skeleton.

## Invariant

Deterministic (BTreeMap/BTreeSet, fixed tie orders); every written content sentence names its source sentence; the description never exceeds max words and is never padded; no language model is involved.

## Cost

One parse per sentence (about a second per 8,000-word paper in release); writing and measuring linear in sentences times skeleton length; longest_run is O(|a||b|) per written sentence.

## Composition

Parent: whale-workers. Uses world::tree (annotator, Tree), world::read::skeleton, world::quant::sentences, en::morph::known_form, mova_octopus::realize; called by paper.rs.

## behavior_complement

Not done: frames/world facts beyond clause skeletons, articles and prepositions the skeleton drops, section detection beyond Markdown headings, figures, tables, formulas, citations as references, paraphrase, any claim not in the source. Counterexample: P17-1074 yields «We classify them according new based framework.» (dry run). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a description and reading.json one recovers, for every content sentence, the source sentence it came from and the section; the dropped words of each skeleton and the unselected skeletons are recoverable from reading.json, not from the description. Many papers give the same skeletons; no unique inverse. No unique inverse is claimed.

## History

### 2026-11-04 · behavior_complement

Not done: frames/world facts beyond clause skeletons, articles and prepositions the skeleton drops, section detection beyond Markdown headings, figures, tables, formulas, citations as references, paraphrase, any claim not in the source. Counterexample: P17-1074 yields «We classify them according new based framework.» (2026-11-04 dry run). Evidence status: source-derived model with finite checks, not a proof.

### 2026-11-04 · document

#### Epoch 2026-11-04 (learning Whales)

read_with(markdown, cap): every admitted sentence also gets whole clauses (clause.rs, verbatim cap) and its noun lemmas; a table or figure caption is restated as «Table N shows …»; a sentence whose whole parse yields no clause is tried by its comma/semicolon segments; splits after «et al.», «e.g.», «i.e.», «cf.», «Fig.», «Eq.», «Sec.», «vs.», «etc.» are joined; a standalone title-case line between prose paragraphs opens a part (PDF subsection titles); acronym definitions «Long Form (ACR)» are collected as the paper's own knowledge. The skeleton writer (write) stays for `paper`.

#### Epoch 2026-11-04 (writer improvements)

Appendices are read (system descriptions and analyses are part of the paper; their tables stay out as residue). Bibliography entries under a content heading are skipped (`reference_entry`: author list + year + venue/pages, a venue line, an arXiv line, at most 80 words): PDF extraction had put whole reference lists under «Conclusion» (paper 4: 162 of 374 sentences were references). A sentence ending in «?» is restated only as the paper's question. Every comma/semicolon/colon segment the whole parse left at least 80% unrestated is parsed alone (`uncovered_segments`, which replaces the whole-sentence-only fallback); a subordinate segment is attached to the nearest clause after it, else before it. Measured 2026-11-04 on the five pilot papers with the saved Whale knowledge (dry learn rounds, no terminal, two byte-identical runs): description words 3,259 / 4,931 / 4,076 / 3,294 / 3,816 (before 3,138 / 3,498 / 3,947 / 3,266 / 3,652); papers 2 and 3 pass every gate, 1, 4 and 5 fail only `words`; verbatim max 12, near duplicates 0, answer overlap 0, traceability 1.0 everywhere.

#### Epoch 2026-11-04 (writer-2026-11-04d: content filter, names, normalized terms, extraction)

`repair` rejoins PDF small-capital splits («S TACK P TR» -> «STACK PTR»: known words, splits that repeat in the text, runs; a lone «A»/«I» before an acronym stays). `furniture` filters page furniture, name lists, table rows, maths and bracketed citation lists before restating or asking; bibliography entries without a venue (`author_year`) and, after two of them in a part, its short continuations are not read; `filtered` counts each reason. `names` (author parts of entries, name lists, cited names) are never unknown terms; unknown terms are counted under `knowledge::norm_term`, `forms` keeps the paper's spelling. Clauses refused by `clause::grammatical` are dropped (`ungrammatical`). `extraction` measures spacing residue, unknown words (any case, names left out), truncated paragraphs and words before the first section heading; `ExtractionLimits` default 2/1000, 0.12, 0.3, 0.4 (calibrated 2026-11-04 on the ten batch papers and 400 library texts: only anantheswaran-2024 among the ten is refused, for lost headings). Measured dry rounds on copies of today's state: words fall 0-370 per paper (old descriptions held fragments and runs that passed the verbatim gate only because split letters broke them). WHALE_READ_DEBUG prints every filtered sentence and refused clause.
