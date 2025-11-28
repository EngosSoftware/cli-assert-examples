#[test]
fn cargo_package_should_work() {
  assert_eq!("example-003", cli_assert::cargo_package!());
}

#[test]
fn cargo_binary_should_work() {
  assert!(cli_assert::cargo_binary!().ends_with("example-003"));
}

#[test]
fn reading_stdout_should_work() {
  let mut command = cli_assert::command!();
  command.execute();
  assert_eq!("", command.get_stdout());
}

#[test]
fn reading_stderr_should_work() {
  let mut command = cli_assert::command!();
  command.execute();
  assert_eq!("", command.get_stderr());
}

#[test]
fn reading_status_should_work() {
  let mut command = cli_assert::command!();
  command.execute();
  assert_eq!(0, command.get_status().code().unwrap());
}

#[test]
fn success_assertion_should_fail() {
  let mut command = cli_assert::command!().success();
  command.execute();
}

#[test]
#[should_panic(expected = "expected failure")]
fn failure_assertion_should_fail() {
  let mut command = cli_assert::command!().failure();
  command.execute();
}

#[test]
fn expected_status_code_should_work() {
  let mut command = cli_assert::command!().code(0);
  command.execute();
}

#[should_panic(expected = "unexpected status")]
#[test]
fn unexpected_status_code_should_fail() {
  let mut command = cli_assert::command!().code(1);
  command.execute();
}

#[test]
fn expected_stdout_should_work() {
  let mut command = cli_assert::command!().stdout("");
  command.execute();
}

#[test]
fn expected_stderr_should_work() {
  let mut command = cli_assert::command!().stderr("");
  command.execute();
}

#[should_panic(expected = "unexpected stdout")]
#[test]
fn unexpected_stdout_should_fail() {
  let mut command = cli_assert::command!().stdout("@");
  command.execute();
}

#[should_panic(expected = "unexpected stderr")]
#[test]
fn unexpected_stderr_should_fail() {
  let mut command = cli_assert::command!().stderr("@");
  command.execute();
}

#[test]
fn arguments_should_work() {
  cli_assert::command!().arg("a").code(1).execute();
  cli_assert::command!().arg("a").arg("b").code(2).execute();
  cli_assert::command!().arg("a").arg("b").arg("c").code(3).execute();
}
