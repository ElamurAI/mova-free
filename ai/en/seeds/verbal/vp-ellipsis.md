# VP ellipsis: the auxiliary becomes the head

**Gist.** English often omits a repeated verb with its dependents, leaving the auxiliary: *Mary didn't leave, John **did***; *I can't, but she **can***; *So please update whatever you need **to***. Then the tree has no main verb, and its place is taken by the auxiliary (or *to*): the subject, negation and adverbials attach to it.

**Conditions and exceptions.** If the verb is omitted but two clause members remain (*Marie went to Paris and Miriam to Prague*), it is gapping (see `gapping-orphan.md`). Coordinated auxiliaries with one verb (*We can and will get there*) are not ellipsis: *will* is a `conj` of *can*. Brown required repeating differing forms in such places (*as he has [made] mine*).

**Examples.** *John will win gold and Mary will too* — `conj(win, will)`, `nsubj(will, Mary)`. — *Change anything you need to* — head *to*.

**In UD.** An auxiliary head keeps UPOS `AUX` (or `PART` for *to*) and receives the relation of the omitted verb (`conj`, `ccomp`, `xcomp`, `root`…), not `aux`.

**Sources.** https://universaldependencies.org/en/specific-syntax.html, VP ellipsis ("the auxiliary inherits the head-status… This includes the to nonfinite auxiliary"); https://universaldependencies.org/en/dep/orphan.html (VP ellipsis — no orphan); Brown 1851, Rule XVII, Note IX; Reed & Kellogg, Higher Lessons, Lesson 57 (Contraction of sentences).
