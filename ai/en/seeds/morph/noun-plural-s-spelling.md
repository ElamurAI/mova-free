# Plural in -s/-es: pronunciation and spelling

**Gist.** The ordinary plural of an English noun is the ending *-s*. It has three pronunciations, and the choice depends on the last sound of the stem: [ɪz] after sibilants and hushing sounds (*boxes, churches, judges*), [z] after vowels and voiced sounds (*days, dogs*), [s] after voiceless ones (*cats, cliffs*). In spelling, [ɪz] is written *-es*, and final *-y* after a consonant changes to *-ies*. The same ending (and the same rules) applies to the possessive case and the verb's 3rd person.

**Conditions and exceptions.**
- *-es* is written after *s, x, z, ch, sh* (*kisses, boxes, buzzes, matches, fishes*); if the stem already ends in silent *-e*, only *-s* is added (*horses, judges*).
- Consonant + *y* → *-ies* (*city → cities, spy → spies*); vowel + *y* → *-ys* (*day → days, boy → boys, valley → valleys*). Proper names do not change *y*: *the Kennedys, two Marys*.
- In *-o*: frequent old words — *-oes* (*potatoes, tomatoes, heroes, echoes, cargoes*); words in *-io*, short and borrowed ones — *-os* (*ratios, photos, pianos, zeros, radios*).
- Letters, digits and cited words — often with an apostrophe: *P's and Q's, 9's*; names of decades — *the 1990s* (in EWT this is NNS with `Number=Ptan`).
- *-f/-fe* → *-ves* only in some words (see `noun-plural-f-ves`); irregular plurals — separately (`noun-plural-mutation-en`, `noun-plural-zero`, `noun-plural-foreign`).
- The lemma of a plural is the singular: *cities → city, potatoes → potato, boxes → box*. A lemmatization error is usually in the spelling: *citie*, *potatoe*, *boxe* are incorrect.

**Examples.** *box → boxes; church → churches; city → cities; day → days; hero → heroes; photo → photos.*

**In UD.** NOUN, XPOS NNS, `Number=Plur`; lemma — the singular with restored spelling (*-ies → -y*, *-es → ∅* after sibilants, *-oes → -o*).

**Sources.** Sweet NEG I §1000 (vol. 1 = text-1, p. 344: three pronunciations of *-s*), §1021 (p. 350–351: spelling *-es*, *-ies/-eys*, *-oes/-os*, *the two Marys*, *P's and Q's*); Whitney §123–124, §128 (p. 71–74: pronunciation, *-ies/-ys*, *cargoes* vs *bravos, zeros*, *i's, 9's*, *the Smiths*); Santorini 1990, §2, p. 3 (NNS); https://universaldependencies.org/en/feat/Number.html; Jespersen MEG VI 16.1₁–16.1₆ (vol. 5 = text-5, p. 268–273: three forms of *-s*; *Marys, the Gregorys*; *heroes, potatoes* vs *photos, pianos, solos*; *C's, 8's*); Kruisinga I §§574–582 (text-1, p. 256–258: *cargoes* vs *photos*; *gases*; *the Cogglesbys*); Mätzner I, p. 224 (text-1, p. 242: *keys, journeys*; *monies, attornies* — incorrect).

Engine rules here need a condition on the ending of the **lemma** (for example, "form in *-ies* → lemma in *-y*"), which language v0 does not have; see the section report.
