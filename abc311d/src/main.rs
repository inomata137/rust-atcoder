use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        field: [Chars; n]
    }
    let field: Vec<Vec<char>> = field;

    let mut stoppable = vec![vec![false; m]; n];
    let mut touchable = vec![vec![false; m]; n];

    stoppable[1][1] = true;
    touchable[1][1] = true;

    let mut pool = VecDeque::new();
    pool.push_back((1, 1));

    while let Some((i, j)) = pool.pop_back() {
        'left: for _j in (1..j).rev() {
            if field[i][_j] == '#' {
                break 'left;
            }
            touchable[i][_j] = true;
            if field[i][_j - 1] == '#' && !stoppable[i][_j] {
                stoppable[i][_j] = true;
                pool.push_back((i, _j));
                break 'left;
            }
        }
        'right: for _j in j..m {
            if field[i][_j] == '#' {
                break 'right;
            }
            touchable[i][_j] = true;
            if field[i][_j + 1] == '#' && !stoppable[i][_j] {
                stoppable[i][_j] = true;
                pool.push_back((i, _j));
                break 'right;
            }
        }
        'up: for _i in (1..i).rev() {
            if field[_i][j] == '#' {
                break 'up;
            }
            touchable[_i][j] = true;
            if field[_i - 1][j] == '#' && !stoppable[_i][j] {
                stoppable[_i][j] = true;
                pool.push_back((_i, j));
                break 'up;
            }
        }
        'down: for _i in i..n {
            if field[_i][j] == '#' {
                break 'down;
            }
            touchable[_i][j] = true;
            if field[_i + 1][j] == '#' && !stoppable[_i][j] {
                stoppable[_i][j] = true;
                pool.push_back((_i, j));
                break 'down;
            }
        }
    }

    let mut ans = 0;
    for row in &touchable {
        ans += row.iter().filter(|x| **x).count();
    }
    println!("{}", ans);
}
