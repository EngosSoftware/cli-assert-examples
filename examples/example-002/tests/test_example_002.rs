#[test]
fn cargo_package_should_work() {
  assert_eq!("example-002", cli_assert::cargo_package!());
}

#[test]
fn cargo_binary_should_work() {
  assert!(cli_assert::cargo_binary!("beispiel-002").ends_with("beispiel-002"));
}

#[test]
fn reading_stdout_should_work() {
  let mut command = cli_assert::command!("beispiel-002");
  command.execute();
  assert_eq!("Welcome to Example-002", command.get_stdout());
}

#[test]
fn reading_stderr_should_work() {
  let mut command = cli_assert::command!("beispiel-002");
  command.execute();
  assert_eq!("Willkommen beim Beispiel-002", command.get_stderr());
}

#[test]
fn reading_status_should_work() {
  let mut command = cli_assert::command!("beispiel-002");
  command.execute();
  assert_eq!(1, command.get_status().code().unwrap());
}

#[test]
#[should_panic(expected = "expected success")]
fn success_assertion_should_fail() {
  let mut command = cli_assert::command!("beispiel-002").success();
  command.execute();
}

#[test]
fn failure_assertion_should_work() {
  let mut command = cli_assert::command!("beispiel-002").failure();
  command.execute();
}

#[test]
fn expected_status_code_should_work() {
  let mut command = cli_assert::command!("beispiel-002").code(1);
  command.execute();
}

#[test]
#[should_panic(expected = "\nexpected status code: 0\n  actual status code: 1")]
fn unexpected_status_code_should_fail() {
  let mut command = cli_assert::command!("beispiel-002").code(0);
  command.execute();
}

#[test]
fn expected_stderr_should_work() {
  let mut command = cli_assert::command!("beispiel-002").stderr("Willkommen beim Beispiel-002");
  command.execute();
}

#[test]
#[should_panic(
  expected = "\nexpected stderr: [87, 101, 108, 99, 111, 109, 101, 32, 116, 111, 32, 69, 120, 97, 109, 112, 108, 101, 45, 48, 48, 50, 46]\n  actual stderr: [87, 105, 108, 108, 107, 111, 109, 109, 101, 110, 32, 98, 101, 105, 109, 32, 66, 101, 105, 115, 112, 105, 101, 108, 45, 48, 48, 50]"
)]
fn unexpected_stderr_should_work() {
  let mut command = cli_assert::command!("beispiel-002").stderr("Welcome to Example-002.");
  command.execute();
}
