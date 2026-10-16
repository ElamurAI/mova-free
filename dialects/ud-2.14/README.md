# ud-2.14 — English UD before 2.15

English UD in releases up to and including 2.14. Release 2.15 (November 2024) merged preposition-less nominal modifiers into one subtype `:unmarked`:
- `obl:tmod`, `obl:npmod` → `obl:unmarked`;
- `nmod:tmod`, `nmod:npmod` → `nmod:unmarked`.

Some 2.18 treebanks still have the old labels: ESLSpok (all 151), partly ATIS, CHILDES, LittlePrince. Third-party annotators were also trained on old releases.

`convert.md` — mova ↔ ud-2.14 rules and a check on ESLSpok and EWT r2.14. So far only `:tmod`/`:npmod` is described. The converter does not yet know other changes to English UD between 2.14 and 2.18.

Sources: English UD documentation — `_en/dep/obl-unmarked.md`, `nmod-unmarked.md` (History section), `obl-tmod.md`, `obl-npmod.md`, `nmod-tmod.md`, `nmod-npmod.md`. Snapshot — `$MOVA_DATA/raw/ud-docs/docs/_en/dep/`. Seed — `en/seeds/errors/tmod-npmod-unmarked.md`.
