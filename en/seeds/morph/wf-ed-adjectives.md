# Adjectives in -ed from nouns: talented, blue-eyed

**Gist.** The suffix *-ed* added to a **noun** forms an adjective meaning "having X": *talent → talented, beard → bearded, money → moneyed*, and with a modifier — a whole group: *blue eye → blue-eyed, long leg → long-legged, bad temper → bad-tempered, open mind → open-minded*. It sounds and is spelled the same as the participle ending, but there is no verb behind it (*to talent* does not exist). So such words are adjectives, not VBN.

**Conditions and exceptions.**
- Pronunciation: [ɪd] in some old words (*talented, landed, wicked, wretched, rugged, ragged, crooked, naked*), [d] in compounds (*blue-eyed, long-legged*).
- From *be-…-ed*: *bespectacled, bewigged, bemedalled* — also adjectives.
- Borderline cases where a verb does exist: *gifted* (*to gift*), *skilled*, *armed*, *winged*; then meaning decides (a state-property — JJ).
- Spelling: *ivied, propertied, honeyed*.

**Examples.** *a **talented** singer; a **blue-eyed** boy; an **open-minded** person; a **bearded** man.*

**In UD.** ADJ, JJ, `Degree=Pos`, the lemma is the form itself (*talented*, *blue-eyed*), without `VerbForm` and `Tense`.

**Sources.** Jespersen MEG VI 24.1₁–24.1₅ (text-5, pp. 441–445: *bearded, talented, moneyed, landed; long-legged, blue-eyed, bad-tempered*; *blue-eyed* = (*blue eye*) + *-ed*), 28.1₃ (p. 535: *bespectacled, bewigged*); Sweet NEG I §907 (text-1, p. 322: *hare-brained, humpbacked* — from the adjective suffix, not participles), §1606 (p. 493: *-ed* from nouns, *big-headed*); Whitney §194 (text-1, pp. 103–104: *red-haired, dark-eyed, old-fashioned*); Santorini 1990, §4.1, pp. 15–16 (JJ if there is no verb).

```rule
rule: en.morph.ed-compound-adjective
what: compounds in -eyed, -haired, -minded, -hearted, -handed etc. — adjectives
match: w[suffix=-eyed|-haired|-legged|-minded|-hearted|-handed|-sided|-shaped|-faced|-headed|-footed|-tongued|-skinned|-coloured|-colored|-tempered|-natured, !feats.Typo]
require: w[upos=ADJ]
severity: warn
source: Jespersen MEG VI 24.1₁–24.1₅; Sweet NEG I §907, §1606; Whitney §194
```

```rule
rule: en.morph.ed-denominal-adjective
what: talented, moneyed, wicked, naked, rugged etc. — adjectives without a verb
match: w[form=talented|untalented|moneyed|monied|salaried|wooded|bigoted|wicked|wretched|naked|rugged|ragged|jagged|sacred|bespectacled, !feats.Typo]
require: w[upos=ADJ]
severity: warn
source: Jespersen MEG VI 4.2₂, 24.1₁–24.1₄; Sweet NEG I §1606
```
