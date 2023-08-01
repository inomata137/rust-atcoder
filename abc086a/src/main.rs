use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let prod = a * b;
    let flg = if prod % 2 == 0 { "Even" } else { "Odd" };
    println!("{}", flg);
}
