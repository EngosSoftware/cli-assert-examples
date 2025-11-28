use cli_assert::PathExt;
use std::path::Path;
use std::time::Instant;

#[test]
fn _0001() {
  let start = Instant::now();
  cli_assert::sleep(0);
  let duration = start.elapsed();
  assert!(duration.as_nanos() < 1_000);
}

#[test]
fn _0002() {
  let start = Instant::now();
  cli_assert::sleep(1);
  let duration = start.elapsed();
  println!("{}", duration.as_nanos());
  assert!(duration.as_nanos() > 1_000_000);
}

#[test]
fn _0003() {
  let p = Path::new("/a/b/c/d");
  let s = Path::new("e/f");
  assert_eq!(None, p.rem(s));
}

#[test]
fn _0004() {
  let p = Path::new("/a/b/c/d");
  let s = Path::new("d/e/f");
  assert_eq!("e/f", p.rem(s).unwrap().to_string_lossy());
}

#[test]
fn _0005() {
  let p = Path::new("a/b/c/d/e");
  let s = Path::new("d/e/f");
  assert_eq!("f", p.rem(s).unwrap().to_string_lossy());
}

#[test]
fn _0006() {
  let p = Path::new("a/b/c");
  let s = Path::new("e/f");
  assert_eq!(None, p.rem(s));
}

#[test]
fn _0007() {
  let p = Path::new("a/b/c");
  let s = Path::new("a/b/c");
  assert_eq!("", p.rem(s).unwrap().to_string_lossy());
}

#[test]
fn _0008() {
  let p = Path::new("a/b/c");
  let s = Path::new("b/c/d");
  assert_eq!("d", p.rem(s).unwrap().to_string_lossy());
}

#[test]
fn _0009() {
  let p = Path::new("a/b/c");
  let s = Path::new("");
  assert_eq!(None, p.rem(s));
}

#[test]
fn _0010() {
  let p = Path::new("");
  let s = Path::new("a/b/c");
  assert_eq!(None, p.rem(s));
}
