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
r#"2
3 1 2
6 1 1
"#,
r#"Yes
"#);
}

#[test]
fn sample2() {
test(
r#"1
2 100 100
"#,
r#"No
"#);
}

#[test]
fn sample3() {
test(
r#"2
5 1 1
100 1 1
"#,
r#"No
"#);
}
}
