use serde_json::Value;
use std::env;
mod copy;
mod json;

#[allow(unused)]
fn main() {
    let path = "./.vscode/settings.json";
    let field = "rust-analyzer.linkedProjects";

    let name = env::args()
        .nth(1)
        .expect("At least 1 argument is required. example: ./init abc000a");

    copy::copy(&name);

    let mut obj: Value = json::read(path);

    if let Value::Array(arr) = &mut obj[field] {
        let project = Value::String(name + "/Cargo.toml");
        if !arr.contains(&project) {
            arr.push(project);
        }
    }

    json::write(path, &obj);
}
