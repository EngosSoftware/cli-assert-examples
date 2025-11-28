#[test]
fn cargo_package_should_work() {
  assert_eq!("example-000", cli_assert::cargo_package!());
}

#[test]
fn cargo_manifest_dir_should_work() {
  let ending = "cli-assert-examples";
  assert!(cli_assert::cargo_manifest_dir!().ends_with(ending));
}

#[test]
fn asserting_stdout_should_work() {
  cli_assert::command!().stdout("Welcome to Example-000.").execute();
  cli_assert::command!().stdout(b"Welcome to Example-000.").execute();
  cli_assert::command!()
    .stdout([
      87, 101, 108, 99, 111, 109, 101, 32, 116, 111, 32, 69, 120, 97, 109, 112, 108, 101, 45, 48, 48, 48, 46,
    ])
    .execute();
}

#[test]
fn asserting_stderr_should_work() {
  cli_assert::command!().stderr("").execute();
  cli_assert::command!().stderr(b"").execute();
  cli_assert::command!().stderr([]).execute();
}

#[test]
fn executing_any_command_should_work() {
  cli_assert::cmd!("echo").execute();
  cli_assert::cmd!("echo").stderr([]).execute();
  cli_assert::cmd!("echo").stdout("\n").execute();
}

#[test]
fn executing_any_command_with_arguments_should_work() {
  let cmd = cli_assert::cmd!("echo");
  assert_eq!("echo", cmd.get_program());
  cmd.arg("hello").stdout("hello\n").stderr([]).execute();
}
