# Articles the, a, an

**Gist.** English has two articles: definite *the* and indefinite *a/an*. *An* is not a separate word but the form of *a* before a vowel sound, so both share one lemma.
**Conditions and exceptions.** An article is a determiner on a noun or on an adjective used as a noun (*the rich*). The letter *a* as a name or list item (*a)*, *Type A*, *vitamin A*) is not an article: NOUN, SYM or LS.
**Examples.** *the book*; *a book*; *an apple*; *an hour*.
**In UD.** *the*: DET, DT, `Definite=Def|PronType=Art`. *a/an*: DET, DT, `Definite=Ind|PronType=Art`, lemma `a`. Relation — `det`. EWT 2.18: *the* 11 017, *a* 4 724, *an* 618 — all like this.
**Sources.** Poutsma GLME vol. 3, Ch. XXXI §1–3 (pp. 533–535 = book pp. 513–515); https://universaldependencies.org/en/feat/Definite.html, `feat/PronType.md`.

```rule
rule: en.nominal.the-feats
what: the definite article the — DET DT with Definite=Def and PronType=Art
match: d[lemma=the, upos=DET]
require: d[xpos=DT, feats.Definite=Def, feats.PronType=Art]
severity: error
source: Poutsma GLME III Ch. XXXI §1–2; UD en Definite, PronType
```

```rule
rule: en.nominal.a-an-feats
what: the indefinite article a/an — DET DT with lemma a, Definite=Ind, PronType=Art
match: d[form=a|an, upos=DET]
require: d[lemma=a, xpos=DT, feats.Definite=Ind, feats.PronType=Art]
severity: error
source: Poutsma GLME III Ch. XXXI §1, §3; UD en Definite, PronType
```
