use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
        b: [u64; m]
    }
    let mut l = 0u64;
    let mut r = 1_000_000_001u64;
    while r - l > 1 {
        let m = (r + l) / 2;
        if check(m, &a, &b) {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{}", r);
}

fn check(x: u64, a: &Vec<u64>, b: &Vec<u64>) -> bool {
    let ac = a.iter().filter(|v| { **v <= x }).count();
    let bc = b.iter().filter(|v| { **v >= x }).count();
    return ac >= bc;
}