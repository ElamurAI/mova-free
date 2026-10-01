# The verbs have and do: has, had, does, did, done

**Gist.** *Have* and *do* are irregular auxiliaries with special forms. *Have*: 3rd person singular *has* (not *haves*), past and participle *had*. *Do*: *does* (spelled regularly but pronounced with a different vowel), past *did*, participle *done*. Both can be auxiliaries (*have gone*, *do you know*) and lexical verbs (*have a car*, *do homework*).

**Conditions and exceptions.**
- *Had* — VBD (*he had a car*, *she had left*) or VBN (*I have had enough*); syntax decides.
- *Did* — only VBD; *done* — only VBN (in non-standard *he done it* EWT tags by function).
- *Does* as the plural of *doe* (female deer) is a noun; the rule does not touch it (we look only at VERB/AUX).
- Contractions: *'ve* → *have*, *'s* → *has*, *'d* → *had* (see `verb-clitics`).

**Examples.** *She **has** left. We **had** fun. **Does** it work? I **did** it. It's **done**.*

**In UD.** Lemma *have* or *do*. *Has* and *does* — VBZ; *had* — VBD or VBN; *did* — VBD; *done* — VBN. Auxiliary use — AUX, lexical — VERB.

**Sources.** https://universaldependencies.org/en/pos/VERB.html (except the auxiliaries *be, have, do, get*), https://universaldependencies.org/en/pos/AUX_.html, `feat/Number.md` (*she does like cats*); Santorini 1990, §2, p. 5; §4.1, p. 17 (*do, have* — ordinary verb tags, not MD); Sweet NEG I §1492–1493 (text-1, pp. 457–458: *has, had; does* [dʌz], *did, done*); Jespersen MEG VI 3.1 (text-5, p. 30: *says, does, has*), 4.1₄ (p. 44: *have–had, make–made*); Kruisinga II.1 §19 (text-2, pp. 52–53: *have, do* — full verbs).

```rule
rule: en.morph.have-has
what: has — have, VBZ
match: v[form=has, upos=AUX|VERB, !feats.Typo]
require: v[lemma=have, xpos=VBZ]
severity: error
source: UD en pos/AUX; Santorini 1990, VBZ
```

```rule
rule: en.morph.have-had
what: had — have, VBD or VBN
match: v[form=had, upos=AUX|VERB, !feats.Typo]
require: v[lemma=have, xpos=VBD|VBN]
severity: error
source: UD en pos/AUX; Santorini 1990, VBD, VBN
```

```rule
rule: en.morph.do-does
what: does — do, VBZ
match: v[form=does, upos=AUX|VERB, !feats.Typo]
require: v[lemma=do, xpos=VBZ]
severity: error
source: UD en feat/Number (she does like cats)
```

```rule
rule: en.morph.do-did
what: did — do, VBD
match: v[form=did, upos=AUX|VERB, !feats.Typo]
require: v[lemma=do, xpos=VBD]
severity: error
source: UD en pos/AUX; Santorini 1990, VBD
```

```rule
rule: en.morph.do-done
what: done — do, VBN
match: v[form=done, upos=AUX|VERB, !feats.Typo]
require: v[lemma=do, xpos=VBN]
severity: warn
source: Santorini 1990, VBN; non-standard he done it is tagged by function in EWT
```
