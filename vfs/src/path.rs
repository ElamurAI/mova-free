//! Paths: splitting into components, length checks, lexical normalization forbidding `..` above the root.

use crate::error::{Errno, FsError};

/// Longest name (bytes), like NAME_MAX in Linux.
pub const NAME_MAX: usize = 255;
/// Longest path (bytes), like PATH_MAX in Linux.
pub const PATH_MAX: usize = 4096;

/// Path components without empty ones and `.`; `..` is kept.
pub fn parts(path: &str) -> impl Iterator<Item = &str> {
    path.split('/').filter(|c| !c.is_empty() && *c != ".")
}

/// Empty path — ENOENT (as in POSIX), NUL byte — EINVAL, too long — ENAMETOOLONG.
pub fn check(path: &str) -> Result<(), FsError> {
    if path.is_empty() {
        return Err(FsError::new(Errno::NoEnt));
    }
    if path.contains('\0') {
        return Err(FsError::new(Errno::Inval));
    }
    if path.len() > PATH_MAX {
        return Err(FsError::new(Errno::NameTooLong));
    }
    Ok(())
}

pub fn check_name(name: &str) -> Result<(), FsError> {
    if name.len() > NAME_MAX {
        return Err(FsError::new(Errno::NameTooLong));
    }
    Ok(())
}

/// Lexical normalization from `cwd` (components from the root): `..` above the root is a root-escape error,
/// not a silent "sticking" to the root. Does not resolve symlinks (for a real directory this is done by
/// the canonical path check in `DirFs`).
pub fn normalize(cwd: &[String], path: &str) -> Result<Vec<String>, FsError> {
    normalize_with(cwd, path, false)
}

/// Same; `clamp` — a fault for the negative control: `..` above the root silently stays at the root.
pub fn normalize_with(cwd: &[String], path: &str, clamp: bool) -> Result<Vec<String>, FsError> {
    check(path)?;
    let mut out: Vec<String> = if path.starts_with('/') { Vec::new() } else { cwd.to_vec() };
    for c in parts(path) {
        if c == ".." {
            if out.pop().is_none() && !clamp {
                return Err(FsError::escape(path));
            }
        } else {
            check_name(c)?;
            out.push(c.to_string());
        }
    }
    Ok(out)
}

/// Components → absolute path: `[] → "/"`, `["a","b"] → "/a/b"`.
pub fn join(comps: &[String]) -> String {
    if comps.is_empty() {
        return "/".into();
    }
    let mut s = String::new();
    for c in comps {
        s.push('/');
        s.push_str(c);
    }
    s
}

/// Append a name to a directory: `("/", "a") → "/a"`, `("/x", "a") → "/x/a"`.
pub fn child(dir: &str, name: &str) -> String {
    if dir.ends_with('/') { format!("{dir}{name}") } else { format!("{dir}/{name}") }
}

/// Last component (for `movefile(src, dir)`): `"/a/b.csv" → "b.csv"`.
pub fn base_name(path: &str) -> &str {
    path.trim_end_matches('/').rsplit('/').next().unwrap_or("")
}

#[cfg(test)]
mod wave31 {
    use super::*;
    #[test]
    fn lexical_boundaries_and_fibers() {
        let cwd = vec!["a".to_string()];
        assert_eq!(normalize(&cwd, "../b/./c").unwrap(), ["b", "c"]);
        assert_eq!(normalize(&cwd, "/b//c").unwrap(), ["b", "c"]);
        assert_eq!(normalize(&[], "..").unwrap_err().errno, Errno::NotCapable);
        assert_eq!(normalize_with(&[], "..", true).unwrap(), Vec::<String>::new());
        assert_eq!(check("").unwrap_err().errno, Errno::NoEnt);
        assert_eq!(check(&format!("{}\0", "x".repeat(PATH_MAX))).unwrap_err().errno, Errno::Inval);
        assert!(check(&"/".repeat(PATH_MAX)).is_ok());
        assert_eq!(check(&"/".repeat(PATH_MAX + 1)).unwrap_err().errno, Errno::NameTooLong);
        assert!(check_name(&"x".repeat(NAME_MAX)).is_ok());
        assert!(check_name(&"é".repeat(128)).is_err());
        assert_eq!(join(&[]), "/");
        assert_eq!(child("/", "a"), "/a");
        assert_eq!(base_name("/a/b///"), "b");
    }
}
