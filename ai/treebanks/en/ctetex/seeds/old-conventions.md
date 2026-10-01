# CTeTex — `compound` without `:prt`, possessives as DET `nmod`, *when* as `mark`, lemma *an*

**Gist.** CTeTex was annotated in 2021 following the UD guidelines of the time, without the EWT English subtypes. Hence four customs that look different in `mova`:
1. **The phrasal-verb particle** (*pick up*, *checked out*) — `compound`, not `compound:prt`: 14 of 14. CTeTex has not a single `compound:prt`.
2. **Possessives *its, their, our, my, his*** — DET with `nmod`, not PRON with `nmod:poss`. CTeTex has not a single `nmod:poss`, and 16 possessive DET.
3. **Subordinate *when*** — SCONJ `mark` (34), not ADV `advmod` as in EWT since 2.11. Likewise *where* SCONJ 2.
4. **The lemma of the indefinite article *an*** — *an* (41 of 41), not *a*.

**Conditions and exceptions.**
- Possessive *'s* — PART `case`, as in EWT (4 times), and *'* — PUNCT `punct` (4). The latter is probably a tokenization error in *users'*.
- In EWT 33 of 3501 *my, its, their…* are also not `nmod:poss`, but these are not modifiers: independent pronouns (*his* as `xcomp`) and typos (*their* instead of *there* as `expl` — 6).
- `obl:agent` is present (22), `nsubj:outer` (1), `csubj:pass` (1) — so the annotators partly knew the 2.10–2.13 subtypes.

**Examples.**
- `270` *User's will be able to pick up a tile from the wall…* — *up*: ADP, `compound` → *pick*. In EWT — `compound:prt`.
- `92` *An FAQ section…* — *An*: lemma *an*.
- `112` *…the following control capabilities: 1. Send and …when…* — *when*: SCONJ, `mark`.

**In UD.**

| rule | CTeTex | EWT |
|---|---:|---:|
| `tb.en.compound-prt`: particle on a verb — not `compound:prt` | 14 / 14 | 40 / 939 |
| `tb.en.wh-advmod`: *when/where/how/why* in `mark` | 36 / 47 | 0 / 1049 |
| `tb.en.an-lemma`: *an* with a lemma other than *a* | 41 / 41 | 0 / 619 |
| `tb.ctetex.poss-nmod`: *my, its, their…* — not `nmod:poss` | 16 / 16 | 33 / 3501 |

**Sources.** https://universaldependencies.org/en/dep/compound-prt.html, `dep/nmod-poss.md`, https://universaldependencies.org/en/pos/PRON.html (possessives — PRON), https://universaldependencies.org/en/pos/DET.html; EWT README v2.11 (#88); Hassert et al. 2021, `2021.udw-1.5`.

```rule
rule: tb.ctetex.poss-nmod
what: possessive modifier my/its/their… — nmod:poss (CTeTex: DET nmod)
match: p[form=my|your|his|its|our|their, rel!=conj]
require: p[rel=nmod:poss]
severity: warn
source: UD en nmod:poss; counter
```
