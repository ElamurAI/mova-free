# The Cambridge Grammar of the English Language

**Authors:** Rodney Huddleston, Geoffrey K. Pullum (and collaborators) · **Year:** 2002 (cited in Mova as "Huddleston & Pullum 2001") · **Venue:** Cambridge University Press (book)
**Link:** unknown
**License of the paper:** unknown (commercially published book, not openly licensed)

## Summary
This is a large reference grammar of contemporary Standard English, written by a team led by the two authors. It describes English syntax and morphology systematically, using a carefully defined set of categories (word classes) and functions (subject, object, complement, modifier and so on). It departs from traditional school grammar in several places, for example in how it treats prepositions, determinatives and subordinators. It is widely used by linguists and computational linguists as an authoritative source on what English constructions are possible. Among many other topics, it discusses which adjectives can take a noun phrase complement directly, such as *worth*, *like* and *unlike*.

## How Mova uses it
- `ai/en/seeds/verbal/core-objects.md` — the rule `en.verbal.object-head-adjective` states that among adjectives only *worth*, *like* and *unlike* take a direct object (`obj`/`iobj`) in UD.
- The rule's source is given as the UD English documentation (core arguments), which itself follows this grammar, plus Poutsma 1923; the book is used indirectly, through UD's guidelines, not read directly.
- The rule is a declarative check on parsed trees with severity "warn", used to flag suspicious annotation or parser output.

## Effectiveness in Mova
Not measured separately. The book contributes one narrow constraint (a short whitelist of adjectives) to the rule-based checks; its value is in catching rare annotation/parse errors rather than in a measurable accuracy gain.

---

👨‍🔬💥
