# discourse on adverbs

**Gist.** Already the 2.18 guideline admits into English `discourse` only non-adverbial discourse markers: *well*, *like*, but not *actually*. EWT has 9 adverbs with `discourse`, and GUM has 78. After 2.18 the exception for adverbs is written out explicitly: `dialects/ud-next/seeds/discourse-scope.md`.

**2.18 guideline.** `_en/dep/discourse.md` refers to the universal page. There, `2.18:https://universaldependencies.org/u/dep/discourse.html:11–12`: «interjections (*oh*, *uh-huh*, *Welcome*), fillers (*um*, *ah*), and non-adverbial discourse markers (*well*, *like*, but not *you know* or *actually*)».

**EWT 2.18 data** (engine and grep):
- `discourse` in total — 1066: INTJ 775, SYM 123 (emoticons), NUM 113 (item numbering), NOUN 27 (*thanks*, *ps*), VERB 12 (*mean*, *idk*), **ADV 9**, PROPN 3, ADJ 3, X 1;
- ADV with `discourse` (train 4, dev 1, test 4): *though* ×3, *So* ×2, *Also*, *Maybe*, *btw*, *FTW*. For *though* this is an exception: ADV *though* is `advmod` 42 times and `discourse` 3;
- *actually* — `advmod` 83 times, `discourse` never; *in other words* — `obl` 5 times.

**Other 2.18 treebanks.** GUM: ADV with `discourse` — 78, of which *so* 57, *anyways* 6, *now* 5. At the same time *so* with `advmod` — 622. LinES — 15, ParTUT — 3, LittlePrince — 3, GENTLE — 1.

**Where the discrepancy comes from.** The boundary between a marker and an adverb is drawn by meaning, not by word class. After 2.18 the guideline explicitly gave priority to word class.

**Sources.** `2.18:https://universaldependencies.org/u/dep/discourse.html, `2.18:https://universaldependencies.org/en/dep/discourse.html.

```rule
rule: ud218.discourse-adv
what: ADV with discourse — 2.18 admits only non-adverbial markers
match: d[rel=discourse, upos=ADV]
require: not d[upos=ADV]
severity: warn
source: UD 2.18 https://universaldependencies.org/u/dep/discourse.html:11–12
```
