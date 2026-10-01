# ESLSpok — prepositional *to* is also PART/TO

**Gist.** The PTB guideline (Santorini 1990) gives the TO tag to every *to*: both the infinitive particle and the preposition. ESLSpok tags exactly that way. UPOS was derived from XPOS, so the preposition *to* in the `case` role got PART. EWT, following the "new" PTB, gives the preposition *to* XPOS IN and UPOS ADP. In `mova` *to* with a noun is ADP/IN, with an infinitive PART/TO.

**Conditions and exceptions.**
- All 576 *to* in ESLSpok are PART/TO:
  - `mark` — 341;
  - `case` — 224;
  - `obl` — 5, `xcomp` — 3, `fixed` — 2: these are errors, see `errors.md`.
- The role is annotated correctly: `case` with a noun, `mark` with a verb. So the converter can rely on it.

**Examples.**
- `file01070.txt_33` *…came to Osaka…* — *to*: PART, TO, `case` → *Osaka*.
- `file00498.txt_23` *And ballet is the decide to movement .* — *to*: `case` with *movement*.

**In UD.**

| rule | ESLSpok | EWT | CHILDES |
|---|---:|---:|---:|
| `tb.en.to-case`: *to* in `case` not ADP | 224 / 224 | 0 / 2029 | 24 / 1104 |

In EWT *to*: PART/TO 3993, ADP/IN 2211, SCONJ/IN 72.

**Sources.** Santorini 1990 (PTB), TO; https://universaldependencies.org/en/pos/ADP.html, `pos/PART.md`; https://universaldependencies.org/en/tokenization.html ("new" PTB, as in OntoNotes); Kyle et al. 2022, `2022.bea-1.7`, sec. 3 (PTB guideline for XPOS, UPOS derived from XPOS).
