# Validator data `tools/data/*.json`: English unchanged

**Gist.** From 2.18 to the snapshot, the validator data — permitted features, relations, auxiliaries and tokens with spaces — changed for 48 languages. 18 languages are new, 7 more got entries they did not have before (auxiliaries or documentation), 23 changed. English is not among them. Our registry `en/data/ud-registry-en.tsv` matches both 2.18 and the snapshot.

**Level.** Universal (validator infrastructure). Language entries are submitted via web forms `quest.ms.mff.cuni.cz/udvalidator/…`, and the system generates the JSON.

**Comparison.** Each JSON was expanded into "path → value" lines and compared with `diff`: `2.18:tools/data/*.json` ↔ `tools/data/*.json`.

| file | languages in 2.18 → in the snapshot | new languages | changed languages | en |
|---|---:|---:|---|---|
| `data.json` (auxiliaries) | 197 → 214 | 17 | ar, axm, cs, el, tn, uk | unchanged |
| `deprels.json` | 269 → 287 | 18 | axm, bgs, dar, egy, es, fa, fr, hy, hyw, ko, lij, mn, oge, pal, pi, uk | unchanged |
| `feats.json` | 269 → 287 | 18 | axm, bgs, ca, cs, dar, egy, el, ess, hi, hy, hyw, kbc, ko, lij, mn, oge, pal, pi, pro, uk, ur | unchanged |
| `docdeps.json`, `docfeats.json` | 269 → 287 | 18 | 9 and 8 languages; universal `gdocs` and `deviations` unchanged | unchanged |
| `tospace.json` | 23 → 23 | 0 | ltg, lv, nl | no entry |
| `edeprels.json`, `udeprels.json`, `upos.json` | — | — | unchanged | — |

New languages: ce, dta, eve, evn, gld, kfx, ks, mjg, mnc, mvf, neg, oac, peh, sce, txg, ude, ulc, xal. ce, evn, kfx, ks, oac, txg, ulc have no auxiliaries. Languages that already existed but got auxiliaries for the first time (`data.json`): dar, mn, pal, pi, pro, sc. Got language documentation (`ldocs`) for the first time: bgs, dar, pal, pi.

**Our registry.** The English registry was rebuilt from both JSON states: `features/en/*/byupos/*/*` pairs with a non-zero value, `deprels/en/*` with `permitted=1`, `auxiliaries/en/*` with functions. Result:
- 2.18 and the snapshot are identical for en;
- `en/data/ud-registry-en.tsv` equals both: 251 `feat`, 58 `deprel`, 16 `aux`, auxiliary functions match.

So the registry is the 2.18 state, and also the state of the 24.09 snapshot. It cannot yet be called the 2.19 state: en entries may change before the data freeze on 01.11.2026 (UD docs/download.md`: release 2.19 — 15.11.2026). The comparison should be repeated before 2.19.

**What it means for English annotation.** Nothing new. As in 2.18, the registry does not accept `Style=Form`, which `_en/pos/PRON.md` gives to *whosoever* (only `Arch, Coll, Expr, Slng, Vrnc` are allowed). The en `feats.json` also has `ExtPos` with `evalues: ["AUX"]`, but the validator accepts only values from `uvalues`, `lvalues` and `unused_*` and does not take `evalues` (`tools/udtools/src/udtools/level4.py:190`), and `_en/feat/ExtPos.md` does not describe the value `AUX`. So `ExtPos=AUX` is not allowed — just as in our registry.

**In passing, for `uk`.** Ukrainian in the snapshot has:
- a new auxiliary *maty* "to have" (`Tense=Fut`, historical spelling, *robyty-mu*);
- `expl:pv` allowed;
- the layer `Animacy[gram]`;
- new `ExtPos` values.

**Sources.** `tools/data/*.json`, `2.18:tools/data/*.json`, `tools/data/README.md`; `en/src/udreg.rs`.
