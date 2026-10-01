# Pluralia tantum: plural-only nouns

**Gist.** Some nouns have only a plural form. These are things made of two identical halves (*trousers, jeans, pants, scissors, pliers, glasses* "spectacles") and collective or abstract names (*clothes, goods, thanks, contents, surroundings, whereabouts, headquarters, premises, outskirts, belongings, proceedings*). There is no singular in this meaning.
**Conditions and exceptions.** Paired things are counted with *pair*: *a pair of scissors*. Some have a singular, but with a different meaning: *glass* "glass (material)" — *glasses* "spectacles"; *content* — *contents*; *good* — *goods*. *Headquarters, means, whereabouts* can also take a singular verb.
**Examples.** *These trousers are new.*; *Thanks a lot.*; *The goods were shipped.*
**In UD.** NOUN NNS with `Number=Ptan` (the UD 2.18 registry for English allows Ptan for NOUN and PROPN). The lemma is the plural form itself: *clothes*, *goods*. EWT 2.18: *regards* 53, *troops* 22, *supplies* 12, *means* 11, *clothes* 9; decades (*1970s*) are also Ptan.
**Sources.** Poutsma GLME vol. 3, Ch. XXV §18–21 (pp. 167–252 = book pp. 147–232); Curme 1931, Ch. XXVI "Nouns without a Singular" (p. 543); registry `en/data/ud-registry-en.tsv` (NOUN `Number=Ptan`).

```rule
rule: en.nominal.ptan-lemmas
what: classic pluralia tantum in plural form have Number=Ptan
match: n[upos=NOUN, xpos=NNS, lemma=trousers|jeans|pants|scissors|pliers|tongs|clothes|goods|outskirts|premises|whereabouts|headquarters|surroundings|belongings|proceedings]
require: n[feats.Number=Ptan]
severity: warn
source: Poutsma GLME III Ch. XXV §18–19; UD en Number=Ptan
```
