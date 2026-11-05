# kg — the Mova knowledge graph

## Why

The graph is a tool, not a goal. The path from a request to an answer is:

```
text/prompt ─► deterministic parser (dictionaries + grammar) ─► commands for Rust code
            └─(not parsed)─► AI rewrites it into a structured request (simple sentences) ─► parser
commands ─► descriptive and result graphs in memory ─► deterministic text generator (Rust,
          no neural nets: Markov, "not-too-slow algorithms") ─► text for the user
                                                   └─(or)─► graph into an AI prompt ─► polished text
```

A domain parser and generator need dictionaries, grammars and research papers (source dossiers).
Requirements for the graph: **density and speed** (SIMD, matrices, quantization, compression on SSD); flags so that
a task can cut out a subgraph instead of walking the whole graph, and quality can be evaluated on compressed subgraphs.

## The unit of knowledge is a simple sentence

One grammatical core (predicate + subject) yields one **fact**: a `fact` vertex and edges from it to the sentence members:
- `predicate`, `subject`;
- `object:<case>`, `adverbial`, `adverbial:<preposition>+<case>`;
- `quantity:<word>`, `predicate:nominal`, `predicate:infinitive`, `particle`;
- `attribute:<relation of the modified word>`.

The service edge `from_sentence` points to the source sentence.

A fact is a vertex rather than a single edge. This way sentences without an object and sentences with adverbials fit without loss. The roles are syntactic, i.e. derived from case, agreement and government. Semantic roles come as the next layer.

Relation labels are English; the case in `object:<case>` and `adverbial:<preposition>+<case>` is the VESUM tag (`v_zna`, `v_mis`, …), and the preposition and quantity word are the Ukrainian words themselves (needed to restore the sentence).

## Identifiers

`u64` = heap (high 32 bits) | number within the heap (low 32).

| Heap range | Purpose |
|---|---|
| `[1, 2^26)` | service: `rel` (relations) … |
| `[2^26, 2^27)` | local |
| `[2^27, 2^32)` | global: `word:uk` (`lemma\|part_of_speech`), `sentence:tatoeba`, `fact:tatoeba:uk` |

Numbers are consecutive from 1. Key → number lives in `keys/<heap>.tsv` and is append-only, so ids are stable across runs.

## Edge and grammar

An edge has the fields `src u64`, `dst u64`, `rel u32`, `gs u32`, `gd u32`, `flags u32`.

`gs` and `gd` are the compact grammar of the edge's ends in 4 bytes (`src/gram.rs`):

| Field | Bits |
|---|---|
| part of speech | 5 |
| case | 3 |
| number | 2 |
| gender | 2 |
| person | 2 |
| tense/form | 3 |
| aspect | 2 |
| animacy | 2 |
| degree | 2 |
| flags: pronoun, proper name, negation, implicit member | 1 each |

`gs` is the grammar of the fact (the predicate), `gd` is the grammar of the member. The word form is not stored: it is restored from the lemma and `gd` via VESUM.

Edge flags (`src/cols.rs`, `flag::*`):

| Group | Values |
|---|---|
| source | tatoeba, manual |
| language | uk, en |
| layer | fact, prov, translation |
| check | checked |

A **subgraph** is a mask filter over the flags column: `kg sub <store> uk,fact,checked <out>`.

## Storage

- Columns (structure of arrays), sorted by `(src, rel, dst)`. This is CSR: a vertex's adjacency is one contiguous slice, filters vectorize.
- On disk:
  - `src` — LEB128 deltas;
  - `dst` — zigzag LEB128 deltas;
  - u32 columns — byte planes;
  - everything compressed with zstd-19.
- The store is `$MOVA_DATA/kg/`: `heaps.tsv`, `keys/`, `edges/`, `sentences.tsv`, reports.

## Tatoeba uk

```
kg tatoeba-uk $MOVA_DATA/raw/multi-tatoeba …/uk-vesum-dict-uk/dict_corp_vis.txt.bz2 $MOVA_DATA/kg
```

**Rejections:** commas, questions, conjunctions, an infinitive after the predicate, a dash with a verb — these are not yet simple sentences. Ambiguous cases are not guessed.

**Manual evaluation:** a development gold standard `examples/gold-v1.tsv` and independent samples `examples/heldout-v2.tsv`, each sample graded before any fixes based on it.
- Error causes the parser handles:
  - proper names instead of common words — names are taken from the corpus;
  - verb government — the object's case is taken from VESUM `rv_*` where VESUM has it;
  - pronouns after prepositions;
  - quantity phrases;
  - "це" as a particle;
  - attributes do not attach to personal pronouns.
- Not handled:
  - substantivized adjectives ("першим" as the noun "перше");
  - genitive of time ("минулої осені" is currently an object);
  - proverbs with inversion.

**Checks:**
- `render_check` checks only **completeness**: the graph together with the grammar reproduces every word of the sentence. It does not see role correctness, so it passes on every parsed sentence.
- A unit test proves the check can show red: a lost member, a wrong grammar, an extra "не".
- The real correctness gate is the manual samples and the **round trip text → graph → text** (section "Generator").

## Commands

```
kg tatoeba-uk <tatoeba-dir> <vesum.bz2> <store>
kg show <store> <tatoeba-id>...
kg stats <store>
kg sub <store> <flags> <out>
kg parse <vesum.bz2> <store> <in.tsv> <out.tsv>
```

## For domain projects

Mova is the language backbone for domain projects. They read the databases, knowledge and code and build their own domain versions.

The first application is **document expansion for RAG**: a deterministic step instead of expensive AI steps, with AI kept as the fallback path.

Rules:
- The formats in this file (identifiers, grammar, columns, flags) are a contract. Change them only with a migration.
- **Licenses:**
  - VESUM — **CC BY-NC-SA** (non-commercial only). Commercial projects need permission from BrUK, or morphology from Wiktionary (CC BY-SA) / Wikidata lexemes (CC0) / public-domain dictionaries.
  - Tatoeba — CC BY 2.0 FR.
  - Data derived from Wiktionary, when distributed — with attribution and share-alike.

## Generator (graph → sentence)

`src/textgen.rs`:
- chunk order is a Markov chain over classes, trained on 90% of parsed sentences;
- forms come from VESUM synthesis, the variant is chosen by the frequency of (lemma, grammar) → form.

Round-trip gate on the held-out 10% of facts: each regenerated sentence is classed as verbatim, the same words in a different order, or different words.

## Limits

- **No AI fallback path** for rejected sentences (commas, conjunctions): a teacher that rewrites them into simple ones is not built or measured.
- **Ukrainian only:** no `translation` edges from Tatoeba `links` and no English parser in `kg`.
- **Simple sentences only:** coordinated members, questions and complex sentences are rejected.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
