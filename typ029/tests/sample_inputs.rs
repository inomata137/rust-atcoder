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
r#"100 4
27 100
8 39
83 97
24 75
"#,
r#"1
2
2
3
"#);
}

#[test]
fn sample2() {
test(
r#"3 5
1 2
2 2
2 3
3 3
1 2
"#,
r#"1
2
3
4
4
"#);
}

#[test]
fn sample3() {
test(
r#"10 10
1 3
3 5
5 7
7 9
2 4
4 6
6 8
3 5
5 7
4 6
"#,
r#"1
2
3
4
3
4
5
5
6
7
"#);
}
}
