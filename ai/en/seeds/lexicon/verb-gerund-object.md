# Verbs that take a gerund, not an infinitive

**Gist.** Some verbs take an infinitive as complement, others only a gerund (*-ing*): *I enjoy swimming*, not *\*I enjoy to swim*. Only a gerund is taken by verbs of admitting and denying (*admit, acknowledge, deny*), avoiding and ceasing (*avoid, escape, finish, give up, leave off, quit, stop* in the sense "cease"), postponing (*postpone, put off, delay, defer*), and also *mind, miss, risk, resist, resent, relish, imagine, fancy, contemplate, consider, suggest, practise, enjoy, appreciate, keep*.

**Conditions and exceptions.** *Stop to smoke* is a different construction: *to smoke* here is purpose (`advcl`), not a complement. *Try, encourage, urge* are among Poutsma's "gerundial" verbs, but they are also used with an infinitive (in another sense or with a personal object), so they are not included in the rule. Non-native text often breaks the rule (*I enjoyed very much to study here*).

**Examples.** *He carefully avoided giving the least sign.* — *I have just finished dusting.* — *\*She denied to know him.*

**In UD.** The gerund with these verbs is `xcomp` with `VBG` (`VerbForm=Ger`); `xcomp` with `VB` + `mark(to)` is suspect.

**Sources.** Poutsma, *A Grammar of Late Modern English*, Part II, ch. XIX, §§17–18 (list of verbs requiring a gerund; vol. 2, p. 320); Poutsma 1923, *The Gerund*, §§43–45 (gerund vs infinitive; vol. 2, pp. 152–153); https://universaldependencies.org/en/feat/VerbForm.html (*I enjoyed working with you* — Ger).

```rule
rule: en.lexicon.gerund-verb-no-to-infinitive
what: verbs that require a gerund do not take a to-infinitive xcomp
match: v[lemma=enjoy|avoid|escape|finish|mind|consider|suggest|deny|admit|acknowledge|risk|miss|imagine|fancy|contemplate|postpone|delay|defer|practise|practice|quit|appreciate|resist|resent|relish|keep]; x[rel=xcomp, xpos=VB, head=v]; t[form=to, rel=mark, head=x]
require: not x[xpos=VB]
severity: warn
source: Poutsma GLME II ch. XIX §18; Poutsma 1923 Gerund §§43–45
```
