# Possessive case: 's and '

**Gist.** The English noun has only two cases: common (*man, men*) and possessive (*man's, men's*). The possessive is written with an apostrophe: *-'s* in the singular and in plurals without *-s* (*the boy's, the children's*), just an apostrophe after plural *-s* (*the boys'*). *'s* is pronounced like the plural: [s], [z] or [ɪz]. In Penn Treebank and UD tokenization *'s* and *'* are a separate word.

**Conditions and exceptions.**
- Nouns ending in a sibilant, especially names: *James's* and *James'*, *Socrates' wisdom*, *for conscience' sake*; mostly pronounced with [ɪz].
- Possessive pronouns have no apostrophe: *its, hers, yours, ours, theirs*; but *one's*.
- *'s* after a noun is also contracted *is* or *has* (*John's here* = *John is here*): such *'s* is a verb (VBZ, lemma *be/have*), not a possessive; see `verb-clitics`.
- Possessive with no noun after it: *at my uncle's* (at the uncle's place), *a friend of John's* (double genitive).
- Possessive *'s* is added to the whole phrase, not to a single word: *the King of England's* (see `noun-genitive-group`).

**Examples.** *the **boy's** bike; the **boys'** bikes; the **children's** toys; **James's** car.*

**In UD.** *'s* / *'* (also *’s*, *’*): UPOS PART, XPOS POS, lemma *'s*, relation `case` to the head of the possessor phrase; the phrase itself is `nmod:poss` of the possessed noun.

**Sources.** Sweet NEG I §1022 (text-1, p. 351–352: *'s* in the singular and irregular plural, *'* after plural *-s*; *Socrates'*; *its, hers, yours* without apostrophe, but *one's*), §998 (p. 342–343), §78 (p. 59–60: *John's book* vs *John's here*), §111 (p. 71: *at his uncle's*); Whitney §133–135, §142 (text-1, p. 75–77: pronunciation of *'s*; *men's, children's*; *cats'*; *the princess' favorite*); Santorini 1990, §2, p. 4 (POS: *John/NNP 's/POS; the parents/NNS '/POS*); https://universaldependencies.org/en/dep/case.html ("possessive clitic 's … separate from what it modifies"), `dep/nmod-poss.md`; Jespersen MEG VI 16.1₃ (text-5, p. 270–271: the apostrophe became fixed in the 17th–18th c.; *hers, ours* without apostrophe), 16.8₃–16.8₈ (p. 286–289: names in *-s*; *for goodness' sake*); Kruisinga II.2 §§751, 826–829 (text-3, p. 23, 57–59); Mätzner I, p. 243 (text-1, p. 261: *Douglas's* and *Douglas'* equally frequent).

```rule
rule: en.morph.possessive-token
what: possessive 's / ' — particle POS with lemma 's and relation case
match: p[xpos=POS]
require: p[upos=PART, lemma='s, rel=case]
severity: error
source: Santorini 1990, §2, p. 4 (POS); UD en dep/case, dep/nmod-poss
```
