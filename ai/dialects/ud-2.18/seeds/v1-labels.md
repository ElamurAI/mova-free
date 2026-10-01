# UD v1 labels in the examples of `specific-syntax.md`

**Gist.** The examples in `specific-syntax.md` are still written with UD v1 labels: `dobj`, `nsubjpass`, `auxpass`, `csubjpass`, `neg`, `mwe`. They do not exist in v2, the validator rejects them, so there are 0 of them in the EWT data. The analyses themselves are correct; only the label names are outdated.

**Guideline.** In the `sdparse` blocks of `2.18:https://universaldependencies.org/en/specific-syntax.html there are 36 lines with v1 labels (grep): `dobj` 20, `mwe` 5, `neg` 4, `auxpass` 3, `nsubjpass` 3, `csubjpass` 1. The text also has v1 names, for example `:96` (*nsubjpass*, *csubjpass*).

**v1 → v2 mapping** (`docs/_v2/summary.md:74–100`):
- `dobj` → `obj`;
- `nsubjpass` → `nsubj:pass`, `csubjpass` → `csubj:pass`;
- `auxpass` → `aux:pass`;
- `neg` → `advmod` (in English — with `Polarity=Neg` on *not*);
- `mwe` → `fixed`.

**EWT 2.18 data.** Not a single v1 label (the registry `en/data/ud-registry-en.tsv` does not have them; the `en` reader does not know them).

**What this means for Mova.** Take examples from `specific-syntax.md` into seeds only with this replacement. They cannot serve as a corpus for checking.

**Sources.** `2.18:https://universaldependencies.org/en/specific-syntax.html; `docs/_v2/summary.md`; UD docs/workgroups/v1_to_v2.md`.
