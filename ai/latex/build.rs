//! Compiles the table `data/commands.txt` into code (`table.rs` in OUT_DIR): the enums `Sym`, `FracKind`,
//! `Accent`, `Font`, `TextStyle`, `Stack`, `XArrow`, `EnvKind`, arrays of canonical forms and constant
//! `match`es: command name → `Entry`, character → `Sym`, environment name → `EnvKind`. Nothing is read
//! from disk at run time; parser rules compare enums. A duplicate form or an unknown kind,
//! class or variant is a build error (fail-fast).

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;

/// An enum grown from table rows: variants in order of appearance, the canonical form is the first one.
#[derive(Default)]
struct En {
    variants: Vec<String>,
    canon: HashMap<String, String>,
    /// For FracKind — the infix form; for EnvKind — the plain TeX command.
    alt: HashMap<String, String>,
}

impl En {
    fn add(&mut self, v: &str) {
        if !self.variants.iter().any(|x| x == v) {
            self.variants.push(v.to_string());
        }
    }
    fn canon(&mut self, v: &str, form: &str) {
        self.add(v);
        self.canon.entry(v.to_string()).or_insert_with(|| form.to_string());
    }
    fn alt(&mut self, v: &str, form: &str) {
        self.add(v);
        self.alt.entry(v.to_string()).or_insert_with(|| form.to_string());
    }
}

struct SymRow {
    id: String,
    class: &'static str,
    delim: bool,
    text: String,
    canon: String,
}

const CLASSES: [(&str, &str); 12] = [
    ("ord", "Ord"),
    ("op", "Op"),
    ("opn", "OpNoLim"),
    ("fn", "Fn"),
    ("fnl", "FnLim"),
    ("bin", "Bin"),
    ("rel", "Rel"),
    ("open", "Open"),
    ("close", "Close"),
    ("fence", "Fence"),
    ("punct", "Punct"),
    ("inner", "Inner"),
];

/// Kinds without an Id: table kind → `Entry` variant.
const SINGLES: [(&str, &str); 31] = [
    ("eqno", "Eqno"),
    ("sqrt", "Sqrt"),
    ("root", "Root"),
    ("genfrac", "Genfrac"),
    ("unicode", "Unicode"),
    ("left", "Left"),
    ("right", "Right"),
    ("middle", "Middle"),
    ("begin", "Begin"),
    ("end", "End"),
    ("opname", "OpName"),
    ("opnamelim", "OpNameLim"),
    ("not", "Not"),
    ("space", "Space"),
    ("spacearg", "SpaceArg"),
    ("kern", "Kern"),
    ("ignore", "Ignore"),
    ("ignorearg", "IgnoreArg"),
    ("phantom", "Phantom"),
    ("splice", "Splice"),
    ("smash", "Smash"),
    ("color", "Color"),
    ("textcolor", "TextColor"),
    ("newcommand", "NewCommand"),
    ("providecommand", "ProvideCommand"),
    ("def", "Def"),
    ("let", "Let"),
    ("declareop", "DeclareOp"),
    ("rowsep", "RowSep"),
    ("sup", "Sup"),
    ("sub", "Sub"),
];

const CLASS_IDS: [&str; 8] = ["Ord", "Op", "Bin", "Rel", "Open", "Close", "Punct", "Inner"];

fn is_ident(s: &str) -> bool {
    let mut cs = s.chars();
    matches!(cs.next(), Some(c) if c.is_ascii_uppercase()) && cs.all(|c| c.is_ascii_alphanumeric())
}

fn main() {
    println!("cargo:rerun-if-changed=data/commands.txt");
    println!("cargo:rerun-if-changed=build.rs");
    let text = std::fs::read_to_string("data/commands.txt").expect("data/commands.txt");

    let mut syms: Vec<SymRow> = Vec::new();
    let mut sym_ids = HashSet::new();
    let mut enums: HashMap<&'static str, En> = HashMap::new();
    for e in ["FracKind", "Accent", "Font", "TextStyle", "Stack", "XArrow", "EnvKind"] {
        enums.insert(e, En::default());
    }
    // command name (without \) → Entry expression
    let mut cmds: Vec<(String, String)> = Vec::new();
    let mut cmd_seen: HashMap<String, usize> = HashMap::new();
    // character → Sym (explicit forms)
    let mut chars: Vec<(char, String)> = Vec::new();
    let mut char_seen: HashSet<char> = HashSet::new();
    // Unicode texts as fallback forms (added if no one has claimed the character explicitly)
    let mut auto_chars: Vec<(char, String)> = Vec::new();
    let mut envs: Vec<(String, String)> = Vec::new();
    let mut env_seen: HashSet<String> = HashSet::new();
    // font switches and infixes must refer to existing variants — checked after the pass
    let mut refs: Vec<(usize, &'static str, String)> = Vec::new();

    let mut add_cmd = |ln: usize, form: &str, expr: String, cmds: &mut Vec<(String, String)>| {
        let name = form.strip_prefix('\\').unwrap_or_else(|| panic!("commands.txt:{ln}: command without \\: {form:?}"));
        assert!(!name.is_empty(), "commands.txt:{ln}: empty command");
        if let Some(prev) = cmd_seen.insert(name.to_string(), ln) {
            panic!("commands.txt:{ln}: duplicate command {form} (already on line {prev})");
        }
        cmds.push((name.to_string(), expr));
    };

    for (i, line) in text.lines().enumerate() {
        let ln = i + 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split_whitespace().collect();
        let kind = cols[0];
        match kind {
            "sym" => {
                assert!(cols.len() >= 5, "commands.txt:{ln}: sym — expected Id class text forms…: {line:?}");
                let (id, cls, txt) = (cols[1], cols[2], cols[3]);
                assert!(is_ident(id), "commands.txt:{ln}: Id {id:?} — not a variant name");
                assert!(sym_ids.insert(id.to_string()), "commands.txt:{ln}: duplicate Sym {id}");
                let (cls, delim) = match cls.strip_suffix("/d") {
                    Some(c) => (c, true),
                    None => (cls, false),
                };
                let class = CLASSES
                    .iter()
                    .find(|(k, _)| *k == cls)
                    .unwrap_or_else(|| panic!("commands.txt:{ln}: unknown class {cls:?}"))
                    .1;
                let delim = delim || matches!(cls, "open" | "close" | "fence");
                for f in &cols[4..] {
                    if f.starts_with('\\') && f.len() > 1 {
                        add_cmd(ln, f, format!("Entry::Sym(Sym::{id})"), &mut cmds);
                    } else {
                        let mut cs = f.chars();
                        let c = cs.next().unwrap();
                        assert!(cs.next().is_none(), "commands.txt:{ln}: form {f:?} — neither a command nor a single character");
                        assert!(char_seen.insert(c), "commands.txt:{ln}: duplicate character {c:?}");
                        chars.push((c, id.to_string()));
                    }
                }
                let mut tc = txt.chars();
                if let (Some(c), None) = (tc.next(), tc.next()) {
                    auto_chars.push((c, id.to_string()));
                }
                syms.push(SymRow {
                    id: id.to_string(),
                    class,
                    delim,
                    text: txt.to_string(),
                    canon: cols[4].to_string(),
                });
            }
            "frac" | "infix" | "accent" | "font" | "fontsw" | "text" | "stack" | "xarrow" | "class" | "plainenv" => {
                assert!(cols.len() >= 3, "commands.txt:{ln}: {kind} — expected Id and forms: {line:?}");
                let id = cols[1];
                assert!(is_ident(id), "commands.txt:{ln}: Id {id:?} — not a variant name");
                let (en, ctor): (&'static str, &str) = match kind {
                    "frac" => ("FracKind", "Frac"),
                    "infix" => ("FracKind", "Infix"),
                    "accent" => ("Accent", "Accent"),
                    "font" => ("Font", "Font"),
                    "fontsw" => ("Font", "FontSwitch"),
                    "text" => ("TextStyle", "Text"),
                    "stack" => ("Stack", "Stack"),
                    "xarrow" => ("XArrow", "XArrow"),
                    "class" => ("Class", "MathClass"),
                    "plainenv" => ("EnvKind", "PlainEnv"),
                    _ => unreachable!(),
                };
                if en == "Class" {
                    assert!(CLASS_IDS.contains(&id), "commands.txt:{ln}: unknown Class {id}");
                } else {
                    let e = enums.get_mut(en).unwrap();
                    match kind {
                        "fontsw" => refs.push((ln, "Font", id.to_string())),
                        "infix" | "plainenv" => e.alt(id, cols[2]),
                        _ => e.canon(id, cols[2]),
                    }
                }
                for f in &cols[2..] {
                    add_cmd(ln, f, format!("Entry::{ctor}({en}::{id})"), &mut cmds);
                }
            }
            "env" => {
                assert!(cols.len() >= 3, "commands.txt:{ln}: env — expected Id and names: {line:?}");
                let id = cols[1];
                assert!(is_ident(id), "commands.txt:{ln}: Id {id:?} — not a variant name");
                enums.get_mut("EnvKind").unwrap().canon(id, cols[2]);
                for f in &cols[2..] {
                    assert!(env_seen.insert(f.to_string()), "commands.txt:{ln}: duplicate environment {f}");
                    envs.push((f.to_string(), id.to_string()));
                }
            }
            _ => {
                let ctor = SINGLES
                    .iter()
                    .find(|(k, _)| *k == kind)
                    .unwrap_or_else(|| panic!("commands.txt:{ln}: unknown kind {kind:?}"))
                    .1;
                assert!(cols.len() >= 3 && cols[1] == "-", "commands.txt:{ln}: {kind} — expected «-» and forms: {line:?}");
                for f in &cols[2..] {
                    add_cmd(ln, f, format!("Entry::{ctor}"), &mut cmds);
                }
            }
        }
    }
    for (ln, en, id) in &refs {
        assert!(
            enums[en].variants.iter().any(|v| v == id),
            "commands.txt:{ln}: {en}::{id} has no primary form (fontsw refers to font)"
        );
    }
    for (c, id) in auto_chars {
        if char_seen.insert(c) {
            chars.push((c, id));
        }
    }

    let mut out = String::from("// Generated by build.rs from data/commands.txt — do not edit.\n\n");
    // ---- Sym
    let n = syms.len();
    out.push_str("/// A math-mode symbol: letter, operator, relation, delimiter, function.\n");
    out.push_str("#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]\n#[repr(u16)]\npub enum Sym {\n");
    for s in &syms {
        writeln!(out, "    {},", s.id).unwrap();
    }
    out.push_str("}\n\n");
    writeln!(out, "const SYM_LATEX: [&str; {n}] = [").unwrap();
    for s in &syms {
        writeln!(out, "    {:?},", s.canon).unwrap();
    }
    out.push_str("];\n");
    writeln!(out, "const SYM_TEXT: [&str; {n}] = [").unwrap();
    for s in &syms {
        writeln!(out, "    {:?},", s.text).unwrap();
    }
    out.push_str("];\n");
    writeln!(out, "const SYM_CLASS: [Class; {n}] = [").unwrap();
    for s in &syms {
        writeln!(out, "    Class::{},", s.class).unwrap();
    }
    out.push_str("];\n");
    writeln!(out, "const SYM_DELIM: [bool; {n}] = [").unwrap();
    for s in &syms {
        writeln!(out, "    {},", s.delim).unwrap();
    }
    out.push_str("];\n");
    writeln!(out, "pub const SYM_ALL: [Sym; {n}] = [").unwrap();
    for s in &syms {
        writeln!(out, "    Sym::{},", s.id).unwrap();
    }
    out.push_str("];\n\n");
    out.push_str(
        "impl Sym {\n    /// Canonical form for LaTeX output (`\\le`, `+`).\n    pub fn latex(self) -> &'static str {\n        SYM_LATEX[self as usize]\n    }\n    /// Text for plain-text output (Unicode or function name).\n    pub fn text(self) -> &'static str {\n        SYM_TEXT[self as usize]\n    }\n    pub fn class(self) -> Class {\n        SYM_CLASS[self as usize]\n    }\n    /// Whether it may follow `\\left`/`\\right`/`\\middle`.\n    pub fn is_delim(self) -> bool {\n        SYM_DELIM[self as usize]\n    }\n}\n\n",
    );

    // ---- other enums
    let docs: [(&str, &str); 7] = [
        ("FracKind", "Fraction kind: `\\frac`, `\\dfrac`, `\\binom`…; `Atop` is infix only."),
        ("Accent", "Accent or decoration with one argument: `\\hat`, `\\overline`, `\\boxed`, `\\pmod`…"),
        ("Font", "Math font: `\\mathbb`, `\\mathcal`, `\\mathrm`…"),
        ("TextStyle", "Text style inside a formula: `\\text`, `\\textbf`…"),
        ("Stack", "Script above/below: `\\overset`, `\\underset`, `\\stackrel`."),
        ("XArrow", "Extensible arrow with labels: `\\xrightarrow[below]{above}`."),
        ("EnvKind", "Tabular environment: matrices, `array`, `cases`, `aligned`…"),
    ];
    for (en, doc) in docs {
        let e = &enums[en];
        let n = e.variants.len();
        writeln!(out, "/// {doc}").unwrap();
        writeln!(out, "#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]\npub enum {en} {{").unwrap();
        for v in &e.variants {
            writeln!(out, "    {v},").unwrap();
        }
        out.push_str("}\n\n");
        writeln!(out, "impl {en} {{").unwrap();
        writeln!(out, "    pub const ALL: [{en}; {n}] = [").unwrap();
        for v in &e.variants {
            writeln!(out, "        {en}::{v},").unwrap();
        }
        out.push_str("    ];\n");
        let opt = |m: &HashMap<String, String>, v: &str| match m.get(v) {
            Some(f) => format!("Some({f:?})"),
            None => "None".to_string(),
        };
        match en {
            "FracKind" => {
                out.push_str("    /// Prefix form (`\\frac`); None — the kind is infix only.\n    pub fn latex(self) -> Option<&'static str> {\n        match self {\n");
                for v in &e.variants {
                    writeln!(out, "            {en}::{v} => {},", opt(&e.canon, v)).unwrap();
                }
                out.push_str("        }\n    }\n    /// Infix form (`\\over`), if any.\n    pub fn infix(self) -> Option<&'static str> {\n        match self {\n");
                for v in &e.variants {
                    writeln!(out, "            {en}::{v} => {},", opt(&e.alt, v)).unwrap();
                }
                out.push_str("        }\n    }\n");
            }
            "EnvKind" => {
                out.push_str("    /// Name for `\\begin{…}`; None — command only (`\\substack`).\n    pub fn name(self) -> Option<&'static str> {\n        match self {\n");
                for v in &e.variants {
                    writeln!(out, "            {en}::{v} => {},", opt(&e.canon, v)).unwrap();
                }
                out.push_str("        }\n    }\n    /// Plain TeX-style command (`\\pmatrix{…}`), if any.\n    pub fn plain(self) -> Option<&'static str> {\n        match self {\n");
                for v in &e.variants {
                    writeln!(out, "            {en}::{v} => {},", opt(&e.alt, v)).unwrap();
                }
                out.push_str("        }\n    }\n");
            }
            _ => {
                out.push_str("    /// Canonical form (first in the table).\n    pub fn latex(self) -> &'static str {\n        match self {\n");
                for v in &e.variants {
                    let f = e.canon.get(v).unwrap_or_else(|| panic!("{en}::{v} has no primary form"));
                    writeln!(out, "            {en}::{v} => {f:?},").unwrap();
                }
                out.push_str("        }\n    }\n");
            }
        }
        out.push_str("}\n\n");
    }

    // ---- lookup
    out.push_str("/// Command name (without `\\`) → built-in table entry.\npub fn lookup_cmd(name: &str) -> Option<Entry> {\n    match name {\n");
    for (name, expr) in &cmds {
        writeln!(out, "        {name:?} => Some({expr}),").unwrap();
    }
    out.push_str("        _ => None,\n    }\n}\n\n");
    out.push_str("/// Character in a formula (`+`, `<`, `≤`, `α`) → `Sym`.\npub fn lookup_char(c: char) -> Option<Sym> {\n    match c {\n");
    for (c, id) in &chars {
        writeln!(out, "        {c:?} => Some(Sym::{id}),").unwrap();
    }
    out.push_str("        _ => None,\n    }\n}\n\n");
    out.push_str("/// Environment name (`pmatrix`, `cases`) → `EnvKind`.\npub fn lookup_env(name: &str) -> Option<EnvKind> {\n    match name {\n");
    for (name, id) in &envs {
        writeln!(out, "        {name:?} => Some(EnvKind::{id}),").unwrap();
    }
    out.push_str("        _ => None,\n    }\n}\n");

    let dir = std::env::var("OUT_DIR").unwrap();
    std::fs::write(format!("{dir}/table.rs"), out).unwrap();
}
