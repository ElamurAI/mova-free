# Articles a/an and the

**Gist.** The indefinite article has two forms: *a* before a consonant **sound** and *an* before a vowel **sound**. The choice depends on the pronunciation of the next word, not on the letter: *an hour, an MP, an FBI agent*, but *a university, a one-time offer, a European*. Both forms are one article with lemma *a*. The definite article *the* is single; only its pronunciation changes (before a vowel sound — [ði]).

**Conditions and exceptions.**
- Before words with silent *h* — *an* (*an honest man, an heir*); before pronounced *h* the modern norm is *a* (*a hotel, a historic*), although old texts have *an hotel*.
- Before abbreviations the name of the first letter decides: *an SMS* ([es]), *a UN report* ([ju:]).
- If an adjective stands between the article and the noun, look at the adjective: *an old car, a used car*.
- Errors in the text (*a apple*) do not change the annotation: lemma and features are the same.

**Examples.** *a book, an apple, an hour, a university, the end.*

**In UD.** UPOS DET; XPOS DT; relation `det`. *A, an*: lemma *a*, `Definite=Ind|PronType=Art`. *The*: lemma *the*, `Definite=Def|PronType=Art`.

**Sources.** https://universaldependencies.org/en/feat/Definite.html, `feat/PronType.md` (Art), https://universaldependencies.org/en/pos/DET.html (lexeme table); Santorini 1990, §2, p. 2 (DT: articles *a(n), the*); Sweet NEG I §1137 (text-1, p. 383: *a history*, but *an historical*; *a unit, a youth*; *an unit, an useless* — survivals); Whitney §220–221 (text-1, p. 113: *an historical, an hotel*, but *a European, a union*); Kruisinga II.2 §§1305–1307 (text-3, p. 331: *an hotel, an university* — traditional or affected; *an M.P.*); Mätzner I, p. 317 (text-1, p. 335: *an hour, an heir, an historical subject*).

```rule
rule: en.morph.article-indefinite
what: a and an — one lemma a, indefinite article
match: d[upos=DET, form=a|an, !feats.Typo]
require: d[lemma=a, xpos=DT, feats.Definite=Ind, feats.PronType=Art]
severity: error
source: UD en feat/Definite (Ind: a, an), pos/DET
```

```rule
rule: en.morph.article-definite
what: the — definite article
match: d[upos=DET, form=the, !feats.Typo]
require: d[lemma=the, xpos=DT, feats.Definite=Def, feats.PronType=Art]
severity: error
source: UD en feat/Definite (Def: the), pos/DET
```
