# nummod — only quantity and only before the noun

**Gist.** `nummod` is a numeral that says "how many": *3 sheep*, *forty dollars*. A number after a noun is not a quantity but an identifier: *page 394*, *Route 66*, *World War II*. Per the UD 2.18 guideline this is `flat` from the generic word. So `nummod` after a noun is an error, as is `nummod` on a word that is not a numeral (*several, many* are `amod` or `det`).

**Conditions and exceptions.**
- Amounts: in *$ 40* the number stands after the $ symbol (SYM) and legitimately has `nummod($, 40)`. The rule takes only NOUN and PROPN heads.
- A phone number after *number:* is `appos`.
- Dates and addresses are `nmod:unmarked`.

**Examples.**
- *Sam ate 3 sheep* → nummod(sheep, 3).
- *page 394* → flat(page, 394).
- *door number 3* → flat(door, number), flat(number, 3).

**Treebank conventions.**
- Before UD 2.11 EWT annotated *Page 3* as `nummod`, and GUM as `dep`; in GUM `nummod` was used only for counting (*3 pages*).
- De Marneffe et al. 2021 recommended `nmod`.
- The current guideline is `flat`.
- Numbers are unstable for parsers: merely replacing the year in a sentence changes the parse in about 44 % of variant batches.

**Check against gold.** EWT 2.18: `nummod` before NOUN/PROPN — 1433 of 1433; `nummod` on NUM — 1794 of 1794. GUM: 3 and 2 violations.

**Sources.** UD `_en/dep/nummod.md`, `_en/dep/nmod-desc.md` (Numbered Entities, note 1); `2023.udw-1.7` (appendix: Page 3 — dep in GUM, nummod in EWT); `2021.udw-1.14`; `2021.udw-1.8` (Kalpakchi, Boye).

The rule is `en.nominal.nummod-before-noun` in `nominal/nummod.md`.

The rule is `en.nominal.nummod-num` in `nominal/nummod.md`.
