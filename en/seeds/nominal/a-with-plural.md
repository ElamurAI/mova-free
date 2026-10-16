# The indefinite article and the plural

**Gist.** *A/an* comes from "one" and stands with a singular countable noun. With a plural it is possible only via a quantity word that makes the group "one portion": *a few days, a great many people, a good two hours, a further 45 days*.
**Conditions and exceptions.** *A hundred people* — the article is on the numeral (= *one hundred*). *A lot of people* — the article is on *lot*. A measure as a whole: *a three months*, *a twelve months* (Poutsma XXVI §17, literary). In a classifying genitive (*a children's story*) the article belongs to *story*, not *children*.
**Examples.** ✓ *a few days*; ✓ *a good many years*; ✓ *a further 45 days*; ✗ *a books*.
**In UD.** `det(a)` on a noun with `Number=Plur`: in EWT 2.18 — 115 times, and almost all have `amod` *few/many* or `nummod` on the same noun.
**Sources.** Poutsma GLME vol. 3, Ch. XXVI §17 (books/p/po/poutsma-1904-grammar-late-modern-english/text-3.md, p. 324 = book p. 304); Poutsma GLME vol. 4, Ch. XL §58–62 (*a few*), §89 (*a many*) (text-4.md, pp. 413–415, 430); UD `docs/_en/dep/nmod-poss.md` (*a children's story*).

The rule is `en.errors.a-singular` in `errors/determiner-noun-number.md`.
