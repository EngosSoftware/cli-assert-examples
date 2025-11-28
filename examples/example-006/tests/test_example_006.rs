#[test]
fn cargo_package_should_work() {
  assert_eq!("example-006", cli_assert::cargo_package!());
}

#[test]
fn reading_temporary_file_should_work() {
  let file = cli_assert::TmpFile::new("text.md");
  assert!(file.path().to_string_lossy().ends_with("text.md"));
  let text = "Hello Example-006!";
  file.write(text);
  cli_assert::command!().current_dir(file.dir()).stdout(text).execute();
  file.assert("Willkommen beim Beispiel-006!");
}

#[should_panic(expected = "unexpected content")]
#[test]
fn asserting_temporary_file_should_fail() {
  let file = cli_assert::TmpFile::new("text.md");
  let text = "Hello Example-006!";
  file.write(text);
  cli_assert::command!().current_dir(file.dir()).stdout(text).execute();
  file.assert(text);
}

#[test]
fn writing_read_only_temporary_file_should_fail() {
  let file = cli_assert::TmpFile::new("text.md");
  assert!(file.path().to_string_lossy().ends_with("text.md"));
  let text = "Hello Example-006!";
  file.write(text);
  file.set_readonly(true);
  cli_assert::command!()
    .current_dir(file.dir())
    .stdout(text)
    .stderr("Permission denied (os error 13)\n")
    .execute();
  file.assert(text);
}
