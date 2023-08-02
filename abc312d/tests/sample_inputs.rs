use cli_test_dir::*;

const BIN: &'static str = "./main";

fn test(input: &str, answer: &str) {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), answer);
    assert!(output.stderr_str().is_empty());
}

mod test {
use crate::test;

#[test]
fn sample1() {
test(
r#"(???(?
"#,
"2\n");
}

#[test]
fn sample2() {
test(
r#")))))
"#,
"0\n");
}

#[test]
fn sample3() {
test(
r#"??????????????(????????(??????)?????????(?(??)
"#,
"603032273\n");
}
}
