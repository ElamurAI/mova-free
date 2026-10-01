# A direct object with a passive — only for certain verbs

**Gist.** A passive verb usually has no direct object: the object of the action has become the subject. An object remains (a "retained object") only with double-object verbs when the recipient has become the subject: *I was given a horse*, *He was told the truth*, *She was offered a job*, *We were charged a fee* — and in idioms like *was taken care of*. The predicative with verbs of naming and electing (*He was elected president*, *was called a fool*) is not an object but `xcomp`.

**Conditions and exceptions.** *Titled/entitled/headed X* (titles) have `obj` in EWT. Other verbs with `obj` under `Voice=Pass` are suspect: either it is not a passive (a perfect without *been*), or the "object" is really `xcomp`/`obl`, or an error.

**Examples.** *I was given a horse* — `nsubj:pass(I)`, `obj(horse)`. — *Pakistan was gifted a slice of the territory.* — *The story was featured ARD* (suspect).

**In UD.** `obj` under a head with `Voice=Pass` — only for lemmas from the list of double-object verbs, verbs of naming, and *take* (take care).

**Sources.** Jespersen, MEG III, ch. XV «Subject of Passive Verb», §§15.1x–15.2x (text-3, p. 313); Reed & Kellogg, Higher Lessons, Lesson 129 (Voice); https://universaldependencies.org/en/dep/cop.html (*I was given a horse*); VerbNet 3.4 (give-13.1, future_having-13.3, bill-54.5 etc.; see `verb-iobj-licensors.md`).

```rule
rule: en.lexicon.passive-object-verb
what: a direct object with a passive — only for double-object verbs, naming verbs and take care
match: v[feats.Voice=Pass]; o[rel=obj, head=v]
require: v[lemma=give|send|tell|show|offer|bring|pay|ask|teach|lend|hand|owe|promise|grant|deny|allow|award|charge|cost|save|spare|forgive|refuse|assign|leave|provide|serve|feed|guarantee|bill|fine|email|inform|notify|call|name|title|entitle|head|elect|appoint|make|sell|buy|issue|afford|extend|gift|quote|take|read|write|mail]
severity: warn
source: Jespersen MEG III ch. XV; VerbNet 3.4 dative classes
```
