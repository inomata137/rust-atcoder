use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let a: Vec<usize> = a.iter().map(|x: &usize| { x - 1 }).collect();
    let mut visited = vec![false; n];
    let mut root = 0;
    while !visited[root] {
        visited[root] = true;
        root = a[root];
    }
    let mut seq = vec![root];
    let mut cur = root;
    while a[cur] != root {
        seq.push(a[cur]);
        cur = a[cur];
    }

    println!("{}", seq.len());
    let seq = seq.iter().map(|x| { (x + 1).to_string() }).collect::<Vec<String>>().join(" ");
    println!("{}", seq);
}
