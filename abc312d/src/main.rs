use proconio::input;
fn main() {
    input! {
        s: String
    }
    let mut v = vec![0u64; 3000];
    v[0] = 1;
    for ch in s.into_bytes() {
        match ch {
            b'(' => {
                for i in 0..2999 {
                    v[2999 - i] = v[2998 - i];
                }
                v[0] = 0;
            },
            b')' => {
                for i in 0..2999 {
                    v[i] = v[i + 1];
                }
            },
            b'?' => {
                let c = v.clone();
                for i in 0..3000 {
                    let a = if i == 0 {0} else {c[i-1]};
                    let b = if i == 2999 {0} else {c[i+1]};
                    v[i] = (a + b) % 998244353;
                }
            }
            _ => {}
        }
    }
    let ans = v[0];
    println!("{}", ans);
}
