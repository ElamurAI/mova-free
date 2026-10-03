# Who can do what to whom — level-1 selectional preferences (2026-10-02)

**Gist.** Some actions need a living, feeling doer: only a person or an animal eats, drinks, thinks, wants, loves, laughs, sleeps or marries. Some actions need a certain undergoer: what is eaten is not furniture, a building, clothing, metal or a vehicle; whom one marries, thanks or forgives is a person or a creature. A dependency tree that breaks these ("the cheese eats the mouse": cheese as the subject of eat) is most likely a parse error — or a deliberately odd sentence (personification in a tale). The lists below are used by the sense check of trees (`world sense-check`): categories come from the Wikidata core (`wikidata/core.md`) and the things seed (`things.md`).

What is said, cried or whispered is words, not a creature, and one does not "come", "live" or "appear" somebody: a person or an animal as the object of such a verb is the inverted subject of a tale ("'Good morning,' said the fox"; "there once lived a king") misread as an object.

Archaic adverbs of place and manner (`archaic_adverbs`: whence, whither, hither, thither…) are rare in modern treebanks, so a tagger trained on them reads "Whence came this stranger?" with whence as a noun subject; in tales they are always adverbs (advmod).

A noun of measure or time right after an intransitive verb is a bare oblique, not an object ("they went a long way", "he waited a while", "she stayed a week"): `measure_time` lists such nouns; intransitivity comes from the absurdity matrix (a verb whose object column is absurd for at least 80% of the nouns).

**Conditions and exceptions.** Precision first: a noun counts as animate or inanimate only if its categories agree (a word in both an animate and an inanimate category is unknown: "seal", "carp"); unknown nouns, pronouns and names are never judged. Body parts and plants are left out on purpose ("the heart wants", "the tree drinks the rain" are ordinary figures of speech). Words below in `not_inanimate` / `not_animate` are Wikidata homonyms that land in the wrong category without context ("hunter" → vehicle, "dove" → liquid, "spinach" → animal) or institutions that stand for their people (metonymy: "the hospital said", "the house voted"). Verbs that inanimate subjects use naturally are deliberately absent: say, show, tell, see ("2010 saw…"), feel ("the cloth feels soft"), speak ("the results speak"), ask, answer. Personification in tales ("the teapot thought") is flagged and is genuinely odd, not a parse error.

**Examples.** "The cheese eats the mouse" (cheese = nsubj) — role swap: the mouse can eat, the cheese can be eaten. "The man ate the table" — implausible object. "'Run!' cried the hare" with hare = obj of cry — implausible object (the inverted subject). "The mouse ate the cheese", "The knight married the princess" — no flag. "The hospital decided" — no flag (metonymy).

**Sources.** General knowledge (selectional restrictions: Katz and Fodor 1963; Resnik 1996 selectional preference; VerbNet thematic-role restrictions +animate, +concrete); categories — Wikidata (CC0) through `wikidata/core.md`.

```selection
animate_cats: wd_human wd_animal wd_occupation wd_mythical_creature people pupil
inanimate_cats: wd_food wd_fruit wd_vegetable wd_furniture wd_building wd_clothing wd_container wd_metal wd_gemstone wd_liquid wd_tool wd_musical_instrument wd_toy wd_vehicle wd_currency wd_color
not_inanimate: hunter conqueror charioteer chieftain crusader avian buzzard ferret gamecock gnat moth nag panther rover scout searcher skipper swarm tender thrush trainer wayfarer kite smack sociable junk launch progress eight mini alarm litter cutter clergyman retainer dove godfather mate sprite seal serpent wealthy must date poke diet strata lead keep top string bass jack mac last stock file match snap twitch romp antic nocturnal house home hotel hospital clinic factory studio supermarket bakery brewery casino courthouse palace temple monastery convent abbey cathedral chapel mosque workshop gym stadium tavern mill orphanage mint warehouse dairy foundry smithy resort shrine oratory cloister
not_animate: spinach browsing yawning taming permit drone pan sweeper mummy merlin mediterranean bleak boxer redhead countess serendib conch ruff top
agent_animate: eat drink devour dine think believe wonder wish hope want love hate adore fear dread enjoy like dislike laugh cry weep smile grin sob sleep dream pray marry kiss decide remember forget understand know learn imagine pretend shout scream whisper sigh breathe thank forgive beg
patient_edible: eat devour dine swallow chew nibble gobble munch
non_edible_cats: wd_furniture wd_building wd_clothing wd_metal wd_gemstone wd_vehicle wd_musical_instrument wd_currency
archaic_adverbs: whence whither thence hither thither hence yonder wherefore thereupon whereupon forthwith
measure_time: way time day while step night hour mile moment minute week month year morning evening afternoon distance pace league yard inch season winter summer spring autumn
patient_not_animate: say quoth reply exclaim cry shout scream whisper murmur mutter remark sigh laugh sob groan come go live appear arrive
patient_animate: marry thank forgive persuade convince punish praise scold comfort frighten scare obey betray greet console bribe flatter
```
