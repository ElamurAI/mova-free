# Pronouns — known annotation errors

**Gist.** Pronouns was annotated manually by one person and constructed so that each sentence is repeated five times. So each error is also multiplied by five. The converter does not fix them.

**Examples and numbers.**
- ***is* with `Number=Sing` without `Person`** — 10 (2 sentences × 5). The other *is* (30) have `Person=3`.
- ***was* with `Person=1`** — 5: one sentence whose subject is *I*. This is agreement with the subject, as in EWT, so it is an inconsistency within Pronouns rather than an error.
- ***it* without `Gender=Neut` and without `Case`** — 40 of 40. EWT: `Case=Nom|Gender=Neut|…`.
- **Adjectives without `Degree`** (*fresh*, *nice*) — 15 of 20. The other 5 have `Degree=Pos`.

**In UD.** The errors and old customs of Pronouns repeat fivefold. As a FEATS test it shows not quality but the difference in norms, until there is a reverse converter (`../convert.md`). As a test of UPOS and lemmas of independent possessives it is accurate, and that is what it was created for.

**Sources.** README `UD_English-Pronouns`; rules in `../../seeds-overview.md`.
