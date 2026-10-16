//! wasi-host — runs WebAssembly modules (`wasm32-wasip1`) in a sandbox (`docs/context.md`, "Execution sandbox").
//!
//! - **Engine — `wasmi`** (an interpreter in Rust, MIT/Apache-2.0): 8 third-party crates in the tree versus ~106 packages
//!   in `wasmtime` with Cranelift; no JIT and no signal handlers; fuel is a deterministic instruction ceiling; it also builds for Mini.
//!   For heavy modules on the server `wasmtime` remains the next step — the same WASI layer over `vfs`.
//! - **WASI preview1** — all 46 functions of `wasi_snapshot_preview1`: file ones (`path_open`, `fd_read`, `fd_write`,
//!   `fd_seek`, `fd_readdir`, `path_filestat_get`…) go to the same [`vfs::Vfs`] as native mlab with `--fs mem`.
//! - **Reproducibility:** the clock is the virtual clock of `vfs` (each read advances it by `clock_step`),
//!   `random_get` is seeded splitmix64, `poll_oneoff` with a sleep only advances the clock. No network: `sock_*` → ENOTSUP.
//! - **Limits:** one preopened directory `/` — the root of `vfs`; escaping the root gives ENOTCAPABLE (and an entry in the `vfs` journal);
//!   guest memory ceiling (`StoreLimits`), fuel ceiling, descriptor ceiling.

use std::collections::BTreeMap;
use std::io::Write;
use vfs::{FileType, FsError, Op, Vfs};
use wasmi::{Caller, Config, Engine, Extern, Linker, Module, Store, StoreLimits, StoreLimitsBuilder};

/// WASI preview1 error numbers the host returns itself (the rest come from `vfs::Errno::wasi`).
pub mod errno {
    pub const SUCCESS: u16 = 0;
    pub const BADF: u16 = 8;
    pub const EXIST: u16 = 20;
    pub const FAULT: u16 = 21;
    pub const ILSEQ: u16 = 25;
    pub const INVAL: u16 = 28;
    pub const ISDIR: u16 = 31;
    pub const LOOP: u16 = 32;
    pub const MFILE: u16 = 33;
    pub const NOENT: u16 = 44;
    pub const NOTDIR: u16 = 54;
    pub const NOTSUP: u16 = 58;
    pub const SPIPE: u16 = 70;
    pub const NOTCAPABLE: u16 = 76;
}
use errno::*;

type W<T> = Result<T, u16>;

fn fe(e: FsError) -> u16 {
    e.errno.wasi()
}

const MODULE: &str = "wasi_snapshot_preview1";
const RIGHT_FD_READ: u64 = 1 << 1;
const RIGHT_FD_WRITE: u64 = 1 << 6;
const RIGHTS_ALL: u64 = (1 << 30) - 1;
const FT_CHAR: u8 = 2;
const FT_DIR: u8 = 3;
const FT_FILE: u8 = 4;
const FT_LINK: u8 = 7;
const FDFLAG_APPEND: u32 = 1;
const IOV_MAX: u32 = 1024;

fn ftype(k: FileType) -> u8 {
    match k {
        FileType::File => FT_FILE,
        FileType::Dir => FT_DIR,
        FileType::Symlink => FT_LINK,
    }
}

#[derive(Clone, Debug)]
enum Kind {
    Stdin,
    Stdout,
    Stderr,
    Dir { path: String, preopen: Option<String> },
    File { path: String, pos: u64, append: bool, read: bool, write: bool },
}

#[derive(Clone, Debug)]
struct Fd {
    kind: Kind,
    rights: u64,
    inheriting: u64,
}

// ---------- guest memory ----------

fn range(m: &[u8], ptr: u32, len: u32) -> W<std::ops::Range<usize>> {
    let start = ptr as usize;
    let end = start.checked_add(len as usize).ok_or(FAULT)?;
    if end > m.len() {
        return Err(FAULT);
    }
    Ok(start..end)
}
fn off(ptr: u32, k: u32) -> W<u32> {
    ptr.checked_add(k).ok_or(FAULT)
}
fn get_u8(m: &[u8], p: u32) -> W<u8> {
    Ok(m[range(m, p, 1)?][0])
}
fn get_u16(m: &[u8], p: u32) -> W<u16> {
    Ok(u16::from_le_bytes(m[range(m, p, 2)?].try_into().unwrap()))
}
fn get_u32(m: &[u8], p: u32) -> W<u32> {
    Ok(u32::from_le_bytes(m[range(m, p, 4)?].try_into().unwrap()))
}
fn get_u64(m: &[u8], p: u32) -> W<u64> {
    Ok(u64::from_le_bytes(m[range(m, p, 8)?].try_into().unwrap()))
}
fn put(m: &mut [u8], p: u32, b: &[u8]) -> W<()> {
    let r = range(m, p, b.len() as u32)?;
    m[r].copy_from_slice(b);
    Ok(())
}
fn put_u32(m: &mut [u8], p: u32, v: u32) -> W<()> {
    put(m, p, &v.to_le_bytes())
}
fn put_u64(m: &mut [u8], p: u32, v: u64) -> W<()> {
    put(m, p, &v.to_le_bytes())
}
fn get_str(m: &[u8], p: u32, len: u32) -> W<String> {
    String::from_utf8(m[range(m, p, len)?].to_vec()).map_err(|_| ILSEQ)
}
fn iovs(m: &[u8], ptr: u32, n: u32) -> W<Vec<(u32, u32)>> {
    if n > IOV_MAX {
        return Err(INVAL);
    }
    (0..n).map(|i| {
        let b = off(ptr, i * 8)?;
        Ok((get_u32(m, b)?, get_u32(m, off(b, 4)?)?))
    })
    .collect()
}
/// filestat: dev, ino, filetype, nlink, size, atim, mtim, ctim — 64 bytes.
fn put_filestat(m: &mut [u8], p: u32, ino: u64, ft: u8, size: u64, mtime: u64, ctime: u64) -> W<()> {
    let mut b = [0u8; 64];
    b[8..16].copy_from_slice(&ino.to_le_bytes());
    b[16] = ft;
    b[24..32].copy_from_slice(&1u64.to_le_bytes());
    b[32..40].copy_from_slice(&size.to_le_bytes());
    b[40..48].copy_from_slice(&mtime.to_le_bytes());
    b[48..56].copy_from_slice(&mtime.to_le_bytes());
    b[56..64].copy_from_slice(&ctime.to_le_bytes());
    put(m, p, &b)
}

/// Sandbox state of one run: file system, descriptors, arguments, output, clock, randomness.
pub struct Ctx {
    pub vfs: Vfs,
    pub args: Vec<String>,
    /// Environment variables `KEY=VALUE` (empty by default).
    pub env: Vec<String>,
    pub stdin: Vec<u8>,
    stdin_pos: usize,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    /// stderr goes to the same buffer as stdout (ordering as in `mlab` output in capture mode).
    pub merge_stderr: bool,
    /// Also echo output to the real stdout/stderr right away (CLI).
    pub echo: bool,
    fds: Vec<Option<Fd>>,
    /// Virtual clock step per time read, ns.
    pub clock_step: u64,
    rng: u64,
    pub exit_code: Option<i32>,
    /// How many times each WASI function was called.
    pub calls: BTreeMap<&'static str, u64>,
    limits: StoreLimits,
    pub max_fds: usize,
}

impl Ctx {
    pub fn new(vfs: Vfs, args: Vec<String>) -> Ctx {
        let std = |kind| Some(Fd { kind, rights: RIGHT_FD_READ | RIGHT_FD_WRITE, inheriting: 0 });
        Ctx {
            vfs,
            args,
            env: Vec::new(),
            stdin: Vec::new(),
            stdin_pos: 0,
            stdout: Vec::new(),
            stderr: Vec::new(),
            merge_stderr: false,
            echo: false,
            fds: vec![
                std(Kind::Stdin),
                std(Kind::Stdout),
                std(Kind::Stderr),
                Some(Fd {
                    kind: Kind::Dir { path: "/".into(), preopen: Some("/".into()) },
                    rights: RIGHTS_ALL,
                    inheriting: RIGHTS_ALL,
                }),
            ],
            clock_step: 1_000,
            rng: 0x5eed_2026_0926,
            exit_code: None,
            calls: BTreeMap::new(),
            limits: StoreLimitsBuilder::new().memory_size(1 << 30).build(),
            max_fds: 1024,
        }
    }
    pub fn with_seed(mut self, seed: u64) -> Ctx {
        self.rng = seed;
        self
    }
    /// Guest linear memory ceiling, bytes ("virtual RAM with limits").
    pub fn with_memory_limit(mut self, bytes: usize) -> Ctx {
        self.limits = StoreLimitsBuilder::new().memory_size(bytes).build();
        self
    }

    fn fd(&self, fd: u32) -> W<&Fd> {
        self.fds.get(fd as usize).and_then(|f| f.as_ref()).ok_or(BADF)
    }
    fn fd_mut(&mut self, fd: u32) -> W<&mut Fd> {
        self.fds.get_mut(fd as usize).and_then(|f| f.as_mut()).ok_or(BADF)
    }
    fn alloc_fd(&mut self, f: Fd) -> W<u32> {
        if let Some(k) = self.fds.iter().skip(3).position(|x| x.is_none()) {
            self.fds[k + 3] = Some(f);
            return Ok((k + 3) as u32);
        }
        if self.fds.len() >= self.max_fds {
            return Err(MFILE);
        }
        self.fds.push(Some(f));
        Ok((self.fds.len() - 1) as u32)
    }
    fn next_random(&mut self) -> u64 {
        // splitmix64
        self.rng = self.rng.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.rng;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    fn emit(&mut self, err: bool, data: &[u8]) {
        if err && !self.merge_stderr {
            self.stderr.extend_from_slice(data);
        } else {
            self.stdout.extend_from_slice(data);
        }
        if self.echo {
            if err {
                let _ = std::io::stdout().flush();
                let _ = std::io::stderr().write_all(data);
            } else {
                let _ = std::io::stdout().write_all(data);
            }
        }
    }
    /// Path relative to a directory descriptor → absolute path in `vfs`. WASI does not allow absolute paths in `path_*`.
    fn at(&mut self, m: &[u8], fd: u32, p: u32, len: u32) -> W<String> {
        let rel = get_str(m, p, len)?;
        let base = match &self.fd(fd)?.kind {
            Kind::Dir { path, .. } => path.clone(),
            _ => return Err(NOTDIR),
        };
        if rel.is_empty() {
            return Err(NOENT);
        }
        if rel.starts_with('/') {
            let e = FsError::escape(&rel);
            self.vfs.journal.record(self.vfs.now(), Op::Open, &rel, 0, Some(&e));
            return Err(NOTCAPABLE);
        }
        Ok(vfs::path::child(&base, &rel))
    }
    fn file(&self, fd: u32) -> W<(String, u64, bool, bool, bool)> {
        match &self.fd(fd)?.kind {
            Kind::File { path, pos, append, read, write } => Ok((path.clone(), *pos, *append, *read, *write)),
            Kind::Dir { .. } => Err(ISDIR),
            _ => Err(SPIPE),
        }
    }
    fn set_pos(&mut self, fd: u32, new: u64) -> W<()> {
        if let Kind::File { pos, .. } = &mut self.fd_mut(fd)?.kind {
            *pos = new;
        }
        Ok(())
    }

    // ---------- arguments, environment, time, randomness ----------

    fn strings_sizes(list: &[String], m: &mut [u8], count: u32, size: u32) -> W<()> {
        put_u32(m, count, list.len() as u32)?;
        put_u32(m, size, list.iter().map(|s| s.len() as u32 + 1).sum())
    }
    fn strings_get(list: &[String], m: &mut [u8], ptrs: u32, buf: u32) -> W<()> {
        let mut p = buf;
        for (i, s) in list.iter().enumerate() {
            put_u32(m, off(ptrs, i as u32 * 4)?, p)?;
            put(m, p, s.as_bytes())?;
            put(m, off(p, s.len() as u32)?, &[0])?;
            p = off(p, s.len() as u32 + 1)?;
        }
        Ok(())
    }
    fn clock_time_get(&mut self, m: &mut [u8], id: u32, out: u32) -> W<()> {
        if id > 3 {
            return Err(INVAL);
        }
        let now = self.vfs.now();
        self.vfs.advance(self.clock_step);
        put_u64(m, out, if id == 0 { now } else { now - vfs::EPOCH_NS })
    }
    fn random_get(&mut self, m: &mut [u8], buf: u32, len: u32) -> W<()> {
        let r = range(m, buf, len)?;
        let mut bytes = Vec::with_capacity(len as usize + 8);
        while bytes.len() < len as usize {
            let x = self.next_random();
            bytes.extend_from_slice(&x.to_le_bytes());
        }
        m[r].copy_from_slice(&bytes[..len as usize]);
        Ok(())
    }
    fn poll_oneoff(&mut self, m: &mut [u8], inp: u32, out: u32, nsubs: u32, nevents: u32) -> W<()> {
        if nsubs == 0 || nsubs > 4096 {
            return Err(INVAL);
        }
        let mut clocks = Vec::new();
        let mut fds = Vec::new();
        for i in 0..nsubs {
            let b = off(inp, i * 48)?;
            let ud = get_u64(m, b)?;
            match get_u8(m, off(b, 8)?)? {
                0 => {
                    let id = get_u32(m, off(b, 16)?)?;
                    let timeout = get_u64(m, off(b, 24)?)?;
                    let flags = get_u16(m, off(b, 40)?)?;
                    let now = if id == 0 { self.vfs.now() } else { self.vfs.now() - vfs::EPOCH_NS };
                    clocks.push((ud, if flags & 1 != 0 { timeout.saturating_sub(now) } else { timeout }));
                }
                t @ (1 | 2) => fds.push((ud, t)),
                _ => return Err(INVAL),
            }
        }
        // ready immediately: descriptors; otherwise the nearest timer (the clock simply advances)
        let events: Vec<(u64, u8)> = if !fds.is_empty() {
            fds
        } else {
            let min = clocks.iter().map(|c| c.1).min().unwrap_or(0);
            self.vfs.advance(min);
            clocks.into_iter().filter(|c| c.1 == min).map(|c| (c.0, 0)).collect()
        };
        for (k, (ud, ty)) in events.iter().enumerate() {
            let b = off(out, k as u32 * 32)?;
            let mut e = [0u8; 32];
            e[0..8].copy_from_slice(&ud.to_le_bytes());
            e[10] = *ty;
            put(m, b, &e)?;
        }
        put_u32(m, nevents, events.len() as u32)
    }

    // ---------- descriptors ----------

    fn fd_read(&mut self, m: &mut [u8], fd: u32, iov: u32, n: u32, pos_at: Option<u64>, nread: u32) -> W<()> {
        let iov = iovs(m, iov, n)?;
        let want: usize = iov.iter().map(|x| x.1 as usize).sum();
        let data = match &self.fd(fd)?.kind {
            Kind::Stdin => {
                let end = (self.stdin_pos + want).min(self.stdin.len());
                let d = self.stdin[self.stdin_pos..end].to_vec();
                self.stdin_pos = end;
                d
            }
            Kind::Stdout | Kind::Stderr => return Err(BADF),
            _ => {
                let (path, pos, _, read, _) = self.file(fd)?;
                if !read {
                    return Err(BADF);
                }
                let at = pos_at.unwrap_or(pos);
                let d = self.vfs.read_at(&path, at, want).map_err(fe)?;
                if pos_at.is_none() {
                    self.set_pos(fd, at + d.len() as u64)?;
                }
                d
            }
        };
        let mut done = 0;
        for (buf, len) in iov {
            let k = (len as usize).min(data.len() - done);
            put(m, buf, &data[done..done + k])?;
            done += k;
            if done == data.len() {
                break;
            }
        }
        put_u32(m, nread, done as u32)
    }
    fn fd_write(&mut self, m: &mut [u8], fd: u32, iov: u32, n: u32, pos_at: Option<u64>, nwritten: u32) -> W<()> {
        let iov = iovs(m, iov, n)?;
        let mut data = Vec::new();
        for (buf, len) in iov {
            data.extend_from_slice(&m[range(m, buf, len)?]);
        }
        match &self.fd(fd)?.kind {
            Kind::Stdout => self.emit(false, &data),
            Kind::Stderr => self.emit(true, &data),
            Kind::Stdin => return Err(BADF),
            _ => {
                let (path, pos, append, _, write) = self.file(fd)?;
                if !write {
                    return Err(BADF);
                }
                let at = match (pos_at, append) {
                    (Some(p), _) => p,
                    (None, true) => self.vfs.peek(&path, true).map_or(0, |s| s.size),
                    (None, false) => pos,
                };
                self.vfs.write_at(&path, at, &data).map_err(fe)?;
                if pos_at.is_none() {
                    self.set_pos(fd, at + data.len() as u64)?;
                }
            }
        }
        put_u32(m, nwritten, data.len() as u32)
    }
    fn fd_seek(&mut self, m: &mut [u8], fd: u32, delta: i64, whence: u32, out: u32) -> W<()> {
        let (path, pos, ..) = self.file(fd)?;
        let base = match whence {
            0 => 0i128,
            1 => pos as i128,
            2 => self.vfs.peek(&path, true).map_or(0, |s| s.size) as i128,
            _ => return Err(INVAL),
        };
        let new = base + delta as i128;
        if new < 0 || new > i64::MAX as i128 {
            return Err(INVAL);
        }
        self.set_pos(fd, new as u64)?;
        put_u64(m, out, new as u64)
    }
    fn fd_fdstat_get(&self, m: &mut [u8], fd: u32, out: u32) -> W<()> {
        let f = self.fd(fd)?;
        let (ft, flags) = match &f.kind {
            Kind::Stdin | Kind::Stdout | Kind::Stderr => (FT_CHAR, 0u16),
            Kind::Dir { .. } => (FT_DIR, 0),
            Kind::File { append, .. } => (FT_FILE, if *append { 1 } else { 0 }),
        };
        let mut b = [0u8; 24];
        b[0] = ft;
        b[2..4].copy_from_slice(&flags.to_le_bytes());
        b[8..16].copy_from_slice(&f.rights.to_le_bytes());
        b[16..24].copy_from_slice(&f.inheriting.to_le_bytes());
        put(m, out, &b)
    }
    fn fd_filestat_get(&self, m: &mut [u8], fd: u32, out: u32) -> W<()> {
        match &self.fd(fd)?.kind {
            Kind::Stdin | Kind::Stdout | Kind::Stderr => put_filestat(m, out, 0, FT_CHAR, 0, 0, 0),
            Kind::Dir { path, .. } | Kind::File { path, .. } => {
                let s = self.vfs.peek(path, true).ok_or(NOENT)?;
                put_filestat(m, out, s.ino, ftype(s.kind), s.size, s.mtime, s.ctime)
            }
        }
    }
    fn fd_prestat_get(&self, m: &mut [u8], fd: u32, out: u32) -> W<()> {
        match &self.fd(fd)?.kind {
            Kind::Dir { preopen: Some(name), .. } => {
                put(m, out, &[0, 0, 0, 0])?;
                put_u32(m, off(out, 4)?, name.len() as u32)
            }
            _ => Err(BADF),
        }
    }
    fn fd_prestat_dir_name(&self, m: &mut [u8], fd: u32, buf: u32, len: u32) -> W<()> {
        match &self.fd(fd)?.kind {
            Kind::Dir { preopen: Some(name), .. } => {
                if (len as usize) < name.len() {
                    return Err(INVAL);
                }
                put(m, buf, name.as_bytes())
            }
            _ => Err(BADF),
        }
    }
    fn fd_readdir(&mut self, m: &mut [u8], fd: u32, buf: u32, len: u32, cookie: u64, used: u32) -> W<()> {
        let path = match &self.fd(fd)?.kind {
            Kind::Dir { path, .. } => path.clone(),
            _ => return Err(NOTDIR),
        };
        let r = range(m, buf, len)?;
        let list = self.vfs.read_dir(&path).map_err(fe)?;
        let self_ino = self.vfs.peek(&path, true).map_or(0, |s| s.ino);
        let mut entries: Vec<(String, u8, u64)> = vec![(".".into(), FT_DIR, self_ino), ("..".into(), FT_DIR, 0)];
        entries.extend(list.into_iter().map(|e| (e.name, ftype(e.kind), e.ino)));
        let mut out = Vec::new();
        for (i, (name, t, ino)) in entries.iter().enumerate().skip(cookie.min(usize::MAX as u64) as usize) {
            if out.len() >= len as usize {
                break;
            }
            out.extend_from_slice(&((i + 1) as u64).to_le_bytes());
            out.extend_from_slice(&ino.to_le_bytes());
            out.extend_from_slice(&(name.len() as u32).to_le_bytes());
            out.extend_from_slice(&[*t, 0, 0, 0]);
            out.extend_from_slice(name.as_bytes());
        }
        // an incomplete last entry is allowed: the guest sees bufused == buf_len and rereads with a bigger buffer
        let n = out.len().min(len as usize);
        m[r.start..r.start + n].copy_from_slice(&out[..n]);
        put_u32(m, used, n as u32)
    }

    // ---------- paths ----------

    #[allow(clippy::too_many_arguments)]
    fn path_open(
        &mut self,
        m: &mut [u8],
        dirfd: u32,
        dirflags: u32,
        p: u32,
        plen: u32,
        oflags: u32,
        rights: u64,
        inheriting: u64,
        fdflags: u32,
        out: u32,
    ) -> W<()> {
        let path = self.at(m, dirfd, p, plen)?;
        let follow = dirflags & 1 != 0;
        let (creat, directory, excl, trunc) = (oflags & 1 != 0, oflags & 2 != 0, oflags & 4 != 0, oflags & 8 != 0);
        let read = rights & RIGHT_FD_READ != 0;
        let write = rights & RIGHT_FD_WRITE != 0 || fdflags & FDFLAG_APPEND != 0;
        let r = (|| {
            let canon = self.vfs.resolve_path(&path, follow || creat).map_err(fe)?;
            match self.vfs.peek(&canon, false) {
                Some(s) if s.kind == FileType::Symlink => return Err(LOOP),
                Some(_) if creat && excl => return Err(EXIST),
                Some(_) => {}
                None if !creat => return Err(NOENT),
                None if directory => return Err(INVAL),
                None => self.vfs.create(&canon, false).map_err(fe)?,
            }
            let st = self.vfs.peek(&canon, false).ok_or(NOENT)?;
            let kind = if st.kind == FileType::Dir {
                if write {
                    return Err(ISDIR);
                }
                Kind::Dir { path: canon.clone(), preopen: None }
            } else {
                if directory {
                    return Err(NOTDIR);
                }
                self.vfs.check_access(&canon, read, write).map_err(fe)?;
                if trunc && write && st.size > 0 {
                    self.vfs.truncate(&canon, 0).map_err(fe)?;
                }
                Kind::File { path: canon.clone(), pos: 0, append: fdflags & FDFLAG_APPEND != 0, read, write }
            };
            Ok((canon, kind))
        })();
        let err = r.as_ref().err().map(|&e| FsError::new(vfs::Errno::from_wasi(e).unwrap_or(vfs::Errno::Io)));
        let jpath = r.as_ref().map_or(path.as_str(), |(c, _)| c.as_str()).to_string();
        self.vfs.journal.record(self.vfs.now(), Op::Open, &jpath, 0, err.as_ref());
        let (_, kind) = r?;
        let fd = self.alloc_fd(Fd { kind, rights, inheriting })?;
        put_u32(m, out, fd)
    }
    fn path_filestat_get(&mut self, m: &mut [u8], fd: u32, flags: u32, p: u32, len: u32, out: u32) -> W<()> {
        let path = self.at(m, fd, p, len)?;
        let s = if flags & 1 != 0 { self.vfs.stat(&path) } else { self.vfs.lstat(&path) }.map_err(fe)?;
        put_filestat(m, out, s.ino, ftype(s.kind), s.size, s.mtime, s.ctime)
    }
    fn set_times(&mut self, path: &str, mtim: u64, fst: u32) -> W<()> {
        let t = if fst & 8 != 0 {
            Some(self.vfs.now())
        } else if fst & 4 != 0 {
            Some(mtim)
        } else {
            None
        };
        match t {
            Some(t) => self.vfs.set_mtime(path, t).map_err(fe),
            None => self.vfs.stat(path).map(|_| ()).map_err(fe),
        }
    }
}

/// Bind all 46 functions of `wasi_snapshot_preview1` to the linker.
pub fn add_wasi(linker: &mut Linker<Ctx>) -> Result<(), String> {
    macro_rules! wasi {
        ($name:literal, |$ctx:ident, $m:ident $(, $a:ident : $t:ty)*| $body:expr) => {
            linker
                .func_wrap(MODULE, $name, |mut caller: Caller<'_, Ctx>, $($a: $t),*| -> i32 {
                    let Some(mem) = caller.get_export("memory").and_then(Extern::into_memory) else {
                        return FAULT as i32;
                    };
                    let ($m, $ctx) = mem.data_and_store_mut(&mut caller);
                    *$ctx.calls.entry($name).or_insert(0) += 1;
                    #[allow(clippy::redundant_closure_call)]
                    let r: W<()> = (|| -> W<()> { $body })();
                    match r {
                        Ok(()) => SUCCESS as i32,
                        Err(e) => e as i32,
                    }
                })
                .map_err(|e| format!("{}: {e}", $name))?;
        };
    }
    wasi!("args_sizes_get", |c, m, a: i32, b: i32| Ctx::strings_sizes(&c.args, m, a as u32, b as u32));
    wasi!("args_get", |c, m, a: i32, b: i32| Ctx::strings_get(&c.args, m, a as u32, b as u32));
    wasi!("environ_sizes_get", |c, m, a: i32, b: i32| Ctx::strings_sizes(&c.env, m, a as u32, b as u32));
    wasi!("environ_get", |c, m, a: i32, b: i32| Ctx::strings_get(&c.env, m, a as u32, b as u32));
    wasi!("clock_res_get", |_c, m, id: i32, out: i32| if id as u32 > 3 { Err(INVAL) } else { put_u64(m, out as u32, 1_000) });
    wasi!("clock_time_get", |c, m, id: i32, _prec: i64, out: i32| c.clock_time_get(m, id as u32, out as u32));
    wasi!("random_get", |c, m, buf: i32, len: i32| c.random_get(m, buf as u32, len as u32));
    wasi!("poll_oneoff", |c, m, i: i32, o: i32, n: i32, ne: i32| c.poll_oneoff(m, i as u32, o as u32, n as u32, ne as u32));
    wasi!("sched_yield", |_c, _m| Ok(()));
    wasi!("proc_raise", |_c, _m, _sig: i32| Err(NOTSUP));
    wasi!("fd_read", |c, m, fd: i32, iov: i32, n: i32, out: i32| c.fd_read(m, fd as u32, iov as u32, n as u32, None, out as u32));
    wasi!("fd_pread", |c, m, fd: i32, iov: i32, n: i32, at: i64, out: i32| c.fd_read(m, fd as u32, iov as u32, n as u32, Some(at as u64), out as u32));
    wasi!("fd_write", |c, m, fd: i32, iov: i32, n: i32, out: i32| c.fd_write(m, fd as u32, iov as u32, n as u32, None, out as u32));
    wasi!("fd_pwrite", |c, m, fd: i32, iov: i32, n: i32, at: i64, out: i32| c.fd_write(m, fd as u32, iov as u32, n as u32, Some(at as u64), out as u32));
    wasi!("fd_seek", |c, m, fd: i32, d: i64, wh: i32, out: i32| c.fd_seek(m, fd as u32, d, wh as u32, out as u32));
    wasi!("fd_tell", |c, m, fd: i32, out: i32| c.fd_seek(m, fd as u32, 0, 1, out as u32));
    wasi!("fd_close", |c, _m, fd: i32| {
        let slot = c.fds.get_mut(fd as u32 as usize).ok_or(BADF)?;
        if slot.take().is_none() { Err(BADF) } else { Ok(()) }
    });
    wasi!("fd_fdstat_get", |c, m, fd: i32, out: i32| c.fd_fdstat_get(m, fd as u32, out as u32));
    wasi!("fd_fdstat_set_flags", |c, _m, fd: i32, flags: i32| {
        match &mut c.fd_mut(fd as u32)?.kind {
            Kind::File { append, .. } => {
                *append = flags as u32 & FDFLAG_APPEND != 0;
                Ok(())
            }
            _ => Ok(()),
        }
    });
    wasi!("fd_fdstat_set_rights", |c, _m, fd: i32, base: i64, inh: i64| {
        let f = c.fd_mut(fd as u32)?;
        f.rights &= base as u64;
        f.inheriting &= inh as u64;
        Ok(())
    });
    wasi!("fd_filestat_get", |c, m, fd: i32, out: i32| c.fd_filestat_get(m, fd as u32, out as u32));
    wasi!("fd_filestat_set_size", |c, _m, fd: i32, size: i64| {
        let (path, _, _, _, write) = c.file(fd as u32)?;
        if !write { return Err(BADF); }
        c.vfs.truncate(&path, size as u64).map_err(fe)
    });
    wasi!("fd_filestat_set_times", |c, _m, fd: i32, _at: i64, mt: i64, fst: i32| {
        let path = match &c.fd(fd as u32)?.kind {
            Kind::Dir { path, .. } | Kind::File { path, .. } => path.clone(),
            _ => return Err(BADF),
        };
        c.set_times(&path, mt as u64, fst as u32)
    });
    wasi!("fd_advise", |c, _m, fd: i32, _o: i64, _l: i64, _a: i32| c.fd(fd as u32).map(|_| ()));
    wasi!("fd_allocate", |c, _m, fd: i32, o: i64, l: i64| {
        let (path, _, _, _, write) = c.file(fd as u32)?;
        if !write { return Err(BADF); }
        let end = (o as u64).checked_add(l as u64).ok_or(INVAL)?;
        let size = c.vfs.peek(&path, true).map_or(0, |s| s.size);
        if end > size { c.vfs.truncate(&path, end).map_err(fe) } else { Ok(()) }
    });
    wasi!("fd_datasync", |c, _m, fd: i32| c.fd(fd as u32).map(|_| ()));
    wasi!("fd_sync", |c, _m, fd: i32| c.fd(fd as u32).map(|_| ()));
    wasi!("fd_prestat_get", |c, m, fd: i32, out: i32| c.fd_prestat_get(m, fd as u32, out as u32));
    wasi!("fd_prestat_dir_name", |c, m, fd: i32, buf: i32, len: i32| c.fd_prestat_dir_name(m, fd as u32, buf as u32, len as u32));
    wasi!("fd_readdir", |c, m, fd: i32, buf: i32, len: i32, cookie: i64, used: i32| {
        c.fd_readdir(m, fd as u32, buf as u32, len as u32, cookie as u64, used as u32)
    });
    wasi!("fd_renumber", |c, _m, from: i32, to: i32| {
        let f = c.fd(from as u32)?.clone();
        c.fd(to as u32)?;
        c.fds[to as u32 as usize] = Some(f);
        c.fds[from as u32 as usize] = None;
        Ok(())
    });
    wasi!("path_open", |c, m, fd: i32, df: i32, p: i32, pl: i32, of: i32, rb: i64, ri: i64, ff: i32, out: i32| {
        c.path_open(m, fd as u32, df as u32, p as u32, pl as u32, of as u32, rb as u64, ri as u64, ff as u32, out as u32)
    });
    wasi!("path_create_directory", |c, m, fd: i32, p: i32, l: i32| {
        let path = c.at(m, fd as u32, p as u32, l as u32)?;
        c.vfs.mkdir(&path).map_err(fe)
    });
    wasi!("path_filestat_get", |c, m, fd: i32, fl: i32, p: i32, l: i32, out: i32| {
        c.path_filestat_get(m, fd as u32, fl as u32, p as u32, l as u32, out as u32)
    });
    wasi!("path_filestat_set_times", |c, m, fd: i32, _fl: i32, p: i32, l: i32, _at: i64, mt: i64, fst: i32| {
        let path = c.at(m, fd as u32, p as u32, l as u32)?;
        c.set_times(&path, mt as u64, fst as u32)
    });
    wasi!("path_link", |_c, _m, _a: i32, _b: i32, _p: i32, _l: i32, _d: i32, _q: i32, _k: i32| Err(NOTSUP));
    wasi!("path_readlink", |c, m, fd: i32, p: i32, l: i32, buf: i32, len: i32, used: i32| {
        let path = c.at(m, fd as u32, p as u32, l as u32)?;
        let t = c.vfs.read_link(&path).map_err(fe)?;
        let n = t.len().min(len as u32 as usize);
        put(m, buf as u32, &t.as_bytes()[..n])?;
        put_u32(m, used as u32, n as u32)
    });
    wasi!("path_remove_directory", |c, m, fd: i32, p: i32, l: i32| {
        let path = c.at(m, fd as u32, p as u32, l as u32)?;
        c.vfs.remove_dir(&path).map_err(fe)
    });
    wasi!("path_rename", |c, m, fd: i32, p: i32, l: i32, fd2: i32, q: i32, k: i32| {
        let from = c.at(m, fd as u32, p as u32, l as u32)?;
        let to = c.at(m, fd2 as u32, q as u32, k as u32)?;
        c.vfs.rename(&from, &to).map_err(fe)
    });
    wasi!("path_symlink", |c, m, t: i32, tl: i32, fd: i32, p: i32, l: i32| {
        let target = get_str(m, t as u32, tl as u32)?;
        let link = c.at(m, fd as u32, p as u32, l as u32)?;
        c.vfs.symlink(&target, &link).map_err(fe)
    });
    wasi!("path_unlink_file", |c, m, fd: i32, p: i32, l: i32| {
        let path = c.at(m, fd as u32, p as u32, l as u32)?;
        c.vfs.remove_file(&path).map_err(fe)
    });
    // no network
    wasi!("sock_accept", |_c, _m, _fd: i32, _f: i32, _o: i32| Err(NOTSUP));
    wasi!("sock_recv", |_c, _m, _fd: i32, _a: i32, _b: i32, _f: i32, _o: i32, _p: i32| Err(NOTSUP));
    wasi!("sock_send", |_c, _m, _fd: i32, _a: i32, _b: i32, _f: i32, _o: i32| Err(NOTSUP));
    wasi!("sock_shutdown", |_c, _m, _fd: i32, _h: i32| Err(NOTSUP));
    linker
        .func_wrap(MODULE, "proc_exit", |mut caller: Caller<'_, Ctx>, code: i32| -> Result<(), wasmi::Error> {
            caller.data_mut().exit_code = Some(code);
            *caller.data_mut().calls.entry("proc_exit").or_insert(0) += 1;
            Err(wasmi::Error::i32_exit(code))
        })
        .map_err(|e| format!("proc_exit: {e}"))?;
    Ok(())
}

/// A compiled module ready for many runs (each one is a fresh instance with its own [`Ctx`]).
pub struct Host {
    engine: Engine,
    module: Module,
    linker: Linker<Ctx>,
    /// Fuel ceiling (≈ wasmi instructions) per run; `None` means no ceiling.
    pub fuel: Option<u64>,
}

/// Run outcome.
pub struct Outcome {
    pub exit_code: i32,
    /// Guest trap: out-of-bounds memory, fuel, `unreachable`…
    pub trap: Option<String>,
    pub fuel_used: Option<u64>,
    pub ctx: Ctx,
}

impl Host {
    pub fn new(wasm: &[u8], fuel: Option<u64>) -> Result<Host, String> {
        let mut cfg = Config::default();
        cfg.consume_fuel(fuel.is_some());
        let engine = Engine::new(&cfg);
        let module = Module::new(&engine, wasm).map_err(|e| format!("WASM module: {e}"))?;
        let mut linker = Linker::new(&engine);
        add_wasi(&mut linker)?;
        Ok(Host { engine, module, linker, fuel })
    }

    /// Module imports the host does not provide (empty means all is fine).
    pub fn missing_imports(&self) -> Vec<String> {
        self.module
            .imports()
            .filter(|i| i.module() != MODULE || !is_wasi_name(i.name()))
            .map(|i| format!("{}::{}", i.module(), i.name()))
            .collect()
    }
    /// WASI functions the module imports.
    pub fn wasi_imports(&self) -> Vec<String> {
        self.module.imports().filter(|i| i.module() == MODULE).map(|i| i.name().to_string()).collect()
    }

    pub fn run(&self, ctx: Ctx) -> Outcome {
        let mut store = Store::new(&self.engine, ctx);
        store.limiter(|c| &mut c.limits);
        if let Some(f) = self.fuel {
            let _ = store.set_fuel(f);
        }
        let result = (|| -> Result<(), wasmi::Error> {
            let inst = self.linker.instantiate_and_start(&mut store, &self.module)?;
            let start = inst.get_typed_func::<(), ()>(&store, "_start")?;
            start.call(&mut store, ())
        })();
        let fuel_used = self.fuel.map(|f| f - store.get_fuel().unwrap_or(0));
        let (exit_code, trap) = match result {
            Ok(()) => (0, None),
            Err(e) => match e.i32_exit_status() {
                Some(c) => (c, None),
                None => (134, Some(e.to_string())),
            },
        };
        Outcome { exit_code, trap, fuel_used, ctx: store.into_data() }
    }
}

const WASI_NAMES: [&str; 46] = [
    "args_get", "args_sizes_get", "environ_get", "environ_sizes_get", "clock_res_get", "clock_time_get", "fd_advise",
    "fd_allocate", "fd_close", "fd_datasync", "fd_fdstat_get", "fd_fdstat_set_flags", "fd_fdstat_set_rights",
    "fd_filestat_get", "fd_filestat_set_size", "fd_filestat_set_times", "fd_pread", "fd_prestat_get",
    "fd_prestat_dir_name", "fd_pwrite", "fd_read", "fd_readdir", "fd_renumber", "fd_seek", "fd_sync", "fd_tell",
    "fd_write", "path_create_directory", "path_filestat_get", "path_filestat_set_times", "path_link", "path_open",
    "path_readlink", "path_remove_directory", "path_rename", "path_symlink", "path_unlink_file", "poll_oneoff",
    "proc_exit", "proc_raise", "sched_yield", "random_get", "sock_accept", "sock_recv", "sock_send", "sock_shutdown",
];

fn is_wasi_name(n: &str) -> bool {
    WASI_NAMES.contains(&n)
}

#[cfg(test)]
mod tests;
