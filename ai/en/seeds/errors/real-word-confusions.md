# Confused words: there/their, to/too, then/than, where/were

**Gist.** The most frequent "real-word" errors in web text are homophones: *their/there/they're*, *to/too*, *then/than*, *where/were*, *your/you're*, *its/it's*. A spell checker does not see them, because each is a real word. EWT annotates such a word as the intended one and sets Typo=Yes: *there* in the role of *their* gets PRP$ and `nmod:poss`, *then* in the role of *than* gets IN and `case`. So a form with the tag of "another" word without Typo=Yes is either a tag error or a missing mark.

**Conditions and exceptions.**
- The rule takes only unambiguous "form + tag" pairs.
- *then* as an adverb (*and then*) and *to* as ADP or PART are legitimate.
- *your/you're* and *its/it's* involve tokenization (multiword tokens), so they are not in the rule.

**Examples.**
- *there car* → there: PRP$, nmod:poss, Typo=Yes, CorrectForm=their.
- *better then me* → then: IN, case, Typo=Yes.
- *it was to hot* → to: ADV (RB), advmod, Typo=Yes.

**Check against gold.** EWT 2.18:
- there/PRP$ — 15/0;
- their/EX or RB — 8/0;
- to/ADV — 12/0;
- then/IN — 7/0;
- where/AUX — 5/0.

**Sources.** EWT 2.18 practice (Typo=Yes + CorrectForm in MISC, 1439 tokens); `2025.udw-1.17` (Masciolini et al.: universal UD guidelines for errors); `P17-1074` (SPELL — only for non-words; real-word substitutions are classified by part of speech); `W19-4406` (LOCNESS — native essays, native errors in BEA-2019).

```rule
rule: en.errors.there-as-their
what: there with the possessive tag PRP$ (instead of their) without Typo=Yes
match: w[form=there, xpos=PRP$]
require: w[feats.Typo=Yes]
severity: warn
source: EWT 2.18 practice (Typo=Yes, CorrectForm); 2025.udw-1.17
```

```rule
rule: en.errors.their-as-there
what: their with the tag EX or RB (instead of there) without Typo=Yes
match: w[form=their, xpos=EX|RB]
require: w[feats.Typo=Yes]
severity: warn
source: EWT 2.18 practice (Typo=Yes, CorrectForm); 2025.udw-1.17
```

```rule
rule: en.errors.to-as-too
what: to as an adverb (instead of too) without Typo=Yes
match: w[form=to, upos=ADV]
require: w[feats.Typo=Yes]
severity: warn
source: EWT 2.18 practice (Typo=Yes, CorrectForm); 2025.udw-1.17
```

```rule
rule: en.errors.then-as-than
what: then with the tag IN (comparative than) without Typo=Yes
match: w[form=then, xpos=IN]
require: w[feats.Typo=Yes]
severity: warn
source: EWT 2.18 practice (Typo=Yes, CorrectForm); 2025.udw-1.17
```

```rule
rule: en.errors.where-as-were
what: where as an auxiliary or copula (instead of were) without Typo=Yes
match: w[form=where, upos=AUX]
require: w[feats.Typo=Yes]
severity: warn
source: EWT 2.18 practice (Typo=Yes, CorrectForm); 2025.udw-1.17
```
