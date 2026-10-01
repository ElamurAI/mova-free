# ATIS — compound names: the head is the first word, the rest `flat`

**Gist.** ATIS annotates names of cities, airports and airlines the same way: the first word is the head, the following ones are `flat` to it (*san* ← *francisco*, *new* ← *york*). *New* in *new york* is PROPN and the head. EWT distinguishes name types:
- person names — `flat` with a left head;
- structured names — `compound` with a right head (*San* → *Francisco*);
- *New* in *New York* since 2.8 — ADJ `amod`;
- since 2.17 names like *Mount Fuji*, *Fort Worth* — `flat` (#595).

**Conditions and exceptions.**
- Of 3373 pairs of adjacent PROPN in ATIS, 2829 are a left head with `flat`, only 27 are `compound` to the right.
- In EWT, of 3741 pairs: `flat` — 1500, `compound` — 1326. In GUM, of 3664 pairs: 1437 and 1298.
- The prefixes *new, san, los, salt, las, st., fort* before PROPN: in ATIS 1861 of 1865 are neither `amod` nor `compound`. In EWT — 67 of 191.
- `compound` exists in ATIS (2377), but mostly between common nouns: *round trip*, *ground transportation*, *twenty one*.

**Examples.**
- `0003.dev` *…from chicago to detroit…* — single-word names, no difference.
- `0007.dev` *could i have listings of flights from new york to montreal canada…* — *new*: PROPN (lemma *New*), head; *york* — `flat` → *new*. In EWT *New* is ADJ `amod` → *York*. *canada* here is `appos` → *montreal*; since 2.17 EWT puts `nmod:unmarked` + `Superlocation=Yes` on such places (#588).

**In UD.** The rule `tb.atis.name-flat` below counts PROPN pairs with a left `flat` head: ATIS 2829 / 3373, EWT 1500 / 3741, GUM 1437 / 3664. Merging ATIS with EWT without a converter will teach the parser that *new* in *new york* is a PROPN head.

**Sources.** EWT README v2.8 (ADJ/VERB in proper names), v2.15 (#81: foreign names `compound` → `flat`), v2.17 (#595 `flat` for place names, #588 `Superlocation`); Zeldes & Schneider 2023, `2023.udw-1.7`, sec. 5 (*Sri Lanka*: `compound` to the right in EWT, `flat` to the left in GUM); https://universaldependencies.org/en/dep/flat.html, `dep/compound.md`.

```rule
rule: tb.atis.name-flat
what: two adjacent PROPN — left head with flat (ATIS annotates all compound names this way)
match: a[upos=PROPN]; b[upos=PROPN, next=a]
require: not b[rel=flat, head=a]
severity: warn
source: UD en flat, compound; counter
```
