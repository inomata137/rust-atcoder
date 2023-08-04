use proconio::input;
use proconio::marker::Chars;

#[allow(unused)]
fn main() {
    input! {
        n: usize,
        d: usize,
        s: [Chars; n]
    }

    let mut ans = 0;
    let mut left = 0;
    for i in 0..d {
        let ok = s.iter().all(|x: &Vec<char>| { x[i] == 'o' });
        if !ok {
            left = i + 1;
            continue;
        }
        ans = ans.max(i - left + 1);
    }

    println!("{}", ans);
}
