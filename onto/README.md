# onto — the world model

A model of abstractions kept separate from the narrative: TBox versus ABox from `docs/context.md`, section "World model — separate from the narrative". The guiding idea: build the world model before the mathematics; it still has to be taught, much like in fairy tales, that an apple is a fruit.

- **World model:** concepts, their hierarchies, parts, typical actions and roles, default properties with exceptions. Shared by all texts.
- **Narrative** (`world`) — the individuals of a single text. Crow1 refers to the concept "crow" rather than copying it.

The core does not depend on `en`: lemmas and parts of speech come from OEWN itself. Only the `hs` module (goods classifier) takes parses from `en` and exact numbers from `math`.

## v1 in brief (26.09.2026)

- **Graph:** 133 213 concepts — 120 564 OEWN 2025+ synsets, 1 221 frames and 11 428 FrameNet 1.7 roles. 154 608 words, 212 659 senses, 13 390 lexical units. 138 125 typed edges.
- **Seeds:**
  - 66 properties with a source on every row: 61 on the real layer, 3 in genres, 1 in a collection, 1 in a tale;
  - 2 extra edges — the Nixon diamond;
  - 26 layers: the real world, 3 genres, 7 collections, 15 tales.
- **Frozen set**, 74 questions, recorded and hashed before the query code:
  - blind round — **70/74 (95%)**: 2 "unknown", 2 wrong;
  - second, NON-blind round — **71/74**: 2 "unknown", 1 wrong.
  - Both wrong answers come from OEWN sense order, not from the logic: the first sense of "dolphin" is the dolphinfish, the first sense of "oak" is the wood.
- **Negative controls:** 8 of 8 green on real data and red on an injected defect. A shuffled hierarchy drops IsA from 95% to 53%, i.e. to the level of the "always yes" baseline.
- **Bridge to the narrative world:**
  - 302 of 305 fable and tale classes found (99%);
  - 14 fable questions via inheritance: blind round — 12/14, second — 14/14.
- **Speed:**
  - build from XML — 1.7–3.1 s;
  - the 19.1 MB snapshot loads in 10–20 ms (median 15); together with seeds and gates — also ~13–15 ms;
  - the graph takes 20 MB of arrays;
  - a query with proof — ~0.01 ms.
- **Wikidata:**
  - lexemes — statistics and a trial alignment: 13 067 synsets got a Q; a manual sample of "by gloss" matches — 17/20;
  - full dump — measured on 1 GB: ~15 h on one core; plan below.

## How it works

### Model (`model.rs`)

- **A concept is a `u32`.** Struct of arrays: source (`Src`: OEWN synset, frame, role, Wikidata Q), key (synset offset, frame ID), part of speech (`Pos`), lexicographer file (index in `LEXFILES`), ILI, gloss, member lemmas.
- **A word is a `u32`.** Sorted dictionary, binary search. A word's senses are in OEWN frequency order, i.e. the order of `<Sense>` in the entry.
- **Relations — enum `Rel`**, direction as it reads:
  - from OEWN: `IsA`, `InstanceOf`, `PartOf` (part → whole), `MemberOf`, `MadeOf` (whole → substance), `Entails`, `Causes`;
  - from FrameNet: `HasRole`, `Inherits`, `Uses`, `SubframeOf`, `PerspectiveOn`, `Precedes`, `CausativeOf`, `InchoativeOf`.
- **Edges — CSR in both directions.** A lexical unit is a word with a part of speech → frame. Within a word, units with more annotated sentences come first.
- **Properties — enum `Prop`:** `Living Animate Human CanSpeak CanFly CanSwim CanWalk Edible LaysEggs Fast Exists Pacifist`; v2 — character for fables: `Cunning Vain Greedy Industrious Wise`. Strength is `Default` or `Strict`; the layer is a `LayerKind`: real world, genre, collection, tale.
- **Strings only at input and output:** XML, TSV, query, answer.

### Build (`build.rs`, `zip.rs`) and snapshot (`store.rs`)

- **OEWN** is taken from `english-wordnet-2025-plus.xml.gz`.
  - The base 2025 edition has no `instance_hypernym` at all: proper names and taxa were moved out. It has only 723 `mero_member`.
  - `-plus` is a superset: it contains all 107 519 base synsets, with 13 045 added. Such concepts are marked with a "plus only" bit.
- **FrameNet** is read directly from the zip by our own minimal reader (central directory + deflate): the `zip` crate is not available offline, and there is no need to unpack to disk. From the archive we take `frame/*.xml` — roles with `coreType`, lexical units and frame relations.
- **Build gates** (violation — refusal):
  - OEWN inverse pairs are consistent: `hypernym`/`hyponym`, `holo_*`/`mero_*`, `entails`/`is_entailed_by` and others — 0 mismatches;
  - the edge endpoint exists;
  - a frame relation points to an existing frame;
  - **an IsA cycle — refusal.**
- **Snapshot** — our own binary format, like `en/src/store.rs` but without serde: the graph already lies in flat arrays.
  - Header: magic `MOVA-ONT`, version, sizes of the enums in the code, length and checksum.
  - A corrupted byte, a truncated file, extra bytes or foreign enums give an error, not a silently different graph.

### Seeds (`seeds/`) and layers

- **`layers.tsv`** — the layer tree. Root `real`; genres `fable`, `fairy`, `detective`; collections `aesop`, `grimm`, `perrault`, `japanese`, `scottish`, `nights`, `holmes`; tales `tale:2451` etc. — following the `world` command files.
- **`defaults.tsv`** — property seeds: id, layer, lemma, synset, property, value, strength, source. Every row has a source:
  - `gloss` — the OEWN synset gloss says so directly: "flightless", "egg-laying", "swift";
  - `manual` — general knowledge, written down by hand;
  - a paper or text — by name: Reiter 1980; Reiter & Criscuolo 1981; Aesop, Perry 226; FairytaleQA `^s65–66`.
- **`relations.tsv`** — extra IsA/InstanceOf. In v1 only Nixon — a Quaker and a Republican.
- **Seed gates:**
  - unknown layer;
  - lemma not a member of the synset — catches a typo in the id;
  - duplicate id;
  - contradiction on the same concept and layer;
  - a descendant against a `Strict` ancestor on the same layer;
  - an IsA cycle through a new edge.

### Inheritance (`kb.rs`)

A query on layer L sees only the seeds of L and its ancestors. An exception applies on its own layer and below: the fable `CanSpeak` does not make real crows speakers.

1. **Layer:** the most specific layer where at least one visible seed concerns the concept or its ancestor decides. A fable overrides the real world only where it has said something of its own.
2. **Concept:** within that layer the most specific concept wins. An ancestor's seed is overridden by a descendant's seed, and the proof shows it.
3. **Nixon diamond:** two incomparable seeds with different values give **"undetermined"** with both proofs, not a random choice.
4. **Strength:** `Strict` is not overridden on its own layer. A lower layer may explicitly override anything — that is what layers are for.

### Word sense selection (`query.rs`)

Sense disambiguation in v1 is the first sense with a hook for context. The context comes from the query itself:
- `isa X Y`: the sense of X in the lexicographer file of Y's main sense plus the first sense of X; Y — any of its senses. "whale" with "fish" is the animal, not the first sense "a very large person".
- **Property — selectional preference.** Bodily abilities (`CanFly`, `CanSwim`, `CanWalk`, `LaysEggs`, `Fast`) look for an animal sense: the first sense of "ostrich" is a person who hides their head, and "emu" is a unit of measure. `Edible` looks for a food sense. Other properties take the first sense.
- The bridge to the world takes context from the object type: `type=animal` → `noun.animal`.
- An explicit sense is a synset id instead of a word: `onto can oewn-01913388-n walk`.

Every answer says which sense was taken, why, and at what frequency rank.

### Queries

| query | what | "no" or "unknown" |
|---|---|---|
| `isa X Y` | X is a kind or instance of Y; IsA/InstanceOf chain | "no" — with two chains to the common ancestor where they diverge |
| `can X V` · `prop X P` `[--layer L]` | property on a layer; chain to the seed, the defeated seed, the layer | "unknown" if no seed applies |
| `haspart X Y` · `partof Y X` · `memberof Y X` · `madeof X Y` | parts, including those inherited from ancestors, parts of parts | OEWN meronymy is incomplete, so we never say "no" — only "unknown" |
| `parts X` | direct parts, members and substances of the concept and its ancestors | |
| `frame V` · `role V R` · `inherits F G` | a verb's frames with Core and other roles, frame inheritance | "unknown" if the word is not among the lexical units |
| `entails X Y` · `causes X Y` | verb entailment and causation, with hypernyms | |
| `word W` · `up X` | a word's senses; ancestors | |

An unknown word gives "unknown", not a guess.

**Examples** (`$MOVA_DATA/runs/onto-2026-09-26/examples.txt`):

```
$ onto can crow speak --layer fable
yes
  sense: crow → oewn-01581669-n «crow» (1st of 6, noun.animal; first sense) — «black birds having a raucous call»
  inherited: «crow» —IsA→ «corvine bird» —IsA→ «oscine» —IsA→ «passerine» —IsA→ «bird» —IsA→ «vertebrate» —IsA→ «chordate» —IsA→ «animal»
  seed d62 [fable — genre, Default]: oewn-00015568-n «animal» CanSpeak=yes — source: fable genre: animals talk (Aesop; …)
  exception of layer fable (applies only on it and below)
$ onto can crow speak          → no: seed d10 [real] animal CanSpeak=no
$ onto can penguin fly         → no: seed d14 penguin CanFly=no (gloss: «flightless»); overrides d13 bird CanFly=yes and d26 object CanFly=no
$ onto isa whale fish          → no: whale (2nd sense, noun.animal) ⊂ cetacean ⊂ aquatic mammal ⊂ placental ⊂ mammal ⊂ vertebrate;
                                  fish ⊂ aquatic vertebrate ⊂ vertebrate; diverge under «vertebrate»
$ onto frame buy               → yes: Commerce_buy ← buy.v (LU 2966); Core: Buyer, Goods; other: Money, Seller, …; Inherits Getting
$ onto prop nixon pacifist     → undetermined: d60 Quaker Pacifist=yes ⇐ Nixon —InstanceOf[r01]→ Friend;
                                  d61 Republican Pacifist=no ⇐ Nixon —InstanceOf[r02]→ Republican
$ onto isa blorft animal       → unknown: the word «blorft» is not in the world model (OEWN 2025+) — not guessing
```

### Bridge to the narrative world (`bridge.rs`, read-only)

- **Input:** 18 `world` command files: `world/data/cmds-*.txt` and `$MOVA_DATA/runs/world-*/cmds-*.txt`. Content duplicates are read once — 16 remain.
- **Entity class → concept.** Order of attempts:
  1. as is;
  2. spelling variant;
  3. head word.
- **Sense** — from the lexicographer file of the entity type.
- **Questions in English** (`Can the crow fly?`, `Is the meat food?`, `Does the crow have a beak?`): the entity is taken from the tale's world, and the query runs on that tale's layer.
- The bridge does not modify `world/` files.

## Sources and licenses

| source | version, file | license | what we take |
|---|---|---|---|
| Open English WordNet | 2025+ (`english-wordnet-2025-plus.xml.gz`; the base `english-wordnet-2025.xml.gz` — only for the "plus" bit) | CC BY 4.0: `Lexicon` header in both files. Commercial use — yes, with attribution | synsets, glosses, lemmas and sense order, IsA, InstanceOf, parts, members, substances, entails, causes |
| Berkeley FrameNet | 1.7 (`framenet_v17.zip` from nltk_data) | CC BY 3.0 per the nltk_data index; no text found on the ICSI site; FrameNet 1.5 in nltk_data is NC. **Check before commercial use** (`raw/en-framenet-1-7/SOURCES.md`) | frames, roles, lexical units, frame relations |
| Wikidata lexemes | dump 2026-09-23 (`wikidata-20260923-lexemes.json.bz2`) | CC0 | P5137 "item for this sense", P8814 "WordNet 3.1 synset", English glosses — trial alignment |
| Full Wikidata dump | 2026-09-15, (`$MOVA_DATA/raw/wikidata-all/latest-all.json.bz2`) | CC0 | full pass on 26.09 → `$MOVA_DATA/db/wikidata.duckdb` (section "Wikidata: pass and database"); the geography project's folder was only read |
| Seeds `seeds/` | our own, 26.09 | ours (closed part) | OEWN glosses are quoted with the attribution above |
| FairytaleQA, "The Jelly Fish and the Monkey" | texts — Project Gutenberg | public domain | one quote in seed d66 |

The papers Reiter 1980 ("A logic for default reasoning") and Reiter & Criscuolo 1981 (Nixon diamond) are only sources of ideas, not of data.

## Numbers

### Frozen set (`data/frozen.tsv`)

- **74 questions:** IsA with traps, properties with exceptions, layers, parts, frames and roles, verbs.
- **Frozen at 15:43Z before the query code:** sha256 `1affbff3…`, commit `1072558b`. The hash is checked before every run.
- **The gold standard is common-sense truth**, not what the data says.

| group | questions | blind round | second round (NON-blind) |
|---|---:|---:|---:|
| isa | 19 | 17 | 18 |
| prop | 23 | 23 | 23 |
| layer | 9 | 9 | 9 |
| part | 9 | 7 (+2 "unknown") | 7 (+2 "unknown") |
| frame | 11 | 11 | 11 |
| verb | 3 | 3 | 3 |
| **total** | **74** | **70 (95%)**, 2 "unknown", 2 wrong | **71 (96%)**, 2 "unknown", 1 wrong |

**Wrong answers:**
- `isa dolphin fish` → "yes". The first sense of "dolphin" in OEWN is dolphinfish (mahi-mahi), which really is a fish. The question meant the mammal.
- `isa oak tree` → "no", only in the blind round. The first sense of "oak" is the wood, and OEWN puts it in `noun.plant`.
  - The second round takes any sense of X in the category's lexicographer file, not only the first. The rule is general, but it was introduced after the blind run.

**"Unknown":**
- `haspart crow feather`: in OEWN 2025 "feather" is a new synset 89570581-n, not linked as a part of a bird. "plume" is recorded as the bird's part.
- `haspart fish wing`: the gold answer is "no", but we never say "no" for parts.

Reports: `$MOVA_DATA/runs/onto-2026-09-26/frozen-blind.md` and `frozen-second.md` — every question with its proof.

### Negative controls (`onto controls`)

| control | on real data | on an injected defect |
|---|---|---|
| IsA cycle | the graph is acyclic | edge animal IsA crow: the seed gate refuses, and bypassing the gate the full check finds a cycle of 9 concepts |
| Strict | all 66 seeds passed the gate; the same exception on layer fable — accepted | penguin LaysEggs=no against strict d42 on the same layer — refused |
| lemma ∈ synset | all 66 seeds and 2 edges passed; bird + synset bird — accepted | crow + synset bird — refused |
| layer exception does not leak | 192 "word × property" pairs on `real` are identical with all layers and with the real one only; crow CanSpeak: real — no, fable — yes | the "all layers visible" defect gives 14 mismatches |
| unknown word | 5 made-up words × 8 query kinds — all "unknown" | the same check on "crow" goes red |
| shuffled hierarchy | IsA 95% | IsA edge endpoints permuted: IsA 53% — the "always yes" baseline level (53%); properties 100% → 39%; parts 78% → 67% |
| set hash | matches | one changed letter — mismatch |
| snapshot | loads | 6 corrupted bytes or a truncated file — error |

### Bridge to the narrative world

- **Entities:** 587, of which 434 with a class, 153 with a type only.
- **Classes:** 305 distinct, 302 found (99%):
  - as is — 278 (91%);
  - spelling variant — 11;
  - by head word — 13: `demon_chief` → chief, `love_potion` → potion etc.;
  - not found — 3: bonze, children (irregular plural), vizir (spelling variant of vizier).
- **Type context** gave a sense in its own lexfile for 217 of the 243 classes that have context.
- **Fable questions:** 14 (`data/fable-questions.tsv`, hashed before the bridge code).
  - Blind round — **12/14**. "Is the meat food?" was parsed as the property "food" — a parsing defect. "Can the jellyfish walk?": the first animal sense of "jellyfish" is the Portuguese man-of-war, while the tale's seed sits on the medusa.
  - Second, NON-blind round — **14/14**. Fixes:
    - a word without an article — IsA, unless it is a property;
    - "one sense per text": if a seed of this tale's layers sits on another sense of the same word in the same lexfile, take that one, and the proof says so.

### Speed (`onto bench`)

- **Build:** 1.7–3.1 s — OEWN 0.8–1.5 s, FrameNet 0.2–0.5 s. Full cycle check — 0.01–0.03 s.
- **Snapshot** 19.1 MB, load 15 ms (median of 5; 10–20 ms). Together with seeds and gates — ~13–15 ms: there are few seeds, and the gates are cheap.
- **Memory:** 20 MB of graph arrays.
- **Query with proof:** ~0.01 ms. The whole set of 74 questions — ~1 ms.

## Wikidata

### Lexemes (CC0, `onto wd-lexemes`, 213 s on one core)

- **All languages:** 1 675 319 lexemes, 934 793 senses. With P5137 — 334 733, with P8814 (WordNet 3.1 synset) — only 5.
- **English:** 75 278 lexemes, 60 293 senses. With P5137 — 33 387 (30 231 distinct Q).

### Trial alignment of lexemes with OEWN (`onto wd-map`)

English senses with P5137 → OEWN synsets. P8814 gives 1; other methods:

| method | senses |
|---|---:|
| unambiguous (one sense on both sides) | 10 097 (30%) |
| by overlap of gloss words (≥ 2 and ahead of the runner-up) | 4 033 (12%) |
| ambiguous — no guess | 11 886 (36%) |
| lemma not in OEWN | 6 434 (19%) |
| different part of speech | 936 (3%) |

- **13 067 synsets got a Q.**
- **Manual check of 20 "by gloss" matches** (every N-th): **17/20**. Errors occur when the needed sense is missing from OEWN or is different: graph as a mathematical structure, a maritime fairway, etymology as an account rather than the science.
- **Keyword traps:**
  - apple Q89 ("a fruit of a tree of the genus Malus") → apple tree: the words "tree" and "genus" pulled it over;
  - tortoise → a lexeme for a Han dynasty sculpture: "unambiguous" trusts a lexeme's single sense.
- bird Q5113, cheese Q10943, meat Q10990, penguin, mouse, hare, genie Q3465 — correct.

### Full dump: measurement and plan

**Measurement** (`onto wd-probe`, `nice`, 1 core, read-only, nothing written to the geography project's folder):
- first 1.07 GB (1.04% of the file) → 11.1 GB of JSON (×10.4);
- 697 311 entities, with P31 — 660 538, with P279 — 33 595 (4.8%), with an English label — 615 835;
- **570 s, of which bzip2 — 548 s (96%)**, JSON parsing — 22 s;
- 1.88 MB/s compressed → **a full pass on one core ≈ 15 h**;
- on 17.09 the geography project, with `lbzip2 -n 12`, went through 121.7 M entities in 1 h 41 min.

**Plan:**
1. **One pass.** `lbzip2 -dc -n K latest-all.json.bz2 | onto wd-classes` (parsing already exists in `wd-probe`) writes into its own folder `$MOVA_DATA/raw/wikidata-derived/`:
   - P31/P279 edges in binary (~1.3 GB);
   - English labels of classes (P279 — ~5–6 M) and of entities whose label matches an OEWN lemma.
   - The bottleneck is bzip2. Time: K=4 — ~4 h, K=12 — ~1.7 h, K=1 — ~15 h. The choice of K is the operator's: an expensive run requires an explicit assignment.
2. **P279 closure in Rust** — minutes.
   - P279 cycles in Wikidata are known: the gate shows them and breaks them with a record, not silently.
3. **Alignment with OEWN.** Via lexemes — 13 067 synsets already exist — and labels. Accept a "synset ↔ Q" pair only when their ancestors are also consistently aligned (cross-checking hierarchies). Then IsA from Wikidata becomes a second source with its own provenance.
4. **Measurement:** the same frozen set and traps plus a manual sample of alignments.

## Wikidata: pass and database (26.09)

Requirements: Wikidata must be done, with the result loaded into DuckDB; it also has translations — we know how an article, or an apple, is rendered in different languages. One streaming pass over the full dump → derived TSV (zstd) `$MOVA_DATA/raw/wikidata-derived/` → a separate database `$MOVA_DATA/db/wikidata.duckdb` (the main `mova.duckdb` — via `ATTACH … (READ_ONLY)`). Runs and reports — `$MOVA_DATA/runs/wikidata-2026-09-26/`.

**How:** dump copier (counts compressed bytes) → `lbzip2 -dc -n 8` → 8 MiB chunks → 2 parser threads (`Cow`, labels first as `RawValue`) → each writes its own `<table>-w<N>.tsv.zst`; 10 threads, `nice -n 10`, a progress log every minute. The bottleneck is bzip2. The budget was estimated with the `wd-sample` sample: bzip2 blocks from an arbitrary bit offset, 48 chunks spread evenly over the file (the first GB of the dump are the most popular items, so a pilot on them overestimates).

**Pass:** 2 h 05 min, 13.8 MB/s compressed, 1.84 TB of JSON. **121 710 188 entities** (121 696 284 items, 13 904 properties), 0 unparsed, `[` and `]` in place — exactly as many as the geography project counted on 17.09 (the `wd-probe` measurement gave ~67 M — a poor extrapolation from the heavy start of the file).

| table | what | rows |
|---|---|---:|
| `items` | Q, labels en/mul/uk, descriptions en/uk, sitelinks, Wikipedias, label languages, `is_class` (P279 as subject or value — 4 482 022), `is_taxon` (4 006 029) | 121 696 284 |
| `labels` | labels and aliases in all languages — for classes, items with a sitelink, and taxa (31.2 M items), 637 languages | 421 364 939 |
| `sitelinks` | article titles of 364 language Wikipedias (without Commons, Species, Wikisource, Abstract Wikipedia; disambiguators in parentheses kept) | 67 553 750 |
| `claims(s, p, o, rank)` | P31 128.4 M, P279 5.24 M, P171 4.0 M, P361 5.46 M, P527 2.80 M, P186 2.35 M, P1034 2 267, P4733 95, P1582 3 886 and added P105, P1672, P2670, P3113, P462, P1419, P2974, P366, P1552, P3094 (why — `PROPS` in `src/wdx.rs`) | 153 165 153 |
| `strs`, `mono`, `quant` | P225 4.0 M, P2888 564 614, P1709, P5063 ILI, P8814; P1843 with language 817 895; mass 1.72 M, length 397 578, height 1.44 M, speed, lifespan — with unit and qualifiers | 4.6 / 0.8 / 3.6 M |
| `props` | property names en/uk, type | 13 904 |
| `isa_edges`, `isa_cycles`, `closure` | P279 + P171 without cycle edges; closure — for nodes with a Wikipedia article | 9.2 M / 14 / 271.7 M |
| `lexemes`, `lemmas`, `forms`, `glosses` | 1 675 319 lexemes, forms with grammatical features (18.2 M), 934 793 senses with glosses | |
| `sense_items`, `sense_links`, `sense_strs`, `lex_claims` | P5137 → Q 339 127; P5972 translation 136 123, P5973 synonym 67 738, P5974 antonym 3 484; P8814 — 5 | |
| `synset_q` | OEWN synset ↔ Q with status and support | 912 382 |

Macros: `wd_label(q, lang)` (lang → mul → en), `wd_ancestors(q)`, `wd_descendants(q)`, `wd_isa(x, y)`.

**Decisions and why:**
- **Wiki service pages** (categories, templates, disambiguations, modules, portals — 8.3 M) — without labels in all languages and without sitelinks: they are about the wiki, not the world; in the sample they are 15% of labels and 36% of article titles. The en label stays in `items`.
- **Scholarly articles** (46 M) — in the database without titles and descriptions: the world model and translation don't need them, and with them the database is 16.7 GB (budget 15). The texts remain in the derived files.
- **Label languages were not restricted:** with these two decisions all 637 fit.
- **Closure** — 613.5 M pairs over 8.46 M nodes (72.5 on average, max depth 43) — exceeds the 400 M limit, so only nodes with a Wikipedia article are materialized (271.7 M); the rest — via the recursive `wd_ancestors`.

**Sizes:** derived files 6.6 GB (budget 10), database 13.3 GB (budget 15). Loading — ~3 min; large tables are not sorted (a full sort of 121 M wide rows — Out of Memory at 20 GB).

**Check queries** (`sql/wikidata-checks.sql`):
- apple Q89 ⊂ fruit (Q3314483, step 1; Q1364, step 2) ⊂ food; uk label «yabluko» (aliases «yabluka», «yabko»); articles: Apple / Yabluko / Tafelapfel / Pomme / リンゴ.
- whale Q1865281 ⊂ Mammalia — yes; but ⊂ fish — also yes: cladistics, Cetacea —P171 ×24→ Osteichthyes (Q27207) —P279→ fish. For the everyday "is a whale a fish", P171 must stop at the class rank — the next step.
- Corvus corone (Q26198) — a taxon, 42 common names in 35 languages (uk «vorona chorna»).
- P4733 "sound" — only 95 statements on 84 items: for the core, sounds are hand-written seeds.

**P279/P171 cycles:** 7 components, 14 nodes (cosmology ⊂ astrophysics ⊂ … ⊂ cosmology; corruption ⇄ abuse of power; two P171 loops). The gate lists them; the edges are removed from the closure and recorded; after breaking — a DAG. Negative control: edge fruit ⊂ apple — the gate goes red, a cycle of 7 nodes.

**Synset ↔ Q** (`wd-align`): 912 382 candidates from lexemes (P5137), P5063 ILI, P8814 and English labels; **53 392 synsets** accepted (62.9% of nouns), 46 405 distinct Q. Silver — direct P5063/P8814 versus choosing from lexemes and labels only: **77.9%** (21 231 of 27 239). Shuffled labels: 7 965 accepted — a 6.7× drop, while the pre-set threshold was 10×: support via general ancestors (up to 5% of nouns) lets random pairs through. Next — a 1–2% threshold and two supports. The manual sample of 50 pairs has not been checked yet: it was postponed because of the limit — file `align-sample.tsv`.

**How to update:** new dump → `wd-extract` (~2 h) → `wd-lex` (48 s) → `wd-senses` + `wd-map` → `wd-closure` → `wd-align` → `sql/wikidata-load.sql` and `sql/wikidata-closure.sql` → `sql/wikidata-checks.sql`. Commands — section "Commands".

**License:** Wikidata — CC0 1.0, both the full dump and lexemes: commercial use without restrictions, attribution not required, but the `wikidata` row is recorded in `en/data/train-licenses.tsv`. The dump lives in the geography project — only read, nothing written to its folder.

## v2 (26.09): Wikidata walker

The task: a small walker that goes through Wikidata. Clarifications: sounds were a joke — sounds (P4733) are not collected; no need for a specific lion in the abstraction, let it come from a normal distribution — numbers as distributions. Runs — `$MOVA_DATA/runs/onto2-2026-09-26/`; database and derived files — read-only.

### A whale is not a fish (`src/wdwalk.rs`)

Wikidata cladistics leads the whale into fish: whale —P279→ Cetacea —P171×16→ mammal [class] —P171×22→ Sarcopterygii [class] → Osteichthyes [superclass] —P279→ fish. **Folk "is a kind of":**
- a P171 chain does not climb from a taxon of rank "class" (P105 = Q37517) or higher (superclass, phylum, kingdom…);
- exception — one step class → superclass: Actinopterygii → Osteichthyes —P279→ fish, otherwise a pike is not a fish;
- bypassing clades (P171 past a class, to a higher class) are discarded: from the P171 region only nodes from which the lowest class is reachable remain;
- P279 — unrestricted; shortest distance by relaxation (whale —P279→ Cetacea —P279→ mammal — 2 steps, although via the P171 chain it's 17).

| query | folk rule | full cladistics (control) |
|---|---|---|
| whale → mammals | yes, 2 steps | yes |
| whale → fish | **no** | yes, 26 steps — without the rule it goes red |
| crow → birds | yes (anchor taxon Corvus, 15 steps) | yes |
| shark → fish | yes (shark —P279→ fish) | yes |
| bat → mammals · → bird | yes · no | yes · no |
| pike → fish | yes | yes |
| crow → mammals | no | no |

- **Crow:** Q388450 "crow" is a common name (P31 Q55983715) with only P279 omnivore; there is no path to birds in Wikidata. For animals without a class among their folk ancestors the walker adds an **anchor taxon** — the best taxon among alignment candidates with ancestor support (genus better than species): Corvus Q43365.
- **Cycle gate:** graph of 8.46 M nodes, 9.22 M edges — acyclic; an injected fruit ⊂ apple goes red.
- Tests on a synthetic graph: bypassing clade, grade, a shark does not climb out of Chondrichthyes, without the rule a whale is a fish.

### Stricter synset ↔ Q alignment (`wd-align` v2)

The v1 rule plus: a generality threshold for the common ancestor of **2%** (was 5%), **two supporting sources** — label, lexeme, direct link (ILI and P8814 together count as one support, they are set together), folk ancestors. Ancestor support but a single source — status `single`.

| | v1 | v2 |
|---|---:|---:|
| synsets accepted | 53 392 (62.9%) | **23 898 (28.1%)** |
| shuffled labels: accepted | 7 965 — 6.7× drop | **586 — 41× drop** (threshold 10×) |
| manual sample of 50 (v1) | **37/50 = 74%** | 17 accepted, 16 of them correct (94%) |
| manual sample of 50 (v2) | | **49/50 = 98%** |
| silver (direct P5063/P8814 vs lex and label) | 77.9% | 78.5% |

- **Manual check of v1:** label only — 17/27 (63%), one direct — 1/3, two or more supports — 19/20 (95%). Errors: a frog species instead of the toad family, a pit viper instead of colubrids, a heraldic juniper, lecturer instead of reader, Tagalog instead of Filipino. In v2, 12 of the 13 wrong ones were rejected; shipwreck remained (event ↔ wreckage).
- **v2 error:** football player (American) ↔ association football player.
- **The price of strictness:** general concepts (bird, mammal, fish, crow) lack support — their OEWN ancestors are too general (vertebrate, animal). The walker takes them with lower confidence: two supports without ancestors — 0.7.
- Gold — `data/align-gold.tsv` (100 pairs, +/− with explanations of errors); `wd-align` measures every run against it.
- Output — `synset_q-v2.tsv` in the run; the derived `synset_q.tsv` and the database stay as they were.

### Core walker (`onto wd-walk`)

- **Core** — `data/core.tsv`, 294 concepts: animals from fables and tales (104), food (50), household (83), family (16), roles (24), magical (7), plants (10). Synset — by domain (`noun.animal`, `noun.food`…), 291 in their own lexfile.
- **Anchors** — for 286 concepts: strict alignment 209, two supports without consistent ancestors 35, ancestor support with one source 42, anchor taxon 14. No anchor: lamb, fish (animal), lamp, key, shield, cloak, shepherd, giant.
- **Pass:** statements of the anchor and folk ancestors — up to 3 steps, plus a class at any depth (bird, mammal, fish, insect).
  - Overly general ancestors (≥ 20 k descendants: product, physical object, food, organism, animal) — no inheritance.
  - A "part" that is a taxon or the concept itself (fox P527 red fox, Cetacea P527 whale) — 44 discarded.
  - Confidence < 0.4 — 151 discarded.
- **Numbers as distributions:** the anchor's own values, otherwise over descendants (P279/P171 down to 3 steps, ≤ 400): mean, σ, min, max, n, SI unit. A single value — "point estimate, σ unknown". Birth weight (P3831 = Q4128476) is not about adults: 159 values discarded. Lion: ≈ 157 ± 44 kg (126–188, female and male; excluding the 1.65 kg newborn).
- **Output — `seeds/wd-candidates.tsv`:** 2 171 candidates for 232 concepts. Columns: concept, domain, synset, property, value, source (Q, P, rank, anchor and steps), confidence, mean, sd, min, max, n, unit. These are **candidates, not truth**: they enter the core only after confirmation.

| property | P | candidates | concepts | confidence ≥ 0.7 |
|---|---|---:|---:|---:|
| part | P527, P2670 | 831 | 174 | 153 |
| use | P366 | 598 | 151 | 122 |
| food | P1034 | 267 | 32 | 164 |
| material | P186 | 226 | 66 | 74 |
| color | P462 | 47 | 19 | 39 |
| no_part | P3113 | 46 | 36 | 0 |
| habitat | P2974 | 34 | 11 | 5 |
| mass · height · length · lifespan · speed | P2067 · P2048 · P2043 · P2250 · P2052 | 52 · 30 · 24 · 12 · 4 | | |

Confidence = anchor (strict 0.9 · two supports 0.7 · one support or anchor taxon 0.6) × statement rank (preferred 1 · normal 0.9) × 0.85^steps to the ancestor. Numbers: own — ×0.95, distribution over descendants — ×0.7.

**Examples:** cat — part tail, claw, paw, tooth, snout; knife — part blade, handle, tang, use cutting; bread — material flour, water, salt; lion — food ungulate; horse — height ≈ 1.47 ± 0.19 m (n = 174, over 108 breeds).

### LLM: a second opinion (`onto wd-confirm`)

`claude -p` (claude-opus-5-5, effort high via `--settings` env, API key from `local.md`, no tools), cap $10, log `vmm-calls.jsonl`. The answer is "yes / no / refine + why". The criterion is the typical instance, default knowledge.

| sample (60) | yes | refine | no | cost |
|---|---:|---:|---:|---:|
| disputed, confidence 0.45–0.8 → `seeds/wd-confirm.tsv` | 10 | 7 | 43 | $0.20 |
| confident, ≥ 0.7 → `runs/…/confirm-high.tsv` (measurement) | 34 | 7 | 19 | $0.13 |

- Confidence separates: 57% "yes" versus 17%.
- **Systematic candidate errors** revealed by the LLM:
  - no_part "human" for mammals (P3113 of a general class);
  - material "human food" / "food ingredient" for fruit and fish;
  - part "livestock", "predation", "she-bear" (P527 not about parts);
  - P1034 — a list of all possible prey rather than typical food: wolf — apple, grapes;
  - the color of water from "colors of water";
  - spade — black, from the card suit (a different sense).
- Among the "no" answers for confident candidates, most are "not typical" rather than "wrong": a tiger rarely eats fish, a therapy dog.

### Character — only in fables

At the cost of 5 `Prop` values (`Cunning Vain Greedy Industrious Wise`, preference `noun.animal`) — 7 seeds on layer `fable` (d67–d73): the fox is cunning, the crow vain (Perry 124), the wolf greedy (Perry 156), the ant industrious, the grasshopper not (Perry 373), the owl wise (Perry 437), the donkey not (Perry 188). The real layer is silent about character: `prop fox cunning` → "unknown", with `--layer fable` → "yes". Sounds were not done by hand — that was a joke.

### v2 check

- `cargo test --release` — 27/27: +3 tests of the folk rule and anchors, +1 LLM answer parsing.
- Frozen set — 71/74 (v1 blind — 70/74), no worse.
- Negative controls — 8/8: the layer exception now catches 16 mismatches, including character.
- Bridge — 302/305.
- Check queries and the cycle gate — green (`walk.md`).

## v3 (26.09): compiled core, context-based sense selection, description

Principles: well-known standard animals live a normal compiled life, not as a dynamic class (`docs/context.md`, "The core is compiled"); a simple test — take the set of animals, trees, etc. from the tales and check how well the current model can describe them. Runs — `$MOVA_DATA/runs/onto3-2026-09-26/`.

### Core in code (`build.rs`, `src/core.rs`, `src/coresel.rs`, `src/coregate.rs`)

- **Selection** (`onto core-select` → `data/core-props.tsv`): the core gets what the LLM confirmed, and confident (≥ 0.7) items without an LLM "no"; the rest stay candidates.
  - The LLM (`onto core-vmm confirm`, Opus, effort high) went through **all 2 071 candidates without a verdict** — both disputed and confident, because among the confident ones v2 saw 32% "no". Color and "no part" were not checked — they are not in the core. Verdicts: yes 876, no 1 009, refine 190; **$3.79**.
  - Risks (`onto core-vmm risks`): action → harm → probability per contact, order of magnitude from the LLM, for all 294 concepts — **$0.60**. Lion: pet → bite p ≈ 0.3, approach → bite p ≈ 0.01.
- **Explicit synsets for 27 concepts** (third column of `data/core.tsv`, gate: the lemma is a member of the synset, the synset is in the domain's lexfile). The first sense in the domain's lexfile was wrong: dolphin — dolphinfish, oak and birch — wood, net — the internet, bow — a bow knot, peacock — a butterfly, hedgehog — porcupine, castle — palace, hut — a military barracks… The walker was rerun: 2 280 candidates, the rows of the other concepts — byte for byte the same.
- **`build.rs`** (the build script in the crate root; `src/build.rs` is the graph build, a different thing) generates `enum CoreId` (294 variants, a duplicate lemma gets its domain: `FishAnimal`, `FishFood`) and `static CORE: [CoreRec; 294]`:
  - food, habitat, parts, material, use — sets of `Val`: Q, label, OEWN synset of the value (strict v2 alignment), a link to a `CoreId` if the value is also a core concept;
  - mass, height, length, lifespan, speed — `Dist {mean, sd, min, max, n, unit}`;
  - risks — `Risk {act, harm, p}`;
  - every value has a `Source`: Q, P, rank, steps from the anchor, confidence, confirmation (LLM "yes", refinement with a note, LLM estimate).
- **Completeness gate.** Every "concept × field" cell is explicit: a value, "no risks", "unknown" with a reason, or "not applicable" by domain (an animal has no material, a table does not eat). A deleted cell makes the build fail: `completeness gate: concept «lion (animal)» has no field mass — neither values nor an explicit «unknown»` (`runs/…/build-gate-red.txt`). The same code (`src/coregate.rs`) is checked by `onto controls`. The first build without `core-props.tsv` — `ONTO_CORE_BOOTSTRAP=1`.

| field | cells known | values | unknown | not applicable |
|---|---:|---:|---:|---:|
| parts | 114 | 339 | 133 | 47 |
| use | 123 | 297 | 124 | 47 |
| material | 47 | 111 | 86 | 161 |
| food | 23 | 111 | 81 | 190 |
| habitat | 9 | 21 | 105 | 180 |
| mass · height · length | 21 · 17 · 11 | distributions | 266 · 220 · 186 | 7 · 57 · 97 |
| lifespan · speed | 7 · 2 | distributions | 107 · 102 | 180 · 190 |
| risks | 293 | 367 (71 — "none") | 1 | 0 |
| **total (3 234 cells)** | **667** | **1 304** | **1 411** | **1 156** |

All 937 values from Wikidata were confirmed by the LLM: "yes" — 896, "refine" — 41. No unchecked confident values remain. Numbers are the bottleneck: mass is known for 21 of 287 concepts, because Wikidata has numbers mostly for species, not for everyday concepts.

- **"Core ↔ graph" gate** (`core::check`, in `load_kb`): the synset is in the graph and the lemma is its member; graph seeds on the real layer do not contradict the domain (an animal is not `Animate=no`, food is not `Edible=no`, an object is not `Living=yes`); a core part or material is neither the concept itself nor its ancestor. A contradiction refuses the load and shows both sides, rather than silently picking one.
  - On real data the gate found **6 contradictions**. Walnut, nut and acorn — food versus seed d55 "an object is not food": resolved by seeds d74–d76 (nut and acorn are edible). Lexfile not of the domain (nut, acorn — noun.plant, coin — noun.possession) — only a warning.
- **Query order — core first, then graph:**
  - `isa`, `haspart` — sense candidates: the query context's lexfile, the first sense, the core concept. They are ranked by a sentence-free WSD score: core 0.5 + corpus 1.5 × share + query lexfile 1; those within 0.3 of the best remain;
  - `prop` — in the preference lexfile, core concepts first;
  - `describe` — core fields, then the graph.

### Word sense selection by context — WSD v1 (`src/wsd.rs`)

A sense's score is a sum of terms, each visible in the explanation (`onto wsd mouse "The cat caught a mouse"`):
1. **Extended Lesk** (Banerjee & Pedersen 2003): words shared between the sentence and the glosses and lemmas of the synset (weight 1) and of its neighbours — hypernyms up to 2 steps, hyponyms, parts, whole, substances (weight ½); IDF weights over OEWN glosses.
2. **Predominant sense from a corpus** — McCarthy et al. 2004 (ACL P04-1036), `onto wsd-prior`:
   - a word's distributional neighbours — from within-sentence co-occurrences in the Leipzig corpora: eng_wikipedia_2016_1M, eng_news_2024_1M, `co_s`, significance, top-200 contexts, cosine;
   - neighbours vote for senses close in the graph: Wu–Palmer similarity to the 8th power (McCarthy used the sharp jcn);
   - output — 383 polysemous words in 18 s → `$MOVA_DATA/onto/sense-prior.tsv`.
3. **Core:** +0.5 for a sense that is a core concept.
4. **Domain:** the share of sentence words whose senses (ranks 1, ½, ⅓) lie in the sense's lexfile.

**Frozen set** — `data/wsd-frozen.tsv`: 60 sentences, 14 words (dolphin, oak, bank, bass, apple, crane, seal, bat, jaguar, mouse, mole, pike, cricket, jellyfish). Sha256 `2be56b43…`, committed before the WSD code (`f28a80d4`).

| system | correct |
|---|---:|
| OEWN "first sense" baseline | 21/60 (35%) |
| **everything together** | **43/60 (72%)** |
| Lesk only | 45/60 (75%) |
| corpus only (McCarthy) | 20/60 (33%) |
| corpus + core (no sentence) | 29/60 (48%) |
| without corpus | 47/60 (78%) |
| without Lesk | 28/60 (47%) |

- **The weights were set in advance.** Blind round — 43/60. The second (NON-blind) round changed the domain vote: previously the six senses of a word voted equally, and "cat" gave five artifacts (CAT scan, Caterpillar…) against one animal; now senses vote by rank. Together — the same 43/60; without corpus — 46 → 47.
- **The corpus weight hurts here** (72% versus 78% without it). News and Wikipedia pull mouse towards the device (neighbours goofy, clubhouse, keyboards), bass towards the instrument, cricket towards the game. In fairy-tale sentences this misfires; the core does not always outweigh it. Next — a fairy-tale corpus or the text's domain.
- **Errors** (all in `runs/…/wsd.md` with an explanation of each choice):
  - the core and corpus pull towards the tree where the context is about wood (oak ×2) and food (dolphin-mahi-mahi);
  - bank → "piggy bank";
  - apple-tree ×2.
- **dolphin / oak:** the v1 errors are gone.
  - Frozen set: `isa dolphin fish` → "no" — the sense is the core mammal, dolphinfish rejected by the score; `isa oak tree` → "yes".
  - Sentences about the animal and the tree (w01, w02, w05, w06, w07, w10) — correct.
- **Negative control — shuffled glosses** (a sense's signature taken from a random synset): Lesk only 45 → 20/60 (baseline 21); everything together 43 → 28 (no sentence 29). Goes red. When only the gloss texts are shuffled while lemmas and neighbours are real, Lesk holds 36/60: neighbour lemmas (whale, cricket bat) carry part of the signal.

### Concept description: `onto describe` and completeness on tale classes

`onto describe fox --layer fable` gives:
- the sense and why it was chosen;
- the IsA chain;
- parts, material, food, habitat, use (core, otherwise OEWN meronymy);
- sizes and mass as distributions;
- risks;
- seed properties, and on the fable layer — character.

Every row has a source.

`onto describe-eval` — 305 classes from the tale and fable worlds (302 described). "Before" — graph and seeds, like v2 without the core; "after" — the v3 core, then the graph. The core covers 89 of 302 classes.

| category | classes | fields per class: before → after | "unknown": before → after |
|---|---:|---|---|
| animals | 31–32 | 3.0 → 5.1 of 12 | 75% → 58% |
| food | 16 | 2.8 → 4.3 of 7 | 60% → 38% |
| objects | 117 | 3.1 → 3.7 of 9 | 66% → 59% |
| plants | 6 | 3.0 → 3.5 of 10 | 70% → 65% |
| people | 78–79 | 2.0 → 2.3 of 5 | 61% → 53% |
| **total** | **302** | **2.5 → 3.1 of 7** | **64% → 55%** |

By field after: parts 96%, risks 37%, use 24%, material 20%, food 29%, habitat 8%, mass 4%.

**Opus judge** (`onto describe-judge`): all 604 descriptions — 302 classes "before" and "after". Blind: versions shuffled under random numbers. Fixed rubric: C — correctness, F — completeness, U — usefulness, E — errors, each 1–5. Classes split by name hash: training — 194, held out — 108. Cost — $4.25.

| version | part | C | F | U | E | mean |
|---|---|---:|---:|---:|---:|---:|
| before | training | 3.88 | 1.81 | 1.91 | 3.44 | 2.76 |
| before | held out | 4.01 | 1.90 | 1.93 | 3.69 | 2.88 |
| after | training | 3.93 | 1.99 | 2.02 | 3.45 | 2.85 |
| after | held out | 4.11 | 2.09 | 2.10 | 3.74 | **3.01** |

- **By category** (before → after):
  - animals 2.95 → **3.31**, food 2.83 → **3.34**, objects 2.83 → 2.95, "other" 2.50 → 2.58;
  - people 2.89 → 2.80 and plants 2.96 → 2.88 — slightly worse: the core has almost no fields for them, and the judge counts "unknown" rows as gaps.
- **Distribution of the mean score**, rounded to an integer:
  - before: 15 / 51 / 164 / 72 / 0;
  - after: 18 / 54 / 141 / 77 / **12** — fives appeared.
- **Best after:** lion, wolf, cake — 4.5; the judge wants habitat and fuller risks.
- **Worst after** — 1.0, all due to a wrong word sense in classes outside the core:
  - flint — a river in Georgia;
  - shoes — "situation": the class uses the plural;
  - shell — an artillery shell.
- **Typical errors:**
  - animals — a taxon as a part (eagle "bird", hare "rabbit"), no habitat;
  - objects — no material, odd inherited parts;
  - plants — "hood" from plant;
  - people — taxonomy only.
- **Judge control:** 20 descriptions scored a second time with a differently worded rubric. Mean difference 0.35, within ±0.5 — 17 of 20, Spearman ρ = 0.75. The scores agree.

**The "student — architect" loop** — only the first round was done: the judge gave scores, the architect (`describe-judge architect`, $0.11, saw only the training part) proposed three changes (`runs/…/architect-r1.md`):
1. **Word sense by score, not the first one:**
   - lemmatization of plurals (shoes → shoe);
   - fairy-tale lexfiles — artifact, animal, plant, food, person — up; state, communication — down;
   - a threshold below which the LLM breaks ties.
2. **Parts and material only from physical ancestors and no further than 3 steps;** general ancestors (whole, structure) give no parts; for magical creatures — a bridge to creature.
3. **WordNet glosses into fields** via templates: "made of", "used for", "having a … and a …", "feeds on"; such values — with source gloss and lower confidence.

The changes were not implemented, and there was no second round. To measure rounds, the judge must use stable ids (class + version): in v3 the numbers depend on the shuffle of a single run.


### Pronoun API (`src/pron.rs`) — for the upcoming coreference

- `pron::info(kb, lemma, &Ctx {sentence, layer}) -> PronInfo {gender, animate, human, persona, why}`; wrappers — `gender`, `animate`, `human`; `PronInfo::pronouns()` — the allowed he / she / it.
- **Sense:** WSD by sentence, otherwise the core concept, otherwise the first sense.
- **Human and animate** — seeds `Human` / `Animate` on the layer.
- **Gender** — hints from the gloss and ancestor names: female / woman / queen… → fem., male / king / stallion… → masc.; both together — unknown.
- **Character:** an animal that talks on this layer (the fox in a fable) allows he / she / it.
- Only the interface and the `pronoun_api` test: crow — it, in a fable — he / she / it; Nixon — he or she; stone — it; unknown word — `None`.
- For a batch of sentences, `Wsd` should be built once (IDF — ~1 s); currently `sense()` builds it on every call with a sentence.

### v3 check

- **Frozen set (74)** — **72/74, 0 wrong, 2 "unknown"**; v2 — 71/74, 1 wrong (`isa dolphin fish`).
- **Bridge:** 302/305 classes, fable questions — 14/14.
- **Negative controls** — 10/10, two added:
  - core completeness: cell fox × habitat deleted → an error naming the concept and field;
  - "core ↔ graph": an injected fox seed `Animate=no` → gate.
- **WSD control** (shuffled glosses) — goes red.
- **`cargo test --release`** — 31/31: +4 — completeness gate, pronouns, WSD basics, LLM risk parsing.
- **API costs:**
  - core LLM — $4.44: confirmation pass $3.79, risks $0.60, probe $0.05;
  - judge — $4.36 (604 descriptions and control $4.25, architect $0.11); total for v3 — $8.80.

## Independent tests (tests3, 26.09)

Goal: more tests for the common-sense builder. The sets are in `tests/data/vmm/` (LLM) and then in `…/ext/`,
so that after merging v3 the same files can be run on it too: `onto vmm <tag> [--haspart rows.json]` (the hash of every file
is checked before the run; report — `vmm-<tag>.md` and `.tsv` in `ONTO_RUNS`).

### LLM questions

- **Author — Opus** (effort high, saw neither the code nor the data; prompts — `tests/data/vmm/prompts/`), gold standard —
  common sense and biology, every question has an explanation. **400 questions**, frozen by hashes before the first run
  (commit `66711cecf`): `isa` with traps (80) and made-up words (10); parts, materials, members (80); abilities,
  properties and `fable`/`fairy` layers (80); `measure X mass|height|length|speed|lifespan` — ranges (70) and
  made-up animals (5); `sense W | sentence` — lexicographer file and keyword (75).
- **Run on v2** (from main): v2 cannot do `measure` and `sense` yet — 150 "not supported", waiting for v3.

| group | correct | "unknown" | wrong | not supported |
|---|---:|---:|---:|---:|
| isa | 74 | 0 | 6 | — |
| unknown (made-up words) | 10 | 0 | 0 | 5 |
| part / material / member | 23 / 4 / 3 | 25 / 16 / 7 | 2 / 0 / 0 | — |
| ability (`can`) / property / layer | 28 / 21 / 16 | 5 / 4 / 1 | 2 / 0 / 3 | — |
| mass, height, length, speed, lifespan, sense | — | — | — | 70 + 75 |
| **total** | **179** | **58** | **13** | **150** |

**Analysis of the 13 wrong answers** (a third path — biology and definitions, not the data): all are our defects.
- sense selection (a known v1 defect): `isa dolphin fish` → yes (first sense — dolphinfish), `isa bamboo grass` →
  no (first sense — stems as material), `isa king woman` → yes (the instance Billie Jean King in lower case);
- OEWN data: `isa moss plant` → no (mosses are bryophytes outside "plant"), `isa tomato fruit` → no (culinary taxonomy:
  a vegetable; botanically — a fruit), `isa doctor person` → no — check the chain;
- part inheritance is too generous: `haspart insect bone`, `haspart jellyfish brain` → yes;
- seeds: `can butterfly fly` → no (no CanFly for insects), `can human speak` → no ("human" is a hominid under
  "animal", with no human seed); the `fairy` layer lacks "animals talk": `can wolf|cat|frog speak --layer fairy` → no.

"Unknown" instead of "yes" for parts (48) — incomplete OEWN meronymy, as intended: we never say "no" about parts.

### From the internet: catalog and samples

| set | where | license | what was done |
|---|---|---|---|
| hasPart KB (Bhakthavatsalam et al. 2020) | HF `community-datasets/has_part` | "unknown" on the card — measurement only, not in the repo | 100 rows via datasets-server → `$MOVA_DATA/raw/tests-onto-ext/`; **23 yes, 77 "unknown", 0 wrong** |
| NumerSense ("a bird has two legs") | HF `INK-USC/numer_sense` | MIT | the set is loaded by a script (data from GitHub): not downloaded |
| CommonsenseQA | HF `tau/commonsense_qa` | MIT | catalog; measurement — after v3 (multiple-choice questions) |
| PIQA | HF `ybisk/piqa` | "unknown" on the card | catalog; measurement only |
| GenericsKB | HF `community-datasets/generics_kb` | CC BY 4.0 | catalog; a source of generalization seeds for v3 |
| DoQ (quantity distributions) | GitHub only | — | family rule: we do not access GitHub |
| Verb Physics (size comparisons) | GitHub only | — | same |

## HS: world model of goods (26–27.09)

The task: read the goods classifier from the import-export project — about 6 thousand entries — and extract a word ontology, as was done for the tales. HS is a ready-made world model of goods (`docs/context.md`, "Thousands of domains"): a tree, "others", features, thresholds. Runs — `$MOVA_DATA/runs/hsonto-2026-09-26/`.

**Data and license.**
- Source — `BACI/product_codes_HS22_V202601.csv` (BACI, read only): 5 609 HS22 subheadings with English descriptions. BACI takes the descriptions from UN Comtrade.
- The description texts belong to the WCO, so they are not in the repo. `data/hs/` carries only derived data: code → concept, features, labels. Verified: not a single description verbatim.
- Section and heading names are not available in our data: the trade data and the translation project have only HS6, and P5471 (HS 2022) was not extracted into `wikidata.duckdb`. So heading labels are ours: the most frequent head concepts of the heading.

### How it works (`src/hs.rs`, `src/hsmap.rs`)

1. **Tree:** 97 chapters → 1 229 headings → 5 609 subheadings, with parents (`data/hs/tree.tsv`). "Others" are marked separately:
   - by code (…9, …90) — 1 375;
   - textual (n.e.c., other than) — 1 476;
   - both together — 906.
2. **Parsing.** `en` (model `ud-ewt-eslspok`) gives lemmas, parts of speech and the tree in 2.6 s for all descriptions. The HS telegraphic style ("Fruit, edible: apples, fresh") is split by colon into levels (0 — the heading top) and by commas into pieces. A piece is either features from closed enums (enum `State`, `Process`, `Attr`) or a noun phrase. Rules that emerged on real descriptions:
   - the list "of sheep, goats, horses" extends across commas;
   - "excluding …" and "not fillets, meat of 0304…" — exclusions to the end of the level, "(excluding X)" — only within the parentheses;
   - "men's or boys'" — a purpose, not a product;
   - Latin names in parentheses — taxa;
   - "Nuts, edible" → edible nut, "Lighters: cigarette" → cigarette lighter, if such a compound exists in OEWN.
3. **Thresholds — via `math`:**
   - "over 1000 but not over 1500cc" → `Capacity ∈ (0.001, 0.0015] m³`: exact rationals `Rat`, bounds with inclusivity;
   - units from `math::units` (kg, g, m, cm, mm, %), g/m² and cm³ — via `Mono`; t, l, kW, kVA, V, dtex, years — with an exact multiplier.
4. **Alignment** (`data/hs/terms.tsv`, `codes.tsv`), with the reason on every row. Steps:
   1. Phrase → the longest one present in OEWN; British spelling as a fallback.
   2. WSD v1 with the heading context: Lesk, corpus, core, domain.
   3. Product state: live → animal or plant; fresh, frozen… → food (also accepts animal). If the longer phrase lacks such a sense, the shorter one is taken: fatty liver — a disease, liver — food.
   4. Chapter domain — in a second pass: the lexfile share of the chapter's head concepts, "one sense per text".
   5. Then — the core concept, and then a Q via strict v2 alignment (`accepted`).
5. **LLM confirmation** (`onto hs confirm` → `data/hs/sense-confirm.tsv`).
   - Key — phrase, chapter and state class: live and frozen fish are not merged.
   - We take 240 disputed (margin < 0.15) and most frequent keys.
   - Final run: the LLM agreed in 112, chose another sense in 105, "none fits" — 23. Cost — $0.71.
   - WSD errors in frequent words are the most costly: rubber in chapter 40 — a condom, iron — a clothes iron, steel — a knife sharpener, glass — a drinking glass, wood — not timber.

### Numbers

| measure | value |
|---|---:|
| head concept found (deepest product, otherwise the heading top) | 5 575 / 5 609 (99.4%) |
| **head concept aligned with a synset** | **5 254 (93.7%)** |
| — core concept / with Q | 434 (7.7%) / 2 536 (45.2%) |
| terms (distinct phrases) / occurrences aligned | 4 298 / 4 792 (89.7%) · 17 282 / 18 176 (95.1%) |
| "unknown": head concept not aligned | 355 (6.3%) |
| label words without a class | 2 086 / 61 317 content words (3.4%) |

**Features** (`data/hs/feats.tsv`): how many features, in how many subheadings.

| state | processing | of X | form | purpose | parts | taxon | threshold | composition | except |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 171 · 735 | 2 236 · 1 469 | 3 100 · 1 732 | 613 · 478 | 1 706 · 1 045 | 238 · 218 | 419 · 209 | 958 · 753 | 1 517 · 927 | 1 890 · 1 166 |

- **Most frequent values:**
  - state — fresh 253, chilled 175, frozen 162, dried 134, edible 131, raw 116, live 70;
  - processing — knitted 292 (¬ 230), coated 232, woven 227, rolled 165.
- **"of X":** material 1 889, source organism 357, other 713, not aligned 162.
- **Thresholds:** share (%) 285, width 125, g/m² 121, thickness 67, mass 48, dtex 39, volume 31, power 28, voltage 21, age 14, Brix 12.

**IsA gate.** HS says "product ⊂ heading top" (apple ⊂ edible fruit). The negative control swaps products between headings.

| verdict | real | control |
|---|---:|---:|
| yes / yes via another sense | 824 / 90 | 22 / 15 |
| part / taxon heading / kind of the top's modifier ("Mollusc preparations: mussels") | 45 / 3 / 106 | 18 / 0 / 0 |
| sisters (common ancestor — the top's parent) | 130 | 26 |
| product made of the top's material / organism — source of the product | 675 / 102 | 862 / 270 |
| **no — contradiction** | **648 (24.7%)** | **1 410 (53.8%)** |
| unknown (product or top not aligned) | 335 | — |

- **Strict verdicts** — "yes", another sense, part, taxon, modifier: 1 068 (40.7%) versus 55 (2.1%) in the control. The gate discriminates.
- **"Made of the material"** and **"source"** — a type check via lexfiles, not a fact check. The control gives even more of them, so these are soft labels, not confirmation.
- **"No" in a sample of 16:**
  - about ⅔ are our defects. The phrase fell back to a more general one: dairy produce → produce, ferro vanadium → vanadium. Wrong sense: lead the object instead of the metal. Parsing garbage.
  - about ⅓ are genuine HS vs WordNet discrepancies: relay ⊄ electrical apparatus; starter motor ⊄ ignition equipment; safety seat belt sits under vehicle in HS, while for us seat belt is neither a kind nor a part of vehicle.

**Feature versus core and seeds** — 21 cases, and the gate found a gap in the seeds:
- HS "edible" contradicts seed d55 "an object is not food by default" on fruit (the fruit as a plant organ), snake, salmon;
- HS "live" contradicts d02 on bulb (a bulb is a living plant part);
- fruit and animals as food lack `Edible`;
- core-material (4) — a parsing defect: "snake of meat".

**Neighbour thresholds.** Subheadings of a heading with otherwise identical features must differ by a threshold.
- Pairs — 313, undistinguished — 33: parsing lost a feature or cut off a threshold. The gate showed "filled / not filled" in chocolate, and this was fixed: an unknown feature now remains a label with negation.
- Control — copying a neighbour's threshold — goes red on 313 of 313.

**New concepts** (`data/hs/new-concepts.tsv`) — 494 phrases (894 occurrences) that are neither in the core nor in the graph:
- monofilament 35, cermet 14, peroxyacid 10, furskins 9, nonwoven 8;
- fish families: Merlucciidae, Moridae, Muraenolepididae;
- toothfish, epoxide;
- some are parsing fragments: mechanical, poly, non-electric.

**Blind Opus judge** (`onto hs judge`).
- **Sample:** 150 subheadings by code hash, across all chapters. Plus 20 decoys: the output of another code attached to a foreign description. Everything shuffled.
- **Rubric** (1–5):
  - M — head concept and its sense;
  - F — completeness of features;
  - P — precision: nothing wrong.
- **Runs.** The sample and decoys are the same in both runs; r1 — before the last parsing fixes.

| run | M | F | P | mean | distribution 1–5 | decoys | $ |
|---|---:|---:|---:|---:|---|---:|---:|
| r1 | 3.45 | 3.59 | 3.44 | 3.49 | 3 / 29 / 42 / 40 / 36 | 1.00 | 0.70 |
| **r2** | **3.59** | **3.59** | **3.65** | **3.61** | 2 / 24 / 40 / 44 / 40 | **1.00** | 0.71 |

- **Worst in r2** — a wrong sense of a word or of the head concept:
  - portal (crane) — an entrance;
  - counterweight;
  - cocoon instead of silk waste;
  - "Reception" — an action, not apparatus.
- **Decoys — exactly 1.00:** the rubric does not praise foreign output.

**Labels in other languages** (`onto hs labels`, `sql/hs-labels.sql`). Q labels from Wikidata (CC0, `data/hs/labels.tsv`) were checked against the translation project's translations of HS names: does the stem of our concept's label occur in the subheading's translation.

| language | uk | de | fr | es | pl | ru | it | nl |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| match | 76.2% | 80.8% | 87.0% | 86.1% | 78.2% | 80.4% | 84.1% | 80.3% |

Mismatches are mostly Q taxa with a Latin label (Bos taurus, Bubalina, Thunnini), families instead of species (Nephropidae), synonyms ("Innereien" versus "Schlachtnebenerzeugnisse"). Less often — our wrong concept: rose → rose hip.

**API costs** — $2.91 of a $15 cap: confirmation $0.09 (probe) + $0.71 + $0.71, judge $0.70 + $0.71.

**`onto hs describe` examples** (in full — `runs/…/describe-examples.txt`):

```
080810 — subheading of heading 0808 [apple, pear, quince] of chapter 08 [fruit, coconut, almond, apple]
head concept (goods): apple → oewn-07755101-n «apple» (noun.food; core: Apple; Q89)
  why: Lesk 0.77 [fruit, edible] + corpus 1.17 + core 0.50 + domain 0.32 + HS state 1.00 + chapter 0.94 = 4.70
  is a kind of (OEWN): apple → edible fruit → produce → solid food → food → …
HS: apple ⊂ edible fruit — ours: yes
features: state: edible; fresh
world model: parts pericarp; mass ≈ 0.167 kg ± 0.047; risks eat → allergy p ≈ 0.01 [core, with sources]

030213 — subheading of heading 0302 [fish, Alaska pollock, Atlantic salmon, Nile perch] of chapter 03
head concept (goods): Pacific salmon → oewn-02537365-n «salmon» (noun.animal; core: Salmon; Q2796766)
HS: Pacific salmon ⊂ fish — ours: yes (salmon → food fish → fish)
features: state fresh; chilled · except fillet; meat; fish offal · reference 0304 · taxon Oncorhynchus nerka, … (7)

870322 — subheading of heading 8703 [vehicle] of chapter 87
head concept (heading head): vehicle → oewn-04531608-n «vehicle» (noun.artifact)
features: threshold Capacity ∈ (0.001, 0.0015] m³ · composition: spark ignition internal combustion piston engine
```

**Check.**
- `cargo test --release` — 36/36: +5 HS tests — exact thresholds, bound intersection with a negative control, telegraphic-style features, CSV and tree, IsA gate on a synthetic graph.
- Frozen set — 72/74 (0 wrong).
- WSD — 43/60.
- Controls — all green.

## Commands

```
export PATH=$HOME/.cargo/bin:$PATH
cargo build --release
../target/release/onto build                        # OEWN 2025+ and FrameNet 1.7 → $MOVA_DATA/onto/onto-v1.bin (ONTO_SNAPSHOT)
../target/release/onto isa apple fruit
../target/release/onto can crow speak --layer fable
../target/release/onto haspart bird beak · parts bird · frame buy · role buy Seller · inherits Commerce_buy Getting
../target/release/onto entails snore sleep · word whale · up crow
../target/release/onto eval blind                   # frozen set → $MOVA_DATA/runs/onto-2026-09-26/frozen-blind.{md,tsv} (ONTO_RUNS)
../target/release/onto controls | bridge | bench | stats
../target/release/onto wd-lexemes $MOVA_DATA/raw/wikidata/20260923/wikidata-20260923-lexemes.json.bz2 <dir>
../target/release/onto wd-map <dir>
nice ../target/release/onto wd-probe $MOVA_DATA/raw/wikidata-all/latest-all.json.bz2 1 <dir>
# Wikidata: pass and database (section "Wikidata: pass and database"; runs — ONTO_WD_RUNS, default $MOVA_DATA/runs/wikidata-2026-09-26)
D=$MOVA_DATA/raw/wikidata-derived
nice -n 10 ../target/release/onto wd-sample $MOVA_DATA/raw/wikidata-all/latest-all.json.bz2 <dir> --k 48 --mb 12 --level 9 --skip-wm
nice -n 10 ../target/release/onto wd-extract $MOVA_DATA/raw/wikidata-all/latest-all.json.bz2 $D --bz 8 --workers 2 --level 9 --skip-wm [--gb 2]
nice -n 10 ../target/release/onto wd-lex $MOVA_DATA/raw/wikidata/20260923/wikidata-20260923-lexemes.json.bz2 $D
../target/release/onto wd-senses $D <run>/wd-senses-en.tsv && ../target/release/onto wd-map <run>
nice -n 10 ../target/release/onto wd-closure $D --threads 8
nice -n 10 ../target/release/onto wd-align $D --wd-map <run>/wd-map.tsv [--generic 0.02] [--min-src 2] [--no-folk] [--out synset_q.tsv]
# v2: walker (section "v2"; runs — ONTO_WD_RUNS=$MOVA_DATA/runs/onto2-2026-09-26)
nice -n 10 ../target/release/onto wd-walk $D [--align <run>/synset_q-v2.tsv] [--core data/core.tsv] [--out seeds/wd-candidates.tsv]
../target/release/onto wd-confirm [--n 60] [--batch 30] [--cap 10] [--lo 0.45 --hi 0.8] [--out seeds/wd-confirm.tsv]   # LLM, $ — API
../target/release/onto prop fox cunning --layer fable
(cd $D && ~/bin/duckdb $MOVA_DATA/db/wikidata.duckdb < <repo>/onto/sql/wikidata-load.sql)
~/bin/duckdb -readonly $MOVA_DATA/db/wikidata.duckdb < sql/wikidata-checks.sql
# v3: core, WSD, description (section "v3"; runs — ONTO_RUNS / ONTO_WD_RUNS=$MOVA_DATA/runs/onto3-2026-09-26)
../target/release/onto core-vmm confirm [--batch 50] [--cap 7]      # LLM goes through candidates without a verdict, $ — API
../target/release/onto core-vmm risks [--batch 50] [--cap 8]        # risks of core concepts
../target/release/onto core-select                                  # → data/core-props.tsv; then cargo build (completeness gate)
../target/release/onto wsd-prior                                    # predominant senses from the Leipzig corpora → $MOVA_DATA/onto/sense-prior.tsv
../target/release/onto wsd mouse "The cat caught a mouse in the kitchen"
../target/release/onto wsd-eval                                     # WSD frozen set + shuffled-gloss control
../target/release/onto describe fox --layer fable [--no-core]
../target/release/onto describe-eval                                # tale classes before/after the core, completeness
../target/release/onto describe-judge [--cap 15] | describe-judge report   # blind Opus judge, $ — API
# HS: world model of goods (section "HS"; runs — ONTO_RUNS, default $MOVA_DATA/runs/hsonto-2026-09-26)
../target/release/onto hs build [--no-confirm]                      # HS22 (BACI) → data/hs/*.tsv, report hs-report.md (~5 s)
../target/release/onto hs describe 080810 [--text]                  # product description; --text — WCO description from a local file
../target/release/onto hs parse 030213                              # parsing debug (tokens and features)
../target/release/onto hs confirm [--n 240] [--margin 0.15]         # LLM confirms disputed and frequent senses, $ — API
../target/release/onto hs judge [--n 150] [--decoys 20]             # blind Opus judge, decoys as control, $ — API
(cd $ONTO_RUNS && ~/bin/duckdb -readonly $MOVA_DATA/db/wikidata.duckdb < <repo>/onto/sql/hs-labels.sql) && ../target/release/onto hs labels
cargo test --release                               # 36: world model, Wikidata, folk rule, anchors, LLM, core, pronouns, HS
```

## What's next

0. **v3 did:** a compiled core with gates, WSD v1 (72% versus the 35% baseline), `describe`, the pronoun API. Next from v3:
   - a fairy-tale corpus for predominant senses (news pulls mouse towards the device);
   - numbers for everyday concepts — mass is known for 21 of 287;
   - the "student — architect" loop: implement the round-1 proposals (word sense by score, physical ancestors for parts, glosses → fields) and measure on the held-out part;
   - one `Wsd` per batch for pronouns.
1. **Word sense from sentence context** (v3 — WSD v1). All v1 errors come from OEWN sense order: dolphin, oak, jellyfish. Needed: sense selection by sentence and by text ("one sense per text" already exists as a hook). A second frequency vote — the sense order of Wikidata lexemes and our corpora.
2. **Wikidata into the core** (v2 did: a whale is not a fish, strict alignment, 2 171 candidates with provenance, LLM on 120). Next: human confirmation → `build.rs` stitches the core from the confirmed ones; filters for the systematic errors revealed by the LLM (P3113 of general classes, P186 "human food", P527 not about parts); P1034 — typical food, not all prey; general concepts (bird, mammal) — support via P8814/ILI without ancestors.
3. **Seeds from OEWN glosses** automatically ("flightless", "edible", "swift" → a property with `gloss` provenance) with a manual sample.
4. **Selectional preferences** from FrameNet roles (role semTypes) and IsA: who can be a Buyer, what can be eaten. Then — arc plausibility for the second parsing pass (`docs/context.md`).
5. **World v4:** the LLM writes an entity's class directly as a synset id. The bridge reads the world state, not just the commands.
6. **Reasoning module:** Horn clauses over the TBox (world model) and the ABox (the tale's world), every conclusion with a proof.
7. **Size, quantity, comparison:** a lion is big, a mouse is small, a hare is faster than a tortoise.

## Files

| file | what |
|---|---|
| `src/model.rs` | concepts, words, `Pos`, `Src`, `Rel`, `Core`, `LEXFILES`, CSR graph |
| `src/build.rs` | OEWN XML and FrameNet → graph, build gates |
| `src/zip.rs` | minimal zip reader |
| `src/store.rs` | binary graph snapshot |
| `src/kb.rs` | layers, seeds, extra edges, inheritance, Nixon diamond, seed gates |
| `src/query.rs` | sense selection, queries with proof |
| `src/bridge.rs` | bridge to the narrative world |
| `src/eval.rs` | frozen set, evaluation, negative controls |
| `src/wikidata.rs` | full dump measurement, lexemes, trial alignment |
| `src/wdx.rs` | pass over the full dump: lbzip2 → multi-threaded parsing → TSV zstd; sampling from the middle of the dump |
| `src/wdlex.rs` | lexemes → lemmas, forms, senses, translations; `wd-senses` for `wd-map` |
| `src/wdgraph.rs` | "is a kind of" graph (P279 + P171), cycle gate, closure |
| `src/wdalign.rs` | synset ↔ Q alignment with ancestor check, shuffled control, silver; v2 — two supports, 2%, folk ancestors, gold |
| `src/wdwalk.rs` | v2: folk "is a kind of" (P171 up to class, grade, bypassing clades), core walker, anchors, number distributions, check queries |
| `src/wdvmm.rs` | v2: LLM via `claude -p` — "yes / no / refine", cap and cost log |
| `sql/wikidata-load.sql`, `sql/wikidata-closure.sql` | loading derived files into `$MOVA_DATA/db/wikidata.duckdb`, macros |
| `sql/wikidata-checks.sql`, `sql/wikidata-core-coverage.sql` | check queries; core coverage of fable classes |
| `build.rs` | v3: build script — core `data/core.tsv` + `data/core-props.tsv` → `enum CoreId`, `CORE` (completeness gate) |
| `src/core.rs`, `src/coregate.rs`, `src/coresel.rs`, `src/corevmm.rs` | v3: core types and the "core ↔ graph" gate; completeness gate; selection; LLM (confirmation pass, risks) |
| `src/wsd.rs` | v3: WSD v1 — Lesk, McCarthy 2004 from the Leipzig corpora, core, domain; shuffled-gloss control |
| `src/describe.rs`, `src/judge.rs` | v3: concept description with proofs, completeness; blind Opus judge |
| `src/pron.rs` | v3: pronoun API — gender, animacy, humanness |
| `src/hs.rs`, `src/hsmap.rs` | HS: code tree, description parsing (`en`), features and thresholds (`math`); alignment with the world model, gates, controls, `hs describe`, judge, LLM confirmation, labels |
| `data/hs/*.tsv` | HS: derived data only — tree, code → concept, features, terms, new concepts, contradictions, LLM confirmations, Wikidata labels (CC0); no WCO texts |
| `sql/hs-labels.sql` | HS: concept labels in eight languages from `wikidata.duckdb` |
| `data/core-props.tsv` | v3: core — every "concept × field" cell explicit, with a source |
| `data/wsd-frozen.tsv` + `.sha256` | v3: WSD frozen set (60 sentences) |
| `seeds/wd-confirm-high.tsv`, `seeds/wd-confirm-v3.tsv`, `seeds/core-risks.tsv` | LLM verdicts: confident v2 (60), v3 confirmation pass (2 071); risks of core concepts |
| `src/tests.rs` | tests on a synthetic graph |
| `seeds/layers.tsv`, `seeds/defaults.tsv`, `seeds/relations.tsv` | seeds (v2: +7 character seeds on layer `fable`) |
| `seeds/wd-candidates.tsv` | v2: seed candidates from Wikidata — 2 171, with source, confidence and distributions; not truth |
| `seeds/wd-confirm.tsv` | v2: LLM second opinion on 60 disputed candidates |
| `data/core.tsv` | v2: core — 294 concepts (lemma, domain) |
| `data/align-gold.tsv` | v2: 100 synset ↔ Q pairs checked by hand (v1 and v2 samples) |
| `data/frozen.tsv` + `.sha256` | frozen set (74) |
| `data/fable-questions.tsv` + `.sha256` | fable questions (14) |
