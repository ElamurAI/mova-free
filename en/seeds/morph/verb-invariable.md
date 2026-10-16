# Invariable verbs: put, cut, set, hit

**Gist.** A group of verbs in *-t/-d* has one form for the base, the past and the participle: *put–put–put, cut–cut–cut, set, hit, let, shut, cost, hurt, burst, cast, spread, split, quit, bet, rid, shed, thrust, wet*. So the tag of such a form (VB, VBP, VBD or VBN) is determined **only by context**: subject, auxiliaries, tense of neighbouring verbs. The lemma always equals the form.

**Conditions and exceptions.**
- A 3rd person singular subject without *-s* (*he put, it cost*) means past tense: VBD. The present would be *puts, costs* (VBZ). So *he/she/it* + an invariable verb tagged VBP is an error.
- After a modal, *to*, *do* — VB; after *have* or in the passive — VBN.
- Variation with *-ed*: *knit/knitted, quit/quitted* (British), *bet/betted, wet/wetted, sweat/sweated, broadcast(ed), forecast(ed)*; *shred* is now more often *shredded*.
- Frozen former participles: *dread moment, roast beef* — adjectives.
- *Read* is spelled the same but pronounced [red] in the past — in writing it behaves as invariable.

**Examples.** *Yesterday he **put** it here* (VBD). *They **cut** costs every year* (VBP). *It has **cost** a lot* (VBN). *Don't **hit** him* (VB).

**In UD.** Lemma = form; tag by context; for VBD — `Tense=Past|VerbForm=Fin`, for VBP — `Tense=Pres`.

**Sources.** Jespersen MEG VI 4.4₁–4.4₂ (text-5, pp. 50–52: class 2, full list and *-ed* variation; *dread, roast* as adjectives); Sweet NEG I §1344–1363 (text-1, pp. 432–435: invariable verbs, all in *t/d*); Whitney §253 (text-1, p. 129: *burst, cast, cost, cut, hit, hurt, knit, let, put, quit, rid, set, shed, shred, shut, slit, spit, split, spread, sweat, thrust, wet, whet*); Santorini 1990, §4.1, p. 21 (VB/VBP test: 3rd person subject).

```rule
rule: en.morph.invariable-3sg-not-vbp
what: invariable verb with subject he/she/it — not VBP (without -s it is past tense)
match: v[upos=VERB, form=put|cut|set|hit|let|shut|cost|hurt|burst|cast|spread|split|quit|bet|shed|thrust|upset|broadcast|forecast|slit|rid]; s[upos=PRON, form=he|she|it, rel~nsubj, head=v]
require: not v[xpos=VBP]
severity: error
source: Jespersen MEG VI 4.4₁; Sweet NEG I §1344–1363; Whitney §253; Santorini 1990, §4.1, p. 21
```
