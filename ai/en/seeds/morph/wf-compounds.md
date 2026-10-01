# Compounds: blackbird, gas-mask, tear gas

**Gist.** A compound is a combination of two stems that works as one word and has a distinct meaning: *blackbird* (a particular species), not *black bird* (any black bird). English has three ways of writing them: solid (*blackbird, toothbrush*), hyphenated (*gas-mask, well-known*) and open (*tear gas, bus stop*); Jespersen calls the spelling "almost chaotic". A more reliable criterion is stress (a compound usually has one strong stress on the first part) and semantic isolation.

**Conditions and exceptions.**
- Types: noun + noun (*sunrise, bookseller* — the most numerous); adjective + noun (*blackboard, greenhouse*); verb + noun (*pickpocket, breakwater*); particle + noun (*outcry, bystander*); bahuvrihi — naming the bearer of a feature (*redcoat, paleface*); compound adjectives (*blue-eyed, heart-breaking, God-given, man-made, colour-blind, light-green*); compound verbs, often via back-formation (*housekeep, henpeck, browbeat*).
- The first part is usually singular even with a plural meaning: *toothbrush, cigar-box, a five-pound note*; the plural stays when it has a distinct meaning (*clothes-brush, newspaper, goods train*) and in official names (*the Aliens Act*).
- The head of the compound takes the plural (see `noun-plural-compounds`).
- Hyphenated compounds in EWT are usually one token (*well-known, long-term*); Penn Treebank tags a hyphenated prenominal modifier as JJ (*income-tax/JJ return*), and parts written separately get separate tags (*income/NN tax/NN return*).
- Open compound nouns (*tear gas, bus stop*) in UD are two words: the first is NOUN with relation `compound` to the second.

**Examples.** *a **blackbird**; a **toothbrush**; a **well-known** writer; the **bus** stop* (`compound`).

**In UD.** Solid or hyphenated — one token tagged by function (NN, JJ…); open — each word separately, the first is `compound`. The lemma of a solid compound is the whole word (*toothbrushes → toothbrush*).

**Sources.** Jespersen MEG VI 8.1₁–8.1₃ (text-5, pp. 150–153: criteria of a compound — inflection, stress, spelling "little short of chaotic", semantic isolation), 8.7–8.9 (pp. 166–170: bahuvrihi; first part in the singular; *first-class passenger*), 9.1–9.7 (pp. 173–182: adjective + noun, noun + participle, colour compounds, back-formed *housekeep*); Sweet NEG I §63–68 (text-1, pp. 54–56), §900–915 (pp. 320–323: types and stress), §1551–1556 (pp. 476–478: stress — the main criterion; first part in the singular); Whitney §102–105, §119, §194, §226 (text-1, pp. 61–62, 69–70, 103–104, 119); Santorini 1990, §4.1, p. 12 (hyphenated modifier — JJ); https://universaldependencies.org/en/dep/compound.html.
