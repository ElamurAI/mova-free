# Prefixes that change the part of speech: a- (asleep), be- (befriend), en- (enlarge)

**Gist.** Most English prefixes do not change the part of speech (*write → rewrite*, both verbs). Three old prefixes are exceptions. *A-* (from the preposition *on*) makes a predicative adjective or adverb from a noun or verb: *sleep → asleep, float → afloat, live → alive, head → ahead*. *Be-* makes a verb from a noun or adjective (*friend → befriend, little → belittle, calm → becalm*) and adjectives in *be-…-ed* (*bespectacled*). *En-/em-* makes a verb meaning "make so": *large → enlarge, rich → enrich, power → empower, able → enable*.

**Conditions and exceptions.**
- Adjectives in *a-* (*asleep, afraid, alive, awake, aware, alike, alone, ablaze, afloat, adrift, aglow, ashamed*) stand only **after** the noun or in the predicate: *the child is asleep*, *the people asleep*, but not *an asleep child*. Before a noun another word is used: *live* (not *alive*) — *a live broadcast*; *sleeping* — *a sleeping child*.
- No *-ly* adverbs are formed from them (*afraidly* is impossible).
- *Alone, alike, ahead, away, aside, apart* are often adverbs (ADV).
- *Em-* — before *b, p* (*embody, empower*); the meaning is like the verbal suffix *-en* (*enrich* ~ *sweeten*).

**Examples.** *The baby is **asleep*** (ADJ, predicate). *Children **asleep** in the car* (ADJ after the noun). *They **befriended** him* (VERB). *We need to **enlarge** it* (VERB).

**In UD.** *A*-adjectives are ADJ (JJ); if they modify a noun, then via relation `amod` and only after it. *Be-, en-* derivatives are ordinary VERB with the lemma including the prefix (*befriend, enlarge*).

**Sources.** Jespersen MEG VI 27.3₁–27.3₂ (text-5, pp. 510–512: *a-* < *on*; predicative *alive* vs. attributive *live*; *aloud* an adverb), 22.9₅–22.9₇ (pp. 430–431: no *-ly* from predicative *a*-adjectives), 27.4₄, 20.5₉ (pp. 515, 374: *en-/em-*), 28.1₁–28.1₃ (pp. 534–535: *be-*); Sweet NEG I §1506 (text-1, p. 463: *away, alive, asleep, aback*), §1588, §1572 (pp. 482, 486: *be-* the only living old verbal prefix), §1653 (pp. 504–505: *en-, em-*); Whitney §225b (text-1, pp. 118–119: *behead, belittle; enthrone, embody, embolden*).

```rule
rule: en.morph.a-adjective-postposed
what: asleep, afraid, alive, aware etc. as modifiers stand only after the noun
match: n[upos=NOUN|PRON|PROPN]; a[form=asleep|afraid|alive|aware|awake|ashamed|ablaze|afloat|adrift|aglow|akin|averse|alike, rel=amod, head=n, !feats.Typo]
require: a[after=n]
severity: warn
source: Jespersen MEG VI 27.3₁–27.3₂ (alive predicative vs live adjunct); Sweet NEG I §1506
```
