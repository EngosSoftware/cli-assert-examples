use cli_assert::Command;

#[should_panic(expected = "command is already spawned")]
#[test]
fn _0001() {
  let mut command = cli_assert::cmd!("echo").arg("hello").stdout("hello\n");
  command.spawn();
  command.spawn();
}

#[test]
fn _0002() {
  let mut command = cli_assert::cmd!("echo").arg("hello").stdout("hello\n");
  command.execute();
  command.execute();
  command.execute();
}

#[should_panic(expected = "command is already spawned")]
#[test]
fn _0003() {
  let mut command = cli_assert::cmd!("echo").arg("hello").stdout("hello\n");
  command.spawn();
  command.execute();
}

#[test]
fn _0004() {
  let mut command = cli_assert::cmd!("echo").arg("hello").stdout("hello\n");
  command.execute();
  command.spawn();
  command.wait();
  command.execute();
}

#[should_panic(expected = "command is not spawned")]
#[test]
fn _0005() {
  let mut command = cli_assert::cmd!("echo").arg("hello").stdout("hello\n");
  command.stop();
}

#[should_panic(expected = "command is not spawned")]
#[test]
fn _0006() {
  let mut command = cli_assert::cmd!("echo").arg("hello").stdout("hello\n");
  command.wait();
}

#[test]
fn _0007() {
  let mut command = cli_assert::cmd!("echo").arg("hello").stdout("hello\n");
  command.spawn();
  cli_assert::sleep(100);
  command.stop();
  command.wait();
}

#[test]
fn _0008() {
  Command::new("echo", "./test_command.rs", ".")
    .arg("hello")
    .stdout("hello\n");
}

#[should_panic(expected = "failed to retrieve parent directory for caller file")]
#[test]
fn _0009() {
  Command::new("echo", "/", "").arg("hello").stdout("hello\n");
}

#[should_panic(
  expected = "failed to spawn requested command: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }"
)]
#[test]
fn _0010() {
  Command::new("kuku", "./test_command.rs", ".").execute();
}
