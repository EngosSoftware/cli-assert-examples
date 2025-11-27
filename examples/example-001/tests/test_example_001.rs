#[test]
fn cargo_package_should_work() {
  assert_eq!("example-001", cli_assert::cargo_package!());
}

#[test]
fn cargo_dir_should_work() {
  let ending = "cli-assert-examples/examples/example-001";
  assert!(cli_assert::cargo_manifest_dir!().ends_with(ending));
}

#[test]
fn cargo_binary_should_work() {
  let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or("target".to_string());
  let ending = format!("/cli-assert-examples/{}/debug/example-001", target_dir);
  assert!(cli_assert::cargo_binary!().ends_with(&ending));
}

#[test]
fn reading_stdout_should_work() {
  let mut command = cli_assert::command!();
  command.spawn();
  command.wait();
  assert_eq!("Welcome to Example-001.", command.get_stdout());
}

#[test]
fn reading_status_should_work() {
  let mut command = cli_assert::command!();
  command.spawn();
  command.wait();
  assert!(command.get_status().success());
}

#[test]
fn reading_current_sir_should_work() {
  let mut command = cli_assert::command!();
  command.spawn();
  command.wait();
  assert_eq!("tests", command.get_current_dir());
}

#[test]
fn success_assertion_should_fail() {
  let mut command = cli_assert::command!().success();
  command.execute();
}

#[test]
#[should_panic(expected = "expected failure")]
fn failure_assertion_should_work() {
  let mut command = cli_assert::command!().failure();
  command.execute();
}

#[test]
fn expected_status_code_should_work() {
  let mut command = cli_assert::command!().code(0);
  command.execute();
}

#[test]
#[should_panic(expected = "\nexpected status code: 1\n  actual status code: 0")]
fn unexpected_status_code_should_fail() {
  let mut command = cli_assert::command!().code(1);
  command.execute();
}

#[test]
fn expected_stdout_should_work() {
  let mut command = cli_assert::command!().stdout("Welcome to Example-001.");
  command.execute();
}

#[test]
#[should_panic(
  expected = "\nexpected stdout: [87, 101, 108, 99, 111, 109, 101, 32, 116, 111, 32, 69, 120, 97, 109, 112, 108, 101, 46]\n  actual stdout: [87, 101, 108, 99, 111, 109, 101, 32, 116, 111, 32, 69, 120, 97, 109, 112, 108, 101, 45, 48, 48, 49, 46]"
)]
fn unexpected_stdout_should_work() {
  let mut command = cli_assert::command!().stdout("Welcome to Example.");
  command.execute();
}
