#[allow(unused)]
use proconio::{input, marker::{Chars, Usize1}};
#[allow(unused)]
use std::collections::*;
#[allow(unused)]
const USIZE_MAX: usize = 18_446_744_073_709_551_615usize;

fn main() {
    input! {
        n: usize,
        m: usize,
        roads: [(Usize1, Usize1, usize); m],
    }
    let roads: Vec<(usize, usize, usize)> = roads;
    let mut dest: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for (a, b, c) in roads {
        dest[a].push((b, c));
        dest[b].push((a, c));
    }

    let mut from_1 = vec![USIZE_MAX; n];
    let mut to_n = vec![USIZE_MAX; n];

    fill(&dest, &mut from_1, 0);
    fill(&dest, &mut to_n, n - 1);

    for i in 0..n {
        println!("{}", from_1[i] + to_n[i]);
    }
}

fn fill(dest: &Vec<Vec<(usize, usize)>>, target: &mut Vec<usize>, start: usize) {
    let mut pool = VecDeque::new();
    pool.push_back(start);
    target[start] = 0;
    while let Some(pivot) = pool.pop_front() {
        for (b, c) in &dest[pivot] {
            if target[*b] <= target[pivot] + c {
                continue;
            }
            target[*b] = target[pivot] + c;
            pool.push_back(*b);
        }
    }
}