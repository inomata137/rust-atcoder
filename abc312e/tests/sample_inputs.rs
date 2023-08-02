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
r#"4
0 0 0 1 1 1
0 0 1 1 1 2
1 1 1 2 2 2
3 3 3 4 4 4
"#,
r#"1
1
0
0
"#);
}

#[test]
fn sample2() {
test(
r#"3
0 0 10 10 10 20
3 4 1 15 6 10
0 9 6 1 20 10
"#,
r#"2
1
1
"#);
}

#[test]
fn sample3() {
test(
r#"8
0 0 0 1 1 1
0 0 1 1 1 2
0 1 0 1 2 1
0 1 1 1 2 2
1 0 0 2 1 1
1 0 1 2 1 2
1 1 0 2 2 1
1 1 1 2 2 2
"#,
r#"3
3
3
3
3
3
3
3
"#);
}
}
