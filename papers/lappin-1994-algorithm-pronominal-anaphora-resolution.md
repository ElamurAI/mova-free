# An Algorithm for Pronominal Anaphora Resolution

**Authors:** Shalom Lappin, Herbert J. Leass · **Year:** 1994 · **Venue:** Computational Linguistics, 20(4)
**Link:** https://aclanthology.org/J94-4002/ (ACL Anthology J94-4002)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, pre-2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The paper presents RAP (Resolution of Anaphora Procedure), a classic rule-based algorithm for resolving third-person pronouns (including reflexives and reciprocals) using syntactic information from a parser. Candidate antecedents are assigned a salience score built from weighted factors such as recency, grammatical role (subject, existential, direct object, indirect object/oblique), head-noun status and not being inside an adverbial phrase. Salience is halved for each sentence boundary crossed, and role parallelism and cataphora adjust the score. Syntactic filters (agreement and binding-style constraints on co-arguments) remove impossible candidates before the highest-scoring one is chosen. The authors evaluate RAP on technical manuals and compare it with Hobbs' syntactic search algorithm, reporting that the salience approach performs at least as well. The work became a standard reference point for knowledge-poor, transparent pronoun resolution.

## How Mova uses it
- the development notes of `ai/coref` ("What next") — the salience weights of this paper (recency 100, subject 80, existential 70, object 50, indirect object and oblique 40, head noun 80, non-adverbial 50; halving per sentence; parallelism +35, cataphora −175) are recorded as a candidate replacement or complement for the Hobbs-style candidate order used in the `pronoun` sieve.
- Not implemented yet; the current resolver orders candidates in a Hobbs-like breadth-first tree traversal.

## Effectiveness in Mova
Not tried yet (the coref README marks it explicitly as "not tried"). Planned idea only; not measured.

---

👨‍🔬💥
