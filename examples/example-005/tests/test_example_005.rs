#[test]
fn writing_stdin_should_work() {
  cli_assert::command!()
    .stdin("Hello fron example 005!\n")
    .stdout("Hello fron example 005!\n")
    .execute();
}

#[test]
fn writing_stdin_again_should_work() {
  cli_assert::command!()
    .stdin("Hello fron example 005 once again!\n")
    .stdout("Hello fron example 005 once again!\n")
    .execute();
}
