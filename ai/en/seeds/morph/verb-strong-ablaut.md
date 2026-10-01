# Strong verbs: sing–sang–sung, write–wrote–written

**Gist.** About two hundred English verbs form the past and the participle not with the ending *-ed* but by changing the root vowel, often also with *-n/-en* in the participle: *sing–sang–sung, write–wrote–written, take–took–taken, know–knew–known*. It is a closed class: it gains no new members, and old ones gradually become regular. For annotation the key point is that for some verbs **the past and the participle differ**, so the form itself suggests the tag: *went, saw, took* — only VBD; *gone, seen, taken* — only VBN.

**Conditions and exceptions.**
- Modern groups (after Jespersen): three different vowels without *-n* (*swim–swam–swum, begin–began–begun, sing, ring, drink, sink; run–ran–run, come–came–come*); participle in *-n* (*drive–drove–driven, ride, write, rise; speak–spoke–spoken, break, steal, freeze, choose; take, shake; blow–blew–blown, grow, know, throw, fly, draw; give, eat, fall, see, do, beat–beat–beaten*); suppletive *go–went–gone*, *be–was/were–been*.
- *-n* always stays after a vowel or *r* (*seen, done, gone, drawn, known, born, sworn, torn, worn*) and never comes after a nasal (*swum, begun*).
- Double forms: *got/gotten* (*gotten* is usual in the US), *proved/proven*, *showed/shown*, *woke/waked*, *hung/hanged* ("executed"), *born* (of birth in the passive) / *borne* ("carried", *has borne*).
- The vernacular levels forms: *I seen it, he done it, had went, have took*. EWT annotates by function (after *have* — VBN even for *went*), so the "only VBN / only VBD" rule is at level `warn`, and gross errors (VB, VBP, VBZ, VBG for such forms) are `error`.

**Examples.** *She **wrote** a letter* (VBD). *It was **written** in 1914* (VBN). *We **went** home* / *We have **gone** home.*

**In UD.** The lemma is the infinitive (*wrote, written → write*; *went, gone → go*). Forms with a separate participle: past — VBD, participle — VBN.

**Sources.** Jespersen MEG VI 4.1₄ (text-5, p. 44: abandoning the Old English classes, 11 modern ones), 5.2–5.4 (pp. 69–86: groups 9–10), 5.6 (p. 91: *go–went–gone*, *be*), 5.7₁–5.7₃ (pp. 92–94: when *-n* stays), 5.1₆–5.1₇, 5.3₃, 5.5₃ (pp. 65–67, 75, 89: *gotten, hung/hanged, born/borne, proven*); Sweet NEG I §1284–1288 (text-1, pp. 421–422: consonantal, vocalic, mixed and invariable verbs), §1364–1456 (pp. 435–448: lists); Whitney §257–275 (text-1, pp. 130–134: groups of strong verbs, levelling of forms).

```rule
rule: en.morph.strong-participle-form-tag
what: gone, seen, taken, written… — participle (in the vernacular sometimes used as past)
match: v[upos=VERB|AUX, form=gone|seen|taken|given|eaten|written|spoken|broken|known|drawn|grown|thrown|flown|blown|begun|drunk|sung|rung|swum|sunk|ridden|driven|risen|chosen|frozen|stolen|worn|torn|sworn|forgotten|forgiven|fallen|shaken|mistaken|undertaken|overtaken|withdrawn|arisen|done|been|hidden|bitten|beaten|shown|woken|forbidden|gotten|proven, !feats.Typo]
require: v[xpos=VBN|VBD]
severity: error
source: Jespersen MEG VI 5.2–5.7; Sweet NEG I §1364–1456; Whitney §257–275
```

```rule
rule: en.morph.strong-participle-vbn
what: gone, seen, taken, written… in the standard language — only VBN
match: v[upos=VERB|AUX, form=gone|seen|taken|given|eaten|written|spoken|broken|known|drawn|grown|thrown|flown|blown|begun|drunk|sung|rung|swum|sunk|ridden|driven|risen|chosen|frozen|stolen|worn|torn|sworn|forgotten|forgiven|fallen|shaken|mistaken|undertaken|overtaken|withdrawn|arisen|done|been|hidden|bitten|beaten|shown|woken|forbidden|gotten|proven, !feats.Typo]
require: v[xpos=VBN]
severity: warn
source: Jespersen MEG VI 5.6 (vulgar seen/done as preterites); Whitney §259
```

```rule
rule: en.morph.strong-preterite-form-tag
what: went, saw, took, wrote… — past tense (in the vernacular sometimes in place of the participle)
match: v[upos=VERB|AUX, form=went|saw|took|gave|ate|wrote|spoke|broke|knew|drew|grew|threw|flew|blew|began|drank|sang|rang|swam|sank|rode|drove|rose|chose|froze|stole|wore|tore|swore|forgot|forgave|fell|shook|mistook|undertook|overtook|withdrew|arose|was|were|did|came|became|overcame|ran, !feats.Typo]
require: v[xpos=VBD|VBN]
severity: error
source: Jespersen MEG VI 5.2–5.6; Sweet NEG I §1364–1456; Whitney §257–275
```

```rule
rule: en.morph.strong-preterite-vbd
what: went, saw, took, wrote… in the standard language — only VBD
match: v[upos=VERB|AUX, form=went|saw|took|gave|ate|wrote|spoke|broke|knew|drew|grew|threw|flew|blew|began|drank|sang|rang|swam|sank|rode|drove|rose|chose|froze|stole|wore|tore|swore|forgot|forgave|fell|shook|mistook|undertook|overtook|withdrew|arose|was|were|did|came|became|overcame|ran, !feats.Typo]
require: v[xpos=VBD]
severity: warn
source: Jespersen MEG VI 5.1₁, 5.3₁, 5.6 (vulgar took for taken, had went)
```
