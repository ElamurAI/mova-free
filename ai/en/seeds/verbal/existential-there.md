# Existential there is

**Gist.** *There is/are* states that something exists or is somewhere: *There is a ghost in the room*. *There* here is not an adverb of place but a formal filler of the subject slot; the real (notional) subject comes after the verb, and the verb agrees with it (*There **are** magicians*). *Be* here is not a copula but a lexical verb "exist".

**Conditions and exceptions.** *There* occurs with *seem, appear, exist, remain, come*: *There seems to be a problem* — then the notional subject attaches to *be* under `xcomp`. Colloquial *there's* + plural is non-agreement in the text. *There* is an adverb of place when it means "in that place" (*Put it there*); Reed & Kellogg considered existential *there* an "independent adverb".

**Examples.** *There's a cow in the field.* — *Is there a ghost?* — *There must be a reason* (expl on *be* with a modal).

**In UD.** *there*: `PRON`, XPOS `EX`, `expl` (the rule on the tag itself — `nominal/there-expletive.md`, `en.nominal.ex-there`). With *be* with `expl` *there*, `be` has UPOS `VERB` (not `AUX`, not `cop`). The notional subject is `nsubj` of the same head to the right of *there*.

**Sources.** https://universaldependencies.org/en/dep/expl.html, https://universaldependencies.org/en/dep/nsubj.html (There is a ghost), https://universaldependencies.org/en/dep/cop.html (There 's/VERB a cow), https://universaldependencies.org/en/pos/AUX_.html (be — VERB in an existential clause); Reed & Kellogg, Higher Lessons, Lesson 44 (there — independent); Brown 1851, Part II, Ch. VI (neuter verb: "There was light").

```rule
rule: en.verbal.existential-be-verb
what: be with expletive there is VERB
match: h[lemma=be]; e[form=there, rel=expl, head=h]
require: h[upos=VERB]
severity: error
source: https://universaldependencies.org/en/pos/AUX_.html; https://universaldependencies.org/en/dep/cop.html
```

```rule
rule: en.verbal.existential-subject
what: a there-clause has a notional subject after there
match: h[]; e[form=there, rel=expl, head=h]
require: exists s[rel=nsubj, head=h, after=e]
severity: warn
source: https://universaldependencies.org/en/dep/nsubj.html (displaced subject)
```
