//! Core completeness gate — shared by the build script (`build.rs`, the build fails) and `onto controls` (negative
//! control): `data/core.tsv` + `data/core-props.tsv` → «concept × field» cells or an error naming the concept and field.

use std::collections::{BTreeMap, HashMap};

pub const FIELDS: [&str; 11] = ["food", "habitat", "part", "material", "use", "mass", "height", "length", "lifespan", "speed", "risk"];
pub const NUMBER: [&str; 5] = ["mass", "height", "length", "lifespan", "speed"];
pub const DOMAINS: [(&str, &str); 7] = [("animal", "Animal"), ("food", "Food"), ("plant", "Plant"), ("household", "Household"), ("family", "Family"), ("role", "Role"), ("magical", "Magical")];

pub type Cells = HashMap<(String, String), BTreeMap<String, Vec<Vec<String>>>>;

pub struct Parsed {
    /// (lemma, domain) in `core.tsv` order
    pub core: Vec<(String, String)>,
    pub cells: Cells,
    pub synset: HashMap<(String, String), String>,
}

pub fn read_core(core_txt: &str) -> Result<Vec<(String, String)>, String> {
    let mut core = Vec::new();
    for l in core_txt.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let f: Vec<&str> = l.split('\t').map(str::trim).collect();
        if f.len() < 2 || !DOMAINS.iter().any(|d| d.0 == f[1]) {
            return Err(format!("core: unknown domain in «{l}»"));
        }
        core.push((f[0].to_string(), f[1].to_string()));
    }
    Ok(core)
}

/// Parsing and gate: every concept × every field has a row; format, statuses, one number per field, «none» only for risks.
pub fn parse(core_txt: &str, props: &str) -> Result<Parsed, String> {
    let core = read_core(core_txt)?;
    let mut cells: Cells = HashMap::new();
    let mut synset: HashMap<(String, String), String> = HashMap::new();
    for (ln, l) in props.lines().enumerate().filter(|(_, l)| !l.starts_with('#') && !l.trim().is_empty()) {
        let f: Vec<String> = l.split('\t').map(String::from).collect();
        let at = format!("core-props.tsv:{}", ln + 1);
        if f.len() != 20 {
            return Err(format!("{at}: {} fields instead of 20", f.len()));
        }
        let k = (f[0].clone(), f[1].clone());
        if !core.contains(&k) {
            return Err(format!("{at}: concept «{} ({})» is not in data/core.tsv", f[0], f[1]));
        }
        if !FIELDS.contains(&f[3].as_str()) {
            return Err(format!("{at}: unknown field «{}»", f[3]));
        }
        if !["known", "none", "unknown", "n/a"].contains(&f[4].as_str()) {
            return Err(format!("{at}: unknown status «{}»", f[4]));
        }
        let prev = synset.entry(k.clone()).or_insert_with(|| f[2].clone());
        if *prev != f[2] {
            return Err(format!("{at}: «{}» has two synsets: {prev} and {}", f[0], f[2]));
        }
        cells.entry(k).or_default().entry(f[3].clone()).or_default().push(f);
    }
    for k in &core {
        let Some(fields) = cells.get(k) else { return Err(format!("completeness gate: concept «{} ({})» has no fields", k.0, k.1)) };
        for field in FIELDS {
            let Some(rows) = fields.get(field) else { return Err(format!("completeness gate: concept «{} ({})» has no field {field} — neither values nor an explicit «unknown»", k.0, k.1)) };
            let st = &rows[0][4];
            if rows.iter().any(|r| &r[4] != st) {
                return Err(format!("core: mixed statuses in cell «{} ({})» × {field}", k.0, k.1));
            }
            if st == "none" && field != "risk" {
                return Err(format!("core: «none» only for risks («{}» × {field})", k.0));
            }
            if st == "known" && NUMBER.contains(&field) && rows.len() != 1 {
                return Err(format!("core: {} numbers in field «{} ({})» × {field}", rows.len(), k.0, k.1));
            }
        }
    }
    Ok(Parsed { core, cells, synset })
}
