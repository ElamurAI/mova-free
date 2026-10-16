# Adjective suffixes: -ous, -ful, -less, -able, -ic, -ish, -y, -en

**Gist.** English adjectives are most often formed from nouns and verbs with suffixes: *fame → famous, care → careful/careless, read → readable, economy → economic, child → childish, noise → noisy, wood → wooden, trouble → troublesome*. The suffix *-ous* yields only adjectives; other suffixes can be ambiguous, since nouns have the same endings too (*handful, animal, native*).

**Conditions and exceptions.**
- *-ous* (*famous, dangerous, various*): in the modern language — an adjective; *-ously* is the adverb from it.
- *-ful*: an adjective from a noun or verb (*careful, forgetful*); but nouns of measure *handful, spoonful, mouthful* (plural *handfuls*).
- *-less* — from nouns (*careless, homeless*), from verbs in the sense "un-…-able" (*countless, dauntless*); *unless* is a conjunction, *bless* is a verb: there it is not a suffix.
- *-able/-ible* — from almost any verb (*readable, eatable*, even *get-at-able*); *-ible* — in Latin loans (*audible*). There are nouns too: *table, cable, vegetable, variable*.
- *-ic/-ical*: *economic* (relating to the economy) / *economical* (thrifty), *historic/historical, comic/comical, politic/political*; *-ical* where there is a noun in *-ic(s)* (*statistical*).
- *-ish*: nationalities (*English, Danish*), "like" (*childish, boyish*), "somewhat" (*reddish, oldish*), with numbers (*sixish*); but the verbs *finish, publish, punish, establish* have a different, borrowed *-ish*.
- *-y* (*noisy, dirty, icy*), *-en* material (*wooden, golden, woollen*; in speech a noun modifier is more frequent: *gold watch*), *-some* (*troublesome, handsome*), *-like* (*childlike, godlike*), *-ly* from nouns (*friendly*; see `adv-ly`), *-ed* from nouns (*talented*; see `wf-ed-adjectives`).

**Examples.** *a **famous** writer; a **careful** driver; **readable** text; an **economic** crisis; a **childish** joke.*

**In UD.** ADJ, JJ (in the comparative and superlative — JJR/JJS), the lemma is the adjective itself.

**Sources.** Jespersen MEG VI 19.7₁–19.7₇ (text-5, pp. 343–346: *-ous*), 23.1–23.3 (pp. 433–437: *-like, -ful, -less*; *countless*), 22.6₁–22.6₅ (pp. 414–419: *-able*, *get-at-able*), 22.3₂–22.3₇ (pp. 403–409: *-ic/-ical*), 19.6₁–19.6₅ (pp. 337–342: *-ish*), 13.3 (pp. 227–231: *-y*), 20.4 (pp. 362–363: *-en*), 25.2 (pp. 469–473: *-some, -ive*); Sweet NEG I §1606–1614, §1719–1755 (text-1, pp. 493–496, 519–528); Kruisinga II.3 §§1682–1705 (text-4, pp. 65–73); Mätzner I, p. 436–438 (text-1, pp. 454–456: *-y, -en, -some*); Whitney §91, §193 (text-1, pp. 57, 102–103).

```rule
rule: en.morph.suffix-ous-adj
what: word in -ous — adjective
match: w[suffix=ous, !feats.Typo]
require: w[upos=ADJ|PROPN|X]
severity: warn
source: Jespersen MEG VI 19.7; Sweet NEG I §1719–1755; Whitney §193
```
