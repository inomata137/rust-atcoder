use proconio::input;

fn main() {
    input! {
        s: String
    }
    let candidate = vec![
        "ACE".to_string(),
        "BDF".to_string(),
        "CEG".to_string(),
        "DFA".to_string(),
        "EGB".to_string(),
        "FAC".to_string(),
        "GBD".to_string(),
    ];
    let yes = candidate.contains(&s);
    println!("{}", if yes { "Yes" } else {"No"});
}
