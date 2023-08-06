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
r#"4 7
1 2
2 1
2 3
4 3
4 1
1 4
2 3
"#,
r#"3
"#);
}

#[test]
fn sample2() {
test(
r#"100 1
1 2
"#,
r#"0
"#);
}

#[test]
fn sample3() {
test(
r#"11 15
1 3
2 1
3 2
4 3
4 7
5 4
6 5
7 6
8 11
8 10
9 8
10 8
11 2
11 5
11 9
"#,
r#"15
"#);
}
}
