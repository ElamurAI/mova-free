# Verbs that take an indirect object

**Gist.** An indirect object (recipient, addressee, beneficiary without a preposition) occurs only with verbs that allow two objects: *give him a book*, *tell them a story*, *buy her a present*, *bake me a cake*. These are verbs of transfer (give, hand, lend, owe, sell, send, pay), communication (tell, show, teach, email, write), action for someone's benefit (buy, build, cook, fix, find, get, make), cost and billing (cost, charge, bill, fine), promising (promise, offer, guarantee), and also verbs of speech and urging that combine an addressee with a clause or infinitive (inform, notify, remind, warn, advise, assure, convince, persuade, urge, ask, allow, cause).

**Conditions and exceptions.** Since UD 2.12 the addressee stays `iobj` even without a direct object (*tell them*, *remind me of Vietnam*, *allow radicals to launch operations*) — if the verb in principle allows a second object. Verbs without that possibility (*help, question*) have `obj`: *She helps her students to succeed*. In EWT `iobj` also appears with *trust* (*trust me*) and *thank* (*thank god*) — this is EWT annotation, not grammar: the rule flags them as suspects for review. The list below is the members of VerbNet double-object classes (more frequent in EWT) plus speech verbs with `ccomp`/`xcomp`; the full VerbNet list is 279 lemmas.

**Examples.** *It took me two hours* (take). — *He quoted me a price* (quote). — *Do yourself a favour* (do). — *\*She explained me the rule* (explain has no iobj — an error of the text or the annotation).

**In UD.** The head of `iobj` is a lemma from the list; otherwise suspect: it is `obj`, `obl` or an error.

**Sources.** VerbNet 3.4, classes with frames «NP V NP-Dative NP», «NP V NP.beneficiary NP», «NP V NP NP»: give-13.1, send-11.1, bring-11.3, throw-17.1, slide-11.2, carry-11.4, get-13.5.1, steal-10.5, future_having-13.3, pay-68, bill-54.5, cost-54.2, build-26.1, create-26.4, preparing-26.3, performance-26.7, feeding-39.7, transfer_mesg-37.1.1, instr_communication-37.4 (`data/raw/en-verbnet-3-4`); PropBank 3.1 (give.01, tell.01: ARG2 — recipient/hearer); UD docs/changes.md` «Sole iobj»; https://universaldependencies.org/u/dep/iobj.html (teach; help — obj); Jespersen, MEG III, ch. XIV «Two Objects» (text-3, p. 292).

```rule
rule: en.lexicon.iobj-verb
what: only verbs that allow two objects take an indirect object (171 lemmas: VerbNet + classes from errors/iobj-verb-classes.md)
match: v[]; i[rel=iobj, head=v]
require: v[lemma=accord|advise|afford|allocate|allow|answer|arrange|ask|assign|assure|attain|award|bake|believe|bet|bid|bill|blend|blow|boil|book|bring|build|buy|call|capture|carry|cast|catch|cause|cc|charge|choose|clean|clear|command|convince|cook|cost|cut|dance|deal|deny|design|develop|dig|do|draw|drop|earn|e-mail|email|envy|express|extend|fax|feed|fetch|find|fine|fire|fix|flip|float|fold|forgive|forward|fry|gain|gather|get|give|grab|grant|grill|grow|guarantee|hand|hire|hit|hook|hurl|inform|instruct|invoice|issue|kick|knock|land|last|launch|lease|leave|lend|let|loan|mail|make|message|mix|name|notify|offer|order|overcharge|owe|paint|pass|pay|permit|persuade|phone|pick|play|pledge|pour|prepare|promise|provide|pull|push|quote|reach|read|reassure|recommend|refund|refuse|remind|render|rent|reserve|roll|run|save|score|secure|sell|send|serve|set|shape|ship|shoot|show|sign|signal|sing|slip|smuggle|spare|steal|supply|take|teach|tell|text|thank|throw|tip|toss|transmit|trust|urge|vote|warn|wash|win|wire|wish|write]
severity: warn
source: VerbNet 3.4 dative/benefactive classes; UD docs/changes.md Sole iobj; UD _en/dep/iobj.md; P17-1074 (VERB, PREP); 2026.udw-1.1
```
