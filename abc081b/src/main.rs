use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans = std::u64::MAX;
    for _ in 0..n {
        input! {mut a: u64}
        let mut b = 0u64;
        while a % 2 == 0 && b < ans {
            a /= 2;
            b += 1;
        }
        ans = ans.min(b);
    }
    println!("{}", ans);
}
