use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }
    let mut holes = vec![vec![false; w]; h];
    for _ in 0..n {
        input! {
            i: usize,
            j: usize
        }
        holes[i][j] = true;
    }

    let mut dp = vec![vec![0; w]; h];
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if i == h - 1 || j == w - 1 {
                dp[i][j] = if holes[i][j] { 0 } else { 1 };
                continue;
            }
            if holes[i][j] {
                continue;
            }
            dp[i][j] = dp[i + 1][j].min(dp[i][j + 1]).min(dp[i + 1][j + 1]) + 1;
        }
    }

    let mut ans = 0_u64;
    for row in dp {
        for cell in row {
            ans += cell;
        }
    }

    println!("{}", ans);
}
