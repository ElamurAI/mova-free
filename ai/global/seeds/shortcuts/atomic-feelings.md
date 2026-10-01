# Feelings of the actor after events — from ATOMIC-2020 (generated, 2026-10-01)

**Gist.** What the one who acts usually feels after an event (`xReact` reactions in ATOMIC-2020): hit, beat, fight → anger; lose, miss, cry, bury → sadness; break, drop, steal, forget → shame; hide, run → fear; receive, thank, marry, hug → gratitude; find, see, meet → surprise.

**Conditions and exceptions.** The feeling of the actor, not of the one acted upon (`oReact` — later); the verb without context. Selection: lift of the category share over the baseline ≥ 2 (ATOMIC is positive overall, so joy is not selected this way — it comes from the seed `feelings.md`), ≥ 6 feeling reactions, function verbs excluded.

**Sources.** ATOMIC-2020 (Hwang et al. 2021, CC BY 4.0; `data/raw/csdata-atomic2020`); rebuild: `duckdb < global/data/atomic-feelings.sql`, then this file from `data/runs/atomic/feel-verbs.tsv`.

```category
atomic_anger_event: throw hit push wait beat fight search draw kick yell slam tear knock lock bite remove stick chase punch complain slap shut ruin ignore snap dump rip stare storm curse refuse shove threaten
atomic_fear_event: run hide rush study accept avoid board rub jump propose fear deny toss scar worry approach
atomic_gratitude_event: love ask spend visit receive pull thank return hold live sit marry adopt kiss treat borrow mean hug suit appreciate
atomic_sadness_event: lose miss crie grow end wipe bury crash shake fail gain fire cancel realize die suffer wreck apologize shed burst divorce
atomic_shame_event: eat leave break drop cut kill forget grab fall spill hurt steal blow scream land cover beg bear smell burn slip exceed stretch poke bump
atomic_surprise_event: find see meet hear watch open catch notice believe
```

```link
atomic_anger_event -> anger : ATOMIC-2020 xReact — after such an event the actor usually feels "anger"
atomic_fear_event -> fear : ATOMIC-2020 xReact — after such an event the actor usually feels "fear"
atomic_gratitude_event -> gratitude : ATOMIC-2020 xReact — after such an event the actor usually feels "gratitude"
atomic_sadness_event -> sadness : ATOMIC-2020 xReact — after such an event the actor usually feels "sadness"
atomic_shame_event -> shame : ATOMIC-2020 xReact — after such an event the actor usually feels "shame"
atomic_surprise_event -> surprise : ATOMIC-2020 xReact — after such an event the actor usually feels "surprise"
```
