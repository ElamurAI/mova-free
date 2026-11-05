# onto — the world model

A model of abstractions kept separate from the narrative: TBox versus ABox, as described in [docs/context.md, "World model — separate from the narrative"](../docs/context.md#world-model--separate-from-the-narrative). The guiding idea: build the world model before the mathematics; it still has to be taught, much like in fairy tales, that an apple is a fruit.

- **World model:** concepts, their hierarchies, parts, typical actions and roles, default properties with exceptions. Shared by all texts.
- **Narrative** (`world`) — the individuals of a single text. Crow1 refers to the concept "crow" rather than copying it.

The core does not depend on `en`: lemmas and parts of speech come from OEWN itself. Only the `hs` module (goods classifier) takes parses from `en` and exact numbers from `math`.

Parts of the crate:

- **Graph:** OEWN 2025+ synsets, FrameNet 1.7 frames and roles, words, senses, lexical units and typed edges, built into a binary snapshot.
- **Seeds and layers:** default properties with a source on every row, extra edges, and a layer tree (real world, genres, collections, tales).
- **Queries with proof** over the graph and seeds, and a read-only bridge to the narrative world.
- **Wikidata:** a streaming pass over the full dump into DuckDB, synset ↔ Q alignment, and a walker that collects seed candidates for the core.
- **Compiled core:** standard concepts (animals, food, household objects…) compiled into code with a completeness gate; word sense disambiguation by context; concept descriptions; a pronoun API.
- **HS:** the goods classifier read into the world model.

## How it works

### Model (`model.rs`)

- **A concept is a `u32`.** Struct of arrays: source (`Src`: OEWN synset, frame, role, Wikidata Q), key (synset offset, frame ID), part of speech (`Pos`), lexicographer file (index in `LEXFILES`), ILI, gloss, member lemmas.
- **A word is a `u32`.** Sorted dictionary, binary search. A word's senses are in OEWN frequency order, i.e. the order of `<Sense>` in the entry.
- **Relations — enum `Rel`**, direction as it reads:
  - from OEWN: `IsA`, `InstanceOf`, `PartOf` (part → whole), `MemberOf`, `MadeOf` (whole → substance), `Entails`, `Causes`;
  - from FrameNet: `HasRole`, `Inherits`, `Uses`, `SubframeOf`, `PerspectiveOn`, `Precedes`, `CausativeOf`, `InchoativeOf`.
- **Edges — CSR in both directions.** A lexical unit is a word with a part of speech → frame. Within a word, units with more annotated sentences come first.
- **Properties — enum `Prop`:** `Living Animate Human CanSpeak CanFly CanSwim CanWalk Edible LaysEggs Fast Exists Pacifist`; character for fables: `Cunning Vain Greedy Industrious Wise`. Strength is `Default` or `Strict`; the layer is a `LayerKind`: real world, genre, collection, tale.
- **Strings only at input and output:** XML, TSV, query, answer.

### Build (`build.rs`, `zip.rs`) and snapshot (`store.rs`)

- **OEWN** is taken from `english-wordnet-2025-plus.xml.gz`.
  - The base 2025 edition has no `instance_hypernym`: proper names and taxa were moved out, and member meronymy is sparse.
  - `-plus` is a superset of the base edition. Concepts present only in `-plus` are marked with a "plus only" bit.
- **FrameNet** is read directly from the zip by our own minimal reader (central directory + deflate): the `zip` crate is not available offline, and there is no need to unpack to disk. From the archive we take `frame/*.xml` — roles with `coreType`, lexical units and frame relations.
- **Build gates** (violation — refusal):
  - OEWN inverse pairs are consistent: `hypernym`/`hyponym`, `holo_*`/`mero_*`, `entails`/`is_entailed_by` and others;
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
- **`relations.tsv`** — extra IsA/InstanceOf. Currently only Nixon — a Quaker and a Republican (the Nixon diamond).
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

Without a sentence, the context comes from the query itself (for `isa` and `haspart` the candidates are then ranked by the WSD score, section [Word sense selection by context](#word-sense-selection-by-context--wsd-srcwsdrs)):
- `isa X Y`: any sense of X in the lexicographer file of Y's main sense plus the first sense of X; Y — any of its senses. "whale" with "fish" is the animal, not the first sense "a very large person".
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

**Examples:**

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

- **Input:** `world` command files: `world/data/cmds-*.txt` and `$MOVA_DATA/runs/world-*/cmds-*.txt`. Content duplicates are read once.
- **Entity class → concept.** Order of attempts:
  1. as is;
  2. spelling variant;
  3. head word (`demon_chief` → chief, `love_potion` → potion).
- **Sense** — from the lexicographer file of the entity type.
- **Questions in English** (`Can the crow fly?`, `Is the meat food?`, `Does the crow have a beak?`): the entity is taken from the tale's world, and the query runs on that tale's layer. A word without an article is read as IsA unless it is a property.
- **One sense per text:** if a seed of this tale's layers sits on another sense of the same word in the same lexfile, that sense is taken, and the proof says so.
- The bridge does not modify `world/` files.

### Evaluation sets and controls

- **Frozen set** (`data/frozen.tsv` + `.sha256`): questions on IsA with traps, properties with exceptions, layers, parts, frames and roles, verbs. The gold standard is common-sense truth, not what the data says. The hash is checked before every run; `onto eval blind` writes a report with every question and its proof.
- **Fable questions** (`data/fable-questions.tsv` + `.sha256`), run through the bridge.
- **Negative controls** (`onto controls`): each check runs on real data and on an injected defect and must go red on the defect — IsA cycle, `Strict` override, lemma ∈ synset, layer exceptions not leaking to `real`, unknown words, a shuffled hierarchy, the set hash, a corrupted snapshot, core completeness and the "core ↔ graph" gate.
- `onto bench` times build, snapshot load and queries; `onto stats` prints graph sizes.

## Sources and licenses

| source | version, file | license | what we take |
|---|---|---|---|
| Open English WordNet | 2025+ (`english-wordnet-2025-plus.xml.gz`; the base `english-wordnet-2025.xml.gz` — only for the "plus" bit) | CC BY 4.0: `Lexicon` header in both files. Commercial use — yes, with attribution | synsets, glosses, lemmas and sense order, IsA, InstanceOf, parts, members, substances, entails, causes |
| Berkeley FrameNet | 1.7 (`framenet_v17.zip` from nltk_data) | CC BY 3.0 per the nltk_data index; no text found on the ICSI site; FrameNet 1.5 in nltk_data is NC. **Check before commercial use** (`raw/en-framenet-1-7/SOURCES.md`) | frames, roles, lexical units, frame relations |
| Wikidata lexemes | dump 2026-09-23 (`wikidata-20260923-lexemes.json.bz2`) | CC0 | P5137 "item for this sense", P8814 "WordNet 3.1 synset", English glosses |
| Full Wikidata dump | `$MOVA_DATA/raw/wikidata-all/latest-all.json.bz2` | CC0 | → `$MOVA_DATA/db/wikidata.duckdb` (section [Wikidata: pass and database](#wikidata-pass-and-database)); the dump and the database are not kept locally, rebuilding needs a fresh copy of the dump |
| Seeds `seeds/` | our own | ours (closed part) | OEWN glosses are quoted with the attribution above |
| FairytaleQA, "The Jelly Fish and the Monkey" | texts — Project Gutenberg | public domain | one quote in seed d66 |

The papers Reiter 1980 ("A logic for default reasoning") and Reiter & Criscuolo 1981 (Nixon diamond) are only sources of ideas, not of data.

## Wikidata

### Lexemes and trial alignment (`onto wd-lexemes`, `onto wd-map`)

`wd-lexemes` collects lexeme statistics over all languages (senses, P5137, P8814). `wd-map` aligns English senses with P5137 to OEWN synsets: P8814 where present; otherwise an unambiguous match (one sense on both sides), or overlap of gloss words (at least 2 and ahead of the runner-up). Ambiguous senses get no guess; lemmas missing from OEWN and part-of-speech mismatches are reported. Keyword overlap can mislead (apple Q89 → apple tree via "tree" and "genus"), and "unambiguous" trusts a lexeme's single sense.

`onto wd-probe` reads the first N GB of the full dump and reports entity counts and throughput; its decompression is bzip2-bound, which is why the full pass uses `lbzip2`.

## Wikidata: pass and database

Wikidata is loaded into DuckDB; it also carries translations — how an article, or an apple, is rendered in different languages. One streaming pass over the full dump → derived TSV (zstd) `$MOVA_DATA/raw/wikidata-derived/` → a separate database `$MOVA_DATA/db/wikidata.duckdb` (the main `mova.duckdb` attaches it via `ATTACH … (READ_ONLY)`). The full dump and `wikidata.duckdb` are not kept locally: the paths below are where the commands write when rebuilt from a fresh copy of the dump.

**How:** dump copier (counts compressed bytes) → `lbzip2 -dc -n 8` → 8 MiB chunks → 2 parser threads (`Cow`, labels first as `RawValue`) → each writes its own `<table>-w<N>.tsv.zst`; `nice -n 10`, a progress log every minute. The bottleneck is bzip2. `wd-sample` estimates the budget from bzip2 blocks at arbitrary bit offsets, chunks spread evenly over the file (the start of the dump holds the most popular items, so a pilot on it overestimates).

| table | what |
|---|---|
| `items` | Q, labels en/mul/uk, descriptions en/uk, sitelinks, Wikipedias, label languages, `is_class` (P279 as subject or value), `is_taxon` |
| `labels` | labels and aliases in all languages — for classes, items with a sitelink, and taxa |
| `sitelinks` | article titles of language Wikipedias (without Commons, Species, Wikisource, Abstract Wikipedia; disambiguators in parentheses kept) |
| `claims(s, p, o, rank)` | P31, P279, P171, P361, P527, P186, P1034, P4733, P1582, P105, P1672, P2670, P3113, P462, P1419, P2974, P366, P1552, P3094 (why — `PROPS` in `src/wdx.rs`) |
| `strs`, `mono`, `quant` | P225, P2888, P1709, P5063 ILI, P8814; P1843 with language; mass, length, height, speed, lifespan — with unit and qualifiers |
| `props` | property names en/uk, type |
| `isa_edges`, `isa_cycles`, `closure` | P279 + P171 without cycle edges; closure — for nodes with a Wikipedia article |
| `lexemes`, `lemmas`, `forms`, `glosses` | lexemes, forms with grammatical features, senses with glosses |
| `sense_items`, `sense_links`, `sense_strs`, `lex_claims` | P5137 → Q; P5972 translation, P5973 synonym, P5974 antonym; P8814 |
| `synset_q` | OEWN synset ↔ Q with status and support (loose rule) |

Macros: `wd_label(q, lang)` (lang → mul → en), `wd_ancestors(q)`, `wd_descendants(q)`, `wd_isa(x, y)`.

**Design decisions:**
- **Wiki service pages** (categories, templates, disambiguations, modules, portals) — without labels in all languages and without sitelinks: they are about the wiki, not the world. The en label stays in `items`.
- **Scholarly articles** — in the database without titles and descriptions: the world model and translation don't need them, and they would push the database over its size budget. The texts remain in the derived files.
- **Label languages are not restricted.**
- **Closure** is materialized only for nodes with a Wikipedia article; the rest are reached via the recursive `wd_ancestors`.
- Large tables are loaded unsorted.

**Check queries** (`sql/wikidata-checks.sql`): apple ⊂ fruit ⊂ food with labels and article titles in several languages; whale ⊂ Mammalia, and also ⊂ fish through cladistics (Cetacea —P171…→ Osteichthyes —P279→ fish), which the walker's folk rule stops (section [A whale is not a fish](#a-whale-is-not-a-fish-srcwdwalkrs)); common names of a taxon in many languages.

**P279/P171 cycles:** the gate lists them; the edges are removed from the closure and recorded; after breaking the graph is a DAG. Negative control: an injected edge fruit ⊂ apple turns the gate red.

**Synset ↔ Q** (`wd-align`): candidates from lexemes (P5137), P5063 ILI, P8814 and English labels, accepted with an ancestor check. The run reports a silver score (direct P5063/P8814 versus choosing from lexemes and labels only) and a shuffled-labels control. This loose rule is the `synset_q` table of the database; the walker and the core use the strict rule (section [Strict synset ↔ Q alignment](#strict-synset--q-alignment-wd-align)).

**How to update:** new dump → `wd-extract` → `wd-lex` → `wd-senses` + `wd-map` → `wd-closure` → `wd-align` → `sql/wikidata-load.sql` and `sql/wikidata-closure.sql` → `sql/wikidata-checks.sql`. Commands — section [Commands](#commands).

**License:** Wikidata — CC0 1.0, both the full dump and lexemes: commercial use without restrictions, attribution not required, but the `wikidata` row is recorded in `en/data/train-licenses.tsv`. The dump lives in the geography project — only read, nothing written to its folder.

## Wikidata walker

A small walker goes through Wikidata and collects seed candidates for the core. Sounds (P4733) are not collected. Numbers are kept as distributions, not as one specific animal: a lion's mass comes from a normal distribution. The database and derived files are read-only.

### A whale is not a fish (`src/wdwalk.rs`)

Wikidata cladistics leads the whale into fish: whale —P279→ Cetacea —P171…→ mammal [class] —P171…→ Sarcopterygii [class] → Osteichthyes [superclass] —P279→ fish. **Folk "is a kind of":**
- a P171 chain does not climb from a taxon of rank "class" (P105 = Q37517) or higher (superclass, phylum, kingdom…);
- exception — one step class → superclass: Actinopterygii → Osteichthyes —P279→ fish, otherwise a pike is not a fish;
- bypassing clades (P171 past a class, to a higher class) are discarded: from the P171 region only nodes from which the lowest class is reachable remain;
- P279 — unrestricted; shortest distance by relaxation (whale —P279→ Cetacea —P279→ mammal is 2 steps, much shorter than the P171 chain).

With the rule: whale → mammals yes, whale → fish no, crow → birds yes, shark → fish yes, bat → bird no, pike → fish yes. With full cladistics (the control) whale → fish goes back to yes.

- **Anchor taxon:** Q388450 "crow" is a common name (P31 Q55983715) with no path to birds in Wikidata. For animals without a class among their folk ancestors the walker adds an anchor taxon — the best taxon among alignment candidates with ancestor support (genus better than species): Corvus Q43365.
- **Cycle gate:** the "is a kind of" graph must be acyclic; an injected fruit ⊂ apple goes red.
- Tests on a synthetic graph: bypassing clade, grade, a shark does not climb out of Chondrichthyes, without the rule a whale is a fish.

### Strict synset ↔ Q alignment (`wd-align`)

The loose rule plus: a generality threshold for the common ancestor of **2%** (loose: 5%), and **two supporting sources** — label, lexeme, direct link (ILI and P8814 together count as one support, they are set together), folk ancestors. Ancestor support with a single source gets status `single`. Flags: `--generic`, `--min-src`, `--no-folk`.

- **The price of strictness:** general concepts (bird, mammal, fish, crow) lack support — their OEWN ancestors are too general (vertebrate, animal). The walker takes them with lower confidence: two supports without ancestors — 0.7.
- Gold — `data/align-gold.tsv` (100 pairs, +/− with explanations of errors); `wd-align` measures every run against it.
- Output — `synset_q-v2.tsv` in the run directory; the derived `synset_q.tsv` and the database keep the loose rule.

### Core walker (`onto wd-walk`)

- **Core** — `data/core.tsv`, 294 concepts: animals from fables and tales, food, household objects, family, roles, magical beings, plants. Synset — by domain (`noun.animal`, `noun.food`…), or explicit in the third column.
- **Anchors** in order of preference: strict alignment, two supports without consistent ancestors, ancestor support with one source, anchor taxon. A concept may have no anchor.
- **Pass:** statements of the anchor and folk ancestors — up to 3 steps, plus a class at any depth (bird, mammal, fish, insect).
  - Overly general ancestors (≥ 20 k descendants: product, physical object, food, organism, animal) — no inheritance.
  - A "part" that is a taxon or the concept itself (fox P527 red fox, Cetacea P527 whale) — discarded.
  - Confidence < 0.4 — discarded.
- **Numbers as distributions:** the anchor's own values, otherwise over descendants (P279/P171 down to 3 steps, ≤ 400): mean, σ, min, max, n, SI unit. A single value — "point estimate, σ unknown". Birth weight (P3831 = Q4128476) is not about adults and is discarded.
- **Output — `seeds/wd-candidates.tsv`.** Columns: concept, domain, synset, property, value, source (Q, P, rank, anchor and steps), confidence, mean, sd, min, max, n, unit. Properties: part (P527, P2670), use (P366), food (P1034), material (P186), color (P462), no_part (P3113), habitat (P2974), mass · height · length · lifespan · speed (P2067 · P2048 · P2043 · P2250 · P2052). These are **candidates, not truth**: they enter the core only after confirmation.

Confidence = anchor (strict 0.9 · two supports 0.7 · one support or anchor taxon 0.6) × statement rank (preferred 1 · normal 0.9) × 0.85^steps to the ancestor. Numbers: own — ×0.95, distribution over descendants — ×0.7.

**Examples:** cat — part tail, claw, paw, tooth, snout; knife — part blade, handle, tang, use cutting; bread — material flour, water, salt; lion — food ungulate.

### LLM: a second opinion (`onto wd-confirm`)

The LLM (the teacher) runs with no tools and with a call cap; log `vmm-calls.jsonl`. The answer is "yes / no / refine + why". The criterion is the typical instance, default knowledge. Disputed candidates (confidence between `--lo` and `--hi`) go to `seeds/wd-confirm.tsv`.

Systematic candidate errors the LLM flags: no_part "human" for mammals (P3113 of a general class); material "human food" / "food ingredient" (P186); "parts" that are not parts (P527); P1034 as all possible prey rather than typical food; colours from a different sense (spade — black, from the card suit).

### Character — only in fables

At the cost of 5 `Prop` values (`Cunning Vain Greedy Industrious Wise`, preference `noun.animal`) — seeds on layer `fable` (d67–d73): the fox is cunning, the crow vain (Perry 124), the wolf greedy (Perry 156), the ant industrious, the grasshopper not (Perry 373), the owl wise (Perry 437), the donkey not (Perry 188). The real layer is silent about character: `prop fox cunning` → "unknown", with `--layer fable` → "yes".

## Compiled core, context-based sense selection, description

Well-known standard animals and objects live a compiled life, not as a dynamic class ([docs/context.md, "World model — separate from the narrative"](../docs/context.md#world-model--separate-from-the-narrative)).

### Core in code (`build.rs`, `src/core.rs`, `src/coresel.rs`, `src/coregate.rs`)

- **Selection** (`onto core-select` → `data/core-props.tsv`): the core gets what the LLM confirmed, and confident (≥ 0.7) items without an LLM "no"; the rest stay candidates.
  - `onto core-vmm confirm` sends every candidate without a verdict to the teacher, disputed and confident alike. Color and "no part" are not checked — they are not in the core.
  - `onto core-vmm risks`: action → harm → probability per contact, order of magnitude from the LLM, for every core concept (lion: pet → bite).
- **Explicit synsets** (third column of `data/core.tsv`; gate: the lemma is a member of the synset, the synset is in the domain's lexfile) where the first sense in the domain's lexfile is wrong: dolphin — dolphinfish, oak and birch — wood, net — the internet, bow — a bow knot, peacock — a butterfly, hedgehog — porcupine, castle — palace, hut — a military barracks.
- **`build.rs`** (the build script in the crate root; `src/build.rs` is the graph build, a different thing) generates `enum CoreId` (one variant per concept; a duplicate lemma gets its domain: `FishAnimal`, `FishFood`) and `static CORE: [CoreRec; N]`:
  - food, habitat, parts, material, use — sets of `Val`: Q, label, OEWN synset of the value (strict alignment), a link to a `CoreId` if the value is also a core concept;
  - mass, height, length, lifespan, speed — `Dist {mean, sd, min, max, n, unit}`;
  - risks — `Risk {act, harm, p}`;
  - every value has a `Source`: Q, P, rank, steps from the anchor, confidence, confirmation (LLM "yes", refinement with a note, LLM estimate).
- **Completeness gate.** Every "concept × field" cell is explicit: a value, "no risks", "unknown" with a reason, or "not applicable" by domain (an animal has no material, a table does not eat). A deleted cell makes the build fail: `completeness gate: concept «lion (animal)» has no field mass — neither values nor an explicit «unknown»`. The same code (`src/coregate.rs`) is checked by `onto controls`. The first build without `core-props.tsv` — `ONTO_CORE_BOOTSTRAP=1`.
- **"Core ↔ graph" gate** (`core::check`, in `load_kb`): the synset is in the graph and the lemma is its member; graph seeds on the real layer do not contradict the domain (an animal is not `Animate=no`, food is not `Edible=no`, an object is not `Living=yes`); a core part or material is neither the concept itself nor its ancestor. A contradiction refuses the load and shows both sides, rather than silently picking one. A lexfile outside the domain (nut, acorn — noun.plant) is only a warning. `ONTO_CORE_GATE=off` turns refusals into warnings for diagnostics.
- **Query order — core first, then graph:**
  - `isa`, `haspart` — sense candidates: the query context's lexfile, the first sense, the core concept. They are ranked by a sentence-free WSD score: core 0.5 + corpus 1.5 × share + query lexfile 1; those within 0.3 of the best remain;
  - `prop` — in the preference lexfile, core concepts first;
  - `describe` — core fields, then the graph.

### Word sense selection by context — WSD (`src/wsd.rs`)

A sense's score is a sum of terms, each visible in the explanation (`onto wsd mouse "The cat caught a mouse"`):
1. **Extended Lesk** (Banerjee & Pedersen 2003): words shared between the sentence and the glosses and lemmas of the synset (weight 1) and of its neighbours — hypernyms up to 2 steps, hyponyms, parts, whole, substances (weight ½); IDF weights over OEWN glosses.
2. **Predominant sense from a corpus** — McCarthy et al. 2004 (ACL P04-1036), `onto wsd-prior`:
   - a word's distributional neighbours — from within-sentence co-occurrences in the Leipzig corpora: eng_wikipedia_2016_1M, eng_news_2024_1M, `co_s`, significance, top-200 contexts, cosine;
   - neighbours vote for senses close in the graph: Wu–Palmer similarity to the 8th power (McCarthy used the sharp jcn);
   - output → `$MOVA_DATA/onto/sense-prior.tsv` (`ONTO_PRIOR`).
3. **Core:** +0.5 for a sense that is a core concept.
4. **Domain:** the share of sentence words whose senses (ranks 1, ½, ⅓) lie in the sense's lexfile; senses vote by rank.

The weights are fixed in advance. `onto wsd-eval` runs the WSD frozen set (`data/wsd-frozen.tsv` + `.sha256`: sentences for ambiguous words such as dolphin, oak, bank, bass, mouse, cricket), reports each component alone and each ablation, and a negative control: shuffled glosses (a sense's signature taken from a random synset) must drop to the first-sense baseline.

### Concept description: `onto describe`

`onto describe fox --layer fable [--no-core]` gives:
- the sense and why it was chosen;
- the IsA chain;
- parts, material, food, habitat, use (core, otherwise OEWN meronymy);
- sizes and mass as distributions;
- risks;
- seed properties, and on the fable layer — character.

Every row has a source.

`onto describe-eval` describes the classes of the tale and fable worlds "before" (graph and seeds, without the core) and "after" (the core, then the graph) and reports fields filled and "unknown" by category.

**Teacher judge** (`onto describe-judge`): scores all "before" and "after" descriptions blind — versions shuffled under random numbers — with a fixed rubric: C — correctness, F — completeness, U — usefulness, E — errors, each 1–5. Classes are split by name hash into a training and a held-out part. `describe-judge report` summarizes; a judge control re-scores a sample with a differently worded rubric.

**Architect** (`describe-judge architect`): sees only the training part of the judge's scores and proposes changes. Its proposals so far (not implemented):
1. word sense by score, not the first one: lemmatization of plurals (shoes → shoe); fairy-tale lexfiles (artifact, animal, plant, food, person) up, state and communication down; a threshold below which the LLM breaks ties;
2. parts and material only from physical ancestors and no further than 3 steps; general ancestors (whole, structure) give no parts; for magical creatures — a bridge to creature;
3. WordNet glosses into fields via templates ("made of", "used for", "having a … and a …", "feeds on"), with source gloss and lower confidence.

To compare rounds, the judge must use stable ids (class + version); currently scores depend on the shuffle of a single run.

### Pronoun API (`src/pron.rs`)

- `pron::info(kb, lemma, &Ctx {sentence, layer}) -> PronInfo {gender, animate, human, persona, why}`; wrappers — `gender`, `animate`, `human`; `PronInfo::pronouns()` — the allowed he / she / it.
- **Sense:** WSD by sentence, otherwise the core concept, otherwise the first sense.
- **Human and animate** — seeds `Human` / `Animate` on the layer.
- **Gender** — hints from the gloss and ancestor names: female / woman / queen… → fem., male / king / stallion… → masc.; both together — unknown.
- **Character:** an animal that talks on this layer (the fox in a fable) allows he / she / it.
- Only the interface and the `pronoun_api` test: crow — it, in a fable — he / she / it; Nixon — he or she; stone — it; unknown word — `None`.

## Independent tests

More tests for the common-sense builder. The sets are in `tests/data/vmm/` (LLM-authored) and in `…/ext/`, and the same files run on any build: `onto vmm <tag> [--haspart rows.json]` (the hash of every file is checked before the run; report — `vmm-<tag>.md` and `.tsv` in `ONTO_RUNS`).

### LLM questions

The teacher wrote the questions without seeing the code or the data (prompts — `tests/data/vmm/prompts/`); the gold standard is common sense and biology, and every question has an explanation. The set is frozen by hashes. Groups: `isa` with traps and made-up words; parts, materials, members; abilities, properties and `fable`/`fairy` layers; `measure X mass|height|length|speed|lifespan` — ranges, and made-up animals; `sense W | sentence` — lexicographer file and keyword. "Unknown" instead of "yes" for parts is expected: OEWN meronymy is incomplete, and we never say "no" about parts.

### From the internet: catalog

| set | where | license | use |
|---|---|---|---|
| hasPart KB (Bhakthavatsalam et al. 2020) | HF `community-datasets/has_part` | "unknown" on the card — measurement only, not in the repo | sample in `$MOVA_DATA/raw/tests-onto-ext/` |
| NumerSense ("a bird has two legs") | HF `INK-USC/numer_sense` | MIT | loaded by a script (data from GitHub): not downloaded |
| CommonsenseQA | HF `tau/commonsense_qa` | MIT | catalog (multiple-choice questions) |
| PIQA | HF `ybisk/piqa` | "unknown" on the card | catalog; measurement only |
| GenericsKB | HF `community-datasets/generics_kb` | CC BY 4.0 | catalog; a source of generalization seeds |
| DoQ (quantity distributions) | GitHub only | — | family rule: we do not access GitHub |
| Verb Physics (size comparisons) | GitHub only | — | same |

## HS: world model of goods

The goods classifier (HS, from the import-export project) is read into a word ontology, as for the tales. HS is a ready-made world model of goods ([docs/context.md, "Thousands of domains"](../docs/context.md#thousands-of-domains)): a tree, "others", features, thresholds.

**Data and license.**
- Source — `BACI/product_codes_HS22_V202601.csv` (BACI, read only, `ONTO_HS_CSV`): HS22 subheadings with English descriptions. BACI takes the descriptions from UN Comtrade.
- The description texts belong to the WCO, so they are not in the repo. `data/hs/` carries only derived data: code → concept, features, labels, with no description verbatim.
- Section and heading names are not available in our data: the trade data and the translation project have only HS6, and P5471 (HS 2022) is not extracted into `wikidata.duckdb`. So heading labels are ours: the most frequent head concepts of the heading.

### How it works (`src/hs.rs`, `src/hsmap.rs`)

1. **Tree:** chapters → headings → subheadings, with parents (`data/hs/tree.tsv`). "Others" are marked separately: by code (…9, …90), textual (n.e.c., other than), or both.
2. **Parsing.** `en` (`ONTO_EN_MODEL`) gives lemmas, parts of speech and the tree. The HS telegraphic style ("Fruit, edible: apples, fresh") is split by colon into levels (0 — the heading top) and by commas into pieces. A piece is either features from closed enums (enum `State`, `Process`, `Attr`) or a noun phrase. Rules:
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
   2. WSD with the heading context: Lesk, corpus, core, domain.
   3. Product state: live → animal or plant; fresh, frozen… → food (also accepts animal). If the longer phrase lacks such a sense, the shorter one is taken: fatty liver — a disease, liver — food.
   4. Chapter domain — in a second pass: the lexfile share of the chapter's head concepts, "one sense per text".
   5. Then — the core concept, and then a Q via strict alignment (`accepted`).
5. **LLM confirmation** (`onto hs confirm` → `data/hs/sense-confirm.tsv`).
   - Key — phrase, chapter and state class: live and frozen fish are not merged.
   - Disputed keys (margin below `--margin`) and the most frequent keys are sent; the LLM agrees, picks another sense, or says none fits.
   - WSD errors in frequent words are the most costly (rubber in chapter 40 — a condom, iron — a clothes iron), so frequent keys are confirmed first.

**Outputs:** `data/hs/feats.tsv` — features per subheading (state, processing, "of X" — material, source organism or other, form, purpose, parts, taxon, threshold, composition, exceptions); `data/hs/new-concepts.tsv` — phrases that are neither in the core nor in the graph; `hs-report.md` in the run directory.

**Gates and controls:**
- **IsA gate.** HS says "product ⊂ heading top" (apple ⊂ edible fruit). Verdicts: yes, yes via another sense, part, taxon heading, kind of the top's modifier ("Mollusc preparations: mussels"), sisters, made of the top's material, organism as source of the product, no (contradiction), unknown (not aligned). The negative control swaps products between headings. "Made of the material" and "source" are a type check via lexfiles, not a fact check, so they are soft labels.
- **Feature versus core and seeds:** HS features ("edible", "live") are checked against the core and the seeds; a contradiction is listed with both sides.
- **Neighbour thresholds:** subheadings of a heading with otherwise identical features must differ by a threshold; the control copies a neighbour's threshold and must go red.

**Blind teacher judge** (`onto hs judge`): a sample of subheadings by code hash across all chapters, plus decoys — the output of another code attached to a foreign description; everything shuffled. Rubric 1–5: M — head concept and its sense; F — completeness of features; P — precision. Decoys must score at the bottom, so the rubric does not praise foreign output.

**Labels in other languages** (`onto hs labels`, `sql/hs-labels.sql`): Q labels from Wikidata (CC0, `data/hs/labels.tsv`) are checked against the translation project's translations of HS names: does the stem of our concept's label occur in the subheading's translation. Mismatches are typically taxa with a Latin label, families instead of species, or synonyms.

**`onto hs describe` examples:**

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

## Commands

```
export PATH=$HOME/.cargo/bin:$PATH
cargo build --release
../target/release/onto build                        # OEWN 2025+ and FrameNet 1.7 → $MOVA_DATA/onto/onto-v1.bin (ONTO_SNAPSHOT)
../target/release/onto isa apple fruit
../target/release/onto can crow speak --layer fable
../target/release/onto haspart bird beak · parts bird · frame buy · role buy Seller · inherits Commerce_buy Getting
../target/release/onto entails snore sleep · word whale · up crow
../target/release/onto eval blind                   # frozen set → frozen-blind.{md,tsv} in ONTO_RUNS (default $MOVA_DATA/runs/onto-2026-09-26)
../target/release/onto controls | bridge | bench | stats
../target/release/onto wd-lexemes $MOVA_DATA/raw/wikidata/20260923/wikidata-20260923-lexemes.json.bz2 <dir>
../target/release/onto wd-map <dir>
nice ../target/release/onto wd-probe $MOVA_DATA/raw/wikidata-all/latest-all.json.bz2 1 <dir>
# Wikidata pass and database (needs a fresh copy of the dump; runs — ONTO_WD_RUNS, default $MOVA_DATA/runs/wikidata-2026-09-26)
D=$MOVA_DATA/raw/wikidata-derived
nice -n 10 ../target/release/onto wd-sample $MOVA_DATA/raw/wikidata-all/latest-all.json.bz2 <dir> --k 48 --mb 12 --level 9 --skip-wm
nice -n 10 ../target/release/onto wd-extract $MOVA_DATA/raw/wikidata-all/latest-all.json.bz2 $D --bz 8 --workers 2 --level 9 --skip-wm [--gb 2]
nice -n 10 ../target/release/onto wd-lex $MOVA_DATA/raw/wikidata/20260923/wikidata-20260923-lexemes.json.bz2 $D
../target/release/onto wd-senses $D <run>/wd-senses-en.tsv && ../target/release/onto wd-map <run>
nice -n 10 ../target/release/onto wd-closure $D --threads 8
nice -n 10 ../target/release/onto wd-align $D --wd-map <run>/wd-map.tsv [--generic 0.02] [--min-src 2] [--no-folk] [--out synset_q.tsv]
# walker (runs — ONTO_WD_RUNS=$MOVA_DATA/runs/onto2-2026-09-26)
nice -n 10 ../target/release/onto wd-walk $D [--align <run>/synset_q-v2.tsv] [--core data/core.tsv] [--out seeds/wd-candidates.tsv]
../target/release/onto wd-confirm [--n 60] [--batch 30] [--cap 10] [--lo 0.45 --hi 0.8] [--out seeds/wd-confirm.tsv]   # LLM
../target/release/onto prop fox cunning --layer fable
(cd $D && ~/bin/duckdb $MOVA_DATA/db/wikidata.duckdb < <repo>/onto/sql/wikidata-load.sql)
~/bin/duckdb -readonly $MOVA_DATA/db/wikidata.duckdb < sql/wikidata-checks.sql
# core, WSD, description (runs — ONTO_RUNS / ONTO_WD_RUNS=$MOVA_DATA/runs/onto3-2026-09-26)
../target/release/onto core-vmm confirm [--batch 50] [--cap 7]      # LLM goes through candidates without a verdict
../target/release/onto core-vmm risks [--batch 50] [--cap 8]        # risks of core concepts
../target/release/onto core-select                                  # → data/core-props.tsv; then cargo build (completeness gate)
../target/release/onto wsd-prior                                    # predominant senses from the Leipzig corpora → $MOVA_DATA/onto/sense-prior.tsv
../target/release/onto wsd mouse "The cat caught a mouse in the kitchen"
../target/release/onto wsd-eval                                     # WSD frozen set + shuffled-gloss control
../target/release/onto describe fox --layer fable [--no-core]
../target/release/onto describe-eval                                # tale classes before/after the core, completeness
../target/release/onto describe-judge [--cap 15] | describe-judge report   # blind teacher judge
# HS (runs — ONTO_RUNS, default $MOVA_DATA/runs/hsonto-2026-09-26)
../target/release/onto hs build [--no-confirm]                      # HS22 (BACI) → data/hs/*.tsv, report hs-report.md
../target/release/onto hs describe 080810 [--text]                  # product description; --text — WCO description from a local file
../target/release/onto hs parse 030213                              # parsing debug (tokens and features)
../target/release/onto hs confirm [--n 240] [--margin 0.15]         # LLM confirms disputed and frequent senses
../target/release/onto hs judge [--n 150] [--decoys 20]             # blind teacher judge, decoys as control
(cd $ONTO_RUNS && ~/bin/duckdb -readonly $MOVA_DATA/db/wikidata.duckdb < <repo>/onto/sql/hs-labels.sql) && ../target/release/onto hs labels
cargo test --release                               # world model, Wikidata, folk rule, anchors, LLM, core, pronouns, HS
```

Variables: `ONTO_SNAPSHOT` (graph snapshot), `ONTO_RUNS` and `ONTO_WD_RUNS` (run folders), `ONTO_PRIOR` (sense prior, default `$MOVA_DATA/onto/sense-prior.tsv`), `ONTO_EN_MODEL` (default `$MOVA_DATA/en/models/en.bin`), `ONTO_HS_CSV` (the BACI HS22 product-codes CSV), `ONTO_MODEL` and `ONTO_EFFORT` (teacher model and effort for the review steps; the teacher is the shared Codex CLI transport [`mova-teacher`](../docs/training.md), effort `medium` by default), `ONTO_VMM_THREADS` (teacher threads, 1 by default, at most 2), `ONTO_CORE_BOOTSTRAP=1` (build time: first build without `core-props.tsv`), `ONTO_CORE_GATE=off` (diagnostics only: warn instead of failing on core contradictions).

## Limits

- **Core and WSD:** predominant senses come from news and Wikipedia, not from a fairy-tale corpus, so news senses can win in tale sentences (mouse as the device, bass as the instrument, cricket as the game); a fairy-tale corpus or the text's domain is not used. Numbers for everyday concepts are scarce: Wikidata has them mostly for species. The architect proposals (word sense by score, physical ancestors for parts, glosses → fields) are not implemented. Pronouns build one `Wsd` per call with a sentence, not per batch.
- **Word sense from the text:** WSD chooses by sentence; "one sense per text" exists only as a hook, and there is no second frequency vote (the sense order of Wikidata lexemes and our corpora).
- **Wikidata in the core:** what `build.rs` stitches into the core has no human confirmation; the systematic candidate errors flagged by the LLM (P3113 of general classes, P186 "human food", P527 not about parts, P1034 as all prey rather than typical food) are not filtered; general concepts (bird, mammal) lack support via P8814/ILI without ancestors.
- **OEWN gaps:** meronymy is incomplete (parts answer "unknown" rather than "no"); some everyday category questions follow OEWN's taxonomy rather than common sense (tomato is not a fruit, moss is outside "plant"); part inheritance from general ancestors can be too generous.
- **Seeds** are not derived from OEWN glosses ("flightless", "edible", "swift") automatically; insects have no `CanFly`, "human" has no human seed, and the `fairy` layer has no "animals talk" seed.
- **No selectional preferences** from FrameNet roles (role semTypes) and IsA, so no arc plausibility for a second parsing pass ([docs/context.md, "Multi-pass parsing"](../docs/context.md#multi-pass-parsing)).
- **World bridge:** the LLM writes an entity's class as a word, not as a synset id; the bridge reads the commands, not the world state.
- **HS:** a phrase can fall back to a more general one (dairy produce → produce) or take a wrong sense of a frequent word; some contradictions are genuine HS versus WordNet differences.
- **No reasoning module** (Horn clauses over the TBox and the ABox with proofs) and no size, quantity or comparison reasoning (a lion is big, a hare is faster than a tortoise).

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
| `src/wikidata.rs` | full dump probe, lexemes, trial alignment |
| `src/wdx.rs` | pass over the full dump: lbzip2 → multi-threaded parsing → TSV zstd; sampling from the middle of the dump |
| `src/wdlex.rs` | lexemes → lemmas, forms, senses, translations; `wd-senses` for `wd-map` |
| `src/wdgraph.rs` | "is a kind of" graph (P279 + P171), cycle gate, closure |
| `src/wdalign.rs` | synset ↔ Q alignment with ancestor check, shuffled control, silver; strict rule — two supports, 2%, folk ancestors, gold |
| `src/wdwalk.rs` | folk "is a kind of" (P171 up to class, grade, bypassing clades), core walker, anchors, number distributions, check queries |
| `src/wdvmm.rs` | LLM — "yes / no / refine", cap and log |
| `sql/wikidata-load.sql`, `sql/wikidata-closure.sql` | loading derived files into `$MOVA_DATA/db/wikidata.duckdb`, macros |
| `sql/wikidata-checks.sql`, `sql/wikidata-core-coverage.sql` | check queries; core coverage of fable classes |
| `build.rs` | build script — core `data/core.tsv` + `data/core-props.tsv` → `enum CoreId`, `CORE` (completeness gate) |
| `src/core.rs`, `src/coregate.rs`, `src/coresel.rs`, `src/corevmm.rs` | core types and the "core ↔ graph" gate; completeness gate; selection; LLM (confirmation pass, risks) |
| `src/wsd.rs` | WSD — Lesk, McCarthy 2004 from the Leipzig corpora, core, domain; shuffled-gloss control |
| `src/describe.rs`, `src/judge.rs` | concept description with proofs, completeness; blind teacher judge |
| `src/pron.rs` | pronoun API — gender, animacy, humanness |
| `src/hs.rs`, `src/hsmap.rs` | HS: code tree, description parsing (`en`), features and thresholds (`math`); alignment with the world model, gates, controls, `hs describe`, judge, LLM confirmation, labels |
| `data/hs/*.tsv` | HS: derived data only — tree, code → concept, features, terms, new concepts, contradictions, LLM confirmations, Wikidata labels (CC0); no WCO texts |
| `sql/hs-labels.sql` | HS: concept labels in eight languages from `wikidata.duckdb` |
| `data/core-props.tsv` | core — every "concept × field" cell explicit, with a source |
| `data/wsd-frozen.tsv` + `.sha256` | WSD frozen set (60 sentences) |
| `seeds/wd-confirm-high.tsv`, `seeds/wd-confirm-v3.tsv`, `seeds/core-risks.tsv` | LLM verdicts: confident sample, confirmation pass; risks of core concepts |
| `src/tests.rs` | tests on a synthetic graph |
| `seeds/layers.tsv`, `seeds/defaults.tsv`, `seeds/relations.tsv` | seeds (including the character seeds on layer `fable`) |
| `seeds/wd-candidates.tsv` | seed candidates from Wikidata, with source, confidence and distributions; not truth |
| `seeds/wd-confirm.tsv` | LLM second opinion on disputed candidates |
| `data/core.tsv` | core — 294 concepts (lemma, domain, optional explicit synset) |
| `data/align-gold.tsv` | 100 synset ↔ Q pairs checked by hand (loose- and strict-rule samples) |
| `data/frozen.tsv` + `.sha256` | frozen set (74 questions) |
| `data/fable-questions.tsv` + `.sha256` | fable questions (14) |
