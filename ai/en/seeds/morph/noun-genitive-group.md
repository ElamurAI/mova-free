# Group genitive: the King of England's crown

**Gist.** The English possessive *'s* attaches not to a word but to a whole phrase, and stands at its end: *the King of England's crown* (the king's crown, not England's), *somebody else's coat*, *the man I saw yesterday's son* (colloquial), *my son-in-law's car*. So *'s* is a clitic, not a case ending: it can end up after a prepositional phrase, a subordinate clause or a conjunction.

**Conditions and exceptions.**
- Coordinated possessors: joint possession — one *'s* at the end (*John and Mary's house*), separate — on each (*John's and Mary's houses*).
- Apposition: *my friend the hunter's rifle*.
- Phrases of time and measure: *a fortnight or three weeks' possession*, *two hours' drive*.
- Instead of a long phrase with *'s*, *of* is more often used: *the crown of the King of England*.

**Examples.** *the King of England's crown; somebody else's idea; my father-in-law's house.*

**In UD.** *'s* (PART, POS) attaches via `case` to the **head** of the possessor phrase, not to its last word: *The head of school 's speech* → `case(head, 's)`; the possessor head is `nmod:poss` of the possessed. So *'s* always stands **after** its head, but not necessarily next to it. In EWT roughly every tenth *'s* is separated from its head by other words.

**Sources.** Sweet NEG I §1016–1017, §443 (text-1, p. 348–349, 184: *the king of England's son, the man I saw yesterday's son, the son-in-law's*); Whitney §137–138 (text-1, p. 76: *father-in-law's, the King of England's crown, God and Nature's hand, a fortnight or three weeks' possession*), §379 (p. 188: *my friend the hunter's rifle*); https://universaldependencies.org/en/dep/case.html (*The head of school 's speech*: `case(head, 's)`), `dep/nmod-poss.md`; Jespersen MEG VI 17.1–17.5 (text-5, p. 297–307: *the King of England's power, somebody else's hat*; with a subordinate clause — colloquial); Kruisinga II.2 §§825, 831 (text-3, p. 56–60: *the Prince of Wales's tour, William and Mary's reign*).

```rule
rule: en.morph.possessive-after-head
what: possessive 's stands after the head of the phrase it attaches to
match: h[upos=NOUN|PROPN|PRON|NUM|ADJ|X|SYM]; p[xpos=POS, head=h]
require: p[after=h]
severity: error
source: Sweet NEG I §1016–1017; Whitney §137–138; UD en dep/case (The head of school 's speech)
```
