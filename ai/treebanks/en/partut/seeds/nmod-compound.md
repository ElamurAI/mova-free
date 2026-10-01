# ParTUT — a noun before a noun — `nmod`, not `compound`

**Gist.** Italian has no compounds like *law firm*: a noun modifier always has a preposition (*studio di avvocati*). The TUT scheme from which ParTUT was converted annotates such a modifier as `nmod`, and the English part kept it that way. So:
- *law firm*, *guidance programmes*, *quota penalties* — the left noun is `nmod` → the right one, without `case`;
- `compound` in ParTUT — 124 per 49,478 words (0.25%). In EWT — 9047 per 254,820 (3.6%).

EWT and `mova`: a noun before a head noun is `compound`.

**Conditions and exceptions.**
- Adjacent NOUN, the left one depends on the right one:
  - `nmod` 797;
  - `compound` 61;
  - `nmod:desc` 45;
  - `obj` 15;
  - `amod` 4.

  In EWT: `compound` 4750, `nmod:unmarked` 16, `nmod:poss` 15.
- `nmod:unmarked` in ParTUT — 194: some such modifiers already have the subtype, but not all.
- For proper nouns ParTUT uses `flat` (441) and `flat:foreign` (88).

**Examples.**
- `en_partut-ud-2` *Creative Commons Corporation is not a law firm…* — *law*: `nmod` → *firm*. In EWT — `compound`.
- `en_partut-ud-85` *The Cunha report on multiannual guidance programmes…* — *guidance*: `nmod` → *programmes*.

**In UD.**

| rule | ParTUT | CTeTex | EWT |
|---|---:|---:|---:|
| `tb.en.nn-compound`: adjacent noun modifier not `compound` | 862 / 974 | 314 / 726 | 19 / 5376 |

For LAS without subtypes, `nmod` versus `compound` are different labels. Merging ParTUT without a converter will teach the parser to put `nmod` on *law firm*.

**Sources.** Sanguinetti & Bosco 2014 (conversion of ParTUT to USD, CLiC-it 2014); Bosco et al. 2013, `W13-2308` (Italian treebanks → Stanford Dependencies); https://universaldependencies.org/en/dep/compound.html; README `UD_English-ParTUT`.
