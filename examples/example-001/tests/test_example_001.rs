#[test]
fn cargo_package_should_work() {
  assert_eq!("example-001", cli_assert::cargo_package!());
}

#[test]
fn cargo_manifest_dir_should_work() {
  let ending = "cli-assert-examples/examples/example-001";
  assert!(cli_assert::cargo_manifest_dir!().ends_with(ending));
}

#[test]
fn cargo_binary_should_work() {
  assert!(cli_assert::cargo_binary!().ends_with("example-001"));
}

#[test]
fn reading_stdout_should_work() {
  let mut command = cli_assert::command!();
  command.spawn();
  command.wait();
  assert_eq!("Welcome to Example-001.", command.get_stdout());
  assert_eq!(b"Welcome to Example-001.", command.get_stdout_raw());
  assert_eq!(
    vec![87, 101, 108, 99, 111, 109, 101, 32, 116, 111, 32, 69, 120, 97, 109, 112, 108, 101, 45, 48, 48, 49, 46],
    command.get_stdout_raw()
  );
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
fn success_assertion_should_work() {
  let mut command = cli_assert::command!().success();
  command.execute();
}

#[should_panic(expected = "expected failure")]
#[test]
fn failure_assertion_should_work() {
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
  let mut command = cli_assert::command!().stdout("Welcome to Example-001.");
  command.execute();
}

#[should_panic(expected = "unexpected stdout")]
#[test]
fn unexpected_stdout_should_work() {
  let mut command = cli_assert::command!().stdout("Welcome to Example.");
  command.execute();
}
