# Verb suffixes: -ize/-ise, -ify, -en, -ate

**Gist.** Verbs are formed from adjectives and nouns with several suffixes meaning "make so, transform": *real → realize, modern → modernize, clear → clarify, just → justify, wide → widen, short → shorten, active → activate*. There are few living suffixes, because English very easily makes verbs without a suffix (conversion: *to dry, to empty*; see `wf-conversion`).

**Conditions and exceptions.**
- *-ize/-ise*: the spelling *-ize* is American and Oxford, *-ise* is common British; lemmas should follow the spelling of the text. Some verbs are spelled only with *-ise*: *advertise, advise, comprise, despise, devise, exercise, supervise, surprise*. Not a suffix in *size, prize, seize, wise, precise*.
- *-ify/-fy*: *clarify, justify, testify, electrify, intensify*; *-efy* in *liquefy, stupefy*. A word in *-ify* in the modern language is a verb.
- *-en*: *widen, redden, sharpen, soften, harden, darken, fasten*; only after consonants that allow syllabic *n* (no *smallen, fullen*); *lengthen, strengthen* — because there are no verbs *long, strong*; in *frighten, listen, happen* the suffix no longer adds anything.
- *-ate* — from Latin participles (*separate, dedicate, situate*); *separate* as an adjective is pronounced [-ɪt], as a verb — [-eɪt].
- *-ish* in verbs (*finish, punish, publish, establish*) is borrowed and not living.

**Examples.** *We need to **realize** it. Please **clarify**. The road will **widen**. They **activated** the account.*

**In UD.** VERB with all verb forms (VB, VBZ, VBD, VBN, VBG); the lemma is the stem with the suffix (*clarified → clarify, widening → widen*).

**Sources.** Jespersen MEG VI 19.4₁–19.4₃ (text-5, pp. 334–336: *-ize/-ise*), 25.1₁ (pp. 467–468: *-fy, -efy*), 20.5₁–20.5₉ (pp. 367–374: *-en*, restriction after consonants; *lengthen*; *frighten*), 4.4₃, 24.8₂ (pp. 52–53, 463: *-ate*), 19.5₁ (p. 336: *-ish*); Sweet NEG I §1616, §1754–1758 (text-1, pp. 497, 527–529); Kruisinga II.3 §§1627–1633 (text-4, pp. 43–45), I §584 (text-1, p. 259: always *-ise*: *advertise, advise, comprise…*); Mätzner I, p. 439, 473–474 (text-1, pp. 457, 491–492); Whitney §95, §225a (text-1, pp. 58, 118).

```rule
rule: en.morph.suffix-ify-verb
what: word in -ify — verb (base form)
match: w[suffix=ify, !feats.Typo]
require: w[upos=VERB|PROPN|X]
severity: warn
source: Jespersen MEG VI 25.1₁; Sweet NEG I §1754–1758
```
