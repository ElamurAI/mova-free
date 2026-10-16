//! TSV for COPY: columns separated by TAB, no quotes or escapes, NULL is `\N`. A value containing TAB, LF, CR or equal
//! to `\N` cannot be written — this is an error (fail-fast), not silent corruption. CoNLL-U lines have none of these by
//! definition; JSON (comments, metadata) is written by serde_json, which escapes control characters.

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

pub const NULL: &str = "\\N";

/// A column value.
pub enum V<'a> {
    Null,
    S(&'a str),
    I(i64),
    B(bool),
}

impl<'a> V<'a> {
    pub fn opt(s: Option<&'a str>) -> V<'a> {
        s.map_or(V::Null, V::S)
    }
    pub fn opt_i(x: Option<i64>) -> V<'a> {
        x.map_or(V::Null, V::I)
    }
}

pub struct Tsv {
    w: BufWriter<File>,
    cols: usize,
    path: PathBuf,
    pub rows: usize,
}

impl Tsv {
    pub fn create(path: &Path, header: &[&str]) -> Result<Tsv> {
        let mut w = BufWriter::new(File::create(path).with_context(|| format!("{}", path.display()))?);
        writeln!(w, "{}", header.join("\t"))?;
        Ok(Tsv { w, cols: header.len(), path: path.to_path_buf(), rows: 0 })
    }

    pub fn row(&mut self, vals: &[V]) -> Result<()> {
        if vals.len() != self.cols {
            bail!("{}: {} columns instead of {}", self.path.display(), vals.len(), self.cols);
        }
        for (i, v) in vals.iter().enumerate() {
            if i > 0 {
                self.w.write_all(b"\t")?;
            }
            match v {
                V::Null => self.w.write_all(NULL.as_bytes())?,
                V::S(s) => {
                    if s.contains(['\t', '\n', '\r']) || *s == NULL {
                        bail!("{}: value not allowed in TSV (TAB, LF, CR or \\N): {:?}", self.path.display(), s.chars().take(80).collect::<String>());
                    }
                    self.w.write_all(s.as_bytes())?;
                }
                V::I(x) => write!(self.w, "{x}")?,
                V::B(b) => self.w.write_all(if *b { b"true" } else { b"false" })?,
            }
        }
        self.w.write_all(b"\n")?;
        self.rows += 1;
        Ok(())
    }

    pub fn finish(mut self) -> Result<usize> {
        self.w.flush()?;
        Ok(self.rows)
    }
}

/// Read TSV from COPY TO (no header): lines → columns, `\N` → None.
pub fn read(path: &Path) -> Result<Vec<Vec<Option<String>>>> {
    let f = File::open(path).with_context(|| format!("{}", path.display()))?;
    let mut out = Vec::new();
    for line in BufReader::new(f).lines() {
        let line = line?;
        out.push(line.split('\t').map(|c| if c == NULL { None } else { Some(c.to_string()) }).collect());
    }
    Ok(out)
}

/// A column as a string (NULL is an error).
pub fn s(row: &[Option<String>], i: usize) -> Result<&str> {
    row.get(i).and_then(|c| c.as_deref()).with_context(|| format!("column {i} is empty: {row:?}"))
}

pub fn int(row: &[Option<String>], i: usize) -> Result<i64> {
    let v = s(row, i)?;
    v.parse().with_context(|| format!("column {i} is not a number: {v}"))
}
