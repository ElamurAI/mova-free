# a or an — by the first sound of the next word

**Gist.** The indefinite article has the form *an* before a vowel sound (*an apple, an hour*) and *a* before a consonant sound (*a book*). Before a vowel letter pronounced as [j] or [w], *a* is written too: *a unit, a eulogy, a one*. The choice depends on the pronunciation of the word right after the article, which is not necessarily the noun. Errors like *a answer, a island, a excuse* occur both with native speakers (typos) and learners.

**Conditions and exceptions.**
- The engine sees only spelling and does not see word adjacency. So the rule fires when there is no dependent between the article and the noun.
- False alarms: *a European, a one-off* (vowel letter, consonant sound); *an FBI agent* (the abbreviation is read starting with a vowel).
- Abbreviations starting with M, N, R, S, X and silent *h* (*an hour, an honest*) are not covered by the rule.

**Examples.**
- ✗ *a answer*, ✗ *a order*, ✗ *a Immigrant*
- ✓ *an apple*, ✓ *a big apple*

**Check against gold.** EWT 2.18: *a* before a vowel — 363/6 (all six are errors in the text); *an* before a consonant — 198/3 (abbreviations).

**Sources.** Fowler MEU 1926, A, AN §1 (p. 13: *a unit, a eulogy, a one*; *an hour*); `P17-1074` (ERRANT: DET); `W19-4406` (DET — 11 % of W&I edits).

```rule
rule: en.errors.a-before-vowel
what: a directly before a noun starting with a vowel letter (a answer) — should be an
match: n[prefix=a|e|i|o]; d[form=a, rel=det, head=n, before=n]
require: exists m[head=n, after=d, before=n]
severity: warn
source: Fowler MEU 1926, A, AN §1 (p. 13); P17-1074 (DET)
```

```rule
rule: en.errors.an-before-consonant
what: an directly before a noun starting with a consonant letter (an book) — should be a
match: n[prefix=b|c|d|f|g|j|k|l|p|q|t|v|w|z]; d[form=an, rel=det, head=n, before=n]
require: exists m[head=n, after=d, before=n]
severity: warn
source: Fowler MEU 1926, A, AN §1 (p. 13); P17-1074 (DET)
```
