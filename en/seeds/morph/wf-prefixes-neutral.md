# Prefixes that keep the part of speech: re-, pre-, over-, mis-, co-, anti-

**Gist.** Most English prefixes only refine the meaning and **do not change** the part of speech: *write → rewrite* (verb), *war → pre-war* (from a noun — a modifier), *confident → overconfident* (adjective), *pilot → co-pilot* (noun). So the tag of the derived word is usually the same as that of the base, and the lemma is the whole word together with the prefix.

**Conditions and exceptions.**
- Living native ones: *un-, mis-* (*misread, misunderstand*), *over-, under-, out-* (*overestimate, underpay, outrun*), *fore-* (*foresee*).
- Living borrowed ones: *re-* "again" (*reconsider, re-enter*), *pre-, post-, ante-* (*pre-war, postwar*), *co-* (*co-author*), *anti-, counter-* (*anti-war, counterattack*), *sub-, super-, ultra-, hyper-* (*subway, superstar*), *semi-, demi-, mini-, multi-, mono-, poly-*, *inter-, trans-* (*international, transatlantic*), *ex-* "former" (*ex-wife*), *vice-* (*vice-president*), *pseudo-, neo-, auto-*.
- A hyphen is used when without it the word would coincide with an older loan: *re-form* "form again" ≠ *reform* "a reform"; likewise *re-sign/resign, re-cover/recover, re-mark/remark*. Stressed *re-* [riː] is living, unstressed [rɪ] is in old words (*receive, repeat*), where the prefix is no longer felt.
- *Pre-war, anti-war, post-2000* before a noun are modifiers (JJ), although the base word is a noun.
- If the prefix is written as a separate word or split off at a hyphen by tokenization, EWT gives it XPOS AFX (see `wf-negative-prefixes`).

**Examples.** *I **rewrote** it* (VBD, lemma *rewrite*). ***pre-war** Europe* (JJ). *an **overconfident** player* (JJ). *the **co-author*** (NN).

**In UD.** Part of speech and tag as in the base word (or by function, as in *pre-war*); the lemma includes the prefix.

**Sources.** Jespersen MEG VI ch. XXVII–XXVIII (text-5, pp. 505–549: *pro-, super-, inter-, trans-, sub-, fore-, pre-, post-, co-, semi-, mono-, poly-, pseudo-, auto-, vice-, arch-*), 28.2₄ (pp. 538–539: *re-form/reform, re-sign/resign*); Sweet NEG I §1584–1588 (text-1, pp. 485–486: living native *un-, mis-*), §1620–1680 (pp. 498–509: *ante-, pre-, semi-, sub-, ultra-, demi-*), §1668 (p. 507: stressed and unstressed *re-*); Whitney §101 (text-1, pp. 60–61: prefixes do not change the word class); Santorini 1990, §4.1, p. 12 (hyphenated modifier — JJ).

**Technical English.** Papers use the same mechanism with a wider set: *bi-, tri-, uni-* "two, three, one" (*bidirectional, trilingual, unidirectional*), *intra-* "within" (*intra-sentential*), *cross-* (*cross-lingual*), *de-* "remove, reverse" (*deduplicate, decompress*), *micro-, macro-, meta-* (*micro-averaged, meta-learning*), *mid-* (*mid-training*), *self-* with a hyphen (*self-attention*), and *up-, down-* with verbs (*upsample, downsample, downweight*). The derived word keeps the class of its base (*downsampled* is a past form like *sampled*).

**Sources (technical prefixes).** Quirk, Greenbaum, Leech & Svartvik 1985, *A Comprehensive Grammar of the English Language*, Appendix I (prefixation: negative, reversative, pejorative, degree, attitude, locative, time, number prefixes); Bauer, Lieber & Plag 2013, *The Oxford Reference Guide to English Morphology* (the chapters on prefixation).

```derive
# prefix <affix> <classes of the stem> <shortest stem> [options] | <gloss>; see wf-negative-prefixes for the format.
# re- takes verbs (a noun in re- comes from its verb: reimplementation = reimplement + -ation); fore- is blocked
# where for- makes the word (*foregetting* is forgetting misspelt).
prefix re      V      4 | again
prefix pre     V,N,A  4 | before
prefix post    N,A,V  4 | after
prefix over    V,A,N  4 | too much; above
prefix under   V,A,N  4 | too little; below
prefix out     V      4 | more than, surpass
prefix fore    V,N    4 block=for | before
prefix co      V,N,A  5 | together, jointly
prefix anti    N,A    4 | against
prefix counter N,V,A  4 | against, opposite
prefix sub     N,A,V  4 | under; a part of
prefix super   N,A    4 | above, beyond
prefix ultra   A,N    4 | beyond
prefix hyper   N,A    4 | over, beyond
prefix semi    A,N,V  4 | half, partly
prefix mini    N      4 | small
prefix multi   N,A    4 | many
prefix mono    N,A    4 | one
prefix poly    N,A    4 | many
prefix inter   N,A,V  4 | between
prefix trans   N,A,V  4 | across
prefix pseudo  N,A    4 | false, seeming
prefix neo     N,A    4 | new
prefix auto    N,V,A  4 | self
prefix bi      N,A    4 | two
prefix tri     N,A    4 | three
prefix uni     N,A    4 | one
prefix intra   N,A    4 | within
prefix cross   N,A,V  4 | across
prefix de      V,N    5 | remove, reverse
prefix micro   N,A,V  4 | small
prefix macro   N,A,V  4 | large
prefix meta    N,A    4 | about, beyond
prefix mid     N      4 | middle
prefix self-   N,A,V  4 | of or by itself
prefix up      V      4 | up, to a higher level
prefix down    V      4 | down, to a lower level
```
