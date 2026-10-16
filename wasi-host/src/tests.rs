//! Unit tests of WASI functions without the engine: guest memory is a plain byte array.

use super::*;

fn ctx() -> Ctx {
    let mut v = Vfs::new();
    v.mkdir("/d").unwrap();
    v.write("/d/a.txt", b"hello world").unwrap();
    v.write("/d/b.txt", b"").unwrap();
    Ctx::new(v, vec!["prog".into(), "arg1".into()])
}

fn put_str(m: &mut [u8], p: u32, s: &str) -> (u32, u32) {
    m[p as usize..p as usize + s.len()].copy_from_slice(s.as_bytes());
    (p, s.len() as u32)
}

/// Open a path from the preopened `/` (fd 3).
fn open(c: &mut Ctx, m: &mut [u8], path: &str, oflags: u32, rights: u64, fdflags: u32) -> W<u32> {
    let (p, l) = put_str(m, 1000, path);
    c.path_open(m, 3, 1, p, l, oflags, rights, 0, fdflags, 900)?;
    Ok(get_u32(m, 900).unwrap())
}

#[test]
fn args_and_prestat() {
    let c = ctx();
    let mut m = vec![0u8; 4096];
    Ctx::strings_sizes(&c.args, &mut m, 0, 4).unwrap();
    assert_eq!((get_u32(&m, 0).unwrap(), get_u32(&m, 4).unwrap()), (2, 10));
    Ctx::strings_get(&c.args, &mut m, 16, 64).unwrap();
    let p1 = get_u32(&m, 20).unwrap() as usize;
    assert_eq!(&m[p1..p1 + 5], b"arg1\0");
    c.fd_prestat_get(&mut m, 3, 100).unwrap();
    assert_eq!(get_u32(&m, 104).unwrap(), 1);
    c.fd_prestat_dir_name(&mut m, 3, 200, 1).unwrap();
    assert_eq!(m[200], b'/');
    assert_eq!(c.fd_prestat_get(&mut m, 4, 100), Err(BADF));
    // out-of-bounds memory is FAULT, not a panic
    assert_eq!(c.fd_prestat_get(&mut m, 3, 4095), Err(FAULT));
    assert_eq!(Ctx::strings_get(&c.args, &mut m, u32::MAX - 2, 0), Err(FAULT));
}

#[test]
fn open_read_seek_write() {
    let mut c = ctx();
    let mut m = vec![0u8; 8192];
    let fd = open(&mut c, &mut m, "d/a.txt", 0, RIGHT_FD_READ, 0).unwrap();
    // iovec: two buffers of 5 bytes
    put_u32(&mut m, 0, 2000).unwrap();
    put_u32(&mut m, 4, 5).unwrap();
    put_u32(&mut m, 8, 2100).unwrap();
    put_u32(&mut m, 12, 5).unwrap();
    c.fd_read(&mut m, fd, 0, 2, None, 16).unwrap();
    assert_eq!(get_u32(&m, 16).unwrap(), 10);
    assert_eq!(&m[2000..2005], b"hello");
    assert_eq!(&m[2100..2105], b" worl");
    c.fd_seek(&mut m, fd, -3, 2, 24).unwrap();
    assert_eq!(get_u64(&m, 24).unwrap(), 8);
    c.fd_read(&mut m, fd, 0, 1, None, 16).unwrap();
    assert_eq!((&m[2000..2003], get_u32(&m, 16).unwrap()), (&b"rld"[..], 3));
    assert_eq!(c.fd_seek(&mut m, fd, -100, 1, 24), Err(INVAL));
    // writing to a read-only descriptor is BADF
    assert_eq!(c.fd_write(&mut m, fd, 0, 1, None, 16), Err(BADF));
    // create, write with a hole via pwrite, append in APPEND mode
    let w = open(&mut c, &mut m, "d/new.bin", 1 | 8, RIGHT_FD_WRITE | RIGHT_FD_READ, 0).unwrap();
    put(&mut m, 3000, b"XY").unwrap();
    put_u32(&mut m, 40, 3000).unwrap();
    put_u32(&mut m, 44, 2).unwrap();
    c.fd_write(&mut m, w, 40, 1, Some(3), 48).unwrap();
    assert_eq!(c.vfs.read("/d/new.bin").unwrap(), b"\0\0\0XY");
    let a = open(&mut c, &mut m, "d/new.bin", 0, RIGHT_FD_WRITE, FDFLAG_APPEND).unwrap();
    c.fd_write(&mut m, a, 40, 1, None, 48).unwrap();
    assert_eq!(c.vfs.read("/d/new.bin").unwrap(), b"\0\0\0XYXY");
    // O_EXCL on an existing file, O_DIRECTORY on a file, a file without O_CREAT
    assert_eq!(open(&mut c, &mut m, "d/a.txt", 1 | 4, RIGHT_FD_WRITE, 0), Err(EXIST));
    assert_eq!(open(&mut c, &mut m, "d/a.txt", 2, RIGHT_FD_READ, 0), Err(NOTDIR));
    assert_eq!(open(&mut c, &mut m, "d/zzz", 0, RIGHT_FD_READ, 0), Err(NOENT));
    assert_eq!(open(&mut c, &mut m, "d", 0, RIGHT_FD_WRITE, 0), Err(ISDIR));
    // closing
    c.fds[fd as usize] = None;
    assert_eq!(c.fd_read(&mut m, fd, 0, 1, None, 16), Err(BADF));
}

#[test]
fn escape_and_absolute_paths_are_not_capable() {
    let mut c = ctx();
    let mut m = vec![0u8; 4096];
    assert_eq!(open(&mut c, &mut m, "../etc/passwd", 0, RIGHT_FD_READ, 0), Err(NOTCAPABLE));
    assert_eq!(open(&mut c, &mut m, "d/../../x", 1, RIGHT_FD_WRITE, 0), Err(NOTCAPABLE));
    assert_eq!(open(&mut c, &mut m, "/etc/passwd", 0, RIGHT_FD_READ, 0), Err(NOTCAPABLE));
    c.vfs.symlink("../../..", "/d/up").unwrap();
    assert_eq!(open(&mut c, &mut m, "d/up/x", 0, RIGHT_FD_READ, 0), Err(NOTCAPABLE));
    assert_eq!(c.vfs.journal.denied(), 4, "every attempt is in the journal, including the absolute path");
}

#[test]
fn readdir_with_small_buffer_and_cookies() {
    let mut c = ctx();
    let mut m = vec![0u8; 4096];
    let d = open(&mut c, &mut m, "d", 2, RIGHT_FD_READ, 0).unwrap();
    // the whole list
    c.fd_readdir(&mut m, d, 100, 1000, 0, 50).unwrap();
    let used = get_u32(&m, 50).unwrap() as usize;
    let mut names = Vec::new();
    let mut k = 100;
    while k < 100 + used {
        let n = get_u32(&m, k as u32 + 16).unwrap() as usize;
        names.push(String::from_utf8(m[k + 24..k + 24 + n].to_vec()).unwrap());
        k += 24 + n;
    }
    assert_eq!(names, [".", "..", "a.txt", "b.txt"]);
    // buffer too small: bufused == buf_len (the guest rereads)
    c.fd_readdir(&mut m, d, 100, 30, 0, 50).unwrap();
    assert_eq!(get_u32(&m, 50).unwrap(), 30);
    // from cookie = 3 — only b.txt
    c.fd_readdir(&mut m, d, 100, 1000, 3, 50).unwrap();
    assert_eq!(get_u32(&m, 50).unwrap(), 24 + 5);
    assert_eq!(&m[124..129], b"b.txt");
}

#[test]
fn clock_is_virtual_and_random_is_seeded() {
    let mut c = ctx();
    let mut m = vec![0u8; 256];
    c.clock_time_get(&mut m, 0, 0).unwrap();
    c.clock_time_get(&mut m, 0, 8).unwrap();
    let (t0, t1) = (get_u64(&m, 0).unwrap(), get_u64(&m, 8).unwrap());
    assert!(t0 >= vfs::EPOCH_NS && t1 == t0 + 1_000);
    c.clock_time_get(&mut m, 1, 16).unwrap();
    assert!(get_u64(&m, 16).unwrap() < 1_000_000_000);
    assert_eq!(c.clock_time_get(&mut m, 9, 0), Err(INVAL));
    c.random_get(&mut m, 100, 20).unwrap();
    let r1 = m[100..120].to_vec();
    let mut c2 = ctx();
    let mut m2 = vec![0u8; 256];
    c2.random_get(&mut m2, 100, 20).unwrap();
    assert_eq!(r1, m2[100..120].to_vec(), "same seed — same bytes");
    let mut c3 = ctx().with_seed(7);
    c3.random_get(&mut m2, 100, 20).unwrap();
    assert_ne!(r1, m2[100..120].to_vec());
    // sleeping via poll_oneoff only advances the clock
    let before = c.vfs.now();
    let sub = 200u32;
    m[sub as usize..sub as usize + 48].fill(0);
    put_u64(&mut m, sub + 24, 5_000_000_000).unwrap(); // 5 s, relative
    c.poll_oneoff(&mut m, sub, 60, 1, 56).unwrap();
    assert_eq!(get_u32(&m, 56).unwrap(), 1);
    assert_eq!(c.vfs.now(), before + 5_000_000_000);
}

#[test]
fn limits_reach_the_guest_as_errno() {
    let mut v = Vfs::new().with_limits(vfs::Limits { max_total_bytes: 1 << 20, max_files: 10, max_file_size: 4 });
    v.mkdir("/d").unwrap();
    let mut c = Ctx::new(v, vec![]);
    let mut m = vec![0u8; 4096];
    let w = open(&mut c, &mut m, "d/f", 1, RIGHT_FD_WRITE, 0).unwrap();
    put(&mut m, 3000, b"12345").unwrap();
    put_u32(&mut m, 40, 3000).unwrap();
    put_u32(&mut m, 44, 5).unwrap();
    assert_eq!(c.fd_write(&mut m, w, 40, 1, None, 48), Err(vfs::Errno::FBig.wasi()));
    assert_eq!(c.vfs.journal.limited(), 1);
}
