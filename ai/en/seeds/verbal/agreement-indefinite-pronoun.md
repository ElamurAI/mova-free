# Each, either, neither, everyone… — singular

**Gist.** The pronouns *each, either, neither, one, everyone, everybody, someone, anybody, nobody, everything, something, nothing* are grammatically singular, even when followed by *of* + plural: *Each of you **is** entitled to his share*, *Neither of them **was** there*.

**Conditions and exceptions.** In colloquial speech *neither/either of* + plural often takes the plural (*Are either of you familiar…?*); Brown calls this an error. *None* can be singular or plural. *All, some, most* agree by meaning.

**Examples.** *Each of those ten million **has** a family.* — *Nothing **changes**.* — *Are either of you familiar with this project?* (colloquial, non-agreeing).

**In UD.** A subject with such a lemma does not go with `VBP` (neither on the head nor via `cop`/`aux`).

**Sources.** Brown 1851, Rule XIV, Note IV ("always in the third person singular") and Note VI; Reed & Kellogg, Higher Lessons, Lesson 142.

```rule
rule: en.verbal.indefinite-subject-not-vbp
what: each/either/neither/everyone… as subject with VBP — suspicious
match: v[xpos=VBP]; s[rel=nsubj, head=v, lemma=each|either|neither|everyone|everybody|someone|somebody|anyone|anybody|nobody|everything|something|anything|nothing]
require: exists c[rel=conj, head=s]
severity: warn
source: Brown 1851 Rule XIV, Note IV
```

```rule
rule: en.verbal.indefinite-subject-not-vbp-aux
what: each/either/neither… as subject with a VBP copula or auxiliary — suspicious
match: h[]; a[xpos=VBP, rel=cop|aux|aux:pass, head=h]; s[rel=nsubj, head=h, lemma=each|either|neither|everyone|everybody|someone|somebody|anyone|anybody|nobody|everything|something|anything|nothing]
require: exists c[rel=conj, head=s]
severity: warn
source: Brown 1851 Rule XIV, Note IV
```
