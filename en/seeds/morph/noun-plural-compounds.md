# Plural of compound nouns: sons-in-law, passers-by, forget-me-nots

**Gist.** In a compound noun the plural usually goes on the head word — the one that names the thing. If the head comes first, followed by a prepositional phrase or an adverb, the *-s* ends up inside: *son-in-law → sons-in-law, passer-by → passers-by, hanger-on → hangers-on, commander-in-chief → commanders-in-chief*. If there is no head noun (the word describes the bearer of a property), the *-s* goes at the end: *forget-me-nots, go-betweens, grown-ups, redcoats, runaways*.

**Conditions and exceptions.**
- Noun + adjective after it: bookish *courts-martial, attorneys general*, colloquial *court-martials*.
- *Handful, spoonful, mouthful* are no longer felt as compounds: *handfuls, spoonfuls* (less often *spoonsful*).
- Compounds with *-man*: *Englishmen, policemen* (see `noun-plural-mutation-en`).
- In colloquial speech the *-s* sometimes moves to the end: *son-in-laws*.
- A hyphenated compound in EWT is usually one token; then the tag is NNS, `Number=Plur`, and the lemma is the singular of the whole compound (*sons-in-law → son-in-law*).

**Examples.** *my **sisters-in-law**; the **passers-by**; the **runners-up**; two **forget-me-nots**.*

**In UD.** NOUN, NNS, `Number=Plur`; lemma — the compound in the singular (*passers-by → passer-by*).

**Sources.** Sweet NEG I §1018–1019 (text-1, p. 349: *hangers-on, fathers-in-law, commanders-in-chief*; *go-betweens, forget-me-nots*; *courts-martial* / *court-martials*), §440–442 (p. 183–184: group compounds *son-in-law*, plural *sons-in-law*); Whitney §130 (text-1, p. 74: *merchantmen, brothers-in-law, hangers-on*; *redcoats, turnkeys, runaways, forget-me-nots*; *mouthfuls, handfuls*); Jespersen MEG II 2.3₁ (text-2, p. 52: usually only the last element changes; *sons-in-law, lookers-on*), VI 17.8 (text-5, p. 314–316: *drawbacks, grown-ups, forget-me-nots, handfuls*); Kruisinga II.2 §§763–773 (text-3, p. 29–35).

```rule
rule: en.morph.compound-plural-lemma
what: sons-in-law, passers-by etc. — plural, lemma is the compound in the singular
match: n[upos=NOUN, form=mothers-in-law|fathers-in-law|sons-in-law|daughters-in-law|brothers-in-law|sisters-in-law|passers-by|lookers-on|hangers-on|runners-up|commanders-in-chief|attorneys-general|courts-martial, !feats.Typo]
require: n[xpos=NNS, feats.Number=Plur, lemma=mother-in-law|father-in-law|son-in-law|daughter-in-law|brother-in-law|sister-in-law|passer-by|looker-on|hanger-on|runner-up|commander-in-chief|attorney-general|court-martial]
severity: warn
source: Sweet NEG I §1018–1019, §440–442; Whitney §130
```
