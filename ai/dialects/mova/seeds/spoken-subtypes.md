# Subtypes for spoken language (`discourse:filler`, `discourse:tag`, `conj:reform`) — proposal

**Proposal: wait.**

**What this is.** After 2.18, French documents three subtypes for spoken treebanks (`dialects/ud-next/seeds/fr-spoken-subtypes.md`). Universally, these constructions are being discussed by the UniDive spoken-language working group: #1290 (parataxis, tag questions, backchannels), #1280, #1273, #1289. Its fork of the guidelines has not yet been merged into UD.

**Why not now.**
- It would be more informative. The filler *um* is not the interjection *oh*, and a tag question is not ordinary parataxis. For spoken English (ESLSpok, the spoken part of GUM, CHILDES) these are different things.
- It could also be converted to the standard without loss: drop the subtype (`discourse:filler` → `discourse`). The reverse only via a lexicon of fillers.
- But there is no universal decision yet. English gold data do not have these subtypes, so there is nothing to train or measure on. Our own names before agreement with UniDive would only complicate compatibility (, section 4, item 8).
- They are not in the en registry, so export would have to drop them.

**When to come back.** When UniDive merges the spoken-language guidelines into UD or when Mova takes on spoken English treebanks.

**Origin.** `dialects/ud-next/seeds/fr-spoken-subtypes.md`; #1290, #1280, #1273, #1289.

Mova decides.
