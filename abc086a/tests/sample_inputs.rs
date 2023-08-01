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
r#"3 4
"#,
"Even\n");
}

#[test]
fn sample2() {
test(
r#"1 21
"#,
"Odd\n");
}

// #[test]
// fn sample3() {
// test(
// r#"2
// 5 1 1
// 100 1 1
// "#,
// "No\n");
// }
}
