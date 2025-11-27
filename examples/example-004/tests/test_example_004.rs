#[test]
fn starting_server_should_work() {
  let mut command = cli_assert::command!()
    .arg("3001")
    .stdout("started server at address 0.0.0.0:3001\n");
  command.spawn();
  cli_assert::sleep(100);
  command.stop();
  command.wait();
}

#[test]
fn address_in_use_should_work() {
  let mut server_1 = cli_assert::command!()
    .arg("3000")
    .stdout("started server at address 0.0.0.0:3000\n");
  server_1.spawn();
  cli_assert::sleep(100);

  cli_assert::command!()
    .arg("3000")
    .stderr("address 0.0.0.0:3000 is already in use\n")
    .execute();

  server_1.stop();
  server_1.wait();
}
