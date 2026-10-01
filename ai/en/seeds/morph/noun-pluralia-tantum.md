# Plural only: clothes, scissors, trousers, goods

**Gist.** Some nouns have only a plural form: they have no singular, or the singular means something else. These are paired objects (*scissors, trousers, jeans, pants, pyjamas, tongs, pliers, binoculars, glasses* "spectacles") and collective, mass-like meanings (*clothes, goods, riches, outskirts, surroundings, whereabouts, belongings, proceedings, earnings, savings, remains, headquarters, barracks*). They agree with a plural verb (*my trousers are*), and are counted with *a pair of*.

**Conditions and exceptions.**
- Paired objects never lose *-s* as the head of a phrase; before a noun the singular occurs: *trouser pocket, scissor kick*.
- Some words have a singular with a different meaning: *good* (benefit) — *goods* (merchandise); *arm* (limb) — *arms* (weapons); *manner* — *manners* (good behaviour); *content* — *contents* (what is inside).
- *Riches, alms, eaves* — former singulars in *-s* that came to be perceived as plurals.
- *Headquarters, barracks, means, crossroads* agree with both singular and plural.
- The lemma of such a word is the plural form itself (*clothes → clothes*, not *clothe*); the UD feature is `Number=Ptan` ("plurale tantum"). EWT uses `Ptan` widely: *regards, troops, supplies, contents, grounds, finances, specifics*, names of decades (*1990s*). On the other hand, EWT 2.18 has *congratulations, belongings* as an ordinary plural with a singular lemma.

**Examples.** *Where are my **scissors**? The **clothes** are dry. We sell **goods** online. On the **outskirts** of town.*

**In UD.** NOUN, XPOS NNS, `Number=Ptan`; lemma = form (*scissors, clothes, goods*).

**Sources.** https://universaldependencies.org/en/feat/Number.html (Ptan: *clothes, scissors, riches*; "the lemma is therefore the plural form"; *linguistics, species* — not Ptan); Sweet NEG I §998 (text-1, p. 343: *alms, eaves, riches* former singulars), NEG II §1979–1980 (vol. 2 = text-2, p. 63: *scissors, bellows, spectacles* never lose *-s*, counted with *a pair of*); Whitney §129 (text-1, p. 74: *bellows, tongs, shears, trousers, measles, victuals, annals, thanks*); Jespersen MEG II 5.73–5.77 (text-2, p. 182–189: *a scissors, a tongs; wages; amends, thanks*), VI 16.7₄ (text-5, p. 285: metanalysis of *alms, riches*); Kruisinga II.2 §§759, 808–819 (text-3, p. 27, 49–53); Mätzner I, p. 233–241 (text-1, p. 251–259: *scissors, tongs, pliers, wages, thanks*).

```rule
rule: en.morph.ptan-lemma
what: clothes, scissors, trousers, goods etc. — plurale tantum, lemma = form
match: n[upos=NOUN, form=clothes|scissors|trousers|jeans|pants|pajamas|pyjamas|sunglasses|tongs|pliers|binoculars|goods|outskirts|surroundings|whereabouts|headquarters|barracks|proceedings|auspices|environs|tenterhooks|remains|savings|earnings, !feats.Typo]
require: n[feats.Number=Ptan, lemma=clothes|scissors|trousers|jeans|pants|pajamas|pyjamas|sunglasses|tongs|pliers|binoculars|goods|outskirts|surroundings|whereabouts|headquarters|barracks|proceedings|auspices|environs|tenterhooks|remains|savings|earnings]
severity: warn
source: UD en feat/Number (Ptan); Sweet NEG II §1979–1980; Whitney §129; EWT 2.18 practice
```
