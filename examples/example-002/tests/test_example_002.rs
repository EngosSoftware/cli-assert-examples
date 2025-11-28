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
  assert_eq!(b"Willkommen beim Beispiel-002", command.get_stderr_raw());
  assert_eq!(
    vec![
      87, 105, 108, 108, 107, 111, 109, 109, 101, 110, 32, 98, 101, 105, 109, 32, 66, 101, 105, 115, 112, 105, 101,
      108, 45, 48, 48, 50
    ],
    command.get_stderr_raw()
  );
}

#[test]
fn reading_status_should_work() {
  let mut command = cli_assert::command!("beispiel-002");
  command.execute();
  assert_eq!(1, command.get_status().code().unwrap());
}

#[should_panic(expected = "expected success")]
#[test]
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

#[should_panic(expected = "unexpected status")]
#[test]
fn unexpected_status_code_should_fail() {
  let mut command = cli_assert::command!("beispiel-002").code(0);
  command.execute();
}

#[test]
fn expected_stderr_should_work() {
  let mut command = cli_assert::command!("beispiel-002").stderr("Willkommen beim Beispiel-002");
  command.execute();
}

#[should_panic(expected = "unexpected stderr")]
#[test]
fn unexpected_stderr_should_work() {
  let mut command = cli_assert::command!("beispiel-002").stderr("Welcome to Example-002.");
  command.execute();
}
