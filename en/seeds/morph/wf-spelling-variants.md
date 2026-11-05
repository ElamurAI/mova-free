# British and American spellings of derived words: -ise/-ize, -isation/-ization, -yse/-yze, -lled/-led

**Gist.** British and American English spell some endings differently: *organise / organize, organisation / organization, analyse / analyze, modelled / modeled, travelling / traveling, colour / color, centre / center*. The word, its class and its meaning are the same; only the spelling differs. The dictionary in code (AGID) lists the variants of common words, but a word derived in the text (*tokenise, tokenised, normalisation*) is often listed in neither spelling.

**Conditions and exceptions.**
- *-ise* is not always a variant of *-ize*: *advertise, advise, comprise, despise, devise, exercise, supervise, surprise, revise, televise* are spelled *-ise* in both varieties (see `wf-verb-suffixes`). Those are dictionary words, and the variant rule applies only to a word the dictionary does not know.
- British spelling doubles a final *l* after a single vowel before *-ed, -ing, -er* whatever the stress (*modelled, labelling, traveller*); American doubles it only under stress (*modeled* but *compelled*).
- The lemma keeps the spelling of the text (*tokenised → tokenise*, UD EWT keeps *organisation*).

**Examples.** *The text was **tokenised*** (VBN, lemma *tokenise*). ***Normalisation** helps* (NN). *We **modelled** it.*

**In UD.** The same UPOS, XPOS and FEATS as the other spelling; the lemma in the spelling of the form.

**Sources.** VarCon, the variant conversion table of SCOWL (Kevin Atkinson; British, American and Canadian spellings, *-ise/-ize, -our/-or, -re/-er, -yse/-yze, -ll-/-l-*); Kruisinga I §584 (always *-ise*: *advertise, advise, comprise…*; see `wf-verb-suffixes`).

```derive
# variant <British ending> <American ending>: a word the dictionary does not know, with the British ending, is read as
# the American spelling (dictionary word or derived by the other seeds' rules).
variant ise       ize
variant ised      ized
variant ises      izes
variant ising     izing
variant isation   ization
variant isations  izations
variant iser      izer
variant isers     izers
variant yse       yze
variant ysed      yzed
variant yses      yzes
variant ysing     yzing
variant lled      led
variant lling     ling
variant ller      ler
variant llers     lers
variant our       or
variant ours      ors
variant tre       ter
variant tres      ters
```
