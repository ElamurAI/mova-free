//! Durable school state: graph + learned role weights, not replayed lessons.
//! Train and exam are separate processes. Caches and exam labels are not state.
//! This is explicit knowledge retention, not evidence of semantic generalization.
use super::*;
use anyhow::{Context, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};

/// Shared syntax, with independent training and examination completeness gates.
pub fn parse_material(text: &str, file: &str) -> Result<Lesson> {
    let l = super::parse_fields(text, file)?;
    ensure!(
        l.n > 0 && !l.course.is_empty(),
        "material needs course and lesson number"
    );
    Ok(l)
}
const SCHEMA: &str = "mova-school-retention-v1";
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// Identity of the scored assessment, including labels, option order and lesson
/// seed. File name, title and teaching text are not inputs to examination.
pub fn assessment_fingerprint(l: &Lesson) -> Result<String> {
    Ok(hash(&serde_json::to_vec(&(
        &l.course, l.n, &l.tests, &l.exams,
    ))?))
}

fn material_digest(l: &Lesson, n: usize) -> Result<String> {
    Ok(hash(&serde_json::to_vec(&(
        &l.course,
        n,
        &l.explain,
        &l.facts,
        &l.exercises,
    ))?))
}

#[derive(Serialize, Deserialize)]
struct Graph {
    frames: Vec<Frame>,
    same: BTreeMap<String, BTreeSet<String>>,
    texts: Vec<((usize, usize), String)>,
    weights: Vec<i64>,
    parts: Vec<((String, String), (i64, (usize, usize)))>,
    isa: BTreeMap<String, (String, (usize, usize))>,
    per: Vec<((String, String), (i64, String, (usize, usize)))>,
}
#[derive(Serialize, Deserialize)]
struct Payload {
    schema: String,
    course: String,
    // Hash of teaching lines and exercises, excluding all test/exam labels.
    lessons: BTreeMap<usize, String>,
    graph: Graph,
    /// The fingerprint of the dictionary store the state learned with (`crate::dictstore::Store::fingerprint`); empty
    /// in states saved before it was recorded. A state is examined only with the same dictionary.
    #[serde(default)]
    dictionary: String,
    /// The share-alike or non-commercial licence of the add-on modules the dictionary held
    /// (`crate::dictstore::Store::derived_licence`); empty (and not written) when the state is ours.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    licence: String,
}
#[derive(Serialize, Deserialize)]
struct Envelope {
    payload: Payload,
    sha256: String,
}

/// A retained course. Different courses currently need separate states because
/// legacy frame provenance identifies a lesson by number alone.
pub struct Retained {
    pub student: PragStudent,
    course: String,
    lessons: BTreeMap<usize, String>,
    dictionary: String,
    licence: String,
}
impl Retained {
    pub fn new(course: &str) -> Self {
        let mut student = PragStudent::default();
        student.read_kinds();
        student.lesson_focus = false;
        Self {
            student,
            course: course.into(),
            lessons: BTreeMap::new(),
            dictionary: crate::dictstore::store().fingerprint(),
            licence: crate::dictstore::store().derived_licence().unwrap_or_default(),
        }
    }
    fn payload(&self) -> Payload {
        let s = &self.student;
        Payload {
            schema: SCHEMA.into(),
            course: self.course.clone(),
            lessons: self.lessons.clone(),
            dictionary: self.dictionary.clone(),
            licence: self.licence.clone(),
            graph: Graph {
                frames: s.frames.clone(),
                same: s.same.clone(),
                texts: s.texts.iter().map(|(k, v)| (*k, v.clone())).collect(),
                weights: s.w.clone(),
                parts: s.parts.iter().map(|(k, v)| (k.clone(), *v)).collect(),
                isa: s.isa.clone(),
                per: s.per.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            },
        }
    }
    /// What the lessons taught about words, as a dictionary layer of state `derived` (`crate::dictstore`): each kind a
    /// lesson stated («a woodlouse is a crustacean» → woodlouse, crustacean) and each pair of names it gave one thing
    /// («Bill, the lizard»), with the course, lesson and line, read from the saved graph alone (no lesson is read).
    pub fn meanings_layer(&self) -> Result<String> {
        let s = &self.student;
        let fp = self.fingerprint()?;
        let mut out = String::from("# word\tpart of speech\tmeaning for a pupil\tsense in the lessons\texamples\tnote\n");
        match crate::dictstore::store().derived_header() {
            // learned with a share-alike or non-commercial module: the layer carries its licence (and never ships)
            Some(h) => out.push_str(&format!("# derived: what the course «{}» taught about words, from the retained state {fp}; not reviewed\n{h}\n", self.course)),
            None => out.push_str(&format!("# derived: what the course «{}» taught about words, from the retained state {fp}; not reviewed; ours, Elamur Cosmic License\n", self.course)),
        }
        let line = |src: &(usize, usize)| s.texts.get(src).cloned().unwrap_or_default();
        let mut rows: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (w, (k, src)) in &s.isa {
            if src.0 == KINDS_SRC {
                continue;
            }
            rows.entry(w.clone()).or_default().push(format!("{w}\tnoun\t\t«{w}» is a kind of «{k}» (lesson {}, line {})\t{}\tderived from the state {fp}", src.0, src.1, line(src)));
        }
        for (a, bs) in &s.same {
            for b in bs {
                rows.entry(a.clone()).or_default().push(format!("{a}\tnoun\t\t«{a}» names the same thing as «{b}» in the lessons\t\tderived from the state {fp}"));
            }
        }
        for r in rows.values().flatten() {
            out.push_str(r);
            out.push('\n');
        }
        Ok(out)
    }

    pub fn fingerprint(&self) -> Result<String> {
        Ok(hash(&serde_json::to_vec(&self.payload())?))
    }

    /// Exclusive new artifact, synced before return. Never overwrites a parent.
    pub fn save(&self, path: &Path) -> Result<()> {
        crate::dictstore::store().guard_derived_write(path)?;
        let payload = self.payload();
        let sha256 = hash(&serde_json::to_vec(&payload)?);
        let bytes = serde_json::to_vec(&Envelope { payload, sha256 })?;
        let mut f = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)?;
        f.write_all(&bytes)?;
        f.sync_all()?;
        Ok(())
    }
    pub fn load(path: &Path) -> Result<Self> {
        let e: Envelope = serde_json::from_slice(&fs::read(path)?)?;
        ensure!(e.payload.schema == SCHEMA, "unsupported retained schema");
        ensure!(
            hash(&serde_json::to_vec(&e.payload)?) == e.sha256,
            "retained state hash mismatch"
        );
        let p = e.payload;
        let now = crate::dictstore::store().fingerprint();
        ensure!(
            p.dictionary.is_empty() || p.dictionary == now,
            "the state learned with another dictionary (fingerprint {}); this process loads {now} (MOVA_DICT)",
            p.dictionary
        );
        let g = p.graph;
        ensure!(
            g.weights.len() == start_weights().len(),
            "role-weight layout mismatch"
        );
        let texts: BTreeMap<_, _> = g.texts.iter().cloned().collect();
        ensure!(texts.len() == g.texts.len(), "duplicate source line");
        ensure!(
            texts.keys().all(|(n, _)| p.lessons.contains_key(n)),
            "source without lesson ledger"
        );
        for f in &g.frames {
            ensure!(
                texts.contains_key(&f.src),
                "frame without source provenance"
            );
            for j in f.quote.iter().copied().chain(f.cause).chain(f.next) {
                ensure!(j < g.frames.len(), "frame link outside graph");
            }
            // Frame::link is sentence-local metadata, not a global graph edge.
        }
        let mut s = PragStudent {
            lesson_focus: false,
            w: g.weights,
            same: g.same,
            texts,
            isa: g.isa,
            ..Default::default()
        };
        let nparts = g.parts.len();
        let nper = g.per.len();
        s.parts = g.parts.into_iter().collect();
        s.per = g.per.into_iter().collect();
        ensure!(
            s.parts.len() == nparts && s.per.len() == nper,
            "duplicate kind relation"
        );
        let source_ok = |src: &(usize, usize)| src.0 == KINDS_SRC || s.texts.contains_key(src);
        ensure!(
            s.parts.values().all(|(_, src)| source_ok(src))
                && s.isa.values().all(|(_, src)| source_ok(src))
                && s.per.values().all(|(_, _, src)| source_ok(src)),
            "kind relation without provenance"
        );
        // Rebuild lookup indexes and numeric codes directly from the learned
        // graph. No parser, lesson reading or exercise update occurs here.
        for f in g.frames {
            let i = s.frames.len();
            for (_, ent) in f.ents() {
                for id in std::iter::once(&ent.id).chain(&ent.and) {
                    s.by_ent.entry(id.clone()).or_default().push(i);
                }
            }
            if !f.verb.is_empty() && !LIGHT.contains(&f.verb.as_str()) {
                s.by_verb.entry(f.verb.clone()).or_default().push(i);
            }
            if let Some(n) = f.next {
                s.prev.entry(n).or_default().push(i);
            }
            if let Some(n) = f.cause {
                s.effect.entry(n).or_default().push(i);
            }
            for &q in &f.quote {
                s.sayer.insert(q, i);
            }
            s.by_line.entry(f.src).or_default().push(i);
            let code = s.mcode(&f);
            s.codes.push(code);
            s.frames.push(f);
        }
        Ok(Self {
            student: s,
            course: p.course,
            lessons: p.lessons,
            dictionary: if p.dictionary.is_empty() { now } else { p.dictionary },
            licence: p.licence,
        })
    }

    /// Find already retained teaching and exercise content, independent of the
    /// staging placeholder number or changes to assessment questions.
    pub fn retained_lesson(&self, l: &Lesson) -> Result<Option<usize>> {
        ensure!(
            l.course == self.course,
            "different course needs a separate retained state"
        );
        for (&n, digest) in &self.lessons {
            if *digest == material_digest(l, n)? {
                return Ok(Some(n));
            }
        }
        Ok(None)
    }
    pub fn next_lesson_number(&self) -> Result<usize> {
        let n = self
            .lessons
            .keys()
            .next_back()
            .copied()
            .unwrap_or(0)
            .checked_add(1)
            .context("lesson number exhausted")?;
        ensure!(n != KINDS_SRC, "lesson number reserved for body plans");
        Ok(n)
    }
    /// New lessons are ingested once; deliberate practice uses exercises only.
    /// Changed teaching material needs a rebuilt state, not duplicate provenance.
    pub fn train(&mut self, l: &Lesson, practice: bool) -> Result<serde_json::Value> {
        ensure!(
            l.tests.is_empty() && l.exams.is_empty(),
            "training input must not include test/exam labels"
        );
        ensure!(
            !l.explain.is_empty() || !l.facts.is_empty(),
            "training input needs teaching evidence"
        );
        ensure!(
            l.course == self.course,
            "different course needs a separate retained state"
        );
        let digest = material_digest(l, l.n)?;
        if let Some(old) = self.lessons.get(&l.n) {
            ensure!(
                *old == digest,
                "lesson teaching version changed; rebuild state"
            );
            ensure!(practice, "lesson already retained; use practice explicitly");
        } else {
            ensure!(
                !practice,
                "cannot practice a lesson that was never retained"
            );
        }
        let before_frames = self.student.frames.len();
        let before_weights = self.student.w.clone();
        let mut teaching = l.clone();
        teaching.tests.clear();
        teaching.exams.clear();
        self.student
            .preparse(&course_texts(std::slice::from_ref(&teaching)))?;
        if !practice {
            self.student.read(&teaching);
            self.lessons.insert(l.n, digest);
        }
        // Persistent learning cannot rely on the currently taught lesson being
        // privileged later at exam time. Use the same neutral scoring here.
        self.student.lesson_focus = false;
        let work = self.student.learn_lesson(&teaching, 2);
        Ok(
            serde_json::json!({"new_frames":self.student.frames.len()-before_frames,
            "changed_weights":self.student.w.iter().zip(before_weights).filter(|(a,b)|**a!=*b).count(),
            "exercise_evaluations":work.evaluated,"practice":practice,"lessons":self.lessons.len()}),
        )
    }

    /// Closed-book evaluation: rejects any supplied teaching or practice text.
    /// Gold labels are used only after answer() returns, never by the learner.
    pub fn exam(&self, l: &Lesson) -> Result<serde_json::Value> {
        ensure!(
            l.course == self.course,
            "exam course differs from retained state"
        );
        ensure!(
            l.explain.is_empty() && l.facts.is_empty() && l.exercises.is_empty(),
            "exam input must contain only test/exam questions, without teaching text"
        );
        ensure!(!l.tests.is_empty() || !l.exams.is_empty(), "empty exam");
        let before = self.fingerprint()?;
        let t = std::time::Instant::now();
        let mut right = 0;
        let mut rows = Vec::new();
        // Parse questions/options only; no training-source file is accessed.
        self.student
            .preparse(&course_texts(std::slice::from_ref(l)))?;
        for (kind, qs, extra) in [("test", &l.tests, 0), ("exam", &l.exams, 0xE8A3)] {
            for (i, (q, r, w)) in qs.iter().enumerate() {
                let (opts, at) = shuffled(((l.n as u64) << 20 | (i as u64) << 1 | 1) ^ extra, r, w);
                let a = self.student.answer(q, &opts);
                let correct = a.choice == at;
                right += usize::from(correct);
                let diagnosis = if correct {
                    "correct"
                } else if a.count.is_some() {
                    "count_rule_error"
                } else if a.options[at].best.is_none() {
                    "missing_correct_support"
                } else if !a.understood {
                    "question_not_understood"
                } else {
                    "supported_selection_error"
                };
                rows.push(serde_json::json!({"kind":kind,"index":i,"question":q,
                    "choice":opts[a.choice],"right":r,"correct":correct,"understood":a.understood,
                    "diagnosis":diagnosis,"why":self.student.why(&a,at)}));
            }
        }
        ensure!(before == self.fingerprint()?, "exam mutated learned state");
        Ok(
            serde_json::json!({"right":right,"total":rows.len(),"rows":rows,
            "state_sha256":before,"assessment_sha256":assessment_fingerprint(l)?,"seconds":t.elapsed().as_secs_f64(),
            "lesson_reads":0,"exercise_updates":0,"lesson_focus":false,
            "frames":self.student.frames.len(),"scope":"closed-book retention; not held-out generalization"}),
        )
    }
}

/// Export physically separate teaching and examination files. No answer-bearing
/// test item enters the teaching artifact.
pub fn split_lesson(input: &str) -> Result<(String, String)> {
    let _ = parse_material(input, "split.lesson")?;
    let (mut train, mut exam) = (String::new(), String::new());
    for line in input.lines() {
        let key = line.split_once(':').map(|(k, _)| k.trim()).unwrap_or("");
        let header = matches!(key, "course" | "lesson" | "title" | "level" | "stars");
        if header || matches!(key, "explain" | "fact" | "exercise") {
            train.push_str(line);
            train.push('\n');
        }
        if header || matches!(key, "test" | "exam") {
            exam.push_str(line);
            exam.push('\n');
        }
    }
    Ok((train, exam))
}
fn write_new(path: &str, bytes: &[u8]) -> Result<()> {
    let mut f = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    Ok(())
}
pub fn main_args(args: &[String]) -> Result<()> {
    let start = std::time::Instant::now();
    let usage = "knowledge retain split LESSON TRAIN_OUT EXAM_OUT | train LESSON NEW_STATE [BASE_STATE] | practice LESSON NEW_STATE BASE_STATE | test STATE EXAM_ONLY_LESSON | meanings STATE LAYER_OUT";
    let arg = |i| args.get(i).map(String::as_str).context(usage);
    let result = match arg(0)? {
        "split" => {
            ensure!(args.len() == 4, "{usage}");
            ensure!(
                !Path::new(arg(2)?).exists() && !Path::new(arg(3)?).exists(),
                "output already exists"
            );
            ensure!(arg(2)? != arg(3)?, "outputs must differ");
            let (train, exam) = split_lesson(&fs::read_to_string(arg(1)?)?)?;
            write_new(arg(2)?, train.as_bytes())?;
            write_new(arg(3)?, exam.as_bytes())?;
            serde_json::json!({"split":true})
        }
        "train" | "practice" => {
            ensure!((3..=4).contains(&args.len()), "{usage}");
            let l = parse_material(&fs::read_to_string(arg(1)?)?, arg(1)?)?;
            let mut st = if args.len() == 4 {
                Retained::load(Path::new(arg(3)?))?
            } else {
                Retained::new(&l.course)
            };
            let result = st.train(&l, args[0] == "practice")?;
            st.save(Path::new(arg(2)?))?;
            serde_json::json!({"training":result,"state_sha256":st.fingerprint()?})
        }
        "meanings" => {
            ensure!(args.len() == 3, "{usage}");
            let st = Retained::load(Path::new(arg(1)?))?;
            let t = st.meanings_layer()?;
            crate::dictstore::store().guard_derived_write(Path::new(arg(2)?))?;
            write_new(arg(2)?, t.as_bytes())?;
            serde_json::json!({"layer":arg(2)?,"rows":t.lines().filter(|l| !l.starts_with('#')).count(),"state_sha256":st.fingerprint()?})
        }
        "test" => {
            ensure!(args.len() == 3, "{usage}");
            let st = Retained::load(Path::new(arg(1)?))?;
            let l = parse_material(&fs::read_to_string(arg(2)?)?, arg(2)?)?;
            st.exam(&l)?
        }
        _ => bail!("{usage}"),
    };
    println!("{}", serde_json::to_string_pretty(&result)?);
    eprintln!(
        "retention wall_seconds={:.9}",
        start.elapsed().as_secs_f64()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn teaching_fixture() -> Lesson {
        Lesson {
            course: "books".into(),
            n: 7,
            explain: vec!["Alice carried a basket.".into()],
            ..Default::default()
        }
    }
    fn fixture() -> Retained {
        // Hand-built graph: tests do not require a parser or any neural weights.
        let mut s = PragStudent::default();
        let ent = Ent {
            id: "alice".into(),
            text: "Alice".into(),
            named: true,
            ..Default::default()
        };
        let frame = Frame {
            kind: Kind::Event,
            agent: Some(ent),
            verb: "carry".into(),
            neg: false,
            patient: Some(Ent {
                id: "basket".into(),
                text: "a basket".into(),
                ..Default::default()
            }),
            recipient: None,
            place: None,
            attrs: vec![],
            manner: vec![],
            obls: vec![],
            quote: vec![],
            quoted: false,
            cause: None,
            next: Some(1),
            cond: false,
            irrealis: false,
            src: (7, 0),
            how: Default::default(),
            unsaid: vec![],
            words: vec![],
            nps: vec![],
            mark: vec![],
            cc: None,
            link: None,
            tam: Default::default(),
        };
        let mut second = frame.clone();
        second.next = None;
        second.cause = Some(0);
        s.frames = vec![frame, second];
        s.w[0] += 3;
        s.texts.insert((7, 0), "Alice carried a basket.".into());
        s.same
            .insert("alice".into(), BTreeSet::from(["girl".into()]));
        s.parts.insert(("bird".into(), "wing".into()), (2, (7, 0)));
        Retained {
            student: s,
            course: "books".into(),
            lessons: BTreeMap::from([(
                7,
                hash(
                    &serde_json::to_vec(&(
                        "books",
                        7,
                        &teaching_fixture().explain,
                        Vec::<String>::new(),
                        Vec::<(String, String, Vec<String>)>::new(),
                    ))
                    .unwrap(),
                ),
            )]),
            dictionary: crate::dictstore::store().fingerprint(),
            licence: String::new(),
        }
    }

    /// What a lesson taught about words stays in the saved state and comes back as a derived dictionary layer from
    /// the state alone; a state learned with another dictionary is refused (its negative control).
    #[test]
    fn word_meanings_are_retained_and_bound_to_the_dictionary() {
        let mut st = fixture();
        st.student.isa.insert("woodlouse".into(), ("crustacean".into(), (7, 0)));
        let p = path("meanings");
        st.save(&p).unwrap();
        let back = Retained::load(&p).unwrap();
        fs::remove_file(&p).unwrap();
        assert_eq!(back.student.isa["woodlouse"], ("crustacean".to_string(), (7, 0)));
        let layer = back.meanings_layer().unwrap();
        assert!(layer.lines().any(|l| l.starts_with("woodlouse\t") && l.contains("«woodlouse» is a kind of «crustacean» (lesson 7, line 0)")), "{layer}");
        assert!(layer.lines().any(|l| l.starts_with("alice\t") && l.contains("«girl»")));
        assert_eq!(layer, back.meanings_layer().unwrap());
        // the layer is a dictionary layer of state derived: knowledge only when unreviewed layers are allowed
        let d = path("meanings-dir");
        fs::create_dir_all(&d).unwrap();
        fs::write(d.join("m.tsv"), &layer).unwrap();
        let h = crate::dictstore::file_hash(&d.join("m.tsv")).unwrap();
        let m = |allow| crate::dictstore::Manifest { schema: 1, allow_unreviewed: allow, layers: vec![crate::dictstore::LayerSpec { id: "learned".into(), path: "m.tsv".into(), sha256: h.clone(), state: "derived".into(), licence: "elamur-cosmic".into(), scope: crate::dictstore::Scope::Public, review: None }] };
        let ds = crate::dictstore::Store::from_manifest(&m(false), &d, crate::dictstore::Purpose::Release).unwrap();
        assert_eq!(ds.knowledge("woodlouse").count(), 0);
        let ds = crate::dictstore::Store::from_manifest(&m(true), &d, crate::dictstore::Purpose::Internal).unwrap();
        assert_eq!(ds.knowledge("woodlouse").count(), 1);
        // another dictionary: refused
        let mut payload = st.payload();
        payload.dictionary = "0".repeat(64);
        let sha256 = hash(&serde_json::to_vec(&payload).unwrap());
        let q = path("otherdict");
        fs::write(&q, serde_json::to_vec(&Envelope { payload, sha256 }).unwrap()).unwrap();
        let err = Retained::load(&q).err().expect("another dictionary must be refused");
        fs::remove_file(&q).unwrap();
        assert!(err.to_string().contains("another dictionary"));
    }
    fn path(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "mova-retain-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    fn load_payload(payload: Payload, label: &str) -> Retained {
        let sha256 = hash(&serde_json::to_vec(&payload).unwrap());
        let p = path(label);
        fs::write(
            &p,
            serde_json::to_vec(&Envelope { payload, sha256 }).unwrap(),
        )
        .unwrap();
        let restored = Retained::load(&p).unwrap();
        fs::remove_file(p).unwrap();
        restored
    }

    #[test]
    fn oracle_canonical_payload_fiber_preserves_values_not_vector_order() {
        let mut st = fixture();
        st.student.texts.insert((7, 1), "A second source.".into());
        st.student
            .parts
            .insert(("fish".into(), "fin".into()), (3, (7, 1)));
        st.student
            .per
            .insert(("bird".into(), "wing".into()), (2, "each".into(), (7, 0)));
        st.student
            .per
            .insert(("fish".into(), "fin".into()), (3, "each".into(), (7, 1)));
        let canonical = serde_json::to_value(st.payload()).unwrap();
        let canonical_id = st.fingerprint().unwrap();
        let mut wire_ids = BTreeSet::new();
        // Enumerate the entire product of the two orders for each two-entry vector.
        for mask in 0..8 {
            let mut p = st.payload();
            if mask & 1 != 0 {
                p.graph.texts.reverse();
            }
            if mask & 2 != 0 {
                p.graph.parts.reverse();
            }
            if mask & 4 != 0 {
                p.graph.per.reverse();
            }
            wire_ids.insert(hash(&serde_json::to_vec(&p).unwrap()));
            let restored = load_payload(p, "canonical");
            assert_eq!(serde_json::to_value(restored.payload()).unwrap(), canonical);
            assert_eq!(restored.fingerprint().unwrap(), canonical_id);
        }
        assert_eq!(wire_ids.len(), 8, "distinct accepted wire antecedents");
        let mut changed = st.payload();
        changed.graph.texts[0].1.push_str(" Changed.");
        assert_ne!(
            load_payload(changed, "changed-value")
                .fingerprint()
                .unwrap(),
            canonical_id
        );
    }

    #[test]
    fn oracle_checkpoint_fiber_erases_stale_indexes_and_focus() {
        let mut left = fixture();
        left.student.lesson_focus = true;
        left.student.by_ent.insert("ghost".into(), vec![999]);
        left.student.by_verb.insert("invented".into(), vec![999]);
        left.student.prev.insert(999, vec![999]);
        let mut right = fixture();
        right.student.lesson_focus = false;
        assert_ne!(left.student.by_ent, right.student.by_ent);
        assert_ne!(left.student.lesson_focus, right.student.lesson_focus);
        assert_eq!(left.fingerprint().unwrap(), right.fingerprint().unwrap());
        let a = load_payload(left.payload(), "loss-left");
        let b = load_payload(right.payload(), "loss-right");
        assert!(!a.student.lesson_focus && !b.student.lesson_focus);
        let expected_entities = BTreeMap::from([
            ("alice".to_string(), vec![0, 1]),
            ("basket".to_string(), vec![0, 1]),
        ]);
        assert_eq!(a.student.by_ent, expected_entities);
        assert_eq!(b.student.by_ent, expected_entities);
        assert_eq!(
            a.student.by_verb,
            BTreeMap::from([("carry".to_string(), vec![0, 1])])
        );
        assert_eq!(a.student.prev, BTreeMap::from([(1, vec![0])]));
        assert_eq!(a.student.effect, BTreeMap::from([(0, vec![1])]));
        let again = load_payload(a.payload(), "loss-idempotent");
        assert_eq!(again.student.by_ent, expected_entities);
        assert_eq!(again.fingerprint().unwrap(), b.fingerprint().unwrap());
    }

    #[test]
    fn oracle_reuse_fiber_selects_smallest_of_multiple_antecedents() {
        let mut st = fixture();
        let l = teaching_fixture();
        // Independent explicit identity tuples: do not use material_digest here.
        for n in [19usize, 3] {
            let bytes = serde_json::to_vec(&(
                "books",
                n,
                vec!["Alice carried a basket."],
                Vec::<String>::new(),
                Vec::<(String, String, Vec<String>)>::new(),
            ))
            .unwrap();
            st.lessons.insert(n, hash(&bytes));
        }
        for placeholder in [0, 1, 7, usize::MAX] {
            let mut candidate = l.clone();
            candidate.n = placeholder;
            candidate.title = "Different presentation".into();
            candidate
                .tests
                .push(("Q?".into(), "A".into(), vec!["B".into()]));
            candidate
                .exams
                .push(("Another?".into(), "C".into(), vec!["D".into()]));
            assert_eq!(st.retained_lesson(&candidate).unwrap(), Some(3));
        }
        st.lessons.remove(&3);
        assert_eq!(st.retained_lesson(&l).unwrap(), Some(7));
        st.lessons.remove(&7);
        assert_eq!(st.retained_lesson(&l).unwrap(), Some(19));
        let mut changed = l.clone();
        changed.facts.push("A new fact.".into());
        assert_eq!(st.retained_lesson(&changed).unwrap(), None);
        changed.course = "another".into();
        assert!(
            st.retained_lesson(&changed)
                .unwrap_err()
                .to_string()
                .contains("different course")
        );
    }

    #[test]
    fn oracle_allocation_frontier_ignores_holes_and_rejects_boundaries() {
        let mut st = fixture();
        for (keys, expected) in [
            (vec![], 1),
            (vec![0], 1),
            (vec![1, 3, 19], 20),
            (vec![2, 7], 8),
            (vec![usize::MAX - 2], usize::MAX - 1),
        ] {
            st.lessons = keys.into_iter().map(|n| (n, String::new())).collect();
            assert_eq!(st.next_lesson_number().unwrap(), expected);
        }
        st.lessons = BTreeMap::from([(KINDS_SRC - 1, String::new())]);
        assert!(
            st.next_lesson_number()
                .unwrap_err()
                .to_string()
                .contains("reserved")
        );
        st.lessons = BTreeMap::from([(usize::MAX, String::new())]);
        assert!(
            st.next_lesson_number()
                .unwrap_err()
                .to_string()
                .contains("exhausted")
        );
    }

    #[test]
    fn split_separates_training_from_exam_labels() {
        let text = "course: books\nlesson: 7\ntitle: retention\nlevel: primary-4\nexplain: Alice carried a basket.\nexercise: Who carried a basket? | Alice | Bob | Carl | Dan\ntest: What did Alice carry? | A basket | A hat | A pen | A cup\n";
        let (t, e) = split_lesson(text).unwrap();
        assert!(!t.contains("test:"));
        assert!(!t.contains("A hat"));
        assert!(!e.contains("explain:"));
        assert!(!e.contains("exercise:"));
        assert!(e.contains("test:"));
        assert!(t.contains("explain:"));
        assert!(
            parse(&t, "legacy.lesson").is_err(),
            "legacy full-lesson validation stays strict"
        );
        assert!(parse_material(&t, "teaching.lesson").is_ok());
        assert!(parse_material(&e, "exam.lesson").is_ok());
    }
    #[test]
    fn state_roundtrip_retains_weights_aliases_and_kind_graph_without_caches() {
        let st = fixture();
        let p = path("roundtrip");
        st.save(&p).unwrap();
        let restored = Retained::load(&p).unwrap();
        assert_eq!(st.fingerprint().unwrap(), restored.fingerprint().unwrap());
        assert!(!restored.student.lesson_focus);
        assert_eq!(restored.student.codes.len(), 2);
        assert_eq!(restored.student.prev[&1], vec![0]);
        assert_eq!(restored.student.effect[&0], vec![1]);
        assert_eq!(restored.student.by_ent["alice"], vec![0, 1]);
        assert!(restored.student.trees.borrow().is_empty());
        assert!(restored.student.reads.borrow().is_empty());
        assert!(st.save(&p).is_err(), "must not overwrite parent");
        fs::remove_file(p).unwrap();
    }
    #[test]
    fn tamper_and_unsupported_schema_are_rejected() {
        let st = fixture();
        let p = path("tamper");
        st.save(&p).unwrap();
        let mut v: serde_json::Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
        v["payload"]["graph"]["weights"][0] = 999.into();
        fs::write(&p, serde_json::to_vec(&v).unwrap()).unwrap();
        assert!(Retained::load(&p).is_err());
        v["payload"]["schema"] = "unknown".into();
        fs::write(&p, serde_json::to_vec(&v).unwrap()).unwrap();
        assert!(Retained::load(&p).is_err());
        fs::remove_file(p).unwrap();
    }
    #[test]
    fn structurally_invalid_graph_with_valid_hash_is_rejected() {
        let st = fixture();
        let p = path("edge");
        let mut payload = st.payload();
        payload.graph.frames[0].next = Some(999);
        let sha256 = hash(&serde_json::to_vec(&payload).unwrap());
        fs::write(
            &p,
            serde_json::to_vec(&Envelope { payload, sha256 }).unwrap(),
        )
        .unwrap();
        assert!(Retained::load(&p).is_err());
        fs::remove_file(p).unwrap();
    }
    #[test]
    fn exam_rejects_teaching_and_does_not_change_state() {
        let st = fixture();
        let before = st.fingerprint().unwrap();
        let l = Lesson {
            course: "books".into(),
            explain: vec!["answer leak".into()],
            ..Default::default()
        };
        assert!(st.exam(&l).is_err());
        assert_eq!(before, st.fingerprint().unwrap());
        let mut l = l;
        l.explain.clear();
        assert!(st.exam(&l).is_err());
    }
    #[test]
    fn duplicate_or_changed_lessons_fail_before_any_parsing() {
        let mut st = fixture();
        let mut l = teaching_fixture();
        assert!(
            st.train(&l, false)
                .unwrap_err()
                .to_string()
                .contains("already retained")
        );
        l.explain.push("Changed evidence.".into());
        assert!(
            st.train(&l, true).is_err(),
            "changed version cannot be practiced"
        );
        l.n = 8;
        assert!(st.train(&l, true).is_err());
        l.course = "biology".into();
        assert!(st.train(&l, false).is_err());
    }
    #[test]
    fn retained_material_is_found_across_placeholder_ids_and_assessment_changes() {
        let st = fixture();
        let mut l = teaching_fixture();
        l.n = 1;
        assert_eq!(st.retained_lesson(&l).unwrap(), Some(7));
        assert_eq!(st.next_lesson_number().unwrap(), 8);
        l.tests
            .push(("unseen assessment".into(), "a".into(), vec!["b".into()]));
        assert_eq!(st.retained_lesson(&l).unwrap(), Some(7));
        l.exercises
            .push(("new exercise".into(), "a".into(), vec!["b".into()]));
        assert_eq!(st.retained_lesson(&l).unwrap(), None);
        l.course = "biology".into();
        assert!(st.retained_lesson(&l).is_err());
    }
    #[test]
    fn assessment_identity_models_scored_inputs_and_ignores_unread_metadata() {
        let l = Lesson {
            course: "books".into(),
            n: 7,
            tests: vec![(
                "Who carried it?".into(),
                "Alice".into(),
                vec!["Bob".into(), "Clara".into(), "Dora".into()],
            )],
            ..Default::default()
        };
        let id = assessment_fingerprint(&l).unwrap();
        let mut same = l.clone();
        same.file = "elsewhere.lesson".into();
        same.title = "another title".into();
        assert_eq!(id, assessment_fingerprint(&same).unwrap());
        let mut q = l.clone();
        q.tests[0].0 = "What was carried?".into();
        assert_ne!(id, assessment_fingerprint(&q).unwrap());
        let mut gold = l.clone();
        gold.tests[0].1 = "Bob".into();
        assert_ne!(id, assessment_fingerprint(&gold).unwrap());
        let mut options = l.clone();
        options.tests[0].2.swap(0, 1);
        assert_ne!(id, assessment_fingerprint(&options).unwrap());
        let mut seed = l.clone();
        seed.n += 1;
        assert_ne!(id, assessment_fingerprint(&seed).unwrap());
        assert_eq!(
            l.tests.len(),
            q.tests.len(),
            "same count does not establish identity"
        );
    }

    fn sealed_rejection(payload: Payload, expected: &str) {
        let p = path("rejection-oracle");
        let sha256 = hash(&serde_json::to_vec(&payload).unwrap());
        let bytes = serde_json::to_vec(&Envelope { payload, sha256 }).unwrap();
        fs::write(&p, &bytes).unwrap();
        let error = match Retained::load(&p) {
            Ok(_) => panic!("accepted candidate expected to fail: {expected}"),
            Err(error) => error.to_string(),
        };
        assert_eq!(error, expected);
        assert_eq!(fs::read(&p).unwrap(), bytes, "load is read-only");
        fs::remove_file(p).unwrap();
    }

    #[test]
    fn oracle_resealed_rejections_partition_validation_gates() {
        // Correct checksums ensure integrity cannot mask structural defects.
        for defect in 0..11 {
            let mut p = fixture().payload();
            let expected = match defect {
                0 => {
                    p.schema = "future".into();
                    "unsupported retained schema"
                }
                1 => {
                    p.graph.weights.pop();
                    "role-weight layout mismatch"
                }
                2 => {
                    p.graph.texts.push(p.graph.texts[0].clone());
                    "duplicate source line"
                }
                3 => {
                    p.lessons.clear();
                    "source without lesson ledger"
                }
                4 => {
                    p.graph.frames[0].src = (7, 8);
                    "frame without source provenance"
                }
                5 => {
                    p.graph.frames[0].quote = vec![2];
                    "frame link outside graph"
                }
                6 => {
                    p.graph.frames[0].cause = Some(2);
                    "frame link outside graph"
                }
                7 => {
                    p.graph.frames[0].next = Some(2);
                    "frame link outside graph"
                }
                8 => {
                    p.graph.parts.push(p.graph.parts[0].clone());
                    "duplicate kind relation"
                }
                9 => {
                    let row = (("bird".into(), "wing".into()), (2, "each".into(), (7, 0)));
                    p.graph.per = vec![row.clone(), row];
                    "duplicate kind relation"
                }
                _ => {
                    p.graph.isa.insert("bird".into(), ("animal".into(), (8, 0)));
                    "kind relation without provenance"
                }
            };
            sealed_rejection(p, expected);
        }
        let mut p = fixture().payload();
        p.graph.texts.push(p.graph.texts[0].clone());
        p.lessons.clear();
        sealed_rejection(p, "duplicate source line");
        let mut p = fixture().payload();
        p.graph.frames[0].src = (7, 9);
        p.graph.frames[0].next = Some(2);
        sealed_rejection(p, "frame without source provenance");
    }

    #[test]
    fn oracle_provenance_and_topology_have_distinct_trust_boundaries() {
        for field in 0..3 {
            for source in [(7, 0), (7, 9), (8, 0), (KINDS_SRC, 9)] {
                let mut p = fixture().payload();
                match field {
                    0 => p.graph.parts[0].1.1 = source,
                    1 => {
                        p.graph.isa.insert("bird".into(), ("animal".into(), source));
                    }
                    _ => p
                        .graph
                        .per
                        .push((("bird".into(), "wing".into()), (2, "each".into(), source))),
                }
                if source == (7, 0) || source.0 == KINDS_SRC {
                    let restored = load_payload(p, "provenance-boundary");
                    let actual = match field {
                        0 => restored.student.parts[&("bird".into(), "wing".into())].1,
                        1 => restored.student.isa["bird"].1,
                        _ => restored.student.per[&("bird".into(), "wing".into())].2,
                    };
                    assert_eq!(actual, source);
                } else {
                    sealed_rejection(p, "kind relation without provenance");
                }
            }
        }
        let mut p = fixture().payload();
        p.graph.frames[0].src = (KINDS_SRC, 9);
        sealed_rejection(p, "frame without source provenance");
        // Bounds checks do not establish acyclicity or causal truth.
        let mut p = fixture().payload();
        p.graph.frames[0].quote = vec![0, 1];
        p.graph.frames[0].cause = Some(0);
        p.graph.frames[1].next = Some(0);
        let restored = load_payload(p, "cycles-accepted");
        assert_eq!(
            restored.student.prev,
            BTreeMap::from([(0, vec![1]), (1, vec![0])])
        );
        assert_eq!(restored.student.effect, BTreeMap::from([(0, vec![0, 1])]));
        assert_eq!(restored.student.sayer, BTreeMap::from([(0, 0), (1, 0)]));
    }

    #[test]
    fn oracle_restore_occurrences_and_quote_ownership_are_ordered() {
        let mut p = fixture().payload();
        p.graph.frames[0].agent.as_mut().unwrap().and = vec!["alice".into(), "bob".into()];
        p.graph.frames[0].patient.as_mut().unwrap().id = "alice".into();
        p.graph.frames[0].quote = vec![1, 1];
        p.graph.frames[1].quote = vec![1];
        p.graph.frames[0].verb = "be".into();
        p.graph.frames[1].verb.clear();
        let restored = load_payload(p, "ordered-indexes");
        assert_eq!(
            restored.student.by_ent,
            BTreeMap::from([
                ("alice".into(), vec![0, 0, 0, 1]),
                ("basket".into(), vec![1]),
                ("bob".into(), vec![0]),
            ])
        );
        assert!(
            restored.student.by_verb.is_empty(),
            "light and empty verbs are omitted"
        );
        assert_eq!(
            restored.student.by_line,
            BTreeMap::from([((7, 0), vec![0, 1])])
        );
        assert_eq!(restored.student.sayer, BTreeMap::from([(1, 1)]));
        assert_eq!(restored.student.codes.len(), 2);
        assert!(restored.student.trees.borrow().is_empty());
        assert!(restored.student.reads.borrow().is_empty());
        // Equal owner projections do not identify all earlier quoting frames.
        let mut alternate = restored.payload();
        alternate.graph.frames[0].quote.clear();
        let other = load_payload(alternate, "owner-preimage");
        assert_eq!(other.student.sayer, restored.student.sayer);
        assert_ne!(
            other.fingerprint().unwrap(),
            restored.fingerprint().unwrap()
        );
    }

    #[test]
    fn oracle_checkpoint_integrity_is_not_authentication_or_validation() {
        let mut st = fixture();
        let original = st.fingerprint().unwrap();
        let p = path("integrity-boundary");
        st.save(&p).unwrap();
        let bytes = fs::read(&p).unwrap();
        assert!(st.save(&p).is_err());
        assert_eq!(fs::read(&p).unwrap(), bytes);
        assert_eq!(st.fingerprint().unwrap(), original);
        fs::remove_file(&p).unwrap();

        let mut forged = st.payload();
        forged.graph.texts[0].1 = "Unverified replacement testimony.".into();
        forged.lessons.insert(7, "not a digest".into());
        let loaded = load_payload(forged, "unauthenticated");
        assert_eq!(
            loaded.student.texts[&(7, 0)],
            "Unverified replacement testimony."
        );
        assert_eq!(loaded.lessons[&7], "not a digest");
        assert_eq!(loaded.retained_lesson(&teaching_fixture()).unwrap(), None);
        assert_ne!(loaded.fingerprint().unwrap(), original);

        // Exclusive serialization does not run the load validator.
        st.student.frames[0].next = Some(2);
        st.save(&p).unwrap();
        assert!(Retained::load(&p).is_err());
        fs::remove_file(&p).unwrap();
        fs::write(&p, br#"{"payload":null,"sha256":"anything"}"#).unwrap();
        assert!(Retained::load(&p).is_err());
        fs::remove_file(&p).unwrap();
        let payload = fixture().payload();
        fs::write(
            &p,
            serde_json::to_vec(&Envelope {
                payload,
                sha256: "wrong".into(),
            })
            .unwrap(),
        )
        .unwrap();
        let err = Retained::load(&p).err().unwrap().to_string();
        assert_eq!(err, "retained state hash mismatch");
        fs::remove_file(&p).unwrap();
    }
}
