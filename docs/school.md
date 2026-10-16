# The Mova School: our own annotation for all languages (principles, draft)

The goal (2026-09-25): over time, where possible, Mova moves to its own more global, abstract annotation across all languages. That needs our own school for the small model, and this page sets out its principles.

These principles grew out of a review of 1326 papers (the UD survey). Each existing school does something better than the others, and none does everything. The Mova School takes the best from each, and treats UD, the Prague scheme, and others as translations, not as the source.

## Three floors of one graph

| floor | what it describes | taken from | what it is for |
|---|---|---|---|
| 1. **Structure** | words, forms, the dependency tree (like a school sentence diagram) | UD, SUD | annotation, standard, exchange |
| 2. **Event** | who did what to whom: predicate, participants with roles, recovered gaps | Prague t-layer, Paninian, PropBank | facts for RAG, compression |
| 3. **Word usage** | which words go together: collocations, lexical functions | Meaning ⇔ Text | natural text generation |

The floors are layers of one graph with explicit links downward, to tokens, as in PDT. No floor loses what is below it.

## Principles

1. **A node is a content word**; its lemma is a number in the dictionary compiled into the code. Function words and endings become node attributes: tense, aspect, number, modality, negation, question. Each attribute is marked with how it is expressed in the text. This is how the Prague t-layer and MSP do it.
2. **Role separate from form** (a lesson from the Paninian school). A label has two axes:
   - **function**: who, to whom, what, where;
   - **strategy**: case, preposition, word order, verb form.

   These are two separate enums, as in Nivre–Croft. The agent, the experiencer, and the causer are not merged into one "subject".
3. **Valency dictionary.** Each use of a predicate refers to a frame in the dictionary: who is obligatory, who may be omitted, which forms express them.
   - Actants are numbered (I, II, III), as in Meaning ⇔ Text.
   - Roles are two-tier, `MAIN:sub`: a coarse source gives the top class, a fine one keeps the subclass.
4. **Omitted elements are recovered.** A dropped subject, ellipsis, and control ("wants to go": who will go) get explicit nodes linked to their "twin".
5. **Coordination is a separate group node**, not a chain. Shared and individual dependents of group members are distinguished explicitly, because this is exactly where all conversions break.
6. **Units at three levels:** token → syntactic word → morph, with correspondence tables. The graph fixes segmentation, and token boundaries for each language are decided by the translation.
7. **Everything is numbers and enums**, like all of Mova. Strings appear only at input and output.

## Translations (projections)

- UD, SUD, Enhanced UD, Prague annotation, PropBank are projections of the graph via rule tables in Rust.
- Each projection has a round-trip gate: translation → back → the same. A construction that does not translate produces an explicit failure, not a silent `dep`.
- Conversion is known to lose what the source lacks: entity type, coreference, shared dependents in coordination, argument vs. adjunct, ellipsis. So the graph holds these itself.

## How we verify

- **Completeness:** text → graph → text (our compressor). Whatever is not reproduced, the graph did not keep.
- **Compatibility:**
  - UD → graph → UD on all English treebanks;
  - the English part of MSP (from EWT) as a ready-made test for the second floor;
  - Enhanced UD via ELAS on EWT gold.
- **Negative control:** a deliberately broken projection rule must turn red.

## Order

1. **First floor in English: done.** Full UD annotation, `en ud-eval`.
2. **Second floor in English: next, and this is also RAG:** "who did what to whom" facts from the graph. That means Enhanced UD by rules, roles from VerbNet frames and WordNet's derivational links (PropBank/NomBank only for tests: share-alike / research-only, licence/DATA.md), negation and sentence type as attributes.
3. **Third floor**: lexical functions for the generator: collocations from the corpus and dictionaries.
4. **Other languages**: later, one at a time. Ukrainian first: Mova already has its dictionary.
