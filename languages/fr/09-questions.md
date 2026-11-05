# 9. Questions

French asks a question in three ways, and the choice is a matter of register more than of meaning:
- **intonation alone**, with the order of a statement: *Tu viens ?*;
- **the marker *est-ce que*** before a statement: *Est-ce que tu viens ?*;
- **inversion of the subject**, the subject pronoun after the verb and joined to it by a hyphen: *Viens-tu ?*

Speech uses the first two, writing the last two. Questions with a question word (*qui, que, quoi, où, quand, comment, pourquoi, combien, quel*) use the same three patterns, plus a fourth that leaves the question word where the answer would be: *Tu vas où ?* This chapter gives the yes/no questions, then the questions with a question word, then the answers and the indirect questions. The interrogative pronouns are listed in [chapter 4](04-pronouns.md); the order of clitics under inversion is also there.

## 9.1 Yes/no questions

### Intonation

A statement with a rising final pitch is a question. In writing it is marked only by the question mark:

> Tu as faim ? — "Are you hungry?"
> Il a été tiré, le signal d'alarme ? (spoken, with dislocation: [chapter 10](10-clause-structure.md))

This is the normal question of conversation. In the spoken UD French ParisStories training set (UD 2.18, counted 2026-10-22), 48 of the 55 sentences that end with a question mark have neither *est-ce que* nor inversion: they are questions by intonation, or by intonation with a question word left in place (9.2).

### *Est-ce que*

*Est-ce que* (before a vowel *est-ce qu'*) turns any statement into a question without changing its order. It belongs to every register, from speech to careful prose:

> **fr-ud-dev_00427** · UD French GSD
> « Est-ce qu'il y a un poème ce matin ?
> *is-it that there has a poem this morning* — "Is there a poem this morning?"

*Est-ce que* is frozen: it never agrees and is never put into another tense (\**était-ce que*), and in speech it is pronounced as one word, /ɛskə/.

### Inversion

**Pronoun subject.** The pronoun follows the finite verb, with a hyphen:

> Viens-tu ? Avez-vous fini ? Peut-on entrer ?

Between a verb ending in a vowel and *il, elle, on* a /t/ is written *-t-*:

> **fr-ud-dev_00602** · UD French GSD
> Va-t-il se circonscrire à cette seule matière ?
> *goes-t-he REFL confine-INF to this single subject* — "Will he confine himself to this one subject?"

With *je*, inversion survives only in a few verbs and set forms (*Puis-je … ?*, *Suis-je … ?*, *Dois-je … ?*, *Que sais-je ?*); elsewhere *est-ce que* replaces it (*Est-ce que je cours ?*, not \**Cours-je ?*).

**Noun subject: complex inversion.** A noun subject stays before the verb, and a pronoun agreeing with it is put after the verb:

> **fr-ud-dev_01214** · UD French GSD
> Le renouveau désiré se produira-t-il ?
> *the renewal desired REFL will-produce-t-it* — "Will the hoped-for renewal come about?"

> Marie est-elle partie ? — "Has Marie left?"

Simple inversion of a noun subject (*Est partie Marie ?*) is not a yes/no question in French. It occurs only after some question words (9.2).

**With negation**: *ne* before the verb and its clitics, the subject pronoun after the verb, *pas* after the subject pronoun ([chapter 8](08-negation.md)):

> Ne viens-tu pas ? — N'a-t-il pas compris ?

On our data (the GSD training set, 2026-10-22), of 259 sentences ending with a question mark, 112 contain an inverted subject pronoun (*-tu, -vous, -il, -t-il …*) and 12 contain *est-ce que*. In ParisStories (spoken) there is no inversion at all.

## 9.2 Questions with a question word

| question word | asks for | example |
|---|---|---|
| *qui* | a person | *Qui est venu ?* |
| *que* / *qu'* (object, before the verb) | a thing | *Que veux-tu ?* |
| *quoi* (strong form: after a preposition, alone, in place) | a thing | *De quoi parles-tu ? Tu veux quoi ?* |
| *où* | a place | *Où vas-tu ?* |
| *quand* | a time | *Quand part-il ?* |
| *comment* | a manner | *Comment ça marche ?* |
| *pourquoi* | a reason | *Pourquoi riez-vous ?* |
| *combien (de)* | a quantity | *Combien de temps reste-t-il ?* |
| *quel, quelle, quels, quelles* (determiner or predicate) | a choice | *Quel livre lis-tu ? Quelle est la date ?* |
| *lequel, laquelle, lesquels, lesquelles* (pronoun) | a choice among known ones | *Lequel préfères-tu ?* |

The same question can take four forms, from formal to familiar:

| form | example | where |
|---|---|---|
| question word + inversion | *Où vas-tu ?* | writing, formal speech |
| question word + *est-ce que* | *Où est-ce que tu vas ?* | all registers |
| question word + statement order | % *Où tu vas ?* | speech |
| question word in place | % *Tu vas où ?* | speech, very common |

### The long forms *qu'est-ce que*, *qui est-ce qui*

With *qui* and *que* the marker shows the function of the question word:

| | subject | object |
|---|---|---|
| a person | *Qui est-ce qui vient ?* | *Qui est-ce que tu vois ?* |
| a thing | *Qu'est-ce qui se passe ?* | *Qu'est-ce que tu veux ?* |

The first part (*qui* / *que*) says person or thing, the last part (*qui* / *que*) subject or object. *Que* alone cannot be a subject: "What happens?" is *Qu'est-ce qui se passe ?* or *Que se passe-t-il ?*, never \**Que se passe ?*

> **fr-ud-dev_00325** · UD French GSD
> Qu'est-ce qui va augmenter ?
> "What is going to rise?"

> mais qu'est-ce que c'est ? (ParisStories, spoken) — "but what is it?"

### Simple inversion of a noun subject

After *où, quand, comment, combien, que* and *qui* (object), a noun subject can simply follow the verb, if nothing else follows:

> Où va ce train ? — Que veut ton frère ? — Comment s'appelle ta sœur ?

It is not possible with *pourquoi* (\**Pourquoi part ton frère ?*, but *Pourquoi ton frère part-il ?*) nor when the verb has an object after it.

### Infinitive questions

A question word with an infinitive asks what to do; it belongs to writing and to titles:

> **fr-ud-dev_00233** · UD French GSD
> Comment remédier aux contradictions et au désarroi des relations humaines ?
> "How can the contradictions and the disarray of human relations be remedied?"

> Que faire ? — Où aller ?

### Spoken forms

Speech adds forms the written standard avoids:
- *que* moved after the verb becomes *quoi*: % *Tu fais quoi ?*
- *c'est que* after the question word, without inversion: % *Où c'est que tu vas ?*
- *que* after the question word: % *Où que tu vas ?*, % *Comment qu'il a fait ?* (regional, very informal);
- in Québec, the particle *-tu* (from *-t-il*) marks a yes/no question with any subject: % *Tu viens-tu ?*, % *C'est-tu correct ?* ([chapter 13](13-varieties-registers.md)).

> tu as fait comment ? (ParisStories, spoken) — "how did you do it?"

## 9.3 Answers

*Oui* answers a positive question; *si* contradicts a negative one; *non* denies either:

> — Tu viens ? — **Oui.** / **Non.**
> — Tu ne viens pas ? — **Si.** / **Non.**

An answer to a question with a question word is usually a fragment: *— Où vas-tu ? — À Paris.* Short confirmations repeat the auxiliary or *faire* less than in English: *— Tu l'as vu ? — Oui (je l'ai vu).*, not \**Oui, j'ai.*

## 9.4 Indirect questions

An indirect question is the object of a verb such as *demander, savoir, se demander, dire*. It keeps the order of a statement, never inversion or *est-ce que*:
- a yes/no question with *si*: *Je me demande **s**'il viendra.* (*si* elides only before *il, ils*: *s'il*, but *si elle*);
- a question word as in a direct question: *Dis-moi **où** tu vas. Je ne sais pas **quand** il part.*;
- *qu'est-ce qui* → ***ce qui***, *qu'est-ce que* / *que* → ***ce que***: *Je ne sais pas **ce qui** se passe. Dis-moi **ce que** tu veux.*

> je dis oui, vous savez pourquoi le signal d'alarme il a été tiré ? (ParisStories, spoken)
> "I say yes, do you know why the alarm signal was pulled?"

In speech the direct order and direct forms leak into the indirect question: % *Je sais pas qu'est-ce que tu veux*.

Tenses follow the sequence of the main verb ([chapter 11](11-subordination.md)): *Il m'a demandé si je venais.*

## How Mova handles it

**In the UD trees** (UD French GSD, 2026-10-22):
- **An inverted subject pronoun** is a separate token beginning with a hyphen (*-tu*, *-t-il*, *-vous*). It is `PRON` attached to the verb as `nsubj` when it is the only subject, and as `expl:subj` when a noun subject is also present (complex inversion: in *Le renouveau désiré se produira-t-il ?*, *renouveau* is `nsubj` and *-t-il* `expl:subj`). Its lemma is the plain pronoun (*-t-il* → *lui*).
- ***Est-ce que*** is not one word in UD. *Est* is `AUX` `cop`, *-ce* is `PRON` `expl:subj`, and *qu'* is `SCONJ` `mark`, all attached to the main predicate (in *Est-ce qu'il y a un poème*, to *a*).
- ***Qu'est-ce qui va augmenter*** is analysed as a cleft: *qu'* is the root, *est* its `cop`, *-ce* its `expl:subj`, and the rest of the question an `advcl:cleft` whose subject is *qui*.
- **Question words** carry `PronType=Int` (*que, qui, quel*) and attach in their argument relation (`obj`, `nsubj`, `advmod`).

**In Mova.** `en xpos-fill` maps the question words to `WP`/`WDT`/`WRB` from `PronType=Int` and an inverted pronoun to `PRP`; the parser learns the French attachments from the trees, including `expl:subj` and the cleft analysis. What Mova does not do yet:
- **The speech act.** Mova's pragmatics marks a question in English from the English word order, *do*-support and the question mark. For French it has only the question mark: it does not yet read *est-ce que*, inversion or a question word in place as a question without a question mark.
- **What is asked.** The frames reader does not know the French question words, so it does not know that *Tu vas où ?* asks for the place of *aller*. In English it fills the asked role from *where*.
- **Indirect questions** with *si*, *ce qui*, *ce que* are parsed as complement clauses, but not marked as questions.

With the speech-act cues of 9.1–9.2 (the marker, the hyphenated pronoun after the verb, `PronType=Int`) and the role of each question word, the French frames reader can mark questions as the English one does.
