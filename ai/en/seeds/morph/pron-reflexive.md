# Reflexive and intensive pronouns in -self

**Gist.** Pronouns in *-self* (singular) and *-selves* (plural) are formed from the possessive form in the 1st and 2nd person (*my-self, your-self, our-selves*) and from the objective in the 3rd (*him-self, them-selves*; *her-self, it-self* — the forms coincide). They have two roles: reflexive — an object that repeats the subject (*She hurt **herself***), and intensive — emphasis on the person (*She **herself** said so*).

**Conditions and exceptions.**
- Generic *oneself* — from *one*.
- Dialectal and nonstandard *hisself, theirselves* are built from the possessive form in the 3rd person; *themself* is used for *singular they*.
- Singular or plural is visible from the word ending: *-self* ↔ singular, *-selves* ↔ plural. *Yourself* is singular, *yourselves* plural, although *you* does not distinguish number.

**Examples.** *I cut **myself**. They enjoyed **themselves**. The president **himself** called.*

**In UD.** UPOS PRON; XPOS PRP; lemma — the form itself (*themselves → themselves*); FEATS `Case=Acc|Reflex=Yes` with person, number and gender; `PronType=Prs` in reflexive use, `PronType=Emp` in intensive use. The guidelines (`Case.md`, `Reflex.md`) say intensives have no `Case` and `Reflex`, but EWT 2.18 assigns `Case=Acc|Reflex=Yes` to both, so the rule below follows the data and has level `warn`.

**Sources.** https://universaldependencies.org/en/feat/Reflex.html, `feat/PronType.md` (Emp: *Thomas himself said so*), `feat/Case.md`, https://universaldependencies.org/en/pos/PRON.html (Reflexive column); Santorini 1990, §2, p. 4 (PRP also covers *-self/-selves*); Sweet NEG I §1109–1114 (text-1, p. 377–378: *him-, them-, it-* — objective stems, *my-, your-, our-* — possessive; nonstandard *hisself, theirselves*; stressed intensive and unstressed reflexive), §1105 (p. 376: *he looked about him*); Whitney §164 (text-1, p. 89: *myself am Naples*; *I dress myself*); Kruisinga II.2 §§1024–1030 (text-3, p. 180–183: intensive use is not reflexive).

```rule
rule: en.morph.reflexive-feats
what: a pronoun in -self/-selves — PRP, Reflex=Yes, Case=Acc, PronType Prs or Emp
match: p[upos=PRON, suffix=self|selves, !feats.Typo]
require: p[xpos=PRP, feats.Reflex=Yes, feats.Case=Acc, feats.PronType=Prs|Emp]
severity: warn
source: UD en feat/Reflex, feat/PronType (Emp), pos/PRON; EWT 2.18 practice
```

```rule
rule: en.morph.reflexive-number
what: -selves — plural
match: p[upos=PRON, suffix=selves, !feats.Typo]
require: p[feats.Number=Plur]
severity: error
source: UD en pos/PRON (yourselves, ourselves, themselves — Plur)
```

```rule
rule: en.morph.reflexive-number-sing
what: -self — singular
match: p[upos=PRON, suffix=self, !feats.Typo]
require: p[feats.Number=Sing]
severity: error
source: UD en pos/PRON (myself, yourself, himself, herself, itself, oneself — Sing)
```

```rule
rule: en.morph.reflexive-lemma
what: the lemma of a reflexive pronoun is the form itself
match: p[upos=PRON, form=myself|yourself|himself|herself|itself|ourselves|yourselves|themselves|oneself, !feats.Typo]
require: p[lemma=myself|yourself|himself|herself|itself|ourselves|yourselves|themselves|oneself]
severity: error
source: UD en pos/PRON (reflexives are not lemmatized to the base pronoun)
```
