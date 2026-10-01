# PTB ↔ mova — converter draft (tags only)

We have no PTB data (`seeds/data.md`). The `convert` language does not read constituent trees (`seeds/conversion.md`). So here there is only the transition between `mova` XPOS ("new" PTB, as in EWT) and the classic Santorini 1990 tagging (`seeds/tagset.md`). It is needed:
- to read treebanks with classic tags: in ESLSpok *to*/TO;
- to output `en` results in classic tags.

The rule language is `train/dialects-and-converters.md`, conventions are in `../seeds-overview.md`.

## ptb → mova

```convert
rule: ptb.to-in
what: prepositional to — IN (new PTB)
from: ptb
to: mova
match: t[form=to, xpos=TO, rel=case]
set: t[xpos=IN]; t[upos=ADP]
source: seeds/tagset.md; https://universaldependencies.org/en/tokenization.html («new» PTB)
```

The condition `rel=case` requires a dependency tree. For tagged text without a tree there is a neighbour heuristic: *to* not followed by VB or RB is a preposition.

```convert
rule: ptb.to-in-text
what: to not followed by VB or RB — IN (text without a tree)
from: ptb
to: mova
match: t[form=to, xpos=TO]
require: none c[xpos=VB|RB, next=t]
set: t[xpos=IN]
source: Santorini 1990, TO; seeds/tagset.md
```

The heuristic errs on *to* before an adverb-preposition (*to there*) and on elliptical *to* at the end of a clause (*I want to.*). For treebanks with a tree the first rule is used.

**Not converted (no actions):**
- **Hyphens.** *long-term*/JJ → *long*/JJ *-*/HYPH *term*/NN. This needs "split token" and "merge tokens" actions. They do not exist, and for trees new nodes are needed as well.
- **Brackets** `-LCB-`/`-RCB-`/`-LSB-`/`-RSB-` → `-LRB-`/`-RRB-`: the rule language knows only the tags from the `Tag` list in `en`, and `-LCB-` is not there. Needs string XPOS.

## mova → ptb

```convert
rule: mova.ptb.to
what: prepositional to — TO (Santorini 1990)
from: mova
to: ptb
match: t[form=to, xpos=IN]
set: t[xpos=TO]
source: seeds/tagset.md
```

```convert
rule: mova.ptb.nfp
what: NFP → : (in PTB 1990 decorative punctuation is :)
from: mova
to: ptb
match: t[xpos=NFP, upos=PUNCT]
set: t[xpos=:]
source: seeds/tagset.md; https://universaldependencies.org/en/pos/PUNCT.html
```

```convert
rule: mova.ptb.nfp-sym
what: NFP emoticon → SYM
from: mova
to: ptb
match: t[xpos=NFP, upos=SYM]
set: t[xpos=SYM]
source: seeds/tagset.md; https://universaldependencies.org/en/pos/SYM.html
```

```convert
rule: mova.ptb.add
what: ADD → NNP
from: mova
to: ptb
match: t[xpos=ADD]
set: t[xpos=NNP]
source: seeds/tagset.md
```

```convert-todo
rule: mova.ptb.hyph
what: X - Y → one word X-Y with the head's tag
from: mova
to: ptb
match: h[xpos=HYPH]; a[next=h]; b[prev=h]
set: a[merge=b]
source: seeds/tagset.md
```

Needs a token-merge action. Likewise `AFX` (merges with the next word) and `GW` (parts of a split word).

Matches on EWT: `to` 2286 (of them with `case` — 2029), `nfp` 344, `nfp-sym` 155, `add` 475. `ptb → mova` on ESLSpok, which has classic TO: `to-in` 224 (by the tree). `to-in-text` gives 243 (576 − 333): 19 more than the real prepositions. This is the limit of the heuristic without a tree.

## Round trip

There is no round trip on PTB data: there are no data. For the rules themselves the check is: EWT → `mova.ptb.*` → `ptb.to-in` → EWT. Prepositional *to* with `case` comes back without loss (2029). *to* with IN not under `case` — 257: *according to*, *next to*, *due to* in `fixed`, *to* as SCONJ etc. On the way there they become TO, and on the way back `ptb.to-in` does not restore them: 257 XPOS divergences. `NFP` → `:`/`SYM` and `ADD` → `NNP` are not restored: 974 XPOS divergences, because classic PTB does not have these distinctions.
