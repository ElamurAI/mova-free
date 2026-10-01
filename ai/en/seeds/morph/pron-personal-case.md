# Personal pronouns: nominative and objective case

**Gist.** English nouns no longer have cases (except possessive *'s*), but personal pronouns have kept two cases: nominative (subjective) — *I, we, he, she, they*, and objective (oblique) — *me, us, him, her, them*. *You* and *it* do not change form. In UD, case is assigned **by the word form**, not by its role in the sentence.

**Conditions and exceptions.**
- The objective form in subject position or after *be* in colloquial speech (*It's me*, *Me and him went*) stays `Case=Acc`: the form decides.
- The nominative form in object position (*between you and I*) stays `Case=Nom`.
- Only for *you* and *it* is case determined by position: subject — `Nom`, object or word after a preposition — `Acc`.
- The lemma of an objective form is the nominative: *me → I, us → we, him → he, her → she, them → they*. *Her* is ambiguous: objective (*I saw her*, PRP) or possessive (*her book*, PRP$; see `pron-possessive`).
- Archaic *thou/thee, ye* and colloquial *'em* (= *them*) behave the same way; *'s* in *let's* is *us* (lemma *we*, `Case=Acc`).

**Examples.** ***She** saw **him**. **They** told **us**. It's **me**. Let**'s** go.*

**In UD.** UPOS PRON; XPOS PRP; `PronType=Prs`; `Case=Nom` or `Case=Acc`; lemma — the nominative form.

**Sources.** https://universaldependencies.org/en/feat/Case.html (Nom, Acc; *you, it* — by position), https://universaldependencies.org/en/pos/PRON.html (table of personal pronouns, lemmas, *'s* in *let's*); Santorini 1990, §2, p. 4 (PRP); Sweet NEG I §141 (text-1, p. 82: nominative and objective cases), §1084–1087 (p. 370–372: *it is me*; hypercorrect *between John and I*; *than me*; *thou, thee, ye*; *'em*); Whitney §155, §158 (text-1, p. 86–87: *you* was once objective only); Kruisinga II.2 §§964–984 (vol. 3 = text-3, p. 150–158: *it's me* is normal; after *than/as* both forms; *her* — objective or possessive by word order); Mätzner I, p. 293–295 (p. 311–313: *you or me, better than him*).

```rule
rule: en.morph.pron-nominative
what: I, we, he, she, they (and archaic thou, ye) — nominative case, tag PRP
match: p[upos=PRON, form=i|we|he|she|they|thou|ye, !feats.Typo]
require: p[xpos=PRP, feats.Case=Nom, feats.PronType=Prs]
severity: error
source: UD en feat/Case (Nom), pos/PRON
```

```rule
rule: en.morph.pron-accusative
what: me, us, him, them, thee, 'em — objective case, tag PRP
match: p[upos=PRON, form=me|us|him|them|thee|'em|’em, !feats.Typo]
require: p[xpos=PRP, feats.Case=Acc, feats.PronType=Prs]
severity: error
source: UD en feat/Case (Acc), pos/PRON
```

```rule
rule: en.morph.pron-accusative-lemma
what: the lemma of an objective form is the nominative (me → I, us → we, him → he, them → they)
match: p[upos=PRON, xpos=PRP, form=me|us|him|them, !feats.Typo]
require: p[lemma=i|we|he|they]
severity: error
source: UD en pos/PRON (accusative lemmatized to match the nominative)
```

```rule
rule: en.morph.pron-her-accusative
what: her tagged PRP — the objective form of she
match: p[upos=PRON, xpos=PRP, form=her, !feats.Typo]
require: p[lemma=she, feats.Case=Acc]
severity: error
source: UD en pos/PRON, feat/Case
```
