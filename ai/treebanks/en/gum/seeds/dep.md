# GUM — `dep` where EWT has a specific label

**Gist.** `dep` in UD is an "unspecified dependency": a last resort. EWT uses it 5 times per 254,820 words, GUM 184 per 256,739. Most GUM uses are three stable customs, each of which EWT annotates differently:
1. ***you guys, you people, you idiot*** — 56. GUM: *you* — `dep` → the noun, the head is the noun. EWT since 2.17 (#436): the head is *you*, the noun is `nmod:unmarked` → *you*. Not only the label changes, but also the direction.
2. **Photo captions and credits**: *Image: David Titley.*, *(credit: Ken Thomas)*, *(copyright Frank Messina…)* — 58 `dep` on *image/credit/copyright/photo/source*. EWT has 4 such captions, and none is `dep`.
3. **Numbers and symbols that do not count**: item numbers *1.*, *2.*, footnotes *[1]*, *ISO 639-3*, *Page 3*. GUM gives them `dep`, and `nummod` only to real counting (*3 pages*) — 24 times NUM/X/SYM. EWT uses `nummod` or `flat` with `FlatType=Enumerated` (2.16). List numbering markers are attached by EWT as `discourse` since 2.14 (#518).

**Conditions and exceptions.**
- The remaining 46 `dep` are isolated cases: *Pursuant to…*, *drum drumming* etc.
- GUM 2025-11 moved footnotes and citations from `dep` to `parataxis`. Some footnotes *[1]* remained `dep`: `GUM_bio_padalecki-2`.
- The GUM family: GENTLE has 96 `dep` per 17,799 words, of them NUM/X/SYM — 16; GUMReddit — 12.

**Examples.**
- `GUM_conversation_grounded-5` *You guys are always in trouble.* — *You*: `dep` → *guys*. In EWT *guys*: `nmod:unmarked` → *You*.
- `GUM_interview_cyclone-3` *Image: David Titley.* — *David*: `dep` → *Image*.
- `GUM_letter_flood-36` *2. Legislation to increase…* — *2.*: `dep` → *Legislation*. In EWT — `discourse`.

**In UD.**

| rule | GUM | EWT |
|---|---:|---:|
| `dep` in total (`grep`) | 184 | 5 |
| `tb.gum.you-dep`: *you* + noun, *you* — `dep` | 56 / 56 | 0 / 0 |
| `tb.gum.credit-dep`: `dep` on *image, credit…* | 58 / 58 | 0 / 0 |
| `tb.gum.num-dep`: `dep` on NUM/X/SYM | 24 / 24 | 2 / 2 |

Merging without a converter will teach the parser `dep` on *you guys* — a label absent from the EWT test.

**Sources.** https://universaldependencies.org/u/dep/dep.html; EWT README v2.14 (#518 numbering markers — `discourse`), v2.16 (numbered entities, `nmod:desc`), v2.17 (#436 *you guys* — `nmod:unmarked`); README `UD_English-GUM`, changelog 2025-11-01 («Changed dep to parataxis for footnotes and citations»); Zeldes & Schneider 2023, `2023.udw-1.7`, sec. 5 (*Page 3*: `dep` in GUM, `nummod` in EWT).

```rule
rule: tb.gum.you-dep
what: you guys — you as dep of the noun (EWT: guys as nmod:unmarked of you)
match: h[upos=NOUN|ADJ|PROPN]; y[form=you, rel=dep, head=h]
require: not y[rel=dep]
severity: warn
source: EWT README v2.17 (#436); counter
```

```rule
rule: tb.gum.credit-dep
what: caption Image:/credit: X — X as dep
match: h[form=image|credit|credits|copyright|photo|source]; d[rel=dep, head=h]
require: not d[rel=dep]
severity: warn
source: UD dep; counter
```

```rule
rule: tb.gum.num-dep
what: dep on a number or symbol (item number, footnote) — EWT: nummod, flat or discourse
match: h[]; d[rel=dep, head=h, upos=NUM|X|SYM]
require: not d[rel=dep]
severity: warn
source: 2023.udw-1.7; EWT README v2.14, v2.16; counter
```
