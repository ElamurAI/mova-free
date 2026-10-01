# CTeTex — vertical lists: `parataxis` to the first item, `list` between items, the number is `nummod`, the break is `LineAfter`

**Gist.** Requirements often look like "introduction: + a list of items on new lines". UD does not say how to attach the list to the introduction, so CTeTex has its own custom (Hassert et al. 2021, sec. 4.3):
- if the introduction is a full sentence, the first item attaches to its predicate as `parataxis`;
- if the introduction is incomplete (*…has the following:*), the relation is the one it would be without the colon, most often `obj`;
- items among themselves — `list`;
- multi-level lists — `list` inside an item.

Line breaks are preserved by `LineAfter=Yes` in MISC. EWT also uses `list` (1061, mostly addresses and signatures) and `parataxis` for such structures, but has no `LineAfter`.

**Conditions and exceptions.**
- `list` — 116: on NOUN 90, VERB 12, SYM 6, ADJ 5.
- `parataxis` — 127: NOUN 64, VERB 25, ADV 17, ADJ 11.
- `LineAfter=Yes` — 175. `# text` does not preserve lines: the list is written on one line.
- Item numbers (*1* + *.*) — NUM `nummod` on the item's predicate: 25 times on a verb, adjective or adverb (`tb.ctetex.enum-nummod`). CTeTex has not a single `discourse`. EWT since 2.14 (#518) attaches numbering markers as `discourse` (LS — 113); `nummod` on a non-noun there — 2, in GUM — 7. UD 2.14 says directly that `nummod` for numbering markers contradicts its definition (UD docs/changes.md` No. 14).

**Examples.** `112` *The TCS shall, as a minimum, have the functionality to provide the following control capabilities: 1. Send and receive tactical communication messages. [SSS222]* — *Send*: `parataxis`; *1*: NUM `nummod` → *Send*; after *:* and *]* — `LineAfter=Yes`.

**In UD.** For `mova` the customs with `list` and `parataxis` do not contradict EWT: the labels are the same, only the frequency differs. The converter does not touch them, but moves numbering markers to `discourse`. `LineAfter` is kept in MISC as is. It is useful for a text generator, because it shows where the document had a line break.

**Sources.** Hassert et al. 2021, `2021.udw-1.5`, sec. 4.3 («Lists and Enumerations», figs. 6–8); README `UD_English-CTeTex` (Specificities: `LineAfter`); https://universaldependencies.org/u/dep/list.html, https://universaldependencies.org/u/dep/parataxis.html.

```rule
rule: tb.ctetex.enum-nummod
what: list item number — nummod on the predicate (EWT 2.14: discourse)
match: h[upos=VERB|ADJ|ADV|AUX]; n[upos=NUM|X, rel=nummod, head=h, before=h]
require: n[rel=discourse]
severity: warn
source: UD docs/changes.md No. 14 (List Item Markers, 2.14); EWT README v2.14 (#518); counter
```
