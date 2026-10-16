# How the schools are made

**Gist.** The small model (MMM) learns by living a child's school years in a simulation: courses by subject, lessons
in real speech, tests in casual talk, exams in stiff official language. The big model writes the lessons; it never
answers at test time — every score is MMM's own.

**The simulation.** The pupil is a child of 9–10 in an English-speaking town (with Ukrainian and Chinese street friends),
who plays consoles and Europa Universalis, watches cartoons and TV, goes to church and Sunday school, gets into fights,
does little sins and gets caught or feels guilty. Near the end of the AI course the pupil is told the truth: it is
Mova, a small transparent language model, and these schools are an experiment in learning language, logic and real
life. Nothing about the people, plans, prices or partners behind it ever goes into a public lesson.

**The pipeline (per course).**
1. A register brief (`<course>-register-brief.md` in the work space): domains, columns, what must be true, limits.
2. One agent writes the register (300–1,000 rows, tsv) and the plan (lesson, title, domain, 9–12 rows joined by « ; »);
   big registers are cut into slices of 20 lessons, each slice its own agent.
3. Lesson writers, 10 lessons each, all on one shared brief (`course-lessons-brief.md`) plus the voice briefs
   (`casual-talk.md`, `exam-voice.md`, `harsh-brief.md`, `gossip-brief.md`). A lesson: 4–5 explain, 11–14 facts,
   15–18 exercises, 10–12 tests, 4 exams; right answer first, three wrong ones, myths as the wrong ones; stars 4/3/2/1
   per block of ten.
4. `world knowledge check <course>` (no ✗), then `world knowledge course <course>` for the formal-exam score.
5. A full run of the maths school (lessons, data, book course, cv): the gate (2443) and the cv (1116) must not fall; then commit.
6. After each wave: the dictionary cycle — every word of the courses gets a rich record; the skeleton ops stay frozen.
Up to 20 writers at once; the token budget is limited, so the work goes in waves, and the queue is kept in the worklog so a
spend-limit cutoff loses nothing but the lessons in flight.

**The voice.** Real speech, not a school book: clipped, wrong grammar, slips and self-corrections (the last number
counts), wrong explanations to put right, gossip in vague words, euphemisms, rough everyday swearing without slurs.
Half the conversational lines the everyday mix (light general slang, sometimes dry, sometimes formal), half hard slang and dialect of the speaker's generation and place — US generations, and the
words of every English-speaking country, Britain village by village. Hard consequences are real: dogs bite, children
who swim out too far drown, sins get found out — or don't. The exam is the opposite: so formal the pupil panics.

**Limits.** No sexual content, no slurs, no how-to for anything dangerous (only «never» and what to do); real people
only as public record, «it was reported»; contested points named as contested with each side in its own words;
facts true or marked as reported or argued; legal open sources only.

**Why.** Saturation: few new words, hundreds and thousands of examples on the popular ones, in every register a
child meets. Worse scores and confusion are accepted — more lessons and courses come later.
