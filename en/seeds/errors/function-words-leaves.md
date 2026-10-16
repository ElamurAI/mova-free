# Function words are leaves of the tree

**Gist.** In UD the heads are content words, and function words (`aux`, `cop`, `case`, `mark`, `cc`, `det`, `punct`) hang on them and themselves have almost no dependents. Only a few relations are allowed under function words: `fixed`, `goeswith`, `conj` between function words (*before and after*, *and/or*), and an `advmod` modifier (*just before*, *not every*). If an auxiliary verb has a subject or object, the parse is inverted. Schemes with functional heads do this — PTB, where *be* heads the VP, and SD, where the copula is the head with a prepositional predicate. LLMs trained on such data repeat these schemes.

**Conditions and exceptions.**
- VP ellipsis: in *Mary will too* the word *will* is not `aux` but the head of the clause (see `vp-ellipsis-aux-head.md`).
- Coordinated function words are linked by `conj` — this is allowed, so `conj` is not in the forbidden list.

**Examples.**
- *He has eaten* → aux(eaten, has); nsubj(eaten, He), not nsubj(has, He).
- *in the house* → case(house, in), det(house, the).

**In UD.** `aux` and `aux:pass` hang flat on the lexical verb; `cop` on the nominal or adjectival predicate; a preposition on the noun.

**Check against gold.** EWT 2.18:
- aux and cop with core dependents — 0 of 15 648;
- cc, mark, case, det — 2 of 57 730;
- punct with dependents — 0 of 29 682.

GUM: 0, 3 and 0.

**Sources.** UD `_en/dep/aux_.md`, `_en/dep/cop.md`; english-banks §2 (table: head of copula and auxiliaries in PTB, SD, UD); `W15-2134` (functional heads are easier for the parser, but UD keeps content heads); `2025.law-1.14`.

```rule
rule: en.errors.aux-cop-leaf
what: an auxiliary or copula has a subject, object or other content dependent — the parse is inverted
match: a[rel=aux|aux:pass|cop]
require: none x[head=a, rel=nsubj|nsubj:pass|obj|iobj|obl|ccomp|xcomp|csubj|advcl|nmod|amod|det|case|mark|aux|cop]
severity: error
source: UD _en/dep/aux_.md, _en/dep/cop.md; english-banks §2
```

```rule
rule: en.errors.function-word-leaf
what: cc, mark, case or det has a content dependent — a function word is never the head of a phrase
match: c[rel=cc|mark|case|det]
require: none x[head=c, rel=nsubj|nsubj:pass|obj|iobj|obl|ccomp|xcomp|csubj|advcl|nmod|amod|acl|acl:relcl]
severity: warn
source: UD _en/dep/case.md, _en/dep/mark.md, _en/dep/cc.md; W15-2134
```

```rule
rule: en.errors.punct-leaf
what: a punctuation mark has dependents — punct is always a leaf
match: p[rel=punct]
require: none x[head=p, rel=nsubj|obj|obl|nmod|amod|det|case|mark|advmod|conj|cc|aux|cop|advcl|acl]
severity: error
source: UD _en/dep/punct.md
```
