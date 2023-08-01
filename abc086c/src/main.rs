use proconio::input;

fn main() {
    input! {
        n: usize
    };
    let mut t: isize = 0;
    let mut x: isize = 0;
    let mut y: isize = 0;
    for _ in 0..n {
        input! {
            _t: isize,
            _x: isize,
            _y: isize
        };
        if (_t + _x + _y) % 2 != 0 {
            println!("No");
            return;
        }
        if _t - t < abs(_x - x) + abs(_y - y) {
            println!("No");
            return;
        }
        t = _t;
        x = _x;
        y = _y;
    }
    println!("Yes");
}

fn abs(x: isize) -> isize {
    if x < 0 { -x } else { x }
}