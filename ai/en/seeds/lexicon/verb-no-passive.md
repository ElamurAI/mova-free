# Verbs without a passive and without an object

**Gist.** The passive is formed only from verbs that have a real object of the action. Verbs that describe an event, state or motion without an agent (*happen, occur, exist, arrive, arise, emerge, belong, consist, matter, seem, appear, remain*) have neither a direct object nor a passive: one cannot say *\*It was happened*. Some verbs with an object do not passivize either: *cost, last, resemble* (*It costs two shillings*, but not *\*Two shillings are cost*).

**Conditions and exceptions.** PropBank has no agent role ARG0 for the first rolesets of these verbs (happen.01, occur.01, exist.01, arrive.01, seem.01, remain.01, consist.01, matter.01, last.01, cost.01, resemble.01) — a formal sign that "there is no agent". *Die* has a "cognate" object (*die a hero's death*), *fall* has a passive only in "be fallen" (archaic perfect), so they are excluded. Brown: a passive is possible only from "active-transitive" verbs.

**Examples.** *What happened?* — *\*The accident was happened.* — *The book costs ten dollars* (no passive). — *\*It had only happened times* (error: happen with no obj).

**In UD.** These lemmas have no `Voice=Pass`; *happen, occur, exist, arrive, arise, emerge, belong, consist, matter, seem, appear, remain* have no `obj`.

**Sources.** Jespersen, MEG III, §15.12 («do not admit of a passive turn»: cost, weigh, last; «His father was resembled by him» — unnatural; text-3, p. 313); Brown 1851, Part II, Ch. VI, «III. Form of Passive Verbs» («made from active-transitive verbs»); PropBank 3.1 frames (rolesets .01 without ARG0; `data/raw/en-propbank-frames`); Jespersen, MEG III, ch. XVI «Transitivity» (text-3, p. 333).

```rule
rule: en.lexicon.no-passive-verb
what: agentless verbs (and cost/last/resemble) have no passive
match: v[lemma=happen|occur|exist|arrive|appear|seem|remain|stay|become|consist|matter|last|cost|resemble|belong|arise|emerge]
require: not v[feats.Voice=Pass]
severity: warn
source: Jespersen MEG III §15.12; PropBank 3.1 (.01 without ARG0); Brown 1851 Part II Ch. VI
```

```rule
rule: en.lexicon.unaccusative-no-object
what: happen/occur/exist/arrive… have no direct object
match: v[lemma=happen|occur|exist|arrive|arise|emerge|belong|consist|matter|seem|appear|remain]; o[rel=obj, head=v]
require: not o[rel=obj]
severity: warn
source: PropBank 3.1 (happen.01, occur.01, exist.01… only ARG1); Brown 1851 Part II Ch. VI (active-intransitive, neuter verbs)
```
