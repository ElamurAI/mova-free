# Interrogative and relative pronouns: who, whom, whose, which, what, that

**Gist.** Interrogative and relative pronouns are the same words in two roles: in a question (*Who called?*) and at the start of a relative clause (*the man **who** called*). Of these, only *who* has cases: *who* (nominative), *whom* (objective), *whose* (possessive). *Which* and *what* do not change; neither does relative *that*.

**Conditions and exceptions.**
- In living speech *whom* gives way to *who* even as an object (*Who did you see?*): the form *who* then still gets lemma *who*, and case by position.
- *Whose* is used for both people and things (*a house whose roof leaks*); do not confuse it with the form *who's* (= *who is*).
- *Which, what, whatever* before a noun — DET (*which book*), standalone — PRON (*Which is yours?*).
- Compounds in *-ever* (*whoever, whatever, whichever*) behave like the base word; *whomever* has lemma *whoever*.
- Relative *that* — PRON with XPOS WDT, `PronType=Rel` (not DT and not SCONJ).

**Examples.** ***Whom** did you invite? The author **whose** book I read. **Which** do you want? The car **that** I bought.*

**In UD.** XPOS: *who, whom, what* — WP; *whose* — WP$; *which, that, whatever* — WDT. `PronType=Int` in a question, `PronType=Rel` in a relative clause. *Whose*: `Poss=Yes`. The guideline `pos/PRON.md` requires lemma *who* and `Case=Acc` for *whom*; EWT 2.18 still has lemma *whom* without `Case` (23 cases), so the rule about *whom* is `warn`, and it must be rechecked against the next EWT release.

**Sources.** https://universaldependencies.org/en/pos/PRON.html (Relative/interrogative pronouns; table *who, whom [who], whose*), `feat/PronType.md` (Int, Rel), `pos/DET.md` (*which, what, whatever*); Santorini 1990, §2, pp. 4–6 and §4.1, p. 22 (WDT, WP, WP$); Sweet NEG I §211, §216 (text-1, p. 108–110: *who* only substantival, *which, what* — also adjectival; relative *that* invariable), §1086 (p. 371–372: *whom* is disappearing from speech, except right after a preposition); Whitney §169–187 (text-1, p. 90–96); Kruisinga II.2 §§1045–1047, 1079, 1094 (text-3, p. 190–191, 209, 217: *whom* mostly written; *whomever* hardly used); Jespersen MEG VI 16.1₃ (text-5, p. 271: *whose* — genitive of *who*; *who's* instead of *whose* — an error).

```rule
rule: en.morph.whose-poss
what: whose — possessive interrogative or relative pronoun WP$
match: w[form=whose, !feats.Typo]
require: w[upos=PRON, xpos=WP$, feats.Poss=Yes, feats.PronType=Int|Rel]
severity: error
source: UD en pos/PRON, feat/Poss; Santorini 1990, WP$
```

```rule
rule: en.morph.wh-prontype
what: a pronoun or determiner tagged WP, WP$, WDT — interrogative or relative
match: w[xpos=WP|WP$|WDT, upos=PRON|DET]
require: w[feats.PronType=Int|Rel]
severity: error
source: UD en feat/PronType (Int: WDT, WP, WP$, WRB unless relative; Rel)
```

```rule
rule: en.morph.whom-acc
what: whom — the objective form of who (lemma who, Case=Acc) per the UD guideline
match: w[upos=PRON, form=whom, !feats.Typo]
require: w[lemma=who, feats.Case=Acc]
severity: warn
source: UD en pos/PRON (whom [who], Case=Acc); EWT 2.18 still annotates it differently
```
