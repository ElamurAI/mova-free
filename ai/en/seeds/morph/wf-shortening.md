# Shortening of words: phone, exam, lab; back-formation: edit, burgle

**Gist.** English easily shortens long words. **Clipping** mostly keeps the beginning: *telephone → phone, examination → exam, laboratory → lab, advertisement → ad, photograph → photo, bicycle → bike, public house → pub, veterinarian → vet, influenza → flu, mathematics → maths/math*. Such words become ordinary nouns with their own plural (*phones, exams*). **Back-formation** derives a new word by dropping an imagined suffix: *editor → edit, burglar → burgle, enthusiasm → enthuse, television → televise, housekeeper → housekeep, pease → pea*.

**Conditions and exceptions.**
- A clipped word is a separate lexeme: the lemma is the clipped form itself (*phones → phone*), not the full word (*telephone*).
- UD gives the feature `Abbr=Yes` to written abbreviations (*Mr., govt, e.g., u* for *you*), not to clipped words that have become ordinary: *phone, exam, info* in EWT have no `Abbr`.
- Sometimes the end is kept (*phone, bus* from *omnibus*) or the middle (*flu*); unstressed syllables drop from the beginning (*mend < amend, fence < defence, sport < disport*) — then different words result.
- Initialisms (*M.P., a.m., BBC*) are a separate type; in annotation their tag is by function (NNP for names), `Abbr=Yes` is set inconsistently in EWT.
- Back-formation often changes the part of speech: a noun yields a verb (*edit, burgle, babysit*).

**Examples.** *My **phone** died. Two **exams** left. She **edits** the paper* (VERB, lemma *edit*).

**In UD.** NOUN/VERB with the lemma of the clipped or back-formed form; without `Abbr=Yes` if it is an ordinary word.

**Sources.** Jespersen MEG VI 29.2–29.6 (text-5, pp. 550–562: *exam, gym, lab, ad, photo, specs, pants*; "stump-words"), 29.3 (pp. 553–554: back-formation, *enthuse, housekeep*), 4.4₃ (p. 53: *edit* from *editor*), 29.8₁ (p. 564: *mend < amend, fence < defence*), 29.9₁ (p. 567: initials); Kruisinga II.3 §§1904–1911 (text-4, pp. 182–186: *photo, bike, pub, lab, vet, phone, exam, maths*; *burgle, caretake, subedit*); Mätzner I, p. 173, 177 (text-1, pp. 191, 195: *wig < periwig, cit, incog*); Sweet NEG I §998, §1001 (text-1, pp. 343–345: *pea, Chinee, stave*); EWT 2.18 practice (*phone, info, exam* without `Abbr`).

```rule
rule: en.morph.clipping-lemma
what: phone, exam, lab, ad, photo, bike… — lemma is the clipped form, not the full word
match: w[upos=NOUN, form=phone|phones|exam|exams|lab|labs|ad|ads|photo|photos|flu|bike|bikes|plane|planes|math|maths|memo|memos|demo|demos|typo|typos|limo|limos|app|apps|fridge|fridges|gym|gyms|pub|pubs|vet|vets|info|intro|intros|promo|promos, !feats.Typo]
require: w[lemma=phone|exam|lab|ad|photo|flu|bike|plane|math|maths|memo|demo|typo|limo|app|fridge|gym|pub|vet|info|intro|promo]
severity: error
source: Jespersen MEG VI 29.2–29.6; Kruisinga II.3 §§1904–1907; EWT 2.18 practice
```

```rule
rule: en.morph.clipping-not-abbr
what: a clipped word that has become ordinary (phone, exam, lab) has no Abbr=Yes
match: w[upos=NOUN, form=phone|phones|exam|exams|lab|labs|photo|photos|bike|bikes|plane|planes|memo|memos|demo|demos|typo|typos|limo|limos|app|apps|fridge|fridges|gym|gyms|pub|pubs|info|intro|intros|promo|promos, !feats.Typo]
require: w[!feats.Abbr]
severity: warn
source: Jespersen MEG VI 29.2–29.6; EWT 2.18 practice
```
