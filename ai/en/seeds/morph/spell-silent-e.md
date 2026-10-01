# Silent -e in inflection: make → making, nice → nicer

**Gist.** Final silent *-e* (not pronounced, it only signals a long vowel: *make, hope, nice*) drops before an ending that begins with a vowel: *make → making, hope → hoped, hoping, nice → nicer, nicest, love → lovable*. Before a consonant it stays: *hopeful, lovely, movement*. For lemmatization this means: *hoping* has lemma *hope* (with *-e*), and *hopping* — *hop* (no *-e*, with doubling).

**Conditions and exceptions.**
- *-ee, -ye, -oe* keep the *e* before *-ing*: *seeing, agreeing, dyeing* (colouring; ≠ *dying* — ceasing to live), *hoeing, shoeing, canoeing, eyeing*; also *singeing* (≠ *singing*), *swingeing*, *ageing/aging*.
- *-ie* before *-ing* → *-y-*: *lie → lying, die → dying, tie → tying, vie → vying*.
- *-ce, -ge* keep the *e* before *a, o* to preserve the pronunciation: *noticeable, manageable, courageous*.
- Before *-ed*, after a stem in *-e* only *-d* is added: *love → loved, agree → agreed*.
- Adverbs: *true → truly, due → duly, whole → wholly* (no *e*), but *sole → solely, vile → vilely*.
- *-able* varies: *likeable/likable, liveable/livable*.

**Examples.** *making → make; hoping → hope; hopping → hop; nicer → nice; lying → lie; dyeing → dye; dying → die.*

**In UD.** The lemma is the stem with the *-e* restored: *writing → write, used → use, larger → large, lying → lie*.

**Sources.** Jespersen MEG VI 21.9₃ (text-5, p. 395–396: *having, hoping, coming*; *ageing, singeing*; *dyeing/dying*), 4.2₃ (p. 46–47: *loved, freed, guaranteed*), 22.8₂–22.8₆ (p. 425–428: *truly, duly; solely, vilely, wholly*), 22.6 (p. 414–419: *likeable, liveable*); Kruisinga I §§567, 569, 573 (text-1, p. 254–255: *making; singeing, swingeing, eyeing, dyeing, hoeing, shoeing, canoeing; ageing/aging; dying*), §568, §575 (p. 254–256: *truly, duly, wholly*); Mätzner I, p. 337 (text-1, p. 355).

```rule
rule: en.morph.silent-e-verb-lemma
what: making, having, coming, writing, used, hoping… — lemma with silent -e
match: v[upos=VERB|AUX, form=making|having|coming|giving|taking|writing|using|used|hoping|hoped|moving|moved|living|lived|loving|loved|leaving|becoming|saving|saved|closing|closed|driving|changing|changed|deciding|decided|providing|provided|including|included|believing|believed|creating|created|hating|hated, !feats.Typo]
require: v[lemma=make|have|come|give|take|write|use|hope|move|live|love|leave|become|save|close|drive|change|decide|provide|include|believe|create|hate]
severity: error
source: Jespersen MEG VI 21.9₃, 4.2₃; Kruisinga I §567; Mätzner I, p. 337
```

```rule
rule: en.morph.ie-ying-lemma
what: lying, dying, tying, vying — lemma in -ie
match: v[upos=VERB, form=lying|dying|tying|vying|untying, !feats.Typo]
require: v[lemma=lie|die|tie|vie|untie]
severity: error
source: Jespersen MEG VI 21.9₃ (dyeing vs dying); Kruisinga I §569
```
