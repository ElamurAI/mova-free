# Irregular weak verbs: keep–kept, tell–told, think–thought

**Gist.** Many irregular verbs are not "strong" but weak with a complication: they add *-t* or *-d*, but also change the vowel or consonant of the stem. Their past **coincides** with the participle (*kept, told, thought*), so only syntax decides between VBD and VBN. The lemma is the base, which often differs noticeably from the form.

**Conditions and exceptions.** Groups after Jespersen:
- *-t* with vowel change: *keep–kept, sleep, sweep, weep, creep, feel–felt, kneel, deal, dream, lean, leap, mean–meant*; with devoicing: *leave–left, lose–lost, bereave–bereft, cleave–cleft*.
- *d* → *t*: *bend–bent, lend, send, spend, build–built*; also *went* (former past of *wend*).
- *-d* with vowel change: *say–said, flee–fled, hear–heard, sell–sold, tell–told, shoe–shod*.
- *-d* with loss of a consonant: *have–had, make–made*.
- *-t* with a deeper change: *bring–brought, think–thought, seek–sought, teach–taught, catch–caught, buy–bought, fight–fought*.
- Only vowel change, past = participle: *feed–fed, lead–led, meet–met, read–read, speed–sped, bleed, breed; hold–held, stand–stood, sit–sat, shoot–shot, get–got, win–won*.
- Vernacular *catched* is non-standard; *wrought* (from *work*) lives only as an adjective (*wrought iron*).

**Examples.** *She **kept** it* (VBD). *It was **kept** secret* (VBN). *I **thought** so. We **met** yesterday.*

**In UD.** VBD or VBN by syntax; the lemma is the base (*brought → bring, sought → seek, fled → flee, met → meet*).

**Sources.** Jespersen MEG VI 4.1₄, 4.5–4.9 (text-5, pp. 44, 54–60: classes 3–7), 5.1 (pp. 60–68: class 8), 4.9₁ (pp. 59–60: *catched* dialectal, *wrought iron*); Sweet NEG I §1293–1343 (text-1, pp. 424–432: consonantal verbs with vowel change, *t* instead of *d*, *t* instead of *-ded*); Whitney §249–255 (text-1, pp. 129–130: *meant, felt, kept, dealt; fed, led, shot, bled, bred, sped, met; brought, bought, sought, caught, taught, thought*).

```rule
rule: en.morph.mixed-verb-lemma
what: kept, told, thought, brought, met, held… tagged VBD/VBN — lemma is the base (keep, tell, think…)
match: v[upos=VERB|AUX, xpos=VBD|VBN, form=kept|slept|swept|wept|crept|knelt|meant|dealt|lost|told|sold|thought|brought|bought|sought|taught|caught|fought|made|had|said|paid|laid|sent|spent|built|lent|bent|heard|stood|understood|held|fled|fed|led|bled|bred|sped|met|shot|got|won|sat, !feats.Typo]
require: v[lemma=keep|sleep|sweep|weep|creep|kneel|mean|deal|lose|tell|sell|think|bring|buy|seek|teach|catch|fight|make|have|say|pay|lay|send|spend|build|lend|bend|hear|stand|understand|hold|flee|feed|lead|bleed|breed|speed|meet|shoot|get|win|sit]
severity: error
source: Jespersen MEG VI 4.5–4.9, 5.1; Sweet NEG I §1293–1343; Whitney §249–255
```
