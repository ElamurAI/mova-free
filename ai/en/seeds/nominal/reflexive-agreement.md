# The reflexive pronoun agrees with the subject

**Gist.** A reflexive pronoun repeats the subject of its verb, so it matches it in person and number: *I hurt myself*, *They blamed themselves*, *We enjoyed ourselves*.
**Conditions and exceptions.** Agreement is with the subject of the same verb (within one clause). In the imperative the subject *you* is not expressed (*Help yourself!*). With "singular they" — *themselves* or the rare *themself*. The pronouns *everyone, someone* (PRON NN, not PRP) take *themselves* or *himself/herself*.
**Examples.** ✓ *We enjoyed ourselves.*; ✓ *She blamed herself.*; ✗ *He hurt myself.*
**In UD.** EWT 2.18: in all 15 pairs "PRP subject + reflexive `obj`/`iobj`" person and number match. Rule language v1 compares features of two nodes (`feats.Person=@s`), so there is one rule for all persons and numbers.
**Sources.** Poutsma GLME vol. 4, Ch. XXXIV §3–4 (pp. 156–158 = book pp. 836–838).

```rule
rule: en.nominal.reflexive-agreement
what: a reflexive object matches the pronoun subject of its clause in person and number (I … myself, we … ourselves, they … themselves); you without number — any; singular they — themselves or themself
match: v[]; s[xpos=PRP, rel~nsubj, feats.Person, head=v]; r[feats.Reflex=Yes, feats.PronType=Prs, rel=obj|iobj|obl, head=v]
require: r[feats.Person=@s]; r[feats.Number=@s] or not s[feats.Number] or s[lemma=they]
severity: warn
source: Poutsma GLME IV Ch. XXXIV §3–4
```
