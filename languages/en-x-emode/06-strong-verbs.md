# 6. Strong verbs, old pasts and participles, old plurals

English has been regularising its verbs for a thousand years. EModE stands in the middle of the process: many
verbs still had the old vowel-change («strong») forms, some had both, and some strong forms of 1611 have since
gone weak (`holpen` → `helped`) or changed vowel (`spake` → `spoke`). The biblical register freezes a selection
of them. Hejná & Walkden (2022, §4.3.2 and §5.3.2) give the background: frequent verbs resist regularisation;
`holpen` for `helped` is their example of a form a reader meets in EModE.

## 6.1 The forms that appear in Mova's texts

Counts: tales pool (132 697 sentences) / KJV Torah / JPS Torah, whole-word, case-insensitive, 2026-10-17. The
last column is what `Morph::lemmatize` returns with the tag shown (a Rust probe linked to the `en` crate of this
branch, 2026-10-17); **bold** marks a wrong lemma.

| Form | Modern | Lemma | Tales | KJV | JPS | `lemmatize` gives |
|---|---|---|---:|---:|---:|---|
| spake | spoke | speak | 48 | 213 | 0 | speak (VBD) |
| brake | broke | break | 4 | 4 | 0 | break |
| bare | bore | bear | 137 (mostly the adjective) | 70 | 1 | **bare** |
| sware | swore | swear | 0 | 41 | 0 | **sware** |
| clave | cleaved, clung | cleave | 1 | 3 | 0 | **clave** |
| gat | got | get | 0 | 5 | 0 | **gat** |
| begat | begot | beget | 0 | 71 | 0 | **begat** |
| begot | — | beget | | 0 | 71 | beget |
| drave | drove | drive | 0 | 1 | 0 | **drave** |
| sate | sat | sit | 15 | 0 | 0 | **sate** |
| bade / bad | bid (past) | bid | 495 / — | 3 | 3 | **bade** / **ba** |
| trod / trode | trod | tread | 59 / 0 | 0 | 0 | tread / **trode** |
| clomb | climbed | climb | 14 | 0 | 0 | **clomb** |
| holp / holpen | helped | help | 3 / 0 | 0 | 0 | **holp** / **holpen** |
| durst | dared | dare | 9 | 0 | 0 | **durst** |
| quoth | said | quoth (defective) | 71 | 0 | 0 | quoth |
| wist / wot | knew / know | wit | 2 / 2 | 4 / 5 | 0 / 0 | wit / **wot** |
| abode | abided, stayed | abide | 90 | 17 | 15 | abide (as a verb; *abode* is also a noun) |
| slew | slew | slay | 68 | 18 | 15 | slay |
| smote | smote | smite | 36 | 30 | 36 | smite |
| smitten | smitten | smite | | | 15 | smite |
| wrought | worked | work | 30 | 17 | 19 | **wrought** |
| strove | strove | strive | | | 10 | strive |
| builded | built | build | 0 | 9 | 11 | **builde** |
| digged | dug | dig | 5 | 15 | 12 | **digg** |
| shew / shewed | show / showed | show | 4 / 5 | 27 / 23 | 0 / 0 | **shew** / **shew** |
| gotten | got | get | 20 | 10 | 10 | get |
| stricken | struck | strike | 25 | 2 | 2 | **stricken** |
| laden | loaded | lade | 69 | 2 | 2 | lade |
| sod | seethed, boiled | seethe | 10 (mostly *sod* = turf) | 1 | 1 | **so** |
| sodden | boiled | seethe | 1 | 4 | 4 | **sodden** |
| waxed / waxen | grew / grown | wax | 14 / 2 | 11 / 8 | 11 / 10 | wax / **waxen** |
| hid (participle) | hidden | hide | | | 12 | hide |
| hewn | hewn | hew | | | 2 | **hewn** |
| graven | carved | grave | | | 12 | grave |
| molten | melted, cast | melt | | | 8 | **molten** |
| shapen | shaped | shape | | | | **shapen** |

Sources: forms from the texts; historical status from Abbott 1884 §§339–345 (past indicative in `u`: «sung» for
*sang*, «begun» for *began*; participles in `-en`; participle formation), and from the OED-style glossaries in
«wanted» (Wright's *Bible Word-Book*).

**The JPS Torah needs only a few of these**: `smote`, `slew`, `wrought`, `builded`, `digged`, `sod`, `sodden`,
`waxen`, `graven`, `molten`, `hewn`, `hid`, `gotten`, `abode`. The tales add `spake`, `bade`, `quoth`, `sate`,
`clomb`, `durst`, `trod`, `holp`, `stricken`. The KJV adds `begat`, `bare`, `sware`, `gat`, `clave`, `drave`,
`shew`.

## 6.2 How the db annotates them (2026-10-17)

| Form | db analyses |
|---|---|
| wrought (19) | `wrought` VERB 19 — **wrong lemma** in all 19 (should be *work*) |
| smote (36) | `smite` VERB 26, `smote` VERB 10 |
| abode (15) | `abide` VERB 10, `abode` VERB 5 |
| digged (12) | `dig` VERB 10, `digg` VERB 2 |
| builded (11) | `build` VERB 11 |
| waxen (10) | `waxen` ADJ 7, `wax` VERB 2, `waxen` VERB 1 |
| graven (12) | `graven` ADJ 7, `grave` VERB 2, `grave` ADJ 2, `graven` VERB 1 |
| molten (8) | `molten` ADJ 8 (fine as an adjective: *molten image*) |
| sod (1) | `sod` VERB 1 — **wrong lemma** («And Jacob sod pottage», `jps:GEN:726`, GEN 25.29) |
| sodden (4) | `sodden` VERB 2, `sodden` ADJ 2 — «Eat not of it raw, nor sodden at all with water» (`jps:EXO:319`, EXO 12.9) |
| gotten (10) | `get` VERB 9, `gotten` VERB 1 |
| hewn (2) | `hew` 1, `hewn` 1 |
| begot (71) | `beget` VERB 70 |
| bore (69) | `bear` VERB 68, `bore` VERB 1 |
| slew (15), strove (10), hid (12), smitten (15) | right |

Counted together, the clearly wrong strong-verb lemmas (`wrought`, `smote`, `abode` as a VERB, `digg`, `sod`,
`waxen` VERB, `hewn`, `gotten`, `sodden` VERB, `graven` VERB, `molten`) are **67 tokens in 62 sentences**. The
teacher wrote most of these lemmas; they pass every gate, because the gates check the lemma of
an AUX only.

## 6.3 Participial adjectives: verb or adjective?

`molten image`, `graven image`, `hewn stones`, `sodden flesh`, `waxen poor`, `stricken in years`, `laden with`,
`well stricken in age`. UD English treats a participle used attributively as ADJ when it is lexicalised (*molten*,
*graven*) and as VERB with `VerbForm=Part` when it keeps verbal force. Whichever the choice, the **lemma** of the
VERB reading must be the verb (*melt*, *grave*, *hew*, *seethe*, *wax*, *strike*, *lade*) and the ADJ reading keeps
the form (*molten*, *graven*). The db mixes both (`graven` ADJ 7, VERB 1 with lemma *graven*, VERB 2 with lemma
*grave*). A rule by context is cheap: before a noun (`amod`) → ADJ with lemma = form; after `be`/`been`, or with
an agent or object → VERB with the verb lemma.

## 6.4 Old plurals

| Form | Modern | Lemma | JPS | db |
|---|---|---|---:|---|
| kine | cows | cow | 17 | `kine` NOUN 17 — lemma should be *cow* |
| brethren | brothers (kin, fellow members) | brother | 123 | `brethren` 120, `brother` 2 |
| oxen | oxen | ox | 29 | `ox` — right |
| children | children | child | | right |
| feet, teeth, mice | | | | right |
| fowl (plural) | birds | fowl | | uncountable plural, `Number=Plur` by context |
| fish (plural) | | fish | | |
| swine | pigs | swine | | |
| sheep, deer | | | | invariable |
| hosen, shoon, eyen | stockings, shoes, eyes | hose, shoe, eye | 0 | in older texts only |

`brethren` deserves a decision. It is the old plural of *brother*, specialised for «kinsmen, members of one
people or order». UD lemmatizes inflection, not meaning, so the lemma is *brother* (EWT has no `brethren` to
compare with). The db's 120 `brethren` lemmas would become *brother*; the special meaning belongs in the world
layer, not in the lemma.

## 6.5 Weak verbs with old forms

| Form | Modern | Note |
|---|---|---|
| builded | built | weak past of *build* still in KJV/JPS: «he builded a city» (`jps:GEN:106`, GEN 4.17) |
| digged | dug | `dug` is the newer strong form |
| shaked | shook | Abbott §341 «I shaked» (*Tempest*) |
| quit, waft, sweat (past) | quitted, wafted, sweated | Abbott §341: `-ed` omitted after `t` «for euphony»: «quit the vessel», «waft her love», «service sweat for duty» |
| fast (past) | fasted | Abbott §341 |
| holp | helped | |
| learn (teach) | teach | lexical, not morphological: «learn me» = teach me |

## 6.6 Lemma decisions for Mova

1. The lemma is the modern verb: `spake` → *speak*, `wrought` → *work*, `sod`/`sodden` → *seethe*, `digged` →
   *dig*, `builded` → *build*, `begat` → *beget*, `bare` (with a verb tag) → *bear*, `shew` → *show*, `durst` →
   *dare*, `clomb` → *climb*, `holpen` → *help*, `wist`/`wot` → *wit* (EModE «know»; keep *wit*, not *know*:
   lemmatization is not translation), `quoth` → *quoth* (it has no other forms; a few treebanks use *quethe*).
2. The table must run **only with a verb tag** (`bare`, `sod`, `sate`, `abode`, `laden`, `bade`, `gat` are also
   or mainly adjectives or nouns in modern text), except for forms that are never anything but verbs (`spake`,
   `begat`, `sware`, `clave`, `holpen`, `wrought` — `wrought iron` is ADJ, so `wrought` needs the verb tag).
3. A list of ~50 forms is enough for the biblical register and the tales; it belongs next to `IRREGULAR` in
   `en/src/morph.rs`, or better as a data file (`en/data/archaic.tsv`) that `build.rs` compiles in, so that the
   word list can grow without code changes. Chapter 13 (L4) gives the list.

## 6.7 A negative control

Any rule for old forms must leave the modern homographs alone. From the db and the tales:

- `art` the noun: «the art of the perfumer» (db `art|NOUN` 2, correct), «the art of healing» (round 121 draft,
  `book:pg3027:3607`), «taught in art and magic» (`book:pg2892:2487`) — 437 tales lines with `art`, most of them
  nouns.
- `bare` the adjective: «bare feet» (most of the 137 tales hits).
- `sod` the noun: «under the sod» (tales).
- `abode` the noun: «his abode».
- `laden` the adjective: «laden with gold».
- `bad` the adjective: never *bid*.

The unit test `archaic_verbs` in `en/src/morph.rs` already has this shape (`art` as a noun, `greatest`, `rest`,
`digest`, `interest` must be left alone); every new entry should come with its noun or adjective counterpart in
the test.
