use std::collections::HashSet;

use proconio::input;
#[allow(unused)]
fn main() {
    input! {
        n: usize,
    }

    let mut space = vec![vec![vec![0usize; 100]; 100]; 100];
    let mut hs = HashSet::new();

    for i in 0..n {
        input! {
            cuboid: [usize; 6]
        }
        for x in cuboid[0]..cuboid[3] {
            for y in cuboid[1]..cuboid[4] {
                for z in cuboid[2]..cuboid[5] {
                    space[x][y][z] = i + 1;
                }
            }
        }
    }

    for x in 0..99 {
        for y in 0..100 {
            for z in 0..100 {
                let a = space[x][y][z];
                let b = space[x + 1][y][z];
                if a != 0 && b != 0 && a != b {
                    hs.insert((a, b));
                }
                let a = space[z][x][y];
                let b = space[z][x + 1][y];
                if a != 0 && b != 0 && a != b {
                    hs.insert((a, b));
                }
                let a = space[y][z][x];
                let b = space[y][z][x + 1];
                if a != 0 && b != 0 && a != b {
                    hs.insert((a, b));
                }
            }
        }
    }

    let mut ans = vec![0; n];

    for (a, b) in hs {
        ans[a - 1] += 1;
        ans[b - 1] += 1;
    }

    for a in &ans {
        println!("{}", a);
    }
}
