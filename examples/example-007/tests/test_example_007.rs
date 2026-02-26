use cli_assert::{and, contains, eq, ge, gt, le, lt, ne, not, or};

fn eq_true(_: i32) -> bool {
  true
}

fn eq_false(_: i32) -> bool {
  false
}

#[test]
fn eq_predicate_for_code_should_pass() {
  cli_assert::command!().code_fn(eq(0)).execute();
  cli_assert::command!().code_fn(|actual| actual == 0).execute();
  cli_assert::command!().code_fn(eq_true).execute();
  cli_assert::command!().code_fn(and(eq(0), ne(1))).execute();
  cli_assert::command!().code_fn(or(eq(0), eq(1))).execute();
  cli_assert::command!().code_fn(or(eq(1), eq(0))).execute();
  cli_assert::command!().code_fn(not(eq(1))).execute();
  cli_assert::command!()
    .stdout_fn(eq("Welcome to Example-007.".into()))
    .execute();
  cli_assert::command!()
    .stderr_fn(eq("Willkommen beim Beispiel-007".into()))
    .execute();
}

#[should_panic(expected = "unexpected status code")]
#[test]
fn eq_predicate_for_code_should_fail() {
  cli_assert::command!().code_fn(eq(1)).execute();
}

#[should_panic(expected = "unexpected status code")]
#[test]
fn eq_predicate_for_code_should_fail_also() {
  cli_assert::command!().code_fn(eq_false).execute();
}

#[test]
fn ne_predicate_for_code_should_pass() {
  cli_assert::command!().code_fn(ne(1)).execute();
}

#[should_panic(expected = "unexpected status code")]
#[test]
fn ne_predicate_for_code_should_fail() {
  cli_assert::command!().code_fn(ne(0)).execute();
  cli_assert::command!()
    .stdout_fn(ne("Willkommen beim Beispiel-007".into()))
    .execute();
  cli_assert::command!()
    .stderr_fn(ne("Welcome to Example-007.".into()))
    .execute();
}

#[test]
fn le_predicate_for_code_should_pass() {
  cli_assert::command!().code_fn(le(1)).execute();
  cli_assert::command!().code_fn(le(0)).execute();
}

#[should_panic(expected = "unexpected status code")]
#[test]
fn le_predicate_for_code_should_fail() {
  cli_assert::command!().code_fn(le(-1)).execute();
}

#[test]
fn lt_predicate_for_code_should_pass() {
  cli_assert::command!().code_fn(lt(1)).execute();
}

#[should_panic(expected = "unexpected status code")]
#[test]
fn lt_predicate_for_code_should_fail() {
  cli_assert::command!().code_fn(lt(0)).execute();
}

#[test]
fn ge_predicate_for_code_should_pass() {
  cli_assert::command!().code_fn(ge(-1)).execute();
  cli_assert::command!().code_fn(ge(0)).execute();
  cli_assert::command!().stdout_fn(gt("".into())).execute();
  cli_assert::command!().stderr_fn(gt("".into())).execute();
}

#[should_panic(expected = "unexpected status code")]
#[test]
fn ge_predicate_for_code_should_fail() {
  cli_assert::command!().code_fn(ge(1)).execute();
}

#[test]
fn gt_predicate_for_code_should_pass() {
  cli_assert::command!().code_fn(gt(-1)).execute();
  cli_assert::command!().stdout_fn(gt("".into())).execute();
  cli_assert::command!().stderr_fn(gt("".into())).execute();
}

#[should_panic(expected = "unexpected status code")]
#[test]
fn gt_predicate_for_code_should_fail() {
  cli_assert::command!().code_fn(gt(0)).execute();
}

#[test]
fn contains_predicate_for_stdout_should_pass() {
  cli_assert::command!().stdout_fn(contains("Welcome".into())).execute();
}

#[should_panic(expected = "unexpected stdout")]
#[test]
fn contains_predicate_for_stdout_should_fail() {
  cli_assert::command!()
    .stdout_fn(contains("Willkommen".into()))
    .execute();
}

#[test]
fn contains_predicate_for_stderr_should_pass() {
  cli_assert::command!()
    .stderr_fn(contains("Willkommen".into()))
    .execute();
}

#[should_panic(expected = "unexpected stderr")]
#[test]
fn contains_predicate_for_stderr_should_fail() {
  cli_assert::command!().stderr_fn(contains("Welcome".into())).execute();
}
