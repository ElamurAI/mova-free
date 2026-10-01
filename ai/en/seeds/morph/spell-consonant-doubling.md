# Doubling of the final consonant: stop → stopped, big → bigger

**Gist.** Before endings that begin with a vowel (*-ed, -ing, -er, -est*, and also *-y, -ish*), the final consonant doubles if it is preceded by a single short **stressed** vowel: *stop → stopped, stopping; run → running; big → bigger, biggest; prefer → preferred; begin → beginning*. Doubling only marks the short vowel in spelling; the lemma has no second letter: *stopped → stop*, not *stopp*.

**Conditions and exceptions.**
- An unstressed final syllable — no doubling: *visit → visited, offer → offered, open → opened, develop → developed, gallop → galloping, benefit → benefited* (*benefitted* occurs); *prefer → preferred*, but *préference* → *preferable*.
- Two vowels or a long vowel before the consonant — no doubling: *rain → rained, beat → beating, cool → cooler*.
- *W, x, y* are not doubled: *snow → snowed, fix → fixed, play → playing*.
- British *l* doubles even after an unstressed vowel: *travel → travelled, cancel → cancelled, cruel → crueller* (but *paralleled*); American does not (*traveled*). A few words with secondary stress double in the US too: *kidnapped, worshipped, handicapped, humbugged*.
- *-c* before *-ed, -ing, -y* → *-ck-*: *panic → panicked, picnic → picnicking, traffic → trafficked, panicky*.
- Final *s*: *gases* (noun), *buses/busses*; *focused/focussed, biased/biassed* — both.
- The lemmatizer must remove **only one** of the two letters, and only when the stem does not itself end in a double consonant (*miss → missed*, lemma *miss*; *add → added*, lemma *add*).

**Examples.** *stopped → stop; running → run; bigger → big; preferred → prefer; travelled → travel; panicked → panic.*

**In UD.** The lemma is the stem without doubling and without *-k-*: *hottest → hot, beginning → begin, trafficked → traffic*.

**Sources.** Jespersen MEG VI 4.2₃ (text-5, p. 46–47: doubling after a short stressed vowel; British *-ll-*; *-ck-*; *worshipped, kidnapped*; *starred/stared, preferred/interfered*), 21.9₃ (p. 395–396: *sitting, begging, travelling*, but *galloping*), 16.1₆ (p. 272–273: *gases/busses*); Kruisinga I §§572, 574, 581–583 (vol. 1 = text-1, p. 254–258: *fitted, preferred; travelled, crueller; mimicked; worshipped, but galloping, developed; biassed/biased*); Mätzner I, p. 337 (text-1, p. 355: doubling after a stressed short vowel; *gossip, worship, kidnap, travel* — grammarians "not agreed"; *trafficked*), p. 273 (p. 291: *bigger, hotter, crueller*).

```rule
rule: en.morph.doubling-verb-lemma
what: stopped, planning, getting, running, preferred… — lemma without the doubled consonant
match: v[upos=VERB|AUX, form=stopped|stopping|planned|planning|shopped|shopping|dropped|dropping|getting|running|sitting|putting|setting|cutting|hitting|letting|beginning|swimming|winning|spinning|preferred|preferring|referred|referring|occurred|occurring|admitted|admitting|committed|committing|controlled|controlling|travelled|travelling|cancelled|cancelling|panicked|trafficked, !feats.Typo]
require: v[lemma=stop|plan|shop|drop|get|run|sit|put|set|cut|hit|let|begin|swim|win|spin|prefer|refer|occur|admit|commit|control|travel|cancel|panic|traffic]
severity: error
source: Jespersen MEG VI 4.2₃, 21.9₃; Kruisinga I §§572–583; Mätzner I, p. 337
```

```rule
rule: en.morph.doubling-adj-lemma
what: bigger, hottest, sadder, thinnest… — adjective lemma without doubling
match: a[upos=ADJ, xpos=JJR|JJS, form=bigger|biggest|hotter|hottest|sadder|saddest|thinner|thinnest|fatter|fattest|wetter|wettest|redder|reddest|madder|maddest|fitter|fittest|flatter|flattest, !feats.Typo]
require: a[lemma=big|hot|sad|thin|fat|wet|red|mad|fit|flat]
severity: error
source: Mätzner I, p. 273 (bigger, hotter); Jespersen MEG VI 4.2₃
```
