use proconio::input;
use proconio::marker::Chars;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }
    let s: Vec<char> = s;
    let mut bh = BinaryHeap::new();
    let mut offset = 0;
    for i in 0..n {
        bh.push(Reverse((s[i], i)));
        if i >= n - k {
            while let Some(Reverse((ch, i))) = bh.pop() {
                if i < offset {
                    continue;
                }
                print!("{}", ch);
                offset = i;
                break;
            }
        }
    }
    println!("");
}
