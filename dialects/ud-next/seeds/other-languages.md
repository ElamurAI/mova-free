# Changes in other languages and the site (summary)

**Gist.** The remaining changes after 2.18 concern individual languages or site infrastructure. They do not affect English annotation, so they are collected in one seed. The comparison ignored the «Interlanguage links updated» lines: a script updates this timestamp in every language-guideline file (4680 files, among them 90 `_en` files).

**Level.** Language-specific and infrastructure.

**Languages** (`diff -rq`: files changed / added / removed):

| language | changes | gist |
|---|---|---|
| hy, hyw, axm (Armenian, Western Armenian, Middle Armenian) | 87 / 1 / 1; 12 / 93 / 0; 86 / 1 / 1 | gloss translations; `compound:redup` → `flat:redup` (new `dep/flat-redup.md`); full hyw guidelines (dep 50, feat 27, pos 16) |
| oge (Old Georgian) | 5 / 15 / 7 | layers `Case[sauf]`, `Case[dsauf]`, `Case[stack]`, `Case[stackb]`, `Number[sauf]`, `Number[dsauf]`; `compound:lvc`; `Polarity`, `Typo`; pages `X[y].md` → `X-y.md` |
| ka (Georgian) | 0 / 6 / 6 | only renaming of layered-feature pages `Number[io].md` → `Number-io.md` |
| lt (Lithuanian) | 8 / 0 / 0 | changes to the pages `acl:relcl`, `advcl`, `ccomp`, `conj` (new examples), `dep`, `reparandum`, `xcomp` and `index.md` |
| el (Modern Greek) | 6 / 0 / 0 | `compound` (reciprocal *ο ένας τον άλλον*), `dislocated`, `expl`, `PronType`, `VerbForm` (gerunds in *-οντας*), `VERB` |
| fr (French) | 1 / 3 / 0 | `discourse:filler`, `discourse:tag`, `conj:reform` — separate seed `fr-spoken-subtypes.md` |
| es (Spanish) | 1 / 1 / 0 | new `dep/dislocated.md`; updated `index.md` |
| egy (Egyptian) | 0 / 2 / 0 | `expl:rel` — an expletive pronoun in a relative clause, agreeing with the antecedent; `AdvType` |
| ru (Russian) | 1 / 0 / 0 | `ccomp` — new examples |
| ps (Pashto) | 3 / 2 / 1 | `Case`, description of the verbal system, feature table |
| ta (Tamil) | 1 / 6 / 0 | reference pages: features, relations, preprocessing, MWE |
| dar, pal, pi, bgs, mjl | new `dep`/`feat`/`pos` folders or `index.md` | first language guidelines |
| sah, hsb, nds, xpg | 1 each | minor changes (`Tense`, a `dep:alt` example, `index.md`) |
| new languages | `index.md` | ce, dta, eve, evn, gld, kfx, ks, mjg, mn, mnc, mvf, neg, oac, peh, sce, txg, ude, ulc, xal |

**Site and general pages** (do not affect annotation):
- `introduction.md` — rewritten introduction and a K'iche' example;
- `index.md`: release 2.18, a link to UD on Hugging Face, language-diversity charts;
- `download.md`: 2.19 — 15.11.2026, data freeze — 01.11.2026;
- `events.md`: UDW 2027 at SyntaxFest in Prague;
- `tools.md`: two new EUD tools — UD Enhanced Graph Visualiser and EUD Annotation for Portuguese (PR #1302);
- `infrastructure/*` — how the site and the release are built;
- `_data/*.yaml`, `_includes/*`, `_layouts/*`, `css`, `lib`, `flags`, `img` — site generation (#1277: accordions in plain JS);
- `languages.md` and `survey-scripts.md` — see `tools-scripts.md`.

**For Mova.** None of these changes alters English annotation. Potentially interesting for the future language-independent layer: reduplication as `flat:redup` rather than `compound:redup` (hy, axm), and `expl:rel` (egy).

**Sources.** `diff -rq -I 'Interlanguage links updated'` between `markdown-source/` of the 2.18 archive and `docs/` of the snapshot: 245 changed files, 186 only in the snapshot, 25 only in 2.18.
