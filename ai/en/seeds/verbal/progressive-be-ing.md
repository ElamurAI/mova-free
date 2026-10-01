# Progressive tenses: be + -ing

**Gist.** The progressive (Poutsma: "expanded") form is *be* + *-ing* participle: *I am writing*, *he was sitting*. It shows an action in progress, temporary or incomplete, and also a planned future (*I am leaving tomorrow*) and disapproving repetition (*you are always grumbling*).

**Conditions and exceptions.** Stative verbs (*know, believe, belong, contain, see, hear* in the literal sense) are hardly used in the progressive — see `lexicon/verb-stative-progressive.md`. The copula *be* itself in the progressive means behaviour: *you are being clever* = "you are acting cleverly"; then *being* is `cop`, and the head is the adjective. An *-ing* form after *be* without progressive meaning can be an adjective (*The film is interesting*) — then it is `cop` + `ADJ`.

**Examples.** *We are eating cake.* — *She must be coming now.* — *You are being very clever* (being — cop).

**In UD.** *be* is `aux` (not `cop`), the head is `VERB`, XPOS `VBG`, `VerbForm=Part|Tense=Pres`. Any `VBG` with `aux` is a participle (`Part`), not a gerund.

**Sources.** Poutsma 1921, *The Expanded Form*, Order of Discussion and §§1–37 (vol. 1, p. 57 ff.; §28 prospective — p. 88; §33 characterizing — p. 91; §38 "being clever" — p. 96); Jespersen, MEG IV, ch. XII–XIV "The Expanded Tenses" (text-4, p. 206 ff.); Brown 1851, Part II, Ch. VI, "II. Compound or Progressive Form"; https://universaldependencies.org/en/feat/VerbForm.html (Part for VBG with aux); https://universaldependencies.org/en/dep/cop.html (Bill is speaking → aux).

```rule
rule: en.verbal.progressive-vbg-feats
what: VBG with an auxiliary is a participle (Part), not a gerund
match: v[upos=VERB, xpos=VBG]; a[rel=aux, head=v]
require: v[feats.VerbForm=Part]
severity: error
source: https://universaldependencies.org/en/feat/VerbForm.html (Part); Poutsma 1921 Expanded Form §1
```
