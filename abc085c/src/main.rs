use proconio::input;

fn main() {
    input! {
        n: isize,
        y: isize
    }
    let mut a: isize = -1;
    let mut b: isize = -1;
    let mut c: isize = -1;

    'all: for ai in 0..=(y / 10000) {
        let left = y - 10000 * ai;
        let bmax = (left / 5000).min(n - ai);
        for bi in 0..=bmax {
            let ci = n - ai - bi;
            if ai == 14 && bi == 27 && ci == 959 {
                println!("{}", 5000 * bi + 1000 * ci == left);
            }
            if 5000 * bi + 1000 * ci == left {
                a = ai;
                b = bi;
                c = ci;
                break 'all;
            }
        }
    }
    println!("{} {} {}", a, b, c);
}
