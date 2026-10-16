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
