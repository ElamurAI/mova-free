# 12. Punctuation and typography

French uses the same marks as English, set differently. The differences that matter for reading and for a tokenizer are:
- the space before the high marks (*;*, *:*, *!*, *?*);
- the guillemets « » for quotations;
- the dash for dialogue;
- the comma in numbers;
- the apostrophe of elision;
- the hyphen of inversion and compounds.

This chapter gives the rules of the French national standard (the *Lexique des règles typographiques en usage à l'Imprimerie nationale*, the reference in France, which we hold only by its card), the Swiss and Canadian differences, and what our treebanks actually contain.

## 12.1 Spaces around the marks

| mark | space before | space after | example |
|---|---|---|---|
| . , … | none | normal | *Il pleut. Oui, il pleut…* |
| ; ! ? | thin non-breaking space (or a non-breaking space) | normal | *Vraiment ? Oui !* |
| : | non-breaking space | normal | *Il a dit : non.* |
| « | none | non-breaking space | *« Bonjour »* |
| » | non-breaking space | normal | |
| ( [ | normal | none | *(voir plus haut)* |
| ) ] | none | normal | |
| – (dash of an incise) | non-breaking space | normal | *Il est venu – enfin – hier.* |

The space before the high marks is a French rule, not a mistake. It must be non-breaking, so that the mark never begins a line. In Switzerland the space before *;*, *!* and *?* is usually left out. In Québec it is kept only before the colon and the closing guillemet in much current usage, but the Office québécois de la langue française recommends the French practice.

On our data the rule holds: in the GSD training set (UD 2.18, 2026-10-22), *;*, *:*, *!* and *?* stand after a space 1 548 times and directly after a word only 16 times. The spaces are ordinary spaces in the treebank text; the non-breaking character is not kept.

## 12.2 Quotations and dialogue

**Guillemets** « … » mark quotations and titles of articles. They are set with non-breaking spaces inside. A quotation inside a quotation takes the English quotes “ … ” (or another pair of guillemets):

> Il a dit : « Le mot “liberté” est usé. »

**Dialogue** is set with a dash at the head of each new speaker's line. Guillemets may open and close the whole exchange:

> — Tu viens ?
> — Non, dit-il.

An incise of the speaker (*dit-il, répondit Marie*) stays inside the dialogue, after a comma, with the subject inverted ([chapter 10](10-clause-structure.md)).

In GSD (2026-10-22) the guillemet occurs 866 times and the straight double quote 823 times: web texts often use the typewriter quote. The typographic quotes “ ” do not occur at all.

## 12.3 Capitals

French uses fewer capitals than English:
- **Languages, nationalities as adjectives, days, months** are lower case: *le français, un livre français, lundi, janvier*. A noun of nationality is capitalised: *un Français*.
- **Titles and functions** are lower case: *le président de la République, le pape, monsieur Dupont* (but *M. Dupont*).
- **Institutions:** the first significant word only: *l'Assemblée nationale, le Conseil constitutionnel, l'Académie française*.
- **Titles of works:** the first word and, in the French usage, a following noun and its adjectives: *Le Rouge et le Noir, À la recherche du temps perdu*.
- **Accents on capitals** are kept in correct typography: *État, À bientôt, ÉCOLE*. Older printing and much web text drop them (*A bientôt*), which makes *à* (preposition) and *a* (verb) look alike.

## 12.4 Numbers, dates and units

- **The decimal mark is a comma:** *3,14*; *1,5 million*.
- **Thousands are grouped by a space** (thin, non-breaking): *1 000 000*; never a comma, and a point only in money in some uses (*1.000 €*), which the standard avoids. Four-digit numbers may be written without the space (*2026*, *1500*).
- **Ordinals:** *1er, 1re, 2e, XXe siècle* (not *2ème, 2nd* in the standard, though both are common).
- **Dates:** *le 22 octobre 2026*, *le 1er mai*; numerically *22/10/2026* or *22.10.2026*.
- **Time:** *15 h 30*, *15h30*, *15:30*.
- **Units and currency** after the number, with a space: *25 km, 10 %, 3 €*.

## 12.5 Abbreviations

An abbreviation that keeps the last letter of the word takes no point: *M.* (monsieur, point, the last letter is not kept) but *Mme, Mlle, Dr, Me* (maître). *etc.* is preceded by a comma and never followed by *…* (\**etc…*). *p. ex.*, *c.-à-d.*. Acronyms are written in capitals without points: *SNCF, ONU*; read as a word, they may take only an initial capital: *Unesco, Insee*.

## 12.6 The apostrophe and the hyphen

**The apostrophe** marks elision ([chapter 1](01-sounds-and-spelling.md)): *l'arbre, j'ai, qu'il, s'il, aujourd'hui*. It is never followed by a space. The typographic apostrophe ’ is standard in print; the typewriter ' is common on the web. In GSD (2026-10-22) all 15 027 apostrophes are the typewriter one: the treebank normalizes them, so text with ’ needs mapping before parsing.

**The hyphen** joins:
- the inverted subject and the verb, with *-t-*: *viens-tu, a-t-il* ([chapter 9](09-questions.md));
- the imperative and its clitics: *donne-le-moi* ([chapter 4](04-pronouns.md));
- compounds: *arc-en-ciel, après-midi, grand-mère*;
- numbers below one hundred: *vingt-trois* (and all compound numbers after the 1990 rectifications: *deux-cent-trois*);
- *-ci, -là*: *ce livre-ci, celui-là*;
- *même* after a strong pronoun: *lui-même, nous-mêmes*.

## How Mova handles it

**Tokenization in UD French.** The words are cut as follows:
- an elided form keeps its apostrophe and is a token of its own: *l'* | *arbre*, *qu'* | *il*;
- an inverted pronoun is a token beginning with a hyphen: *viens* | *-tu*, *a* | *-t-il* ([chapter 9](09-questions.md));
- *-ce* in *est-ce* likewise;
- *-ci* and *-même* stay inside the word in GSD (*celui-ci*, *lui-même*: one token each), while *-là* is mostly split off (*ce jour* | *-là*; 13 times as a token of its own in the training set);
- compounds with a hyphen stay one token (*arc-en-ciel*);
- numbers with a space are one token with the space inside it (*1 000*: 144 such tokens in the GSD training set);
- the contracted articles *du, des, au, aux* are multiword tokens: *du* = *de* + *le* ([chapter 3](03-determiners.md)); 9 643 of them in the GSD training set (all counts 2026-10-22).

**In Mova.** French goes through Mova's English tokenizer when Mova reads raw text. With gold tokens from a treebank, as in the measurements of this manual, the tokenizer is not involved. On raw French text the English tokenizer:
- does not split the elisions as UD does (*qu'il*, *l'arbre*);
- does not split the inverted pronouns;
- does not expand *du, des, au, aux*;
- leaves the French space before *?* and *!* as a gap, which is harmless, but does not know the non-breaking space as part of a number (*1 000*).

A French tokenizer with these rules is the first piece of the French stack to write; until then Mova's French numbers are measured on gold tokens only. Punctuation in the trees is `PUNCT` attached as `punct` to the head of the phrase it opens or closes, as in English. `en xpos-fill` maps « to `` and » to '', so the English punctuation rules apply.
