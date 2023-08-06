#[allow(unused)]
use proconio::{input, marker::{Chars, Usize1}};
#[allow(unused)]
use std::collections::*;
#[allow(unused)]
const USIZE_MAX: usize = 18_446_744_073_709_551_615usize;

#[allow(unused)]
fn main() {
    input! {
        w: usize,
        n: usize,
    }
    let mut ans = vec![1; n];
    let mut bricks = Vec::with_capacity(n);
    for i in 0..n {
        input! {
            l: Usize1,
            r: Usize1,
        }
        for ip in 0..i {
            let (lp, rp) = bricks[ip];
            if lp > r || rp < l {
                continue;
            }
            ans[i] = ans[i].max(ans[ip] + 1);
        }
        bricks.push((l, r));
    }

    for a in ans {
        println!("{}", a);
    }
}
