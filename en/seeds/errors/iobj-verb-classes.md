# iobj — only with double-object verbs

**Gist.** Two preposition-less objects are allowed by a limited class of verbs:
- transfer: *give, hand, sell, pay, send, offer, owe, promise*;
- communication: *tell, ask, show, teach, write, email*;
- "for someone": *buy, get, make, find, save, cook*;
- urging and informing with a clause: *convince, persuade, remind, warn, inform* + that or to — in EWT this is iobj + ccomp or xcomp.

The verbs *explain, describe, suggest, say, announce, mention, propose, introduce, donate* have no double object: ✗ *explain me the rule* → ✓ *explain the rule to me*. This is a typical error of English learners: a calque from the native language (Ukrainian "explain me"). LLMs err the other way round: they put `iobj` on a single object with *help, let, call*, where it should be `obj`.

**Conditions and exceptions.** The list is open: rare *envy, forgive, spare, fine, bill* also take two objects. EWT puts `iobj` with *tell, ask, trust, notify* even without a direct object (*tell me about it*), so the rule checks the verb, not the presence of obj. GUM uses `iobj` more widely (*presented me*).

**Examples.**
- ✓ *She told me the news.*
- ✗ *Can you explain me this?*
- ✓ *Help me!* → obj(Help, me).

**Check against gold.** EWT 2.18: 795 `iobj`, outside the class — 1 (*explain me* — an error in the text). GUM — 17 of 482.

**Sources.** UD `_en/dep/iobj.md` (iobj — «dative object»); `P17-1074` (ERRANT: verb and preposition replacements — VERB, PREP); `2026.udw-1.1` (iobj is unstable in LLMs).

The rule is `en.lexicon.iobj-verb` in `lexicon/verb-iobj-licensors.md`.

```rule
rule: en.errors.no-double-object
what: iobj with explain/describe/suggest/say — these verbs take a recipient only with to (explain it to me)
match: v[lemma=explain|describe|suggest|say|announce|mention|propose|introduce|donate|repeat|dedicate|confess]; i[rel=iobj, head=v]
require: not i[rel=iobj]
severity: warn
source: UD _en/dep/iobj.md, _en/dep/obl.md; P17-1074 (ERRANT: M:PREP, R:VERB)
```
