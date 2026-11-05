# The verb: the system

The Greek verb is the richest part of the language. A finite form shows:
- person and number (with a dual);
- voice (active, middle, passive);
- mood (indicative, subjunctive, optative, imperative);
- tense, which in Greek is two things at once: the time of the action, in the indicative only, and its **aspect**, in every mood.

Beside the finite forms stand the infinitives and participles of each tense and voice ([chapter 10](10-nonfinite.md)). This chapter gives the system as a whole:
- the categories;
- the tense stems and how they are built (augment, reduplication, the suffixes);
- the endings;
- the thematic verb λύω as the model;
- the principal parts by which a verb is learnt.

The contract verbs are in [chapter 8](08-contract-verbs.md), the athematic (-μι) verbs and the irregular ones in [chapter 9](09-mi-verbs.md).

## Contents

1. The categories
2. Aspect and time
3. The tense systems and their stems
4. The augment
5. Reduplication
6. The endings
7. The model: λύω
8. Voice: active, middle, passive
9. The principal parts
10. How the treebanks annotate the verb
11. How Mova handles it
12. Sources

---

## 1. The categories

| category | values |
|---|---|
| person | 1st, 2nd, 3rd |
| number | singular, dual (2nd and 3rd person, rare outside Homer and Attic verse), plural |
| voice | active, middle, passive (a separate passive only in the aorist and the future) |
| mood | indicative, subjunctive, optative, imperative; and the non-finite infinitive, participle and verbal adjectives |
| tense | present, imperfect, future, aorist, perfect, pluperfect, future perfect |

The finite verb has its subject in its ending, and a pronoun subject is left out unless stressed ([chapter 6](06-pronouns.md), §1) (smyth-1920 §§355–357).

## 2. Aspect and time

Every Greek verb form shows the **stage of the action**, its aspect (smyth-1920 §§1850–1852):
- **imperfective** (the present stem): the action as going on, repeated or attempted: λύω "I am loosing", ἔλυον "I was loosing, I kept loosing";
- **perfective** (the aorist stem): the action as a whole, a simple event, without regard to its duration: ἔλυσα "I loosed";
- **stative** (the perfect stem): the state that results from a completed action: λέλυκα "I have loosed (and it stays loosed)"; οἶδα "I know" (originally "I have seen").

**Time** is shown only in the indicative: the augment marks the past (§4), the future has its own stem. In the other moods and the infinitive, present and aorist differ in aspect only: λῦσαι "to loose (once)" against λύειν "to be loosing, to loose (as a process)". The difference is lost in English, which has a past tense but no perfective infinitive.

| aspect | non-past | past (with augment) |
|---|---|---|
| imperfective | present λύω | imperfect ἔλυον |
| perfective | (subjunctive, optative, imperative, infinitive, participle only) | aorist ἔλυσα |
| stative | perfect λέλυκα | pluperfect ἐλελύκη |
| (future) | future λύσω | — |

The **aorist indicative** is the narrative tense ("he did"), the imperfect the tense of background and repeated action ("he was doing, used to do"), and the perfect the tense of present result. In the narrative of the Septuagint (PTNK training set, Genesis and Ruth, UD 2.18, counted 2026-10-22), the aorist is 1,264 of the 1,942 indicatives with a tense, the imperfect 150 and the perfect 61.

## 3. The tense systems and their stems

The forms of a verb are built on up to six stems, the **tense systems** (smyth-1920 §§383–462):

| system | built from | tenses | λύω |
|---|---|---|---|
| present | the present stem | present, imperfect | λυ- |
| future | stem + σ | future active and middle | λυσ- |
| aorist | stem + σα (first aorist) or a bare stem (second aorist) | aorist active and middle | λυσα- |
| perfect active | reduplication + stem + κα (first perfect) or + α (second perfect) | perfect, pluperfect active | λελυκ- |
| perfect middle | reduplication + stem | perfect, pluperfect middle-passive; future perfect | λελυ- |
| aorist passive | stem + θη (first) or η (second) | aorist and future passive | λυθη- |

**The second aorist** is built on a bare stem with the thematic vowel, often a different grade of the root from the present: λείπω "leave", aorist ἔλιπον; βάλλω "throw", ἔβαλον; λαμβάνω "take", ἔλαβον. The present stem often adds a suffix to the root (λαμβ-άν-ω, βάλλω < \*βαλ-ι̯ω, γι-γνώ-σκω), and the aorist shows the root without it ([chapter 1](01-script-sounds.md)).

## 4. The augment

The augment marks the past in the indicative of the imperfect, the aorist and the pluperfect (smyth-1920 §§428–461):
- **syllabic**: ε- before a consonant: λύω → ἔλυον, ἔλυσα;
- **temporal**: before a vowel, the vowel is lengthened: ἄγω → ἦγον, ἐλπίζω → ἤλπιζον, οἰκέω → ᾤκουν;
- **in compound verbs** it stands between the prefix and the verb: ἀπο-βάλλω → ἀπ-έ-βαλον, προσ-βάλλω → προσ-έ-βαλλον; a prefix ending in a vowel loses it (ἀπο- → ἀπ-), and ἐκ becomes ἐξ before it (ἐξ-έ-βαλον).

Homer and the poets may leave the augment out. The augment is the only sign that tells an aorist indicative from an aorist subjunctive or infinitive with the same stem: ἔλυσα against λῦσαι.

## 5. Reduplication

The perfect system repeats the first consonant of the stem with ε (smyth-1920 §§439–448):
- λύω → λέλυκα, παιδεύω → πεπαίδευκα, γράφω → γέγραφα;
- an aspirate is repeated as its plain stop: φιλέω → πεφίληκα, θύω → τέθυκα;
- before two consonants, a double consonant or ῥ, the reduplication is only ε-: στέλλω → ἔσταλκα, ζητέω → ἐζήτηκα, ῥίπτω → ἔρριφα;
- before a vowel it is the temporal augment: ἄγω → ἦχα (and the Attic reduplication ἀκήκοα, ἐλήλυθα).

Reduplication is kept in every mood of the perfect, so a reduplicated form is a perfect wherever it stands. The present stems of some -μι verbs have a reduplication with ι (δί-δωμι, τί-θημι, ἵ-στημι) ([chapter 9](09-mi-verbs.md)).

## 6. The endings

**Primary** endings stand in the present, the future and the perfect (the non-past tenses) and in the subjunctive. **Secondary** endings stand in the augmented tenses and in the optative (smyth-1920 §§462–467).

| | active primary | active secondary | middle primary | middle secondary |
|---|---|---|---|---|
| 1 sg. | -ω, -μι | -ν | -μαι | -μην |
| 2 sg. | -εις, -ς | -ς | -ῃ, -ει (< -σαι) | -ου (< -σο) |
| 3 sg. | -ει, -σι | — | -ται | -το |
| 1 pl. | -μεν | -μεν | -μεθα | -μεθα |
| 2 pl. | -τε | -τε | -σθε | -σθε |
| 3 pl. | -ουσι (< -ντι) | -ν, -σαν | -νται | -ντο |

The **subjunctive** lengthens the thematic vowel (λύωμεν, λύητε); the **optative** has the mood sign ι or ιη (λύοιμι, λύοιμεν; λυθείην).

## 7. The model: λύω

The thematic verb λύω "loose, release" (smyth-1920 §383), in its indicative:

| | present | imperfect | future | aorist | perfect |
|---|---|---|---|---|---|
| 1 sg. | λύω | ἔλυον | λύσω | ἔλυσα | λέλυκα |
| 2 sg. | λύεις | ἔλυες | λύσεις | ἔλυσας | λέλυκας |
| 3 sg. | λύει | ἔλυε(ν) | λύσει | ἔλυσε(ν) | λέλυκε(ν) |
| 1 pl. | λύομεν | ἐλύομεν | λύσομεν | ἐλύσαμεν | λελύκαμεν |
| 2 pl. | λύετε | ἐλύετε | λύσετε | ἐλύσατε | λελύκατε |
| 3 pl. | λύουσι(ν) | ἔλυον | λύσουσι(ν) | ἔλυσαν | λελύκασι(ν) |

The other moods of the present and the aorist:

| | present | aorist |
|---|---|---|
| subjunctive | λύω, λύῃς, λύῃ | λύσω, λύσῃς, λύσῃ |
| optative | λύοιμι, λύοις, λύοι | λύσαιμι, λύσαις (λύσειας), λύσαι (λύσειε) |
| imperative | λῦε, λυέτω | λῦσον, λυσάτω |
| infinitive | λύειν | λῦσαι |
| participle | λύων, λύουσα, λῦον | λύσᾱς, λύσᾱσα, λῦσαν |

The middle: λύομαι, ἐλυόμην, λύσομαι, ἐλυσάμην, λέλυμαι. The aorist passive: ἐλύθην, future passive λυθήσομαι.

Note the final ν "movable" (ἔλυσε / ἔλυσεν), added before a vowel and at the end of a sentence, and in the Koine almost everywhere. A tokenizer and a lexicon must take both forms as one.

## 8. Voice: active, middle, passive

**The middle** shows that the subject acts on itself or for itself, or takes part in the action with special interest (smyth-1920 §§1713–1734):
- direct reflexive: λούομαι "I wash (myself)";
- indirect, for oneself: λύομαι τὸν υἱόν "I ransom my son" (loose him for myself), against λύω "I release";
- with a change of meaning: ἄρχω "I rule", ἄρχομαι "I begin"; πείθω "I persuade", πείθομαι "I obey"; παύω "I stop (someone)", παύομαι "I stop (myself), cease".

**The passive** has its own forms only in the aorist (ἐλύθην) and the future (λυθήσομαι) (smyth-1920 §§1735–1758). In the present, imperfect and perfect the middle forms serve as passive too: λύομαι is "I loose for myself" or "I am being loosed". The agent of a passive is ὑπό + genitive: ὑπὸ τῶν πολεμίων "by the enemy" (smyth-1920 §1698).

**Deponent verbs** have no active forms but an active meaning: βούλομαι "I want", ἔρχομαι "I come", γίγνομαι "I become". They are middle deponents (aorist middle: ἐγενόμην) or passive deponents (aorist passive: ἐβουλήθην).

Several verbs form the future in the middle even when active in meaning: ἀκούω, future ἀκούσομαι; λαμβάνω, λήψομαι; εἰμί, ἔσομαι.

## 9. The principal parts

A Greek verb is learnt by its **six principal parts**, one for each tense system: the first person singular of the present, future, aorist active, perfect active, perfect middle, aorist passive (smyth-1920 §§368–369):

| present | future | aorist | perfect | perfect middle | aorist passive |
|---|---|---|---|---|---|
| λύω | λύσω | ἔλυσα | λέλυκα | λέλυμαι | ἐλύθην |
| λείπω | λείψω | ἔλιπον | λέλοιπα | λέλειμμαι | ἐλείφθην |
| λαμβάνω | λήψομαι | ἔλαβον | εἴληφα | εἴλημμαι | ἐλήφθην |
| ὁράω | ὄψομαι | εἶδον | ἑόρᾱκα, ἑώρᾱκα | ἑώρᾱμαι, ὦμμαι | ὤφθην |
| λέγω | λέξω, ἐρῶ | ἔλεξα, εἶπον | εἴρηκα | εἴρημαι, λέλεγμαι | ἐλέχθην, ἐρρήθην |
| ἔρχομαι | εἶμι, ἐλεύσομαι | ἦλθον | ἐλήλυθα | — | — |
| φέρω | οἴσω | ἤνεγκα, ἤνεγκον | ἐνήνοχα | ἐνήνεγμαι | ἠνέχθην |

Some of the commonest verbs join several roots (suppletion): ὁράω, ὄψομαι, εἶδον "see" from three roots; φέρω, οἴσω, ἤνεγκα "carry"; λέγω, ἐρῶ, εἶπον "say". A lemmatizer that does not know them reads εἶδον and ὁράω as two verbs.

## 10. How the treebanks annotate the verb

The UD Ancient Greek treebanks mark tense and aspect in two features, and their combinations stand for the Greek tenses. In PTNK (UD 2.18, counted 2026-10-22):

| Greek tense | `Aspect` | `Tense` | indicatives in PTNK train | example |
|---|---|---|---:|---|
| aorist | `Perf` | `Past` | 1,264 | ἐγέννησεν |
| imperfect | `Imp` | `Past` | 150 | ἦν |
| present | `Imp` (or none) | `Pres` | 204 | ἐστιν, καταριθμοῦνται |
| future | `Imp` (or none) | `Fut` | 255 | ἔσται |
| perfect | `Perf` | `Pres` | 61 | κέκτημαι, εἴληφεν |
| pluperfect | `Perf` | `Pqp` | 8 | ᾔδεις |

The aorist is `Aspect=Perf` (perfective), and the Greek perfect is `Aspect=Perf` with `Tense=Pres`: the UD name "perfective" and the Greek tense name "perfect" are not the same thing. The voice is `Act`, `Mid`, `Pass` or the set `Mid,Pass` (106 finite forms) where the form cannot tell. The mood sets `Ind,Sub` (27) keep the ambiguity of forms like λύω (present indicative or subjunctive).

## 11. How Mova handles it

Greek goes through Mova's English parser after `en xpos-fill` (PTNK dev LAS 74.41 with its own tags, 78.29 with gold tags; 2026-10-22). The tag set it maps to has the English verb tags (base, past, present third person, participles), so the Greek tense, aspect, mood and voice are not in the tags the parser sees, only in the `FEATS`. What a Greek frames reader needs from this chapter, in order:
1. **Tense from the two features** (§10): the aorist as a past event, the imperfect as a past background or repeated event, the perfect as a present state. The English frames reader already marks tense and aspect (`tam`); the Greek values map onto it.
2. **Voice**: the agent of a passive in ὑπό + genitive, and the middle read as reflexive or as the subject's own interest. A `Mid,Pass` form needs the context (an ὑπό phrase makes it passive).
3. **The augment and reduplication for unknown words**: a lemmatizer that strips ε- in the indicative and the reduplication in the perfect finds the stem of a verb it has never seen.
4. **The suppletive verbs of §9** joined under one lemma, so that εἶδον and ὁράω are one event "see".
5. **Mood**: the subjunctive and optative read as not asserted ([chapter 14](14-subordination.md)).

## 12. Sources

- Smyth, H. W. 1920 (smyth-1920): §§355–467 (the forms), §§1698–1734 (the voices), §§1850–1965 (the tenses), paraphrased from the Perseus text (CC BY-SA).
- Goodwin, W. W. 1892 (goodwin-1892): the verb, public domain; Goodwin, *Syntax of the Moods and Tenses of the Greek Verb* (1890; goodwin-1890), public domain.
- UD Ancient Greek PTNK (CC BY-SA 4.0): counts and examples.
