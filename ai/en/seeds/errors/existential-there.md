# Existential there: expl, and be is a VERB

**Gist.** In *There is a ghost in the room* the word *there* is an empty placeholder subject: PRON with XPOS EX and the relation `expl`. The real subject is *ghost* (`nsubj`), and *is* is the root verb with UPOS VERB, not AUX and not cop. The verb agrees with the subject that follows it: *There are two dogs*. In colloquial English *there's* + plural is widespread (*There's lots of towns*). Fowler allows the singular when the verb precedes a coordinated subject, but for a simple plural it is an agreement error in the standard.

**Conditions and exceptions.** The place adverb *there* (*I went there*) is ADV with `advmod`. Colloquial *there's* + plural — the rule only warns.

**Examples.**
- *There's a cow in the field* → expl('s, There), nsubj('s, cow), 's: VERB.
- ✗ *Is there any tricks I could use?* → ✓ *Are there*.

**Check against gold.** EWT 2.18:
- EX not as `expl` — 0 of 458;
- *be* with `expl` *there* not VERB — 3 of 466;
- *there* + VBZ + plural subject — 10 (colloquial).

**Sources.** UD `_en/dep/expl.md`, `_en/dep/cop.md` (There 's/VERB a cow in the field); Fowler MEU 1926, NUMBER §2, 7 (pp. 401–402: singular before a coordinated subject); `P17-1074` (VERB:SVA).

The rule is `en.nominal.ex-there` in `nominal/there-expletive.md`.

```rule
rule: en.errors.there-be-verb
what: the verb with existential there is not VERB — in an existential sentence be is the VERB root
match: v[]; e[rel=expl, lemma=there, head=v]
require: v[upos=VERB]
severity: warn
source: UD _en/dep/cop.md (There 's/VERB a cow)
```

```rule
rule: en.errors.there-agreement
what: there + VBZ with a plural subject (there's lots of…) — colloquial, standard is are
match: v[xpos=VBZ]; e[rel=expl, lemma=there, head=v]; s[rel=nsubj, head=v, feats.Number=Plur]
require: not v[xpos=VBZ]
severity: warn
source: Fowler MEU 1926, NUMBER §7 (pp. 401–402); P17-1074 (VERB:SVA)
```
