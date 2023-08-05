#[allow(unused)]
use proconio::{input, marker::{Chars, Usize1}};
#[allow(unused)]
use std::collections::*;
#[allow(unused)]
const USIZE_MAX: usize = 18_446_744_073_709_551_615usize;

fn main() {
    input! {
        n: usize,
        mut plan: [(i32, i32, i32); n],
    }
    plan.insert(0, (0, 0, 0));
    let yes = plan.windows(2).all(|w| {
        let (t0, x0, y0) = w[0];
        let (t1, x1, y1) = w[1];
        let time = t1 - t0;
        let dist = (x1 - x0).abs() + (y1 - y0).abs();
        dist <= time && time % 2 == dist % 2
    });
    println!("{}", if yes { "Yes" } else { "No" });
}
