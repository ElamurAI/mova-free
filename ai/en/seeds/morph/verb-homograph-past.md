# Homographs: the past of one verb = the base of another (found, saw, lay)

**Gist.** The irregular forms of several verbs coincide with the base of **another** verb or with a noun: *found* is the past of *find* and the base *found* ("to establish"); *saw* — from *see* and *saw* ("to cut with a saw", "a saw"); *lay* — from *lie* ("to recline") and the base *lay* ("to put"). An automatic lemmatizer that sees only the form easily assigns a wrong lemma. The tag decides: as VBD/VBN the form belongs to the strong verb, and as VB/VBP/NN — to the homograph.

**Conditions and exceptions.**
- Pairs: *found* (find / found → founded), *saw* (see / saw → sawed), *fell* (fall / fell "to cut down" → felled), *lay* (lie / lay → laid), *rose* (rise / the flower), *bore* (bear / bore "to weary" → bored), *wound* (wind / wound "to injure" → wounded), *ground* (grind / ground → grounded), *bound* (bind / bound "to leap" → bounded), *left* (leave / left-hand), *felt* (feel / the fabric), *lit* (light), *spoke* (speak / wheel spoke), *stole* (steal / the wrap), *tore, wore, dove*.
- Regular homograph verbs form the past in *-ed*: so *found* tagged VBD is always from *find*, *wound* VBD is always from *wind*.
- The confusion of *lie–lay–lain* with *lay–laid–laid* in colloquial speech is old and spreading; annotation follows the form: *lay* tagged VBD has lemma *lie*, even if the speaker meant "to put".
- *Left, bound, found* as adjectives (*the left hand, bound to happen*) are ADJ with their own lemma.

**Examples.** *I **found** it* (VBD, lemma *find*). *They **found** a company* (VBP, lemma *found*). *He **lay** down* (VBD, lemma *lie*). *We **saw** the film* (VBD, lemma *see*).

**In UD.** VERB tagged VBD/VBN with the lemma of the strong verb; homographs have other lemmas and tags.

**Sources.** Jespersen MEG VI 5.1 (text-5, pp. 60–68: class 8 — *find, grind, wind* etc.), 5.3–5.4 (pp. 72–86: *lie–lay–lain, see–saw–seen, fall–fell–fallen, bear–bore–borne*); Sweet NEG I §1364 (text-1, p. 435: *bind/find/grind/wind → bound/found/ground/wound*), §1405 (p. 441: *lie–lay–lain* is levelled to *lay*), §1408 (pp. 441–442: *lit*); Whitney §96, §262, §264 (text-1, pp. 59, 131–132: *found, bound, ground, wound*; causatives *lay, fell*).

```rule
rule: en.morph.homograph-past-lemma
what: found, saw, fell, wound, ground, bound, left, felt… tagged VBD/VBN — lemma of the strong verb
match: v[upos=VERB|AUX, xpos=VBD|VBN, form=found|saw|fell|rose|bore|wound|ground|bound|left|felt|lit|spoke|stole|tore|wore|dove, !feats.Typo]
require: v[lemma=find|see|fall|rise|bear|wind|grind|bind|leave|feel|light|speak|steal|tear|wear|dive]
severity: error
source: Jespersen MEG VI 5.1, 5.3–5.4; Sweet NEG I §1364, §1405; Whitney §262, §264
```

```rule
rule: en.morph.lay-past-of-lie
what: lay tagged VBD — past of lie "to recline"
match: v[upos=VERB, xpos=VBD, form=lay, !feats.Typo]
require: v[lemma=lie]
severity: error
source: Jespersen MEG VI 5.4 (lie–lay–lain); Sweet NEG I §1405
```
