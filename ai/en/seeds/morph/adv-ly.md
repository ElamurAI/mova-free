# Adverbs in -ly and adjectives in -ly

**Gist.** The suffix *-ly* is the main way to form an adverb from an adjective: *quick → quickly, careful → carefully*. It is productive: an adverb can be made from almost any adjective, even from a participle (*willingly, reportedly*). But *-ly* is not a reliable mark of an adverb: the same suffix added to a **noun** gives an adjective (*friend → friendly, man → manly, day → daily*).

**Conditions and exceptions.**
- Spelling: *-y* after a consonant → *-ily* (*happy → happily*); *-le* → *-ly* (*simple → simply, able → ably*); *-ll* + *-ly* → *-lly* (*full → fully*); *-ic* → *-ically* (*basic → basically*; exception *publicly*); *true → truly, due → duly, whole → wholly*.
- Adjectives in *-ly* (*friendly, lovely, lonely, ugly, silly, holy, elderly, costly, orderly, manly, sickly*) do not form an *-ly* adverb of their own: people say *in a friendly way*.
- *Daily, weekly, monthly, yearly, early, kindly, only* — both adjectives and adverbs without change of form; syntax decides the part of speech.
- An *-ly* adverb from a participle or a derived adjective is unambiguous: *-ously, -ically, -fully, -lessly, -ably/-ibly, -tively/-sively, -ingly, -edly* give only adverbs (*obviously, basically, carefully, probably, relatively, surprisingly, reportedly*).
- The lemma of an *-ly* adverb is the adverb itself (*quickly → quickly*), not the adjective: in UD the adverb is a separate lexeme.
- Such adverbs are compared periphrastically (*more quickly*) and have no `Degree` feature in UD (see `adv-flat-degree`).

**Examples.** *She spoke **quietly**. He is **friendly*** (ADJ). *The paper comes out **daily*** (ADV) / *a **daily** paper* (ADJ). ***Surprisingly**, it worked.*

**In UD.** Adverb: ADV, XPOS RB, no `Degree`; lemma = form. Adjective in *-ly*: ADJ, JJ, `Degree=Pos`.

**Sources.** Sweet NEG I §1500 (text-1, p. 460: *-ly* is productive; *fully, nobly, merrily*; adjectives in *-ly* avoid adverbial *-ly*: *in a friendly manner*; *daily, yearly* — adjectives used as adverbs), §341 (text-1, p. 149: *-ly* is not a mark of an adverb: *goodly, manly*); Whitney §94, §313a (text-1, p. 58, 156: *-ble → -bly, -ic → -ically*), §193 (text-1, p. 102–103: *manly, brotherly, daily, deadly* — adjectives); Santorini 1990, §2, p. 2 (RB: most words in *-ly*), §4.1, p. 14 (JJ or RB — by what the word modifies); UD EWT 2.18 (the lemma of an *-ly* adverb is the form itself); Jespersen MEG VI 22.7₁–22.9₄ (text-5, p. 422–430: adjectives in *-ly*; *in a friendly manner*), 22.8₂–22.8₆ (p. 425–428: spelling; *publicly* — the only exception to *-ically*); Kruisinga II.3 §§1709–1716 (text-4, p. 74–77); Mätzner I, p. 395, 441 (text-1, p. 413, 459: *fatherly, friendly, lovely, daily, deadly, only*).

```rule
rule: en.morph.derived-adverb-suffix
what: words in -ously, -ically, -fully, -lessly, -ably, -ibly, -tively, -sively, -ingly, -edly — adverbs
match: w[suffix=ously|ically|fully|lessly|ably|ibly|tively|sively|ingly|edly, !feats.Typo]
require: w[upos=ADV|PROPN|X]
severity: warn
source: Sweet NEG I §1500; Whitney §94, §313a; Santorini 1990, §2, p. 2
```

```rule
rule: en.morph.ly-adjective
what: friendly, lovely, lonely, ugly, silly etc. — adjectives (noun + -ly), not adverbs
match: w[form=friendly|unfriendly|lovely|lonely|ugly|silly|holy|elderly|costly|orderly|curly|chilly|cowardly|worldly|homely|comely|ghastly|burly|surly|stately|sickly|manly|womanly|motherly|fatherly|brotherly|scholarly, !feats.Typo]
require: w[upos=ADJ|NOUN|PROPN]
severity: warn
source: Sweet NEG I §341, §1500, §1614; Whitney §94, §193
```
