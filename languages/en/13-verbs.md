# 13. Verbs

The English verb has at most **five forms** (*walk, walks, walked, walked, walking*; *sing, sings, sang, sung, singing*), eight for *be*, and only one or two for the modals. Everything else (future, perfect, progressive, passive, most of the subjunctive) is expressed with auxiliaries. This chapter gives the forms and their spelling and sound (§§1–3), the paradigms of *be, have, do* (§4), the modals and semi-modals (§§5–6), the periphrastic forms and the features UD gives them (§7), the non-finite forms (§8), negation and contraction (§9), and how Mova lemmatizes, tags and tokenizes verbs (§10). The irregular verbs have their own chapter ([14-irregular-verbs.md](14-irregular-verbs.md)).

## 1. The five forms

| form | traditional name | PTB | UD FEATS (EWT) | regular | irregular | uses |
|---|---|---|---|---|---|---|
| base | infinitive, plain form | VB, VBP | `VerbForm=Inf` (VB after *to*/modal); `Mood=Ind\|Tense=Pres\|VerbForm=Fin` (VBP); `Mood=Imp` (imperative); `Mood=Sub` (subjunctive) | *walk* | *sing, go, be* | infinitive, imperative, subjunctive, present tense except 3rd sg |
| *-s* form | 3rd person singular present | VBZ | `Mood=Ind\|Number=Sing\|Person=3\|Tense=Pres\|VerbForm=Fin` | *walks* | *sings, goes, is, has, does* | present with a 3rd-singular subject |
| past | preterite, simple past | VBD | `Mood=Ind\|Tense=Past\|VerbForm=Fin` | *walked* | *sang, went, was/were* | past tense; past subjunctive (*if I were*) |
| past participle | *-ed* participle, *-en* participle | VBN | `Tense=Past\|VerbForm=Part` | *walked* | *sung, gone, been* | perfect (*has sung*), passive (*was sung*), adjectival (*a broken cup*) |
| *-ing* form | present participle, gerund | VBG | `VerbForm=Ger` or `Tense=Pres\|VerbForm=Part` | *walking* | *singing, going, being* | progressive (*is singing*), gerund (*singing is fun*), participle clauses |

Sweet NEG I §1290 describes the regular verb by its three endings *-s, -ed, -ing* and notes that the base serves as present, subjunctive, imperative and infinitive. In UD English EWT + ESLSpok train (`en/data/forms.tsv`, counted 2026-10-17) the verb tags have these shares of all tokens: VB 4.6%, VBD 2.6%, VBP 2.6%, VBZ 2.2%, VBN 1.9%, VBG 1.6%, MD 1.6%.

Reynolds 2026 (§5.4, and ch. 5 generally) uses the CGEL names: *plain form, 3rd singular present, preterite, past participle, gerund-participle*. He counts two tenses (present and preterite) and treats the perfect and progressive as aspects formed with auxiliaries (§§5.5–5.8).

## 2. The regular inflections

### 2.1 The *-s* form

| base ends in | add | examples | sound |
|---|---|---|---|
| ⟨s x z ch sh⟩ | *-es* | *misses, fixes, buzzes, watches, pushes* | /ɪz/ |
| silent ⟨e⟩ after a sibilant | *-s* | *rises, judges* | /ɪz/ |
| consonant + ⟨y⟩ | *-ies* | *carries, tries, flies* | /z/ |
| ⟨o⟩ after a consonant | *-es* | *goes, does, echoes, vetoes* | /z/ |
| anything else | *-s* | *walks* /s/, *runs, plays, sees* /z/ | /s/ or /z/ |

Irregular *-s* forms: *is* (*be*), *has* (*have*), *does* /dʌz/ (*do*), *says* /sez/ (*say*; vowel change in speech only). The modals have none (*he can*, not \**cans*), and *need* and *dare* lose it when used as modals (*he need not go*) (Sweet NEG I §§1290, 1293, 1478, 1487, 1492–1493; Whitney §243; seed `en/seeds/morph/verb-vbz-3sg.md`).

The **third person singular is the only person ending** left in the present. Non-standard and dialectal varieties regularize it in either direction: zero *-s* (*he go*, East Anglia, African American English) or *-s* throughout (*I goes, they says*: the Northern Subject Rule and South-West England) ([08](08-national-varieties.md) §12; Hejná & Walkden 2022 §2.3.1).

### 2.2 The past and past participle *-ed*

| base ends in | add | examples | sound |
|---|---|---|---|
| ⟨e⟩ | *-d* | *loved, hoped, agreed* | /d/, /t/ |
| consonant + ⟨y⟩ | *-ied* | *carried, tried* | /d/ |
| stressed vowel + single consonant | double + *-ed* | *stopped, preferred* | /t/, /d/ |
| ⟨c⟩ | *-ked* | *panicked* | /t/ |
| ⟨t d⟩ | *-ed* | *wanted, needed* | /ɪd/ |
| other | *-ed* | *walked* /t/, *played* /d/ | /t/ or /d/ |

Exceptions in spelling: *laid, paid, said* (not \**layed*); in sound: *-ed* adjectives with /ɪd/ (*aged, learned, blessed*; [04](04-connected-speech.md) §6.2). All new verbs are regular: *googled, texted, emailed, friended, zoomed* (Sweet NEG I §§1286–1287 already: "all new verbs are regular"). Spelling details: [06](06-spelling-rules.md) §§2–5; seed `en/seeds/morph/verb-ed-regular.md`.

### 2.3 The *-ing* form

| base ends in | rule | examples |
|---|---|---|
| silent ⟨e⟩ | drop it | *making, hoping, coming* |
| ⟨ee, ye, oe⟩ | keep | *seeing, dyeing, hoeing* |
| ⟨ie⟩ | → *y* | *lying, dying, tying* |
| stressed vowel + single consonant | double | *running, beginning* |
| ⟨c⟩ | add ⟨k⟩ | *panicking, picnicking* |
| other | add *-ing* | *walking, playing, carrying* |

## 3. Irregular verbs in brief

About 150 simple verbs (and many more prefixed and compound ones: *understand, overcome, forgive*) have irregular past or participle forms: *sing–sang–sung, go–went–gone, put–put–put, keep–kept–kept, bring–brought–brought*. They are among the most frequent verbs. Classes, the full table and variants are in [14-irregular-verbs.md](14-irregular-verbs.md).

## 4. *Be*, *have*, *do*

These three are both lexical verbs and auxiliaries. UD tags them AUX when they are auxiliaries (and *be* also as the copula), VERB otherwise (seed `en/seeds/verbal/copula-be-only.md`: only *be* is a copula in UD English).

### 4.1 *Be*

| | present | past | negative contracted | notes |
|---|---|---|---|---|
| 1 sg | *am* ('m) | *was* | *I'm not*; *aren't I?* (question tag); *amn't* (Scotland, Ireland); *ain't* (non-standard) | |
| 2 sg / all plural | *are* ('re) | *were* | *aren't, weren't* | |
| 3 sg | *is* ('s) | *was* | *isn't, wasn't* | |
| base / infinitive | *be* | | | subjunctive *be* (*if it be*), imperative *be quiet* |
| past subjunctive | | *were* (all persons) | | *if I were you*; informal *if I was* |
| participles | *being* | *been* | | |
| archaic | *art* (thou), *be* (pl.) | *wast, wert* | | [09](09-early-modern-and-archaic.md) §3 |

*Be* is the only verb with person agreement beyond the 3rd singular (*am*) and number agreement in the past (*was/were*). Sweet NEG I §§1488–1491 and Jespersen MEG VI 5.6 (three roots: *be*, *is*, *was*); seed `en/seeds/morph/verb-be-forms.md`. Non-standard *you was, we was* and *it weren't* level the past ([08](08-national-varieties.md) §12).

Uses as auxiliary: progressive (*is singing*), passive (*was sung*), *be to* (*you are to leave*), *be going to* (§6), archaic perfect with verbs of motion (*he is come*).

### 4.2 *Have*

Forms: *have, has, had, had, having*; contracted *'ve, 's, 'd*; negative *haven't, hasn't, hadn't*. *Has* and *had* are irregular only in spelling and sound: the stem loses /v/ (Sweet NEG I §1492; Jespersen MEG VI 4.1₄).

Uses:

- **perfect auxiliary**: *she has gone, they had left, to have seen* (UD AUX);
- **possession and other lexical uses**: *I have a car* (VERB). In questions and negatives, lexical *have* takes *do*-support in AmE and increasingly in BrE (*Do you have a car? I don't have one*); the older British pattern is *Have you a car? I haven't a car*, and the most common informal British form is ***have got*** (*Have you got a car? I've got one*). *Have got* is present in meaning (*got* here is not past);
- **causative and experiential**: *have something done* (*I had my hair cut*); *have someone do something*;
- **obligation**: *have to, have got to* (§6).

### 4.3 *Do*

Forms: *do, does* /dʌz/, *did, done, doing*; negative *don't, doesn't, didn't*. Uses:

- **do-support**: the dummy auxiliary in questions, negatives, emphasis and ellipsis when no other auxiliary is present: *Do you know? I don't know. I DO know. – Yes, I do.* (seed `en/seeds/verbal/do-support.md`). Hejná & Walkden 2022 §4.4.1 trace its rise in Early Modern English ([09](09-early-modern-and-archaic.md) §3.5);
- **lexical *do***: *do homework, do the dishes* (VERB); it needs *do*-support itself: *Did you do it?*;
- **pro-verb**: *She sings better than I do.*

Non-standard *he don't* (for *doesn't*) is old and widespread in speech.

## 5. Modal auxiliaries

### 5.1 The central modals

| present | past form | negative contracted | meanings (main) |
|---|---|---|---|
| *can* | *could* | *can't* (BrE /kɑːnt/, AmE /kænt/), *cannot* (one word), *couldn't* | ability, permission, possibility |
| *may* | *might* | *mayn't* (rare), *mightn't* (BrE) | permission, possibility |
| *shall* | *should* | *shan't* (BrE), *shouldn't* | future (1st person, BrE, formal), offers (*Shall I?*), obligation (legal); *should*: advice, expectation |
| *will* | *would* | *won't*, *wouldn't* | future, willingness, habit; *would*: conditional, past habit, polite requests |
| *must* | — | *mustn't* /ˈmʌsnt/ | obligation, logical necessity |
| *ought (to)* | — | *oughtn't* | advice, expectation |
| *need* (modal use) | — | *needn't* | necessity in negative and questions: *You needn't come. Need I go?* |
| *dare* (modal use) | (*dared*) | *daren't* | *How dare you! I daren't tell him.* |

**Morphological properties** of the modals (Sweet NEG I §§1477–1487; Jespersen MEG VI 3.8–3.9; Reynolds 2026 §5.3):

- no *-s* in the 3rd person singular (*she can*, \**cans*);
- no infinitive, no participles, no *-ing* form: \**to can*, \**canning*, \**has could*; the gaps are filled by periphrases: *be able to, be allowed to, have to* (*I'd like to be able to swim; she has had to leave*);
- followed by the bare infinitive (*can go*, not \**can to go*; except *ought to*); seed `en/seeds/verbal/modal-bare-infinitive.md`;
- negation with *not* directly after them and inversion in questions without *do* (*Can you? You can't.*);
- the "past" forms *could, might, should, would* are past in time only in some uses (*I could swim when I was five*); mostly they express tentativeness or hypothesis (*Could you help me? It might rain*);
- historically, *can, may, shall, must, ought, dare* and the old *wot* are **preterite-present** verbs: old past forms with present meaning, which is why they lack the *-s* of the present (Sweet NEG I §§1477–1487).

UD tags them AUX with PTB MD and the single feature `VerbForm=Fin` (seed `en/seeds/morph/verb-md-modals.md`); EWT has MD 3 292 times in train (*will* 834 as MD).

### 5.2 Variation

- ***shall*** is rare in AmE except in offers and legal texts; in BrE it survives in first-person questions and formal future.
- ***need*** and ***dare*** are both modal (*needn't, daren't*) and lexical (*doesn't need to, doesn't dare to*); the lexical use is now dominant, the modal mostly British and formal (seed `en/seeds/verbal/need-dare.md`, after Poutsma 1923 §§6–31).
- **Double modals** (*might could, may can*) occur in the US South and in Scotland and northern England.
- ***ought to*** competes with *should*; negative and question forms (*oughtn't, ought we to?*) are formal.

## 6. Semi-modals and other periphrases

Verb combinations that behave like modals but inflect like lexical verbs:

| form | meaning | notes |
|---|---|---|
| *have to* /ˈhæftə/, *has to* /ˈhæstə/ | obligation | lexical: *do*-support (*Do I have to?*); UD VERB (*have*) + `xcomp` |
| *have got to*, *gotta* | obligation (informal) | |
| *be going to*, *gonna* | future, intention, prediction from present evidence | EWT tokenizes *gonna* as *gon* + *na* |
| *be able to* | ability | fills the gaps of *can* |
| *be allowed to* | permission | |
| *be supposed to* | expectation, obligation | |
| *used to* /ˈjuːstə/ | past habit or state | negative *didn't use(d) to*; BrE also *usedn't to* |
| *had better*, *'d better* | advice, warning | followed by a bare infinitive |
| *would rather*, *'d rather* | preference | |
| *be about to* | immediate future | |
| *want to*, *wanna* | desire | informal *wanna* |

Hejná & Walkden 2022 §3.4.2 traces the growth of the semi-modals (*have to, be going to, want to, need to*) in Late Modern and present-day English at the expense of the central modals; seed `en/seeds/verbal/semi-modals.md`.

## 7. Periphrastic forms: tense, aspect, voice, mood

### 7.1 The system

| name | active | passive | auxiliary chain |
|---|---|---|---|
| present simple | *writes* | *is written* | — / *be* + VBN |
| present progressive | *is writing* | *is being written* | *be* + VBG |
| present perfect | *has written* | *has been written* | *have* + VBN |
| present perfect progressive | *has been writing* | (*has been being written*, rare) | *have* + *been* + VBG |
| past simple | *wrote* | *was written* | |
| past progressive | *was writing* | *was being written* | |
| past perfect | *had written* | *had been written* | |
| past perfect progressive | *had been writing* | | |
| future (with *will*) | *will write* | *will be written* | modal + VB |
| future progressive | *will be writing* | | |
| future perfect | *will have written* | *will have been written* | |
| conditional | *would write* | *would be written* | |
| conditional perfect | *would have written* | *would have been written* | |

The order of auxiliaries is fixed: **modal – perfect *have* – progressive *be* – passive *be* – lexical verb**, each auxiliary selecting the form of the next (modal → base; *have* → past participle; progressive *be* → *-ing*; passive *be* → past participle): *It **might have been being** repaired.* Reynolds 2026 §5.8 lays out the same system as two tenses times two aspects plus modality; the syntax chapters (chapters [20](20-syntax-overview.md)–38) treat their meanings.

**The *get*-passive**: *get* + past participle (*he got fired*), informal and often adversative; UD tags *get* as AUX with `aux:pass` in EWT (seed `en/seeds/verbal/get-passive.md`); Hejná & Walkden 2022 §2.4.2 describe its spread.

### 7.2 How UD annotates them

UD English does not mark tense or aspect on the whole construction; it annotates each word: the auxiliaries (AUX) carry their own finite features, the lexical verb its non-finite form. *has been writing* = *has* AUX VBZ `Tense=Pres|VerbForm=Fin`, *been* AUX VBN `Tense=Past|VerbForm=Part`, *writing* VERB VBG `Tense=Pres|VerbForm=Part`. A meaning layer that reconstructs "present perfect progressive" from this chain is part of Mova's frames, not of `en` (docs/architecture.md).

### 7.3 Mood

- **Indicative**: the ordinary finite forms.
- **Imperative**: the base form, without a subject (*Sit down!*), negated with *don't* (*Don't go!*), emphatic *do* (*Do sit down*); *let's* for the 1st plural (*Let's go*; *'s* lemma *us*/*we*). UD `Mood=Imp`.
- **Subjunctive**: the **present subjunctive** is the base form in all persons, used in *that*-clauses after verbs and adjectives of demand, suggestion, necessity (the **mandative subjunctive**: *I suggest that he **see** a doctor; It is essential that she **be** informed*), more in AmE than BrE (which prefers *should see*), and in fixed phrases (*God save the King, be that as it may, so be it, come what may, if need be*). The **past subjunctive** *were* appears in unreal conditions and wishes: *if I were you, I wish it were Friday, as if he were*. UD `Mood=Sub` (seed `en/seeds/verbal/subjunctive.md`; Hejná & Walkden 2022 §3.3.2 on the "subjunctive" in Late Modern English). The past tense of other verbs in unreal conditions (*If I had a car…*) is the ordinary past, used modally.

## 8. Non-finite forms

| form | example | UD |
|---|---|---|
| bare infinitive | *I can **go**; let him **go**; make her **laugh**; help me **carry** it* | VB, `VerbForm=Inf` |
| *to*-infinitive | *I want **to go**; easy **to read**; in order **to win*** | *to* PART (TO) + VB |
| split infinitive | *to **boldly** go* | (accepted; old prescriptive rule) |
| perfect infinitive | *to have gone* | |
| gerund | ***Swimming** is fun; I enjoy **reading**; after **leaving*** | VBG, `VerbForm=Ger` |
| present participle | *a **sleeping** child; **Walking** home, I saw…* | VBG, `VerbForm=Part`, `Tense=Pres` |
| past participle | *a **broken** window; **Built** in 1900, the house…* | VBN, `VerbForm=Part`, `Tense=Past` |

The infinitive marker *to* can stand alone when the verb is understood (*I didn't want to*), and it is not a preposition (`en/seeds/verbal/to-infinitive-marker.md`); *to* before an *-ing* form is a preposition (*I look forward to seeing you*; `to-before-ing.md`).

## 9. Negation and contraction

### 9.1 Negative forms

Finite auxiliaries are negated by *not*, usually contracted to *n't* in speech and informal writing. Several stems change in contraction:

| full | contracted | stem change |
|---|---|---|
| *is not, are not, was not, were not* | *isn't, aren't, wasn't, weren't* | |
| *am not* | — (*aren't I?* in questions; *amn't* Scot./Ir.; *ain't*) | |
| *have not, has not, had not* | *haven't, hasn't, hadn't* | |
| *do not, does not, did not* | *don't* /dəʊnt/, *doesn't, didn't* | *do* /duː/ → /dəʊ/ |
| *can not / cannot* | *can't* | BrE /kɑːnt/ |
| *will not* | *won't* /wəʊnt/ | from older *wol not* |
| *shall not* | *shan't* | *ll* lost |
| *could, would, should, might, must, need, dare, ought not* | *couldn't, wouldn't, shouldn't, mightn't, mustn't, needn't, daren't, oughtn't* | *must* loses /t/ |
| (non-standard) *am/is/are not; have/has not* | *ain't* | |

Hejná & Walkden 2022 §3.3.1 describe the history of negative contraction in Late Modern English. Contracted *n't* behaves as an inflection of the auxiliary: it inverts with it (*Isn't it? Don't you know?*), while full *not* stays after the subject (*Is it not?*).

### 9.2 Auxiliary contractions

| full | contracted | after |
|---|---|---|
| *am* | *'m* | *I* only |
| *are* | *'re* | pronouns, *there, what, who* |
| *is, has* | *'s* | any noun, pronoun, *there, here, what, where, how, that* (not after a sibilant: \**the bus's late*) |
| *have* | *'ve* | pronouns, modals (*could've, should've*) |
| *will, shall* | *'ll* | pronouns, nouns (informal) |
| *would, had* | *'d* | pronouns (*I'd, you'd, they'd*) |
| *us* | *'s* | *let's* only |

The ambiguity of *'s* (*is, has, us*, genitive) and *'d* (*would, had*) is resolved by what follows: *he's gone* (has + participle), *he's going* (is + *-ing*), *he's tall* (is), *he'd gone* (had + participle), *he'd go* (would + base) (seed `en/seeds/morph/verb-clitics.md`).

### 9.3 Restrictions

- A contracted auxiliary cannot be stressed or stand at the end of a clause: *Yes, I am* (\**Yes, I'm*); *I don't know where he is* (\**…where he's*) ([04](04-connected-speech.md) §2).
- Two contractions on one auxiliary are possible in speech (*I'd've, wouldn't've*), rare in writing.
- **Double negatives** (*I didn't see nothing*) are standard in many dialects and in older English, stigmatized in the written standard (seed `en/seeds/nominal/no-none.md`; `en/seeds/errors/double-negation.md`).

## 10. How Mova handles verbs

- **Tokenization** (`en/src/tok.rs`): the clitics *n't, 's, 're, 've, 'll, 'd, 'm* (constant `CLITICS`) are split off by rule, also with the typographic apostrophe; the stem left behind keeps its form: *do|n't*, *ca|n't*, *wo|n't*, *sha|n't*, *ai|n't*. *cannot* is split *can|not*, as in EWT (a fix found by the absurdity matrix on 2026-10-02, when *cannot help* was read as a verb *cannot* with an object; comment in `tok.rs`).
- **Lemmas of contracted forms** come from the UD counts in the lexicon: *ca* → *can* (78 tokens in EWT + ESLSpok train), *wo* → *will* (48), *'ll* → *will* (149), *'m* → *be* (197), *'re* → *be* (101), *'ve* → *have* (123), *'d* → *would* (42) or *have* (11), *'s* → *be* (329) or *have* (15) or genitive (550), *n't* → *not* (649), *ai* → *be* (4) (`en/data/forms.tsv`, counted 2026-10-17).
- **Lemmatization and inflection** (`en/src/morph.rs`): `Morph::lemmatize(form, tag)` first applies `archaic` (Early Modern forms: [09](09-early-modern-and-archaic.md) §5), then the most frequent UD analysis of the form with that tag, then the AGID paradigm, then reverse suffix rules (`unsuffix`: *-ied → -y*, *-ed*, *-d*, undoubling; *-ying → -ie*, *-ing*, *-ing + e*, undoubling; *-ies → -y*, *-es*, *-s*). `Morph::inflect(lemma, tag)` goes the other way (UD, then AGID, then `suffix`); [06](06-spelling-rules.md) §11 lists what the fallback rules do and do not cover (no consonant doubling for unknown verbs). Lemma accuracy on UD test sets is 99.3–99.5% (`en/README.md`).
- **The paradigm slot** of a tag is `Slot::of(tag)`: VBD → Past, VBN → PastPart, VBG → PresPart, VBZ → ThirdSg; VB and VBP share the base. The lexicon stores irregular pasts and participles separately, so *put* (VBD) and *put* (VBN) both lemmatize to *put* and *went → go*, *gone → go* (unit test `dictionary`).
- **AUX vs VERB** and the features (`Mood`, `Tense`, `VerbForm`, `Person`, `Number`) are assigned from the PTB tag and context by tables; the closed list of auxiliaries is in `en/seeds/verbal/aux-closed-list.md`, and rules in `perfect-have.md`, `progressive-be-ing.md`, `passive-aux.md`, `modal-bare-infinitive.md` check the auxiliary chain.
- **Not yet**: *gonna, wanna, gotta* are not split by the tokenizer itself; *amn't, ain't* as general negatives are handled only as far as EWT shows them; tense–aspect meanings are not computed in `en` (they are read into frames later).

## Sources for this chapter

- Sweet 1891, NEG I §§1286–1293 (regular verbs), §§1477–1493 (anomalous verbs: modals, *be, have, do*). https://archive.org/details/newenglishgramma01sweeuoft
- Jespersen 1942, MEG VI chapters II–V (personal endings, tense formation), 3.8–3.9 (modals without *-s*), 4.1–4.2, 5.6 (*be*). https://archive.org/details/modernenglishgra06jesp
- Whitney 1877, §§234–244. https://archive.org/details/essentialsofengl01whit
- Poutsma, H. 1921/1923. *The Characters of the English Verb*; *The Infinitive, the Gerund and the Participles of the English Verb.* Library `books/p/po/poutsma-1921-characters-english-verb`.
- Hejná & Walkden 2022, §2.3.1, §2.4.2 (*get*-passive), §3.3.1 (negative contraction), §3.3.2 (subjunctive), §3.4.2 (semi-modals), §4.4.1 (*do*-support). https://zenodo.org/records/6560337
- Reynolds 2026, ch. 5 (§5.3 auxiliaries, §§5.5–5.8 tense and aspect). https://zenodo.org/records/20612113
- Universal Dependencies, English verbs and auxiliaries: https://universaldependencies.org/en/pos/AUX_.html ; features `VerbForm`, `Mood`, `Tense`: https://universaldependencies.org/u/feat/
- Santorini 1990, PTB tagging guidelines (VB, VBD, VBG, VBN, VBP, VBZ, MD). https://repository.upenn.edu/cis_reports/570/
- Mova: `en/src/tok.rs`, `en/src/morph.rs`, `en/data/forms.tsv`, `en/README.md`; seeds `en/seeds/morph/verb-*.md`, `en/seeds/verbal/*.md`.
