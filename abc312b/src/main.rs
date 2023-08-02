use proconio::input;

fn main() {
    input! {
        // 縦n 横m
        n: usize,
        m: usize,
    }
    let mut s: Vec<Vec<char>> = vec![];
    for _ in 0..n {
        input! {
            _s: String
        }
        let mut v = vec![];
        for c in _s.as_bytes() {
            v.push((*c) as char);
        }
        s.push(v);
    }
    for ni in 0..=n-9 {
        for mi in 0..=m-9 {
            // println!("{} {} {}", ni, mi, check(&s, ni, mi));
            if check(&s, ni, mi) {
                println!("{} {}", ni + 1, mi + 1);
            }
        }
    }
}

fn check(s: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    for _i in 0..3 {
        for _j in 0..3 {
            if s[i + _i][j + _j] != '#' {
                return false;
            }
            if s[i + 8 - _i][j + 8 - _j] != '#' {
                return false;
            }
        }
    }
    for _i in 0..3 {
        if s[i + _i][j + 3] != '.' {
            return false;
        }
        if s[i + 8 - _i][j + 5] != '.' {
            return false;
        }
        if s[i + 3][j + _i] != '.' {
            return false;
        }
        if s[i + 5][j + 8 - _i] != '.' {
            return false;
        }
    }
    return true;
}