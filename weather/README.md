# weather — the Mova weather domain

A domain version on top of the graph compressor [`../en`](../en): weather forecasts and queries, **two language variants** (design note: weather needs two models, standard English with Celsius and US English with Fahrenheit):

| variant | units | style | source (open) |
|---|---|---|---|
| US | °F, mph | NWS: «Mostly sunny, with a high near 72. South wind 5 to 10 mph.» | api.weather.gov, public domain |
| international English | °C, km/h | Environment Canada: «Mainly sunny. High 22. Wind south 20 km/h.» | MSC Datamart, ECCC Open Licence (commercial use allowed, with attribution) |

Everything is Rust structures:
- **Data.** `data.rs`: `Period` — a forecast period; `Temp{value, unit}`, `Wind{dir, lo, hi, gust, unit}`, `Dir` — 16 compass points; `Variant`.
- **Collectors.** `nws.rs`, `eccc.rs` — serde (JSON / quick-xml), polite, no retries.
- **Queries.** `query.rs`: query graph → `WeatherQuery { place, time, condition, unit }` — a ready "command" for the weather code.

## Two models (`weather eval`)

Each model is the general one (UD) plus generator fine-tuning on queries and forecasts of its own variant (self-training: nobody has annotated trees for this domain).

| model (exact / chrF) | queries (358) | US forecasts (1,895) | intl. forecasts (4,920) |
|---|---|---|---|
| general (UD) | 38.5% / 88.2 | 86.3% / 97.6 | 89.2% / 95.5 |
| **US** (°F, mph; 10,948 sentences) | 74.9% / 96.3 | **97.8% / 99.7** | 91.4% / 96.0 |
| **international** (°C, km/h; 22,721 sentences) | 74.0% / 95.9 | 84.1% / 97.7 | **97.9% / 99.6** |

**Changes on 2026-09-25:**
- the tagger is now a perceptron (`en/src/ptag.rs`);
- the order generator got a "heavy" dependent feature.

**With the TnT tagger it was:**
- US on its own forecasts — 96.2% / 99.5;
- international on its own — 95.8% / 99.1.

**The weather parser is greedy** (`GREEDY` in `src/main.rs`). With the beam, which is more accurate on UD, US on its own forecasts gives only 90.2%. Trees of this domain are uncertain without silver: «as high as 20 mph» attaches sometimes to gusts, sometimes to wind. The beam will come back after Opus silver on weather sentences.

The test uses other places: every fifth one alphabetically; the model never sees texts of test cities. US: 153 cities, two NWS snapshots from 2026-09-24 (8,449 training sentences, 1,895 test sentences from 30 cities). International: 844 ECCC stations (20,222 / 4,920 from 168 stations).
- **Own style.** Each model is best on its own style.
- **Other style.** The international model on US forecasts is worse even than the general one: the style does not transfer, so there really are two variants.
- **Queries.** Both models handle queries equally well.

## Input query parser (`weather slots`)

Training: MASSIVE en-US weather_query (CC BY 4.0) + SNIPS GetWeather (CC0), 2,499 queries. Test: 356 queries.

- **Slots.** BIO on graph nodes, a "graph feature → label weights" table (averaged perceptron). Features are numbers: lemma number, tag, relation, head, preposition attached to the word, neighbours, form.
- **Units.** A rule on word constants: `lemma == w::CELSIUS` / `w::FAHRENHEIT`. The unit word is not part of any slot.

| slot | P | R | F1 |
|---|---|---|---|
| place | 86.9% | 87.6% | 87.3% |
| time | 87.8% | 89.2% | 88.5% |
| condition | 90.5% | 87.7% | 89.1% |
| **all** | | | **88.1%**; exact query 80.3% |

```
«what is the temperature in celsius in Kyiv this weekend»
  → {"place":"Kyiv","time":"this weekend","condition":"temperature","unit":"C"}
«should i bring an umbrella tomorrow in boston»
  → {"place":"boston","time":"tomorrow","condition":"umbrella","unit":null}
```

## "Data → text" generator (`weather nlg`, `weather say`)

Input is a typed `Period`: condition `Cond { sky, wx, windy }` (shared by both sources: NWS and ECCC icon codes), temperature with unit, probability of precipitation, wind (direction, from–to, gusts), UV, humidex, period number. Output is text in the style of the variant.

- **How it learns.**
  - Each forecast sentence is aligned with the data: numbers become slots by context, wind direction becomes a slot, the remaining words become form numbers from the in-code dictionary of `en`. This turns the sentence into a template.
  - The template gets a component: weather phenomenon, sky, sky+temperature, temperature, wind, probability, UV.
  - Tables per variant:
    - whether the component is present;
    - which template — a key from data features, with backoff;
    - component order.
  - Training takes 0.1 s.
- **Honesty.**
  - Sentences with a number, direction, phenomenon or time that is not in the data do not become templates.
  - Sky and phenomenon keys never back off beyond the sky or phenomenon itself.
  - Temperature is mandatory.
  - **Fact gate:** every generated sentence is aligned with its own data again. Negative control: alignment with the data of a neighbouring period.
- **Period number matters.** NWS mentions wind in the first 5 periods (2.5 days) in 98–100% of texts, later in 21–35% (bureau policy). The model learns this from the data.

| | US (NWS, test — 30 other cities) | international (ECCC, test — 168 other stations) |
|---|---|---|
| exact text | **43.9%** | **72.6%** |
| chrF | 79.5 | 85.2 |
| baseline (most frequent template; key only by sky or phenomenon) | 18.2% / 74.7 | 27.0% / 53.8 |
| sentences by component | sky+temp. 90%, phenomenon 83%, wind 61%, probability 100% | temperature 98%, sky 90%, wind 95%, phenomenon 74%, UV 100% |
| coverage: reference sentences that fit the data | 66.9% | 87.8% |
| fact gate: sentences that do not fit their own data | **0 of 1,655** | **0 of 4,632** |
| negative control: the same on other data | 84% | 58% |

**Why coverage is not 100%.** Reference texts contain things that are not in the data:
- US — gusts and trends («falling to around 67»), precipitation amounts (349 sentences), time («after 1pm», 198);
- Canada — time («developing overnight», 543).

This can be recovered once gusts, hourly progression and precipitation amounts appear in the data.

**Same data — two styles.** Units are converted, speed is rounded to a multiple of 5:

```
Atlanta-GA «This Afternoon»
  us  : Partly sunny, with a high near 73. Northeast wind around 10 mph.
  intl: Mainly cloudy. High 23.
```

**Data limitations.** In a third of NWS periods the icon does not say what the sky is (only `rain_showers,20`). Then the generator takes the sky from similar periods. For the geography project's data, cloud cover must always be provided.

## Commands

```
weather fetch-nws <cities.csv> <out-dir> [pause ms]      NWS snapshot
weather fetch-eccc <out-dir> [pause ms]                  ECCC snapshot: the freshest file of each station
weather build <out-dir> <nws-dir>... -- <eccc-dir>...    periods-<variant>.jsonl + train/test sentences split by place
weather eval <train.conllu>... -- <data-dir> [weight]    general model vs the two variant models
weather slots <train.conllu>... -- <massive.jsonl> <snips.tsv>   input query parser
weather nlg <train.conllu>... -- <data-dir>              "data → text" generator: evaluation on other cities
weather say <train.conllu>... -- <data-dir> <us|intl> <periods.jsonl>   texts from data (units → variant)
```

Data lives in `$MOVA_DATA/en/weather/`: `nws/`, `eccc/` (with `SOURCES.md`), `queries/`, `sets/`.

## Next

- Ranges for the geography project's units («High 14 to 17» — weighted spread over the footprint): the NWS/ECCC corpora have none, a style decision is needed.
- Gusts, hourly progression, precipitation amounts — once they are in the data.
- An adapter for the geography project's data (WMO codes → `Cond`), once the source is known.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
