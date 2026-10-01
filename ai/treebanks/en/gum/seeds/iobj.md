# GUM — `iobj`: the data do not show wider use than in EWT

**Gist.** The brief assumed that GUM uses `iobj` more widely. The UD 2.18 data do not confirm this:
- `iobj` occurs less often in GUM: 482 per 256,739 words (0.19%), in EWT — 795 per 254,820 (0.31%);
- the share of "sole" `iobj` without `obj`, `ccomp` and `xcomp` is almost the same: GUM 101 of 482 (21%), EWT 150 of 795 (19%);
- a personal pronoun with *tell/ask/show* is `iobj` in 125 of 132 in GUM and in 177 of 180 in EWT.

Since UD 2.12 (the "Sole iobj" amendment) both treebanks write `iobj` also where there is no direct object: *tell them* just like *tell them a story*.

**Conditions and exceptions.** The only lexical difference visible: *excuse me* — in GUM *me* is `iobj` 6 times out of 6. EWT has no *excuse me*. By UD, *me* here is a direct object (`obj`), so this is a candidate GUM error, not a custom.

**Examples.** GUM, 6 sentences with *Excuse me* — *me*: `iobj` → *Excuse*.

**In UD.**

| rule | GUM | EWT |
|---|---:|---:|
| `tb.en.sole-iobj`: `iobj` without `obj` | 255 / 482 | 399 / 795 |
| `tb.gum.excuse-me`: *excuse me* — *me* not `obj` | 6 / 6 | 0 / 0 |

The rule `tb.en.sole-iobj` does not count `ccomp` and `xcomp`: *tell me that…* is also "sole" there. Hence 255 versus 101 above.

**Sources.** UD docs/changes.md` No. 7 (Sole iobj, 2.12), No. 16 (Multiple Objects, 2.17); https://universaldependencies.org/en/dep/iobj.html; EWT README v2.12 (#55).

```rule
rule: tb.gum.excuse-me
what: excuse me — me as obj (GUM uses iobj)
match: v[lemma=excuse]; o[form=me|us, head=v]
require: o[rel=obj]
severity: warn
source: UD en obj, iobj; counter
```
