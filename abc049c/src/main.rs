use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let ans = t(&s);
    println!("{}", ans);
}

fn t(s: &str) -> String {
    let l = s.len();
    if l == 0 {
        return String::from("YES");
    }
    if s.ends_with("dream") {
        return t(&s[..l-5]);
    } else if s.ends_with("dreamer") {
        return t(&s[..l-7]);
    } else if s.ends_with("erase") {
        return t(&s[..l-5]);
    } else if s.ends_with("eraser") {
        return t(&s[..l-6]);
    }
    return String::from("NO");
}
