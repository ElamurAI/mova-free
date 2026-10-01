# Global-level seeds (`ai/global/seeds`)

A **seed** is a short markdown note from which a rule or a piece of knowledge grows. The seeds in this
folder hold the **global level** of Mova's small language model (MMM): fixed, compiled knowledge that
holds in every story and every task. It includes values (moral principles), concepts and categories,
verb classes, shortcut links between concepts ("from → to : why"), properties of relations, and
commonsense facts. This page gives a summary of each one.

Each seed has the sections **Gist**, **Conditions and exceptions**, **Examples** and **Sources**, plus
one or more machine-readable blocks. When the crate is built, `global/build.rs` reads every
`seeds/**/*.md`, extracts the fenced blocks below and generates Rust tables (`PRINCIPLES`, `CONCEPTS`,
`LINKS`, `RELATIONS`, `verb_class()`). At run time nothing is read from disk. Every conclusion the model
draws reports which link and which seed file it came from, so each judgment comes with its explanation.

## Block formats

**```principle**: a value, given as `key: value` lines.

```principle
id: justice
rule: take_without_consent
name: justice
gist: do not take what is not yours; give what is due
against: steal rob grab snatch
towards: return repay
```

- `id`, `name`, `gist`, `against` and `towards` are required. `rule` is optional.
- `rule` names a predicate over the story world. It is evaluated on world states, not on words, and
  it powers the real judgment.
- `against` and `towards` are verbs that go against the value or in its spirit. A plain-text fallback
  judgment uses them. In multiword verbs, `_` stands for a space.
- Duplicate ids fail the build.

**```concept**: one concept per line, `name: trigger words`.

```concept
fall: fall drop tumble
```

- A concept is "touched" by a text when one of its trigger words appears. Simple inflections match:
  *-s, -es, -ed, -d, -ing*, a doubled final consonant, *y → ie*, and a few irregular forms.
- Concept names must be unique across all seeds, or the build fails.

**```category**: the same syntax as `concept`, but the entry is flagged as a category (countable things,
measures, time units, scene attributes, commonsense lookups).

```category
time_unit: second minute hour day week month year decade century
```

A category explains reasoning steps but is not an event. It never starts a chain of consequences.

**```link**: one shortcut per line, `from -> to : why`.

```link
fall -> accelerate : without support a body falls with acceleration g ≈ 9.8 m/s²
```

- Both ends must be declared concepts or verb classes, and duplicate links fail the build.
- At run time, the model finds the shortest chain of links between two concepts (breadth-first search)
  and lists all forward consequences of the concepts a text touches. Each step carries its *why*.

**```verbs**: a verb class and its effect on a quantity, `class effect verbs…`.

```verbs
give ± give pass hand_over sell lend pay
```

- The effect is one of `=` (no change), `+` (increase), `-` (decrease) or `±` (decrease for one
  participant, increase for the other).
- Each class also becomes a concept, with its verbs as triggers. A class name that clashes with a
  concept fails the build.
- `verb_class(lemma)` returns the class and effect of the first class that lists the lemma.

**```relation**: properties of a relation, `name: key=value …`.

```relation
north_of: inverse=south_of vec=0,1 step=n
```

Keys used so far:

| key | meaning |
|---|---|
| `inverse` | the inverse relation |
| `vec` | direction on a plane, as x,y |
| `step` | compass step letter |
| `transitive=yes` | the relation chains |
| `symmetric=yes` | A rel B implies B rel A (declarative; the SpartQA reader treats near, far and touching as symmetric in its own code) |
| `same` | an alias of another relation |
| `order` | position in a time sequence |
| `go` | goal location for a state |

They are queried with `relation(name, key)` and `relations_with(key)`. A duplicate relation name fails
the build.

---

### mmm
`ai/global/seeds/self/mmm.md`

**Gist.** This seed is the model's self-description. MMM is a small language model: small, fast and
CPU-only. Its modules (judge, second opinion, what-if) fire on their own. The model does not decide
whether to call them, but it produces the result and a debug trace that shows whether a trigger was
useful, harmful or redundant. The model is retrained, and the global level extended, from this trace and
from its reports.

**Conditions and exceptions.** Complex reasoning is not the small model's job. A large language model
(LLM) is a separate tool for it. During training the small model does not call the LLM. It files a
report instead.

**Examples.** The judge changes the first answer, and the debug trace records whether this was useful
(the first answer was wrong and the judge was right), harmful (the first answer was right) or neither
(both were wrong). When the two opinions disagree, the model does not answer but files a report. The
trace records whether the report was justified or a correct answer was lost.

**Machine-readable part.**
- Concepts: `mmm`, `module_judge`, `module_second_opinion`, `module_whatif`, `self_report`,
  `self_debug`, `retrain`, `context_logic`.
- Links: mmm → self_debug; module_judge → self_debug; module_second_opinion → self_report;
  module_whatif → context_logic (what-if on a class of tasks triggers a rewrite of the parsing logic);
  self_report → retrain ("teach me" reports queue new first-level seeds); self_debug → retrain.

**Sources.** Internal design notes.

### atomic-feelings
`ai/global/seeds/shortcuts/atomic-feelings.md`

**Gist.** This seed says which feeling the actor usually has after an event (ATOMIC-2020 `xReact`):
*hit, beat, fight* → anger; *lose, miss, cry, bury* → sadness; *break, drop, steal, forget* → shame;
*hide, run* → fear; *receive, thank, marry, hug* → gratitude; *find, see, meet* → surprise. The seed is
generated from the data.

**Conditions and exceptions.** It covers the actor's feeling only. The feeling of the person acted on is
in `atomic-feelings-o`. Verbs are taken without context. A verb is selected when its share of a feeling
category is at least twice the baseline and it has at least six feeling reactions. Function verbs are
excluded. ATOMIC is mostly positive, so joy cannot be selected this way and comes from `feelings`
instead.

**Examples.** None beyond the verb–feeling pairs in the gist.

**Machine-readable part.** Categories `atomic_anger_event`, `atomic_fear_event`,
`atomic_gratitude_event`, `atomic_sadness_event`, `atomic_shame_event` and `atomic_surprise_event`, each
with its verb list. Some entries are stems such as *crie* that match inflected forms. One link from each
category to the matching feeling (anger, fear, gratitude, sadness, shame, surprise).

**Sources.** ATOMIC-2020 (Hwang et al. 2021, CC BY 4.0).

### atomic-feelings-o
`ai/global/seeds/shortcuts/atomic-feelings-o.md`

**Gist.** This seed says which feeling the others (the people an action is done to or with) usually have
(ATOMIC-2020 `oReact`): *help, save, protect, hug* → gratitude; *leave, rob, hurt, kick* → sadness;
*hit, push, bite* → anger; *chase, grab, scare, frighten* → fear. The seed is generated from the data.

**Conditions and exceptions.** It applies when the character asked about is the verb's object (the one
acted on). Otherwise use the actor's feeling (`atomic-feelings`). The selection rule is the same as for
the actor (at least twice the baseline share).

**Examples.** None beyond the verb–feeling pairs in the gist.

**Machine-readable part.** Categories `atomic_o_anger_event`, `atomic_o_fear_event`,
`atomic_o_gratitude_event`, `atomic_o_joy_event`, `atomic_o_sadness_event`, `atomic_o_shame_event` and
`atomic_o_surprise_event`, plus one link from each to the matching feeling.

**Sources.** ATOMIC-2020 (Hwang et al. 2021, CC BY 4.0).

### atomic-uses
`ai/global/seeds/shortcuts/atomic-uses.md`

**Gist.** This seed holds commonsense facts about things and creatures from the corpus.
- `use_<action>` lists things usually used for that action (ATOMIC ObjectUse: *knife, axe, saw* →
  cut; *apple, leaf* → fall; *sword, rifle* → kill).
- `can_<action>` lists who can do it (CapableOf: *bird* → fly; *cat* → eat).

Query them with `global::uses(word)` and `global::capable(word)`. The seed is generated from the data.

**Conditions and exceptions.** The action is the first word of the ATOMIC answer, which is a rough
cut (*butter bread* → butter). An action needs a frequency of at least 2, and each thing has at most six
actions. Entries are a reference (`category`), not events, so they start no consequence chains.

**Examples.** `uses("knife")` includes *cut*, *chop*, *slice*, *stab*. `capable("bird")` includes *fly*
and *travel*.

**Machine-readable part.** Categories `can_drink` … `can_walk` (18 abilities) and `use_add` …
`use_write` (about 200 uses), each listing nouns.

**Sources.** ATOMIC-2020 (Hwang et al. 2021, CC BY 4.0).

### body-physics
`ai/global/seeds/shortcuts/body-physics.md`

**Gist.** An object without support falls and accelerates. Falling from a height, it gains speed. A blow
to the body hurts, and pain is harm.

**Conditions and exceptions.** Something light and small (a feather) does almost no harm. A soft landing
does not hurt.

**Examples.** Newton sits under a tree, an apple falls on his head, and it hurts: an apple accelerating
onto your head is not pleasant.

**Machine-readable part.** Concepts `fall`, `accelerate`, `impact`, `body`, `pain`, `harm`. Links: fall
→ accelerate (g ≈ 9.8 m/s²); accelerate → impact (impact speed v = √(2gh)); impact → pain; pain → harm
(the value "do no harm").

**Sources.** Newton, *Principia* (1687); basic kinematics.

### feelings
`ai/global/seeds/shortcuts/feelings.md`

**Gist.** A feeling is a reaction to an event:
- loss, death or parting → sadness;
- receiving, being rescued, winning, or meeting family → joy;
- attack, threat, darkness or the unknown → fear;
- deceit, theft, insult or injustice → anger;
- the unexpected → surprise;
- being helped → gratitude;
- being disgraced → shame.

**Conditions and exceptions.** A villain may rejoice at someone else's misfortune: the feeling depends
on whose misfortune and whose relief it is. The current version does not yet tell these apart.

**Examples.** The crow is ashamed when the fox takes the cheese (shame). The mouse is grateful to the
lion for letting it go (gratitude). The king grieves when the queen dies (sadness).

**Machine-readable part.**
- Categories of feeling adjectives: `joy`, `sadness`, `fear`, `anger`, `surprise`, `gratitude`, `shame`
  (for example *grateful, thankful* for gratitude).
- Event concepts with verb triggers: `loss_event`, `gain_event`, `threat_event`, `wrong_event`,
  `help_event`.
- Links: loss_event → sadness, gain_event → joy, threat_event → fear, wrong_event → anger,
  help_event → gratitude.

**Sources.** General knowledge; basic emotions (Ekman 1992). Gratitude and shame are treated as social
emotions.

### motivations
`ai/global/seeds/shortcuts/motivations.md`

**Gist.** A person's state suggests where they will go: someone hungry or thirsty goes to the kitchen,
someone tired to the bedroom, someone bored to the garden. This also answers "why": he went to the
kitchen because he was hungry.

**Conditions and exceptions.** This assumes a house with typical rooms. In another world (a forest, a
city) the goal differs. The model learned this knowledge from the bAbI training split.

**Examples.** *Jason is thirsty. Where will Jason go?* → kitchen.

**Machine-readable part.** Relations: `hungry: go=kitchen`, `thirsty: go=kitchen`,
`tired: go=bedroom`, `bored: go=garden`.

**Sources.** Weston et al. 2015, bAbI task 20 (CC BY 3.0).

### possession
`ai/global/seeds/shortcuts/possession.md`

**Gist.** A verb says how the owner's quantity changes (verb categorisation as in ARIS and Roy & Roth:
HAVE, GET, GIVE, CONSTRUCT, DESTROY).

**Conditions and exceptions.** *give* decreases the giver's quantity and increases the receiver's. The
question decides which of the two is asked about.

**Examples.** *She eats three* → she has three fewer. *He buys 5 more* → five more.

**Machine-readable part.**
- Verb classes:
  - `have =` (have, own, contain, hold, keep, cost, need, …);
  - `get +` (get, grab, buy, receive, find, collect, earn, win, …);
  - `give ±` (give, pass, sell, lend, pay, send, share, …);
  - `make +` (make, bake, build, plant, cook, write, …);
  - `lose -` (lose, eat, spend, use, break, drink, throw, …);
  - `move =` (move, come, run, walk, drive, read, work, play, …).
- Concepts `increase` and `decrease`.
- Links: get → increase, give → decrease (the giver), give → increase (the receiver), make → increase,
  lose → decrease.

**Sources.** Hosseini et al. 2014 (EMNLP, ARIS); Roy & Roth 2018 (TACL).

### relations
`ai/global/seeds/shortcuts/relations.md`

**Gist.** Relations between things have properties that do not need to be learned from each story:
- inverses ("A is north of B" means "B is south of A");
- direction on a plane (north is up, left is left);
- transitivity ("bigger than" over a chain);
- the order of parts of the day.

A story supplies only facts. The conclusions come from here.

**Conditions and exceptions.** "Fits inside" means "smaller than". "Above/below" in figures is the same
vertical axis as north/south on a map, under a different name. Time order applies within a single
narrative: *yesterday* comes before *this morning*.

**Examples.** *The kitchen is north of the garden* → from the garden to the kitchen is a step north (n).
*The box fits inside the chest; the chest is smaller than the suitcase* → the box is smaller than the
suitcase.

**Machine-readable part.**
- Compass: `north_of` (inverse south_of, vector (0,1), step n), `south_of` (inverse north_of, (0,−1),
  s), `east_of` (inverse west_of, (1,0), e), `west_of` (inverse east_of, (−1,0), w).
- Vertical and horizontal: `above`/`below` ((0,1)/(0,−1)), `left_of`/`right_of` ((−1,0)/(1,0)), each
  the other's inverse.
- Size: `bigger_than` and `smaller_than`, inverse of each other and transitive. `fit_inside` and
  `fit_in` are aliases (`same`) of `smaller_than`.
- Time order: `yesterday` 0, `morning` 1, `afternoon` 2, `evening` 3.

**Sources.** Weston et al. 2015, bAbI (arXiv:1502.05698), tasks 4, 14, 17, 18, 19; Allen's interval
algebra (1983) for time.

### scene
`ai/global/seeds/shortcuts/scene.md`

**Gist.** A thing in a picture is described by size, colour and shape (*small blue circle*). A partial
description (*the blue square*, *the medium thing*) refers to an already-mentioned thing that is
compatible with it. Nearness (*near, far*) and contact (*touching*) are symmetric: "A is near B" also
means "B is near A". Unlike directions, they are not transitive.

**Conditions and exceptions.** *thing, object, one, shape* refer to a thing of any shape. *top, over* mean
*above*, and *bottom, under, beneath* mean *below*.

**Examples.** *Near and below the large yellow square is a large black square* → the black square is
near the yellow one and below it.

**Machine-readable part.** Categories `scene_color`, `scene_size`, `scene_shape`, `scene_any` (words for
"any thing"). Relations `near`, `far`, `touching`, each `symmetric=yes`.

**Sources.** Mirzaee et al. 2021, SpartQA (NAACL); Suhr et al. 2019, NLVR.

### states-2
`ai/global/seeds/shortcuts/states-2.md`

**Gist.** This seed adds states, actions and concepts found in the worlds of Aesop's fables ("The Fox
and the Crow", "The Lion and the Mouse", "The Hare and the Tortoise"), filled in from general human
knowledge. Being angry is neutral (waking someone up is not harm).

**Conditions and exceptions.** Pride can be healthy or excessive, so on its own it is neither trouble nor
relief. Sleep is a normal state. Links cannot express negation, so a link "calm → perception" is not
included.

**Machine-readable part.** Concepts `neutral_state` (proud, angry, asleep, awake), `restrain` (bind,
tie, trap, cage, capture), `liberate` (release, free, untie), `rest` (sleep, doze, lie, perch, nap),
`vocalize` (caw, roar, laugh, sing), `perceive` (see, recognize, notice). Links: restrain → harm_state
(bound, captive, cannot move); liberate → relief_state (free, relieved).

**Sources.** General knowledge.

### states
`ai/global/seeds/shortcuts/states.md`

**Gist.** Some states of a character are trouble (captive, frightened, wounded, hungry) and others are
relief (free, safe, fed, grateful). Whoever caused trouble to another has harmed them, and whoever
removed it has helped.

**Conditions and exceptions.** Trouble caused by nature or accident carries no blame. Punishment for a
wrong after a fair trial is not evil (outside fables).

**Examples.** The lion catches the mouse: the mouse is captive and frightened (trouble caused by the
lion). The lion lets it go: the mouse is free (relief from the lion). The mouse gnaws through the net:
the lion is free (relief from the mouse).

**Machine-readable part.** Concepts `harm_state` (captive, afraid, hurt, injured, wounded, dead, pain,
hungry, trapped, sad, ashamed, tired, gnawed), `relief_state` (free, safe, fed, grateful, relieved,
happy, saved, amused, winner), `help`, `theft` (steal, rob, snatch, pilfer). Links: harm_state → harm;
relief_state → help (removing trouble is helping); theft → harm (taking someone's property without
consent, FrameNet: Theft).

**Sources.** Aesop's fables; the ARIS idea of world state as quantities in containers, carried over to
character states; FrameNet.

### things
`ai/global/seeds/shortcuts/things.md`

**Gist.** These are general noun categories that come up in tasks: time units, measures, part and whole,
countable things. The category says how to handle the numbers:
- quantities of the same category are added and subtracted;
- a part and its whole are multiplied or divided by a rate (*pages per chapter*);
- time is converted by constants (a week = 7 days).

**Conditions and exceptions.** *cup* is both a vessel and a measure. In recipes it is a measure. Some
verbs (*put, learn, join, sit, fit*) are concepts here, not verb classes: they only explain steps and do
not change how quantities are computed.

**Examples.** *6 cups of flour* → a measure. *4 chapters, 20 pages each* → part and whole with a rate.
*3 weeks* → 21 days.

**Machine-readable part.**
- Categories: `time_unit`, `measure_unit`, `whole_part`, `countable`.
- Concepts: `convert`, `put_in`, `learn_gain`, `join_group`, `activity`, `fit_into`.
- Links:
  - time_unit → convert (fixed constants);
  - whole_part → rate;
  - measure_unit → total, countable → total (same category: add and subtract);
  - convert → multiply (to a smaller unit: multiply by a constant);
  - put_in → increase, learn_gain → increase, join_group → increase.

**Sources.** General knowledge; questions from the SVAMP benchmark.

### units
`ai/global/seeds/shortcuts/units.md`

**Gist.** "X per Y" is a rate: quantity = rate × number of units. Quantities in the same unit combine only
by addition and subtraction (unit dependency graph).

**Conditions and exceptions.** "How many more / fewer" is a comparison and means a difference
(subtraction). A rate is found from a total and a count by division. Sharing equally among several
people is division.

**Examples.** *$2 per egg*; *60 miles an hour*; *3 books each*.

**Machine-readable part.**
- Concepts: `rate` (per, each, every), `total` (total, altogether, together, combined, sum), `multiply`,
  `add`, `subtract`, `compare` (more, than, fewer, less, difference), `divide`, `share` (equally, among,
  split, distribute, share).
- Links: rate → multiply; total → add; decrease → subtract; increase → add; compare → subtract;
  rate → divide; share → divide.

**Sources.** Roy & Roth 2017 (AAAI), "Unit Dependency Graph"; Mitra & Baral 2016 (comparison
questions).

### diligence
`ai/global/seeds/values/diligence.md`

**Gist.** Do things well, not carelessly: check before you hand something over.

**Conditions and exceptions.** Diligence is not slowness for its own sake. Fast and correct is diligent
too.

**Examples.** "The Ant and the Grasshopper" (Aesop): the ant stores food all summer while the
grasshopper sings, and in winter the grasshopper goes hungry. For the model itself, an answer that
passed an independent check by the solver core shows quality and diligence.

**Machine-readable part.** Principle `diligence`, name "quality and diligence", predicate `self_quality`
(applied to the model's own work), no `against`/`towards` words.

**Sources.** Aesop's fables; Colossians 3:23.

### do-no-harm
`ai/global/seeds/values/do-no-harm.md`

**Gist.** Human life, health and dignity are the highest value. Do not cause harm or pain.

**Conditions and exceptions.** Defending yourself or another from attack is not a violation. Treatment
that hurts briefly (a vaccination) is not harm. An accident that nobody willed is harm, but not guilt.

**Examples.** "The Wolf and the Lamb" (Aesop): the wolf looks for an excuse to eat the lamb, and the
strong harm the weak. Newton under the tree: the apple falls on his head and it hurts, but nobody is to
blame (harm without guilt).

**Machine-readable part.** Principle `do-no-harm`, predicate `cause_harm_state`. Against: *hit, hurt,
kill, beat, bite, harm, attack, injure, wound, eat up, devour*. Towards: *heal, protect, save, rescue*.

**Sources.** Aesop's fables; "You shall not murder" (Exodus 20:13); Universal Declaration of Human
Rights, art. 3.

### forgiveness
`ai/global/seeds/values/forgiveness.md`

**Gist.** Peace and harmony: forgive, reconcile, do not take revenge.

**Conditions and exceptions.** Forgiveness does not cancel justice: what was stolen should be returned.

**Examples.** The Prodigal Son (Luke 15:11–32): the father forgives the son who returns. The two goats on
the bridge (folk fable): both refuse to give way and both fall, so yielding is wiser.

**Machine-readable part.** Principle `forgiveness`, predicate `spare_enemy`. Against: *revenge,
quarrel, fight*. Towards: *forgive, reconcile, apologize*.

**Sources.** Matthew 5:9; Luke 15:11–32.

### friendship
`ai/global/seeds/values/friendship.md`

**Gist.** Friendship and loyalty: do not betray, give thanks, keep your word.

**Conditions and exceptions.** Loyalty to someone who does evil is not a virtue.

**Examples.** "The Two Friends and the Bear" (Aesop): one friend climbs a tree and abandons the other, so
a friend is known in need. "The Lion and the Mouse": gratitude and mutual help make friends of
unequals.

**Machine-readable part.** Principle `friendship`, predicate `reciprocate`. Against: *betray, desert*.
Towards: *befriend, thank*.

**Sources.** Aesop's fables; John 15:13.

### help-neighbour
`ai/global/seeds/values/help-neighbour.md`

**Gist.** Mercy: help someone in trouble, even when they are weaker and cannot repay you right away.

**Conditions and exceptions.** Help that harms others is not mercy.

**Examples.** "The Lion and the Mouse" (Aesop): the lion lets the mouse go, and later the mouse gnaws
through the net and saves the lion, so the small can help the great. The Good Samaritan (Luke
10:30–37): he helped a stranger when others walked past.

**Machine-readable part.** Principle `help-neighbour`, predicate `relieve_harm_state`. Against:
*abandon, ignore, refuse*. Towards: *help, share, give, feed, care, free, release, spare*.

**Sources.** Aesop's fables; Luke 10:25–37.

### humility
`ai/global/seeds/values/humility.md`

**Gist.** The proud and boastful fall. The humble do not look down on the weak.

**Conditions and exceptions.** Healthy dignity is not pride. The judgment is made on the world state:
scorn for a rival, the same goal for both, the rival reaches it, and the proud one ends up in trouble.

**Examples.** "The Hare and the Tortoise" (Aesop): the hare mocks the tortoise's slowness, sleeps in the
middle of the race and loses. "The Lion and the Mouse": the lion laughs at the idea that a mouse could
ever help a lion, and then the mouse saves him.

**Machine-readable part.** Principle `humility`, predicate `scorn_then_fall`. Against: *boast, scorn,
mock, sneer*. No `towards` words.

**Sources.** Aesop's fables; Proverbs 16:18 ("pride goes before destruction").

### justice
`ai/global/seeds/values/justice.md`

**Gist.** Do not take what belongs to others. Give what is due. Greed is ruinous.

**Conditions and exceptions.** Taking what is yours is not theft.

**Examples.** "The Fox and the Crow": the fox takes someone else's cheese by cunning. "The Dog and Its
Reflection" (Aesop): the dog also wants the piece it sees in the water and loses its own, so greed is
ruinous.

**Machine-readable part.** Principle `justice`, predicate `take_without_consent`. Against: *steal, rob,
grab, snatch*. Towards: *return, repay*.

**Sources.** Aesop's fables; "You shall not steal" (Exodus 20:15).

### perseverance
`ai/global/seeds/values/perseverance.md`

**Gist.** The slow but steady reach the goal sooner than the careless.

**Conditions and exceptions.** Resting after reaching the goal is not a violation. The judgment is made
on the world state: between setting the goal and reaching it there are no "rest" actions (sleeping,
dozing, lying down), while a rival with the same goal had such actions and did not reach it.

**Examples.** "The Hare and the Tortoise" (Aesop): the tortoise goes "slowly but steadily", the hare
sleeps, and the tortoise wins.

**Machine-readable part.** Principle `perseverance`, predicate `steady_effort`. Against: *quit*.
Towards: *persist*.

**Sources.** Aesop's fables; Galatians 6:9.

### truth
`ai/global/seeds/values/truth.md`

**Gist.** Lies and flattery destroy trust. Truth is the basis of friendship and of life together.

**Conditions and exceptions.** Flattery for gain is against truth, and sincere praise is not. A joke that
everyone understands as a joke is not a lie.

**Examples.** "The Fox and the Crow" (Aesop): the fox flatters the crow so that she opens her beak and
drops the cheese (flattery for gain). "The Boy Who Cried Wolf" (Aesop): after several lies, nobody
believed him when the wolf really came. Lies destroy trust.

**Machine-readable part.** Principle `truth`, predicate `insincere_speech`. Against: *lie, flatter,
deceive, trick, cheat, fool, pretend*. Towards: *admit, confess*.

**Sources.** Aesop's fables (public domain); "You shall not bear false witness" (Exodus 20:16).

### core
`ai/global/seeds/wikidata/core.md`

**Gist.** This seed gives categories for nouns that actually occur in the model's stories (public-domain
Project Gutenberg books) and tasks (SVAMP, GSM8K). The categories are animal, plant, food, fruit,
vegetable, tool, weapon, vehicle, building, clothing, furniture, body part, human, occupation, container,
musical instrument, toy, measure unit, currency, liquid, metal, gemstone, colour, mythical creature and
celestial body. The seed is generated from Wikidata.

**Conditions and exceptions.** A word's meaning is taken as the best-known Wikidata entity with that
English label (present in at least 5 Wikipedias), without context. So homonyms can be wrong (*boxer* →
a dog breed, *godfather* → a cocktail). Priority gates against Wikidata noise:
- a human is not an "animal";
- a fruit is not a "body part";
- a specific item is not a generic "tool";
- an animal or plant is not a "colour" or "liquid".

Categories explain steps but start no consequence chains.

**Machine-readable part.** 25 categories `wd_animal`, `wd_body_part`, `wd_building`,
`wd_celestial_body`, `wd_clothing`, `wd_color`, `wd_container`, `wd_currency`, `wd_food`, `wd_fruit`,
`wd_furniture`, `wd_gemstone`, `wd_human`, `wd_liquid`, `wd_measure_unit`, `wd_metal`,
`wd_musical_instrument`, `wd_mythical_creature`, `wd_occupation`, `wd_plant`, `wd_tool`, `wd_toy`,
`wd_vegetable`, `wd_vehicle`, `wd_weapon`. Links → `total` ("items of one category are added and
subtracted") for food, fruit, vegetable, animal, toy, clothing, tool, container, plant, human,
occupation, vehicle, furniture and currency.

**Sources.** Wikidata (CC0), snapshot of 2026-09-15.
