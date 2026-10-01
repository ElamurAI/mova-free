//! Files and directories (vfs, 26.09): `isfile isfolder mkdir rmdir delete movefile copyfile readdir`, `exist(…, 'file')`.
//! Everything goes through `it.fs` (the `vfs::FileSystem` trait), like `readtable`/`writetable`: in memory, in a directory, or the real FS.

use super::{Registry, boolv, str_arg, str_of};
use crate::interp::{Interp, MError};
use crate::value::{StrArr, Value};
use std::rc::Rc;
use vfs::FileType;

pub fn register(r: &mut Registry) {
    r.register("isfile", |it, a, _| {
        let p = str_arg(a, 0, "isfile")?;
        boolv(matches!(it.fs.stat(&p), Ok(s) if s.kind == FileType::File))
    });
    r.register("isfolder", |it, a, _| {
        let p = str_arg(a, 0, "isfolder")?;
        boolv(matches!(it.fs.stat(&p), Ok(s) if s.kind == FileType::Dir))
    });
    r.register("mkdir", mkdir);
    r.register("rmdir", rmdir);
    r.register("delete", delete);
    r.register("movefile", |it, a, n| move_or_copy(it, a, n, "movefile"));
    r.register("copyfile", |it, a, n| move_or_copy(it, a, n, "copyfile"));
    r.register("readdir", readdir);
}

/// `exist(name)` for files and directories: 2 — file, 7 — directory, 0 — none (variables and functions are checked by `exist` in `core`).
pub fn exist_path(it: &mut Interp, name: &str) -> f64 {
    match it.fs.stat(name) {
        Ok(s) if s.kind == FileType::Dir => 7.0,
        Ok(_) => 2.0,
        Err(_) => 0.0,
    }
}

/// Result `[status, msg]` for functions that do not fail when nargout ≥ 1 (as in MATLAB).
fn status(ok: bool, msg: &str, nargout: usize) -> Result<Vec<Value>, MError> {
    let mut out = vec![Value::boolean(ok)];
    if nargout > 1 {
        out.push(Value::str(msg));
    }
    Ok(out)
}

/// `mkdir(p)` / `mkdir(parent, name)`: creates with parents; an existing directory — a warning (nargout = 0).
fn mkdir(it: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let mut p = str_arg(a, 0, "mkdir")?;
    if a.len() > 1 {
        p = vfs::path::child(&p, &str_arg(a, 1, "mkdir")?);
    }
    let existed = matches!(it.fs.stat(&p), Ok(s) if s.kind == FileType::Dir);
    let r = if existed { Ok(()) } else { it.fs.create_dir_all(&p) };
    match r {
        Ok(()) if existed => {
            if nargout == 0 {
                it.warn("Directory already exists.");
                return Ok(vec![]);
            }
            status(true, "Directory already exists.", nargout)
        }
        Ok(()) => {
            if nargout == 0 { Ok(vec![]) } else { status(true, "", nargout) }
        }
        Err(e) => {
            let msg = format!("mkdir: cannot create directory '{p}': {e}");
            if nargout == 0 { Err(MError::new(msg)) } else { status(false, &msg, nargout) }
        }
    }
}

/// `rmdir(p)` — an empty directory; `rmdir(p, 's')` — with all its contents.
fn rmdir(it: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let p = str_arg(a, 0, "rmdir")?;
    let recursive = a.get(1).and_then(str_of).is_some_and(|s| s == "s");
    let r = if recursive { it.fs.remove_dir_all(&p) } else { it.fs.remove_dir(&p) };
    match r {
        Ok(()) => {
            if nargout == 0 { Ok(vec![]) } else { status(true, "", nargout) }
        }
        Err(e) => {
            let msg = format!("rmdir: cannot remove '{p}': {e}");
            if nargout == 0 { Err(MError::new(msg)) } else { status(false, &msg, nargout) }
        }
    }
}

/// `delete(p, …)`: files; a nonexistent one — a warning (as in Octave), a directory — an error.
fn delete(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    if a.is_empty() {
        return Err(super::invalid_call("delete"));
    }
    for k in 0..a.len() {
        let p = str_arg(a, k, "delete")?;
        match it.fs.stat(&p) {
            Err(_) => it.warn(&format!("delete: no such file: {p}")),
            Ok(s) if s.kind == FileType::Dir => {
                return Err(MError::new(format!("delete: '{p}' is a directory (use rmdir)")));
            }
            Ok(_) => it.fs.remove_file(&p).map_err(|e| MError::new(format!("delete: cannot delete '{p}': {e}")))?,
        }
    }
    Ok(vec![])
}

/// `movefile(src, dst)` / `copyfile(src, dst)`: if `dst` is a directory, then into it with the same name.
fn move_or_copy(it: &mut Interp, a: &[Value], nargout: usize, fname: &str) -> Result<Vec<Value>, MError> {
    let src = str_arg(a, 0, fname)?;
    let mut dst = str_arg(a, 1, fname)?;
    if matches!(it.fs.stat(&dst), Ok(s) if s.kind == FileType::Dir) {
        dst = vfs::path::child(&dst, vfs::path::base_name(&src));
    }
    let r = if fname == "movefile" {
        it.fs.rename(&src, &dst)
    } else {
        match it.fs.stat(&src) {
            Ok(s) if s.kind == FileType::Dir => {
                Err(vfs::FsError::with(vfs::Errno::IsDir, "copying directories is not supported yet"))
            }
            _ => it.fs.copy(&src, &dst).map(|_| ()),
        }
    };
    match r {
        Ok(()) => {
            if nargout == 0 { Ok(vec![]) } else { status(true, "", nargout) }
        }
        Err(e) => {
            let msg = format!("{fname}: cannot {} '{src}' to '{dst}': {e}", if fname == "movefile" { "move" } else { "copy" });
            if nargout == 0 { Err(MError::new(msg)) } else { status(false, &msg, nargout) }
        }
    }
}

/// `readdir(p)` (Octave): names in a directory including `.` and `..`, byte order. There are no cell arrays in v2 — returns an array
/// of strings (a column); the second value — error code (0 — ok), the third — message.
fn readdir(it: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let p = str_arg(a, 0, "readdir")?;
    match it.fs.read_dir(&p) {
        Ok(list) => {
            let mut names = vec![Some(".".to_string()), Some("..".to_string())];
            names.extend(list.into_iter().map(|e| Some(e.name)));
            let mut out = vec![Value::Str(Rc::new(StrArr::col(names)))];
            if nargout > 1 {
                out.push(Value::Mat(crate::value::Mat::scalar(0.0)));
            }
            if nargout > 2 {
                out.push(Value::str(""));
            }
            Ok(out)
        }
        Err(e) => {
            let msg = format!("readdir: cannot read '{p}': {e}");
            if nargout < 2 {
                return Err(MError::new(msg));
            }
            let mut out = vec![Value::Str(Rc::new(StrArr::col(vec![]))), Value::Mat(crate::value::Mat::scalar(-1.0))];
            if nargout > 2 {
                out.push(Value::str(&msg));
            }
            Ok(out)
        }
    }
}
