# Plurals in -ves: wife → wives, leaf → leaves

**Gist.** Some nouns in *-f/-fe* voice the final consonant in the plural: *f* → *v*, spelled *-ves*. These are old words with a long vowel or with *l* before *f*: *wife → wives, life → lives, knife → knives, thief → thieves, leaf → leaves, loaf → loaves, half → halves, calf → calves, elf → elves, self → selves, shelf → shelves, wolf → wolves, sheaf → sheaves*. Other nouns in *-f* take ordinary *-s*: *roofs, chiefs, beliefs, proofs, cliffs, safes*.

**Conditions and exceptions.**
- Variation: *scarfs/scarves, hoofs/hooves, dwarfs/dwarves, wharfs/wharves*; *staff* has *staffs* (personnel) and *staves* (rods; giving a new singular *stave*).
- Dangerous overlaps with verbs: *lives* — plural of *life* and 3rd person of *live*; *leaves* — from *leaf* (foliage), from *leave* (noun "time off") and from the verb *leave*; *halves, shelves, calves* — also verbs *halve, shelve, calve*. The part of speech decides the lemma.
- The possessive does not voice: *wife's, calf's*.
- A few words in *-th* voice only in pronunciation: *paths, mouths, oaths* [ðz]; *house → houses* [zɪz].

**Examples.** *three **knives**; they saved many **lives*** (NOUN, lemma *life*); *she **lives** here* (VERB, lemma *live*); *autumn **leaves*** (lemma *leaf*).

**In UD.** NOUN, NNS, `Number=Plur`, lemma — the singular in *-f/-fe* (*wives → wife, wolves → wolf*). Verb homonyms — VERB VBZ with a lemma in *-ve* (*lives → live, leaves → leave*).

**Sources.** Sweet NEG I §1001 (text-1, p. 344–345: voicing only after an old long vowel or *l*; *wife, life, knife, thief, leaf, loaf, half, calf, elf, self, shelf, wolf*; *house*; *bath, path, oath, mouth*), §999 (p. 343: *dwarf, scarf, wharf* now *-fs*; *hoofs, roofs, beliefs*; *staff–staves*; the possessive does not voice); Whitney §124a–b (text-1, p. 71: *halves, leaves, wives, shelves, staves*, but *puffs, cliffs, fifes, hoofs*); Jespersen MEG VI 16.2₁–16.2₅ (text-5, p. 274–278: full list of *-ves*; *hoofs* ordinary, *hooves* poetic; *briefs, chiefs, proofs*; *beeves/beefs*), 16.3₁, 16.4 (p. 278–279: *baths, mouths; houses*); Kruisinga II.2 §§755–758 (text-3, p. 25–27); Mätzner I, p. 223–224 (text-1, p. 241–242).

```rule
rule: en.morph.ves-plural-lemma
what: knives, wives, wolves, halves etc. as a noun — lemma in -f/-fe
match: n[upos=NOUN, form=knives|wives|wolves|halves|shelves|thieves|calves|loaves|elves|selves|sheaves|scarves|hooves|wharves|dwarves, !feats.Typo]
require: n[lemma=knife|wife|wolf|half|shelf|thief|calf|loaf|elf|self|sheaf|scarf|hoof|wharf|dwarf]
severity: error
source: Sweet NEG I §999, §1001; Whitney §124a–b
```

```rule
rule: en.morph.lives-noun-lemma
what: lives as a noun — plural of life
match: n[upos=NOUN, form=lives, !feats.Typo]
require: n[lemma=life]
severity: error
source: Sweet NEG I §1001
```

```rule
rule: en.morph.lives-leaves-verb-lemma
what: lives, leaves as a verb — lemma live, leave
match: v[upos=VERB, form=lives|leaves, !feats.Typo]
require: v[lemma=live|leave]
severity: error
source: Sweet NEG I §1290 (verbal -s is homonymous with the plural)
```

```rule
rule: en.morph.leaves-noun-lemma
what: leaves as a noun — usually from leaf (but leave "time off" is possible)
match: n[upos=NOUN, form=leaves, !feats.Typo]
require: n[lemma=leaf|leave]
severity: warn
source: Sweet NEG I §1001; Whitney §124a
```
