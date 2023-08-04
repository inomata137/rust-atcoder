use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String
    }
    let mut a = false;
    let mut b = false;
    let mut c = false;
    let s: Vec<char> = s.chars().collect();
    for i in 1..=n {
        match s[i - 1] {
            'A' => a = true,
            'B' => b = true,
            'C' => c = true,
            _ => {}
        }
        if a && b && c {
            println!("{}", i);
            return;
        }
    }
}
